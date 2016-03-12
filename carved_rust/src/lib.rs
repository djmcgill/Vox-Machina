#![allow(dead_code)]

//#[macro_use] extern crate log;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nalgebra;
mod svo;

pub mod carved_rust;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<(f32, f32, f32, i32, i32)>> = Mutex::new(Vec::new());
}

#[test]
fn on_blocks() {
    use nalgebra::ApproxEq;
    use svo::SVO;

    let mut svo = SVO::floor();

    svo.on_voxels(test_function_pointer);

    {
        let ref results: Vec<(f32, f32, f32, i32, i32)> = *ARRAY.lock().unwrap();
        let (x, y, z, depth, voxel_type) = results[0];
        assert_approx_eq_eps!(x, 0., 0.01);
        assert_approx_eq_eps!(y, 0., 0.01);
        assert_approx_eq_eps!(z, 0., 0.01);
        assert_eq!(2, depth);
        assert_eq!(1, voxel_type);

        let (x, y, z, depth, voxel_type) = results[1];
        assert_approx_eq_eps!(x, 0.5, 0.01);
        assert_approx_eq_eps!(y, 0., 0.01);
        assert_approx_eq_eps!(z, 0., 0.01);
        assert_eq!(2, depth);
        assert_eq!(1, voxel_type);

        let (x, y, z, depth, voxel_type) = results[2];
        assert_approx_eq_eps!(x, 0., 0.01);
        assert_approx_eq_eps!(y, 0.5, 0.01);
        assert_approx_eq_eps!(z, 0., 0.01);
        assert_eq!(2, depth);
        assert_eq!(0, voxel_type);

        let (x, y, z, depth, voxel_type) = results[3];
        assert_approx_eq_eps!(x, 0.5, 0.01);
        assert_approx_eq_eps!(y, 0.5, 0.01);
        assert_approx_eq_eps!(z, 0., 0.01);
        assert_eq!(2, depth);
        assert_eq!(0, voxel_type);

        let (x, y, z, depth, voxel_type) = results[4];
        assert_approx_eq_eps!(x, 0., 0.01);
        assert_approx_eq_eps!(y, 0., 0.01);
        assert_approx_eq_eps!(z, 0.5, 0.01);
        assert_eq!(2, depth);
        assert_eq!(1, voxel_type);

        let (x, y, z, depth, voxel_type) = results[5];
        assert_approx_eq_eps!(x, 0.5, 0.01);
        assert_approx_eq_eps!(y, 0., 0.01);
        assert_approx_eq_eps!(z, 0.5, 0.01);
        assert_eq!(2, depth);
        assert_eq!(1, voxel_type);

        let (x, y, z, depth, voxel_type) = results[6];
        assert_approx_eq_eps!(x, 0., 0.01);
        assert_approx_eq_eps!(y, 0.5, 0.01);
        assert_approx_eq_eps!(z, 0.5, 0.01);
        assert_eq!(2, depth);
        assert_eq!(0, voxel_type);

        let (x, y, z, depth, voxel_type) = results[7];
        assert_approx_eq_eps!(x, 0.5, 0.01);
        assert_approx_eq_eps!(y, 0.5, 0.01);
        assert_approx_eq_eps!(z, 0.5, 0.01);
        assert_eq!(2, depth);
        assert_eq!(0, voxel_type);
    }

    *ARRAY.lock().unwrap() = vec![];



}

fn test_function_pointer(x: f32, y: f32, z: f32, depth: i32, voxel_type: i32) {
    ARRAY.lock().unwrap().push((x, y, z, depth, voxel_type));
}

#[test]
fn setting_blocks() {
    use svo::SVO;

    let mut svo = SVO::floor();
    println!("{:?}", &svo);
    svo.set_block_and_recombine(&[1, 3], 2);
    println!("{:?}", &svo); 
    svo.set_block_and_recombine(&[1, 3], 1);
    println!("{:?}", &svo);
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