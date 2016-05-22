use nalgebra::{Vec3, Norm, Iterable};
use svo::*;

#[cfg(test)]
mod tests;

impl<R: RegistrationTrait> SVO<R> {
    // Cast a ray into the octree and return the position of collision with a non-type-zero voxel (if any).
    // x = t*d + o where t = length of ray.
    // t = (x-o)/d
    // BUT we know that the hit will have be on the boundary on the cube.
    // So for each axis independently, work out the length to hit 0. and 1.
    pub fn cast_ray(&self, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> Option<Vec3<f32>> {
        let eps = 0.001;
        let sanitise = |f: f32| if f.abs() < eps {eps * f.signum()} else {f};
        let sanitised_dir = ray_dir.normalize().iter().cloned().map(sanitise).collect();
        let inv_dir = Vec3::new(1., 1., 1.)/sanitised_dir;
        self.cast_ray_sanitised(ray_origin, sanitised_dir, inv_dir)
    }

    fn cast_ray_sanitised(&self, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>, inv_ray_dir: Vec3<f32>) -> Option<Vec3<f32>> {
        let min_dist = 0.;
        let max_dist = 100000.;
        // TODO: can simplify this by mirroring the ray direction so they all point the same way
        let (t_min_x, t_max_x) = sorted_ts(ray_origin.x, inv_ray_dir.x);
        let (t_min_y, t_max_y) = sorted_ts(ray_origin.y, inv_ray_dir.y);
        let (t_min_z, t_max_z) = sorted_ts(ray_origin.z, inv_ray_dir.z);
        let t_min = [t_min_x, t_min_y, t_min_z].iter().cloned().fold(min_dist, f32::max);
        let t_max = [t_max_x, t_max_y, t_max_z].iter().cloned().fold(max_dist, f32::min);
        if t_min > t_max {return None};
        let hit_position = ray_dir * t_min + ray_origin;
        match *self {
            SVO::Voxel { data: VoxelData{ voxel_type, .. }, .. } if voxel_type == 0 => None,
            SVO::Voxel { .. } => Some(hit_position),
            SVO::Octants(ref octants) => {
                // work out which voxels are hit in turn, and if they're solid or not
                // TODO: rather than trying dumbly, we could instead calculate which child is hit. Compare speeds?
                let children : [(bool, bool, bool); 8] = {
                    let x = hit_position.x > 0.5;
                    let y = hit_position.y > 0.5;
                    let z = hit_position.z > 0.5;
                    [ (x, y, z),
                      (!x, y, z), (x, !y, z), (x, y, !z),
                      (x, !y, !z), (!x, y, !z), (!x, !y, z),
                      (!x, !y, !z)]
                };

                // TODO: stop throwing away the hit position between iterations - if it's on the "near" edge
                //       then it's the same as for the nearer children
                let test_child = |(above_x, above_y, above_z): (bool, bool, bool)| -> Option<Vec3<f32>> {
                    let (above_x, above_y, above_z) = (above_x as usize, above_y as usize, above_z as usize);
                    let child_ix = above_x | (above_y<<1) | (above_z<<2);
                    let above_center = Vec3::new(above_x as f32, above_y as f32, above_z as f32);
                    let new_origin = to_child_space(ray_origin, above_center);
                    octants[child_ix].cast_ray_sanitised(new_origin, ray_dir, inv_ray_dir).map (|child_hit: Vec3<f32>| {
                        from_child_space(child_hit, above_center)
                    })
                };

                children.iter().cloned().map(test_child).find(|x| x.is_some()).and_then(|x| x)
            }
        }
    }
}

// TODO: test these specifically
fn to_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    (vec - offsets*0.5)*2.
}

// TODO: test these specifically
fn from_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    vec*0.5 + offsets*0.5
}

fn sorted_ts(ray_origin: f32, inv_ray_dir: f32) -> (f32, f32) {
    let t1 = (0.-ray_origin) * inv_ray_dir;
    let t2 = (1.-ray_origin) * inv_ray_dir;
    if t1 < t2 { (t1, t2) } else { (t2, t1) }
}
