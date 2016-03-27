use svo::*;

use std::mem::transmute;
use std::slice;
use nalgebra::{Vec3, zero};

// FFI INTERFACE
#[no_mangle]
pub extern "stdcall" fn svo_create
    (voxel_type: i32,
     register_voxel_extern: extern "stdcall" fn(Vec3<f32>, i32, i32) -> u32,
     deregister_voxel_extern: extern "stdcall" fn(u32)
    ) -> *mut ExternalSVO<'static> {

    let register_voxel = &|vec, depth, VoxelData { voxel_type }| {
        println!("rust callback register_voxel {:?} {} {}", vec, depth, voxel_type);
        let uid = register_voxel_extern(vec, depth, voxel_type);
        println!("rust callback register_voxel done");
        uid
    };

    println!("test deregister start");
    deregister_voxel_extern(0);
    println!("test deregister end");

    let deregister_voxel = &|external_id| {
        println!("rust callback deregister_voxel {}", external_id);
        deregister_voxel_extern(external_id);
        println!("rust callback deregister_voxel done");
    };


    let voxel_data = VoxelData::new(voxel_type);
    let uid = register_voxel(zero(), 0, voxel_data);
    let external_svo = ExternalSVO {
        register_voxel: register_voxel,
        deregister_voxel: deregister_voxel,
        svo: SVO::new_voxel(voxel_data, uid)
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
    let mut svo_ref: &mut ExternalSVO = unsafe { &mut *svo_ptr };
    let index: &[u8] = unsafe { slice::from_raw_parts(index_ptr, index_len) };
    println!("about to deregister in other function");
    (svo_ref.deregister_voxel)(0);
    println!("deregistered in other function");
    let voxel_data = VoxelData::new(new_voxel_type);
    svo_ref.svo.set_block(&svo_ref.deregister_voxel, &svo_ref.register_voxel, index, voxel_data);
}

// UTILS
#[repr(C)]
pub struct Callbacks {
    register_voxel: extern "stdcall" fn(Vec3<f32>, i32, i32) -> u32,
    deregister_voxel_ptr: extern "stdcall" fn(u32)
}

pub struct ExternalSVO<'a> {
    pub register_voxel: &'a Fn(Vec3<f32>, i32, VoxelData) -> u32,
    pub deregister_voxel: &'a Fn(u32),
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