use svo::SVO;

use std::mem::transmute;

#[no_mangle]
pub extern "C" fn svo_create(voxel_type: i32) -> *mut SVO {
	println!("svo_create with {}", voxel_type);
	unsafe { transmute(Box::new(SVO::new_voxel(voxel_type))) }
}

#[no_mangle]
pub extern "C" fn svo_get_voxel_type(svo_ptr: *const SVO) -> i32 {
	println!("svo_get_voxel_type with {:?}", svo_ptr);
	let svo: &SVO = unsafe { transmute(svo_ptr) };
	svo.get_voxel_type().unwrap()
}

#[no_mangle]
pub extern "C" fn svo_set_voxel_type(svo_ptr: *mut SVO, voxel_type: i32) {
	println!("svo_set_voxel_type with {:?} and {}", svo_ptr, voxel_type);
	let svo_ref: &mut SVO = unsafe { transmute(svo_ptr) };
	*svo_ref = SVO::new_voxel(voxel_type);
}

#[no_mangle]
pub extern "C" fn svo_destroy(svo_ptr: *mut SVO) {
	println!("svo_destroy with {:?}", svo_ptr);
	let _svo: Box<SVO> = unsafe { transmute(svo_ptr) };
}
