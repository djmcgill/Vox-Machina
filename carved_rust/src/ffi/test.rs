use svo::*;
use nalgebra::{ApproxEq, Vec3};
use ffi;

//=== FFI tests ===
extern "stdcall" fn ext_register(_: Vec3<f32>, _: i32, _: VoxelData) -> u32 { 0 }
extern "stdcall" fn ext_deregister(_: u32) {}

#[test]
fn ffi_integration() {
    let svo_ptr = ffi::svo_create(1, ext_register, ext_deregister);

    let index = &[1u8];
    ffi::svo_set_block(svo_ptr, index.as_ptr(), index.len(), 2);

    {
        let esvo: &ffi::ExternalSVO = unsafe { &*svo_ptr };
        esvo.svo.assert_contains(vec![
            (0. , 0. , 0. , 1, 1),
            (0.5, 0. , 0. , 1, 2),
            (0. , 0.5, 0. , 1, 1),
            (0.5, 0.5, 0. , 1, 1),
            (0. , 0. , 0.5, 1, 1),
            (0.5, 0. , 0.5, 1, 1),
            (0. , 0.5, 0.5, 1, 1),
            (0.5, 0.5, 0.5, 1, 1)]);
    }

    let maybe_hit = ffi::svo_cast_ray(svo_ptr, Vec3::new(0.52, 2., 0.52), Vec3::new(0., -1., 0.));
    assert!(maybe_hit.is_some != 0);
    assert_approx_eq_eps!(maybe_hit.value, Vec3::new(0.52, 1., 0.52), 0.001);

    let maybe_hit2 = ffi::svo_cast_ray(svo_ptr, Vec3::new(1.52, 2., 0.52), Vec3::new(0., -1., 0.));
    assert!(maybe_hit2.is_some == 0);

    ffi::svo_destroy(svo_ptr);
}