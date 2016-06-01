use svo::*;

use std::mem::transmute;
use std::slice;
use nalgebra::{Vec3, zero};

#[cfg(test)]
mod test;

// FFI INTERFACE
#[no_mangle]
pub extern "stdcall" fn svo_create(
        voxel_type: i32,
        register_extern: RegisterExtern,
        deregister_extern: DeregisterExtern) -> *mut ExternalSVO<'static> {

    let voxel_data = VoxelData::new(voxel_type);
    let registration_fns = RegistrationFunctions::external(register_extern, deregister_extern);
    let external_id = (registration_fns.register)(zero(), 0, voxel_data);
    let svo = SVO::new_voxel(voxel_data, external_id);
    let external_svo = ExternalSVO {
        registration_fns: registration_fns,
        svo: svo
    };
    unsafe { transmute(Box::new(external_svo)) }
}

#[no_mangle]
pub extern "stdcall" fn svo_destroy(svo_ptr: *mut ExternalSVO) {
    let _svo: Box<ExternalSVO> = unsafe { transmute(svo_ptr) };
}

#[no_mangle]
pub extern "stdcall" fn svo_cast_ray(svo_ptr: *const ExternalSVO, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> BadOption<Vec3<f32>> {
    let svo_ref: &ExternalSVO = unsafe { &*svo_ptr };
    let maybe_hit = svo_ref.svo.cast_ray(ray_origin, ray_dir);
    BadOption::new(maybe_hit, zero())
}

#[no_mangle]
pub extern "stdcall" fn svo_set_block(svo_ptr: *mut ExternalSVO, index_ptr: *const u8, index_len: usize, new_voxel_type: i32) {
    let svo_ref: &mut ExternalSVO = unsafe { &mut *svo_ptr };
    let index: &[u8] = unsafe { slice::from_raw_parts(index_ptr, index_len) };
    let voxel_data = VoxelData::new(new_voxel_type);

    svo_ref.svo.set_block(&svo_ref.registration_fns, index, voxel_data);
}

// UTILS
#[repr(C)]
pub struct ExternalSVO<'a> {
    pub registration_fns: RegistrationFunctions<'a>,
    pub svo: SVO
}

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