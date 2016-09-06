use nalgebra::{Vec3, zero};
use svo::*;

fn register(_: Vec3<f32>, _: i32, _: VoxelData) -> u32 { 0 }
fn deregister(_: u32) {}

#[test]
fn minimal_subdivide() {
    let registration_fns = &RegistrationFunctions::dummy();
    let data = VoxelData::new(1);
    let external_id = (registration_fns.register)(zero(), 0, data);
    let mut svo = SVO::new_voxel(data, external_id);
    svo.set_block(registration_fns, &[1], VoxelData::new(0));

    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 0),
        (0. , 0.5, 0. , 1, 1),
        (0.5, 0.5, 0. , 1, 1),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 1),
        (0.5, 0.5, 0.5, 1, 1)]);
}

#[test]
fn setting_blocks() {
    let mut svo = SVO::floor();
    let registration_fns = &RegistrationFunctions::dummy();

    svo.set_block(registration_fns, &[1, 3], VoxelData::new(3));
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
            (0.5 , 0.  , 0.  , 2, 1),
            (0.75, 0.  , 0.  , 2, 1),
            (0.5 , 0.25, 0.  , 2, 1),
            (0.75, 0.25, 0.  , 2, 3),
            (0.5 , 0.  , 0.25, 2, 1),
            (0.75, 0.  , 0.25, 2, 1),
            (0.5 , 0.25, 0.25, 2, 1),
            (0.75, 0.25, 0.25, 2, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);

    svo.set_block(registration_fns, &[1, 3], VoxelData::new(1));
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);
}