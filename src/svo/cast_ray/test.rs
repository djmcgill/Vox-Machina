use svo::*;
use nalgebra::{ApproxEq, Vector3};
use quickcheck::*;

#[test]
fn ray_casting() {
    let svo = SVO::floor();

    let hit1 = svo.cast_ray(Vector3::new(0.5, 2., 0.5), Vector3::new(0., -1., 0.));
    assert_approx_eq_eps!(hit1.unwrap(), Vector3::new(0.5, 0.5, 0.5), 0.01);

    let hit2 = svo.cast_ray(Vector3::new(-3., 0.25, 0.5), Vector3::new(1., 0., 0.));
    assert_approx_eq_eps!(hit2.unwrap(), Vector3::new(0., 0.25, 0.5), 0.01);

    let hit3 = svo.cast_ray(Vector3::new(5., 5., 0.25), Vector3::new(-1., -1., 0.));
    assert_approx_eq_eps!(hit3.unwrap(), Vector3::new(0.5, 0.5, 0.25), 0.01);

    let hit4 = svo.cast_ray(Vector3::new(0.75, 0.6, 0.25), Vector3::new(-1., -1., 0.1));
    assert_approx_eq_eps!(hit4.unwrap(), Vector3::new(0.65, 0.5, 0.26), 0.01);

    let no_hit1 = svo.cast_ray(Vector3::new(2., 0.6, 2.), Vector3::new(-0.006, 0., -0.006));
    assert!(no_hit1.is_none());
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
