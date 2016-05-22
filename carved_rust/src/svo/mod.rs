pub mod registration;
pub mod voxel_data;

// mod set_block;
mod cast_ray;

#[cfg(test)]
mod svo_tests;

use nalgebra::{Vec3, zero};
pub use self::registration::*;
pub use self::voxel_data::VoxelData;
use std::mem;

// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
#[derive(Debug, PartialEq)]
pub enum SVO<R: RegistrationTrait> {
    Voxel { data: VoxelData, registration: R },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO<R>>; 8])
}

impl SVO<Unregistered> {
    pub fn new_voxel(voxel_data: VoxelData) -> SVO<Unregistered> {
        SVO::Voxel { data: voxel_data, registration: Unregistered::new() }
    }

    pub fn register_from(
            mut self,
            registration_fns: &RegistrationFunctions<Unregistered>,
            origin: Vec3<f32>,
            depth: i32
        ) -> SVO<Registered> {

        self.register_helper(registration_fns, zero(), 0);
        unsafe { mem::transmute(self) }
    }

    fn register_helper(
            &mut self,
            registration_fns: &RegistrationFunctions<Unregistered>,
            origin: Vec3<f32>,
            depth: i32) {
        match *self {
            SVO::Octants (ref mut octants) => for ix in 0..8 {
                let new_origin = origin + offset(ix, depth);
                octants[ix as usize].register_helper(registration_fns, new_origin, depth+1);
            },
            SVO::Voxel { data, registration } => {
                let uid = (registration_fns.register)(origin, depth, data);
                *self = SVO::Voxel { data: data, registration: Unregistered { _padding: uid } };
            }
        }
    }
}

impl SVO<Registered> {
    fn deregister_helper(&self, registration_fns: &RegistrationFunctions<Registered>) {
        match *self {
            SVO::Voxel { registration: Registered { external_id }, .. } =>
                (registration_fns.deregister)(external_id),
            SVO::Octants (ref octants) =>
                for octant in octants { octant.deregister_helper(registration_fns); }
        }
    }

    fn deregister(self, registration_fns: &RegistrationFunctions<Registered>) -> SVO<Unregistered> {
        self.deregister_helper(registration_fns);
        unsafe { mem::transmute(self) }
    }
}

impl<R: RegistrationTrait> SVO<R> {
    pub fn new_octants<F: FnMut(u8) -> SVO<R>>(mut make_octant: F) -> SVO<R> {
        SVO::Octants([
            Box::new(make_octant(0)), Box::new(make_octant(1)),
            Box::new(make_octant(2)), Box::new(make_octant(3)),
            Box::new(make_octant(4)), Box::new(make_octant(5)),
            Box::new(make_octant(6)), Box::new(make_octant(7))])
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_data(&self) -> Option<VoxelData> {
        match *self {
            SVO::Voxel { data, .. } => Some(data),
            _ => None
        }
    }

    // If the SVO is Octants, return its contents.
    fn get_octants(&self) -> Option<&[Box<SVO<R>>; 8]> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }

    fn get_mut_octants(&mut self) -> Option<&mut [Box<SVO<R>>; 8]> {
        match *self {
            SVO::Octants(ref mut octants) => Some(octants),
            _ => None
        }
    }
}

// Returns a vector with either 0. or 1. as its elements
fn above_axis(ix: u8) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}

// Returns the new origin of the child at the given index in global space.
pub fn offset(ix: u8, depth: i32) -> Vec3<f32> {
    above_axis(ix) / ((1 << (depth+1)) as f32)
}
