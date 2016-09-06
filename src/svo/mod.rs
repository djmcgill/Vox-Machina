pub mod registration;
pub mod voxel_data;

mod set_block;
mod cast_ray;
mod save_load;
mod generator;

#[cfg(test)]
mod test;

use nalgebra::Vec3;
pub use self::registration::*;
pub use self::voxel_data::VoxelData;

// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
#[derive(Debug, PartialEq)]
pub enum SVO {
    Voxel { data: VoxelData, external_id: u32 },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO>; 8])
}

impl SVO {
    pub fn new_voxel(voxel_data: VoxelData, external_id: u32) -> SVO {
        SVO::Voxel { data: voxel_data, external_id: external_id }
    }

    pub fn new_octants<F: Fn(u8) -> SVO>(make_octant: F) -> SVO {
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
    fn get_octants(&self) -> Option<&[Box<SVO>; 8]> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }

    fn get_mut_octants(&mut self) -> Option<&mut [Box<SVO>; 8]> {
        match *self {
            SVO::Octants(ref mut octants) => Some(octants),
            _ => None
        }
    }

    fn deregister(&mut self, registration_fns: &RegistrationFunctions) {
        match *self {
            SVO::Voxel { external_id, .. } =>
                (registration_fns.deregister)(external_id),
            SVO::Octants (ref mut octants) =>
                for octant in octants { octant.deregister(registration_fns); }
        }
    }
}

// Returns a vector with either 0. or 1. as its elements
pub fn above_axis(ix: u8) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}

// Returns the new origin of the child at the given index in global space.
pub fn offset(ix: u8, depth: i32) -> Vec3<f32> {
    above_axis(ix) / ((1 << (depth+1)) as f32)
}

pub fn index(v: Vec3<f32>) -> u8 {
    let above_x = (v.x > 0.5) as u8;
    let above_y = (v.y > 0.5) as u8;
    let above_z = (v.z > 0.5) as u8;
    (above_x << 0) | (above_y << 1) | (above_z << 2)
}
