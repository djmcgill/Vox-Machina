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

//private static extern BadOptionVec3 svo_cast_ray_float (IntPtr svo, float ox, float oy, float oz, float dx, float dy, float dz);
#[no_mangle]
pub extern "stdcall" fn svo_cast_ray_float(svo_ptr: *const SVO, 
	ox: i32, oy: i32, oz: i32, dx: i32, dy: i32, dz: i32) -> BadOption<Vec3<f32>> {
	println!("{} {} {} {} {} {}", ox, oy, oz, dx, dy, dz);

	let origin = Vec3::new(ox as f32, oy as f32, oz as f32);
	let dir = Vec3::new(dx as f32, dy as f32, dz as f32);

	println!("calling svo_cast_ray_float with {:?} {:?} {:?}", svo_ptr, origin, dir);
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	let maybe_hit = svo_ref.cast_ray(origin, dir);
	let ret = BadOption::new(maybe_hit, zero());
	println!("returning from float {:?}", ret);
	ret

}




#[no_mangle]
pub extern "stdcall" fn svo_cast_ray(svo_ptr: *const SVO, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> BadOption<Vec3<f32>> {
	println!("calling svo_cast_ray with {:?} {:?} {:?}", svo_ptr, ray_origin, ray_dir);
	let svo_ref: &SVO = unsafe { &*svo_ptr };
	let maybe_hit = svo_ref.cast_ray(ray_origin, ray_dir);
	let ret = BadOption::new(maybe_hit, zero());
	println!("returning {:?}", ret);
	ret
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