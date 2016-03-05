#![allow(dead_code)]

#[macro_use] extern crate lazy_static;
extern crate nalgebra;
mod svo;

mod unity_interface_enums;
mod unity_interface_guid;
pub mod carved_rust;
pub mod unity_interfaces;

#[test]
fn it_works() {
}

#[test]
fn main_works() {
	use svo::SVO;
	use nalgebra::Vec3;

    let mut svo = SVO::floor();
    println!("{:?}", &svo);
    svo.set_block_and_recombine(&[1, 3], 2);
    println!("{:?}", &svo); 
    svo.set_block_and_recombine(&[1, 3], 1);
    println!("{:?}", &svo);

    let hit1 = svo.cast_ray(Vec3::new(0.5, 2., 0.5), Vec3::new(0., -1., 0.));
    let hit2 = svo.cast_ray(Vec3::new(-3., 0.25, 0.5), Vec3::new(1., 0., 0.));
    println!("{:?}", hit1);
    println!("{:?}", hit2);
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