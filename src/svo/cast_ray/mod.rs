use nalgebra::{Vector3, Norm};
use svo::*;

#[cfg(test)]
mod test;

const EPS: f32 = 0.001;
const MIN_DIST: f32 = 0.;
const MAX_DIST: f32 = 100000.;

impl SVO {    
    // Cast a ray into the octree and return the position of collision with a non-type-zero voxel (if any).
    // x = t*d + o where t = length of ray.
    // t = (x-o)/d
    // BUT we know that the hit will have be on the boundary on the cube.
    // So for each axis independently, work out the length to hit 0. and 1.
    pub fn cast_ray(&self, ray_origin: Vector3<f32>, ray_dir: Vector3<f32>) -> Option<Vector3<f32>> {
        let sanitised_dir = map_vec_3(ray_dir.normalize(), &sanitise);
        let flip_mask = map_vec_3(sanitised_dir, &|f| f.signum() < 0.0);
        let flipped_origin = flip(ray_origin, flip_mask);
        let flipped_dir = flip(sanitised_dir, flip_mask);

        let inv_dir = Vector3::new(1., 1., 1.)/flipped_dir;

        self.cast_ray_sanitised(flip_mask, flipped_origin, flipped_dir, inv_dir).map(|v| flip (v, flip_mask))
    }

    fn cast_ray_sanitised(&self, flip_mask: Vector3<bool>, ray_origin: Vector3<f32>, ray_dir: Vector3<f32>, inv_ray_dir: Vector3<f32>) -> Option<Vector3<f32>> {
        println!("flip_mask {:?}", flip_mask);
        println!("ray_origin {:?}", ray_origin);
        println!("ray_dir {:?}", ray_dir);
        println!("inv_ray_dir {:?}", inv_ray_dir);
        
        // TODO: can simplify this by mirroring the ray direction so they all point the same way
        let (t_min_x, t_max_x) = sorted_ts(ray_origin.x, inv_ray_dir.x);
        let (t_min_y, t_max_y) = sorted_ts(ray_origin.y, inv_ray_dir.y);
        let (t_min_z, t_max_z) = sorted_ts(ray_origin.z, inv_ray_dir.z);
        let t_min = fold_arr_4([t_min_x, t_min_y, t_min_z, MIN_DIST], &f32::max);
        let t_max = fold_arr_4([t_max_x, t_max_y, t_max_z, MAX_DIST], &f32::min);
        if t_min > t_max {println!("miss"); return None};
        let hit_position = ray_dir * t_min + ray_origin;
        match *self {
            SVO::Voxel { data: VoxelData{ voxel_type, .. }, .. } if voxel_type == 0 => {println!("hit air");;None},
            SVO::Voxel { .. } => {println!("hit"); Some(hit_position)},
            SVO::Octants(ref octants) => {
                // work out which voxels are hit in turn, and if they're solid or not
                // TODO: rather than trying dumbly, we could instead calculate which child is hit. Compare speeds?
                let children : [(bool, bool, bool); 8] = {
                    let above_x = hit_position.x > 0.5;
                    let above_y = hit_position.y > 0.5;
                    let above_z = hit_position.z > 0.5;
                    let x = if !flip_mask.x { above_x } else { !above_x };
                    let y = if !flip_mask.y { above_y } else { !above_y };
                    let z = if !flip_mask.z { above_z } else { !above_z };
                    [ (x, y, z),
                      (!x, y, z), (x, !y, z), (x, y, !z),
                      (x, !y, !z), (!x, y, !z), (!x, !y, z),
                      (!x, !y, !z)]
                };

                // TODO: stop throwing away the hit position between iterations - if it's on the "near" edge
                //       then it's the same as for the nearer children
                let test_child = |(above_x, above_y, above_z): (bool, bool, bool)| -> Option<Vector3<f32>> {
                    println!("testing {:?}{:?}{:?}", above_x, above_y, above_z);
                    let (above_x, above_y, above_z) = (above_x as usize, above_y as usize, above_z as usize);
                    let child_ix = above_x | (above_y<<1) | (above_z<<2);
                    let above_center = Vector3::new(above_x as f32, above_y as f32, above_z as f32);
                    let new_origin = to_child_space(ray_origin, above_center);
                    octants[child_ix].cast_ray_sanitised(flip_mask, new_origin, ray_dir, inv_ray_dir).map (|child_hit: Vector3<f32>| {
                        from_child_space(child_hit, above_center)
                    })
                };

                children.iter().cloned().map(test_child).find(|x| x.is_some()).and_then(|x| x)
            }
        }
    }
}

// TODO: test these specifically
fn to_child_space(vec: Vector3<f32>, offsets: Vector3<f32>) -> Vector3<f32> {
    (vec - offsets*0.5)*2.
}

// TODO: test these specifically
fn from_child_space(vec: Vector3<f32>, offsets: Vector3<f32>) -> Vector3<f32> {
    vec*0.5 + offsets*0.5
}

fn sorted_ts(ray_origin: f32, inv_ray_dir: f32) -> (f32, f32) {
    let t1 = (0.-ray_origin) * inv_ray_dir;
    let t2 = (1.-ray_origin) * inv_ray_dir;
    if t1 < t2 { (t1, t2) } else { (t2, t1) }
}

fn sanitise(f: f32) -> f32 {
    if f.abs() < EPS {EPS * f.signum()} else {f}
}

fn fold_arr_4<A: Copy>(arr: [A; 4], f: &Fn(A, A) -> A) -> A {
    f(f(f(arr[0], arr[1]), arr[2]), arr[3])
}

fn map_vec_3<A, B>(v: Vector3<A>, f: &Fn(A) -> B) -> Vector3<B> {
    Vector3::new(f(v.x), f(v.y), f(v.z))
}

fn flip(v: Vector3<f32>, dir_sig: Vector3<bool>) -> Vector3<f32> {
    let mut ret = v;
    ret -= 0.5;
    if !dir_sig.x { ret.x *= 1.0 }
    if !dir_sig.y { ret.y *= 1.0 }
    if !dir_sig.z { ret.z *= 1.0 }
    ret + 0.5
}
