use carved_rust;
use nalgebra::{ApproxEq, Vec3, zero};
use std::cell::RefCell;
use svo::*;

fn register(_: Vec3<f32>, _: i32, _: i32) -> u32 { 0 }
fn deregister(_: u32) {}

// === SVO tests ====
#[test]
fn on_blocks() {
    let svo = SVO::floor();
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);
}

#[test]
fn minimal_subdivide() {
    let mut svo = SVO::new_voxel(1, 0);
    svo.set_block(&deregister, &register, &[1], 0);

    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 0),
        (0. , 0.5, 0. , 1, 1),
        (0.5, 0.5, 0. , 1, 1),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 1),
        (0.5, 0.5, 0.5, 1, 1)]);
}

#[test]
fn setting_blocks() {
    let mut svo = SVO::floor();

    svo.set_block(&deregister, &register, &[1, 3], 2);
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
            (0.5 , 0.  , 0.  , 2, 1),
            (0.75, 0.  , 0.  , 2, 1),
            (0.5 , 0.25, 0.  , 2, 1),
            (0.75, 0.25, 0.  , 2, 2),
            (0.5 , 0.  , 0.25, 2, 1),
            (0.75, 0.  , 0.25, 2, 1),
            (0.5 , 0.25, 0.25, 2, 1),
            (0.75, 0.25, 0.25, 2, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);

    svo.set_block(&deregister, &register, &[1, 3], 1);
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);
}

#[test]
fn ray_casting() {
    let svo = SVO::floor();

    let hit1 = svo.cast_ray(Vec3::new(0.5, 2., 0.5), Vec3::new(0., -1., 0.));
    assert_approx_eq_eps!(hit1.unwrap(), Vec3::new(0.5, 0.5, 0.5), 0.01);

    let hit2 = svo.cast_ray(Vec3::new(-3., 0.25, 0.5), Vec3::new(1., 0., 0.));
    assert_approx_eq_eps!(hit2.unwrap(), Vec3::new(0., 0.25, 0.5), 0.01);

    let hit3 = svo.cast_ray(Vec3::new(5., 5., 0.25), Vec3::new(-1., -1., 0.));
    assert_approx_eq_eps!(hit3.unwrap(), Vec3::new(0.5, 0.5, 0.25), 0.01);

    let hit4 = svo.cast_ray(Vec3::new(0.75, 0.6, 0.25), Vec3::new(-1., -1., 0.1));
    assert_approx_eq_eps!(hit4.unwrap(), Vec3::new(0.65, 0.5, 0.26), 0.01);

    let no_hit1 = svo.cast_ray(Vec3::new(2., 0.6, 2.), Vec3::new(-0.006, 0., -0.006));
    assert!(no_hit1.is_none());
}

// #[test]
// fn test_ffi() {
//  let ptr = carved_rust::svo_create(1);
//  let block_type_1 = carved_rust::svo_get_voxel_type(ptr);
//  assert!(block_type_1 == 1);
//  carved_rust::svo_set_voxel_type(ptr, 2);
//  let block_type_2 = carved_rust::svo_get_voxel_type(ptr);
//  assert!(block_type_2 == 2);
//  carved_rust::svo_destroy(ptr);
// }



impl SVO {
    fn assert_contains(&self, expected: Vec<(f32, f32, f32, i32, i32)>) {
        let results_vec: RefCell<Vec<(f32, f32, f32, i32, i32)>> = RefCell::new(Vec::new());
        self.collect_svo(&results_vec, zero(), 0);
        let results = results_vec.into_inner();

        assert_eq!(results.len(), expected.len());

        for (actual_element, expected_element) in results.iter().zip(expected.iter()) {
            let &(x, y, z, depth, voxel_type) = actual_element;
            let &(x_, y_, z_, depth_, voxel_type_) = expected_element;

            assert_approx_eq_eps!(x, x_, 0.01);
            assert_approx_eq_eps!(y, y_, 0.01);
            assert_approx_eq_eps!(z, z_, 0.01);
            assert_eq!(depth, depth_);
            assert_eq!(voxel_type, voxel_type_);
        }
    }

    fn collect_svo(&self, results_vec: &RefCell<Vec<(f32, f32, f32, i32, i32)>>, origin: Vec3<f32>, depth: i32) {
        match *self {
            SVO::Voxel { voxel_type, .. } => results_vec.borrow_mut().push((origin.x, origin.y, origin.z, depth, voxel_type)),
            SVO::Octants (ref octants) => {
                for ix in 0..8 {
                    let new_origin = origin + offset(ix, depth);
                    octants[ix as usize].collect_svo(results_vec, new_origin, depth + 1);
                }
            }

        }
    }

    fn floor() -> SVO {
        let mut svo = SVO::new_voxel(1, 0);
        svo.set_block(&deregister, &register, &[2], 0);
        svo.set_block(&deregister, &register, &[3], 0);
        svo.set_block(&deregister, &register, &[6], 0);
        svo.set_block(&deregister, &register, &[7], 0);
        svo
    }

}

// === FFI tests ===
// #[test]
// fn ff_integration() {
//     let svo_ptr = carved_rust::svo_create(1);

//     let index = &[1u8];
//     carved_rust::svo_set_block(svo_ptr, index.as_ptr(), index.len(), 2);

//     // I can't actually (be bothered to) produce a extern "stdcall" fn(Vec3<f32>, i32, i32)
//     // so cheat and replicate the code from carved_rust::on_voxels. Luckily (by design) it's a shallow wrapper.
//     {
//         let svo_ref: &SVO = unsafe { &*svo_ptr };
//         svo_ref.assert_contains(vec![
//             (0. , 0. , 0. , 1, 1),
//             (0.5, 0. , 0. , 1, 2),
//             (0. , 0.5, 0. , 1, 1),
//             (0.5, 0.5, 0. , 1, 1),
//             (0. , 0. , 0.5, 1, 1),
//             (0.5, 0. , 0.5, 1, 1),
//             (0. , 0.5, 0.5, 1, 1),
//             (0.5, 0.5, 0.5, 1, 1)]);
//     }

//     let maybe_hit = carved_rust::svo_cast_ray(svo_ptr, Vec3::new(0.52, 2., 0.52), Vec3::new(0., -1., 0.));
//     assert!(maybe_hit.is_some != 0);
//     assert_approx_eq_eps!(maybe_hit.value, Vec3::new(0.52, 1., 0.52), 0.001);

//     let maybe_hit2 = carved_rust::svo_cast_ray(svo_ptr, Vec3::new(1.52, 2., 0.52), Vec3::new(0., -1., 0.));
//     assert!(maybe_hit2.is_some == 0);

//     carved_rust::svo_destroy(svo_ptr);
// }

