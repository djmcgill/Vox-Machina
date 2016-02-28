use svo::SVO;

use std::mem::transmute;

#[no_mangle]
pub extern "C" fn svo_create(voxel_type: i32) -> *mut SVO {
	unsafe { transmute(Box::new(SVO::new_voxel(voxel_type))) }
}

#[no_mangle]
pub extern "C" fn svo_get_voxel_type(svo: *const SVO) -> i32 {
	let svo: &SVO = unsafe { transmute(svo) };
	svo.get_voxel_type().unwrap()
}

#[no_mangle]
pub extern "C" fn svo_set_voxel_type(svo_ptr: *mut SVO, voxel_type: i32) {
	let mut svo_box: Box<SVO> = unsafe { transmute(svo_ptr) };
	*svo_box = SVO::new_voxel(voxel_type);
}

#[no_mangle]
pub extern "C" fn svo_destroy(svo: *mut SVO) {
	let _svo: Box<SVO> = unsafe { transmute(svo) };
}
