pub mod registration;
pub mod voxel_data;

mod set_block;
mod cast_ray;
mod save_load;
mod generator;

#[cfg(test)]
mod test;

use nalgebra::Vector3;
pub use self::registration::*;
pub use self::voxel_data::VoxelData;
use std::io::Result;

use arrayvec::ArrayVec;
pub type SubOctants = ArrayVec<[Box<SVO>; 8]>;

// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
#[derive(Debug, PartialEq)]
pub enum SVO {
    Voxel { data: VoxelData },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants (SubOctants),
}

impl SVO {
    pub fn example() -> SVO {
        SVO::new_octants(|i| {
            let data = [1, 1, 1, 0, 1, 1, 0, 0][i as usize];
            SVO::new_voxel(VoxelData::new(data))
        })
    }

    pub fn new_voxel(voxel_data: VoxelData) -> SVO {
        SVO::Voxel { data: voxel_data }
    }

    pub fn new_octants<F>(mut make_octant: F) -> SVO
            where F: FnMut(u8) -> SVO {
        SVO::Octants(
            (0..8).map(|i| Box::new(make_octant(i)))
                  .collect::<SubOctants>()
        )
    }

    pub fn new_octants_mut_err<F>(mut make_octant: F) -> Result<SVO> 
            where F: FnMut(u8) -> Result<SVO> {
        (0..8).map(|i| make_octant(i).map(Box::new))
              .collect::<Result<SubOctants>>()
              .map(SVO::Octants)
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_data(&self) -> Option<VoxelData> {
        match *self {
            SVO::Voxel { data, .. } => Some(data),
            _ => None
        }
    }

    // If the SVO is Octants, return its contents.
    fn get_octants(&self) -> Option<&SubOctants> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }
}

// Returns a vector with either 0. or 1. as its elements
pub fn above_axis(ix: u8) -> Vector3<f32> {
    Vector3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}

// Returns the new origin of the child at the given index in global space.
pub fn offset(ix: u8, depth: i32) -> Vector3<f32> {
    let side_len = 1.0 / (1 << (depth+1)) as f32;
    offset_float(ix, side_len)
}

pub fn offset_float(ix: u8, side_len: f32) -> Vector3<f32> {
    above_axis(ix) * side_len
}

pub fn index(v: Vector3<f32>) -> u8 {
    let above_x = (v.x > 0.5) as u8;
    let above_y = (v.y > 0.5) as u8;
    let above_z = (v.z > 0.5) as u8;
    (above_x << 0) | (above_y << 1) | (above_z << 2)
}
