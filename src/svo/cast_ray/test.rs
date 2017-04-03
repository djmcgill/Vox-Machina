use nalgebra::{ApproxEq, Vector3};
use quickcheck::*;
use super::*;

#[test]
fn ray_casting() {
    let svo = SVO::floor();

    // let hit1 = svo.cast_ray(Vector3::new(0.5, 2., 0.5), Vector3::new(0., -1., 0.));
    // assert_approx_eq_eps!(hit1.unwrap(), Vector3::new(0.5, 0.5, 0.5), 0.01);

    // let hit2 = svo.cast_ray(Vector3::new(-3., 0.25, 0.5), Vector3::new(1., 0., 0.));
    // assert_approx_eq_eps!(hit2.unwrap(), Vector3::new(0., 0.25, 0.5), 0.01);

    // let hit3 = svo.cast_ray(Vector3::new(5., 5., 0.25), Vector3::new(-1., -1., 0.));
    // assert_approx_eq_eps!(hit3.unwrap(), Vector3::new(0.5, 0.5, 0.25), 0.01);

    let hit4 = svo.cast_ray(Vector3::new(0.75, 0.6, 0.25), Vector3::new(-1., -1., 0.1));
    assert_approx_eq_eps!(hit4.unwrap(), Vector3::new(0.65, 0.5, 0.26), 0.01);

    // let no_hit1 = svo.cast_ray(Vector3::new(2., 0.6, 2.), Vector3::new(-0.006, 0., -0.006));
    // assert!(no_hit1.is_none());
}

#[test]
fn same_as_old_results() {
    fn same_as_old_results_inner(svo: SVO, origin_tuple: (f32, f32, f32), dir_tuple: (f32, f32, f32)) -> bool {
        let origin = Vector3::new(origin_tuple.0.abs(), origin_tuple.1.abs(), origin_tuple.2.abs());
        let dir = Vector3::new(-dir_tuple.0.abs(), -dir_tuple.1.abs(), -dir_tuple.2.abs());
        svo.cast_ray(origin, dir) == svo.cast_ray_old(origin, dir)
    }
    quickcheck(same_as_old_results_inner as fn(SVO, (f32, f32, f32), (f32, f32, f32)) -> bool)
}

use nalgebra::Iterable;
impl SVO {
        // Cast a ray into the octree and return the position of collision with a non-type-zero voxel (if any).
    // x = t*d + o where t = length of ray.
    // t = (x-o)/d
    // BUT we know that the hit will have be on the boundary on the cube.
    // So for each axis independently, work out the length to hit 0. and 1.
    pub fn cast_ray_old(&self, ray_origin: Vector3<f32>, ray_dir: Vector3<f32>) -> Option<Vector3<f32>> {
        let sanitise = |f: f32| if f.abs() < EPS {EPS * f.signum()} else {f};
        let sanitised_dir = ray_dir.normalize().iter().cloned().map(sanitise).collect();
        let inv_dir = Vector3::new(1., 1., 1.)/sanitised_dir;
        self.cast_ray_old_sanitised(ray_origin, sanitised_dir, inv_dir)
    }

    fn cast_ray_old_sanitised(&self, ray_origin: Vector3<f32>, ray_dir: Vector3<f32>, inv_ray_dir: Vector3<f32>) -> Option<Vector3<f32>> {
        // TODO: can simplify this by mirroring the ray direction so they all point the same way
        let (t_min_x, t_max_x) = sorted_ts(ray_origin.x, inv_ray_dir.x);
        let (t_min_y, t_max_y) = sorted_ts(ray_origin.y, inv_ray_dir.y);
        let (t_min_z, t_max_z) = sorted_ts(ray_origin.z, inv_ray_dir.z);
        let t_min = [t_min_x, t_min_y, t_min_z].iter().cloned().fold(MIN_DIST, f32::max);
        let t_max = [t_max_x, t_max_y, t_max_z].iter().cloned().fold(MAX_DIST, f32::min);
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
                let test_child = |(above_x, above_y, above_z): (bool, bool, bool)| -> Option<Vector3<f32>> {
                    let (above_x, above_y, above_z) = (above_x as usize, above_y as usize, above_z as usize);
                    let child_ix = above_x | (above_y<<1) | (above_z<<2);
                    let above_center = Vector3::new(above_x as f32, above_y as f32, above_z as f32);
                    let new_origin = to_child_space(ray_origin, above_center);
                    octants[child_ix].cast_ray_old_sanitised(new_origin, ray_dir, inv_ray_dir).map (|child_hit: Vector3<f32>| {
                        from_child_space(child_hit, above_center)
                    })
                };

                children.iter().cloned().map(test_child).find(|x| x.is_some()).and_then(|x| x)
            }
        }
    }
}
