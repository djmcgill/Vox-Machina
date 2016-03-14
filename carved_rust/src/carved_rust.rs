use svo::SVO;

use std::mem::transmute;
use nalgebra::{Vec3, zero};

// FFI INTERFACE
#[no_mangle]
pub extern "stdcall" fn svo_create(voxel_type: i32) -> *mut SVO {
	println!("svo_create with {}", voxel_type);
	unsafe { transmute(Box::new(SVO::new_voxel(voxel_type))) }
}

#[no_mangle]
pub extern "stdcall" fn svo_destroy(svo_ptr: *mut SVO) {
	println!("svo_destroy with {:?}", svo_ptr);
	let _svo: Box<SVO> = unsafe { transmute(svo_ptr) };
}

#[no_mangle]
pub extern "stdcall" fn svo_on_voxels(svo_ptr: *const SVO, on_voxel: fn(Vec3<f32>, i32, i32)) {
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	svo_ref.on_voxels(on_voxel);
}

#[no_mangle]
pub extern "stdcall" fn svo_cast_ray(svo_ptr: *const SVO, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> BadOption<Vec3<f32>> {
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	let maybe_hit = svo_ref.cast_ray(ray_origin, ray_dir);
	BadOption::new(maybe_hit, zero())
}


// UTILS
#[repr(C)]
pub struct BadOption<T : Sized> {
	is_some : bool,
	value : T
}

impl<T : Sized> BadOption<T> {
	fn new(maybe_value: Option<T>, default: T) -> BadOption<T> {
		BadOption {
			is_some: maybe_value.is_some(),
			value: maybe_value.unwrap_or(default)
		}
	}
}