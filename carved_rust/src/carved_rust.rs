use svo::*;

use std::mem::transmute;
use std::slice;
use nalgebra::{Vec3, zero};

// FFI INTERFACE
#[no_mangle]
pub extern "stdcall" fn svo_create<'a>
	(voxel_type: i32, 
     register_voxel_extern: extern "stdcall" fn(Vec3<f32>, i32, i32) -> u32,
     deregister_voxel_extern: extern "stdcall" fn(u32)
    ) -> *mut ExternalSVO<'a> {
	unsafe { 
		let external_svo = ExternalSVO {
			register_voxel: &|vec, depth, voxel_type| register_voxel_extern(vec, depth, voxel_type),
			deregister_voxel: &|external_id| deregister_voxel_extern(external_id),
			svo: SVO::new_voxel(voxel_type)
		};
		transmute(Box::new(external_svo))
	}
}

#[no_mangle]
pub extern "stdcall" fn svo_destroy(svo_ptr: *mut ExternalSVO) {
	let _svo: Box<ExternalSVO> = unsafe { transmute(svo_ptr) };
}

#[no_mangle]
pub extern "stdcall" fn svo_redraw(svo_ptr: *mut ExternalSVO) {
	let svo_ref: &ExternalSVO = unsafe { &*svo_ptr };
	svo_ref.svo.redraw(&svo_ref.register_voxel);
}

#[no_mangle]
pub extern "stdcall" fn svo_cast_ray(svo_ptr: *const ExternalSVO, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> BadOption<Vec3<f32>> {
	let svo_ref: &ExternalSVO = unsafe { &*svo_ptr };
	let maybe_hit = svo_ref.svo.cast_ray(ray_origin, ray_dir);
	BadOption::new(maybe_hit, zero())
}

#[no_mangle]
pub extern "stdcall" fn svo_set_block(svo_ptr: *mut ExternalSVO, index_ptr: *const u8, index_len: usize, new_block_type: i32) {
	let svo_ref: &mut ExternalSVO = unsafe { &mut *svo_ptr };
	let index: &[u8] = unsafe { slice::from_raw_parts(index_ptr, index_len) };
	svo_ref.svo.set_block_and_recombine(index, new_block_type, &svo_ref.deregister_voxel);
}

// UTILS
#[repr(C)] #[derive(Debug)]
pub struct BadOption<T : Sized> {
	pub is_some : i32,
	pub value : T
}

impl<T : Sized> BadOption<T> {
	fn new(maybe_value: Option<T>, default: T) -> BadOption<T> {
		BadOption {
			is_some: maybe_value.is_some() as i32,
			value: maybe_value.unwrap_or(default)
		}
	}
}