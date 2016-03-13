
use carved_rust;
use nalgebra::Vec3;
use std::sync::Mutex;
use svo::*;

lazy_static! {
    static ref ARRAY: Mutex<Vec<(f32, f32, f32, i32, i32)>> = Mutex::new(Vec::new());
}

#[test]
fn on_blocks() {
    let svo = SVO::floor();
    assert_contains(&svo, vec![
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
fn setting_blocks() {
    let mut svo = SVO::floor();

    svo.set_block_and_recombine(&[1, 3], 2);
    assert_contains(&svo, vec![
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

    svo.set_block_and_recombine(&[1, 3], 1);
    assert_contains(&svo, vec![
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
	use svo::SVO;
	use nalgebra::{ApproxEq, Vec3};
    
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

#[test]
fn test_ffi() {
	let ptr = carved_rust::svo_create(1);
	let block_type_1 = carved_rust::svo_get_voxel_type(ptr);
	assert!(block_type_1 == 1);
	carved_rust::svo_set_voxel_type(ptr, 2);
	let block_type_2 = carved_rust::svo_get_voxel_type(ptr);
	assert!(block_type_2 == 2);
	carved_rust::svo_destroy(ptr);
}

fn assert_contains(svo: &SVO, expected: Vec<(f32, f32, f32, i32, i32)>) {
    use nalgebra::ApproxEq;

    svo.on_voxels(test_function_pointer); 
    let ref mut results = *ARRAY.lock().unwrap();
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

    *results = vec![];
}

fn test_function_pointer(vec: Vec3<f32>, depth: i32, voxel_type: i32) {
    ARRAY.lock().unwrap().push((vec.x, vec.y, vec.z, depth, voxel_type));
}