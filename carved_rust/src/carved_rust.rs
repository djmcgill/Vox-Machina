use svo::SVO;

use std::mem::transmute;
use std::slice;
use nalgebra::{Vec3, zero};

// FFI INTERFACE
#[no_mangle]
pub extern "stdcall" fn svo_create(voxel_type: i32) -> *mut SVO {
	unsafe { transmute(Box::new(SVO::new_voxel(voxel_type))) }
}

#[no_mangle]
pub extern "stdcall" fn svo_destroy(svo_ptr: *mut SVO) {
	let _svo: Box<SVO> = unsafe { transmute(svo_ptr) };
}

#[no_mangle]
pub extern "stdcall" fn svo_on_voxels(svo_ptr: *const SVO, on_voxel: extern "stdcall" fn(Vec3<f32>, i32, i32)) {
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	let on_voxel_closure = |vec, depth, voxel_type| on_voxel(vec, depth, voxel_type);
	svo_ref.on_voxels(&on_voxel_closure);
}

#[no_mangle]
pub extern "stdcall" fn svo_cast_ray(svo_ptr: *const SVO, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> BadOption<Vec3<f32>> {
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	let maybe_hit = svo_ref.cast_ray(ray_origin, ray_dir);
	BadOption::new(maybe_hit, zero())
}

#[no_mangle]
pub extern "stdcall" fn svo_set_block(svo_ptr: *mut SVO, index_ptr: *const u8, index_len: usize, new_block_type: i32) {
	let svo_ref: &mut SVO = unsafe { &mut *svo_ptr };
	let index: &[u8] = unsafe { slice::from_raw_parts(index_ptr, index_len) };
	svo_ref.set_block_and_recombine(index, new_block_type);
}

// UTILS
#[repr(C)] #[derive(Debug)]
pub struct BadOption<T : Sized> {
	pub is_some : bool,
	pub value : T
}

impl<T : Sized> BadOption<T> {
	fn new(maybe_value: Option<T>, default: T) -> BadOption<T> {
		BadOption {
			is_some: maybe_value.is_some(),
			value: maybe_value.unwrap_or(default)
		}
	}
}