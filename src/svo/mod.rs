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

use arrayvec::ArrayVec;

// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
#[derive(Debug, PartialEq)]
pub enum SVO {
    Voxel { data: VoxelData },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants (ArrayVec<[Box<SVO>; 8]>)
}

impl SVO {
    pub fn example() -> SVO {
        let sub_svo = SVO::Octants(ArrayVec::from ([
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
            Box::new(SVO::new_voxel(VoxelData::new(0))), Box::new(SVO::new_voxel(VoxelData::new(0))),
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
        ]));
        SVO::Octants(ArrayVec::from ([
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(sub_svo),
            Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
        ]))
    }

    pub fn new_voxel(voxel_data: VoxelData) -> SVO {
        SVO::Voxel { data: voxel_data }
    }

    pub fn new_octants<F: Fn(u8) -> SVO>(make_octant: F) -> SVO {
        SVO::Octants(ArrayVec::from ([
            Box::new(make_octant(0)), Box::new(make_octant(1)),
            Box::new(make_octant(2)), Box::new(make_octant(3)),
            Box::new(make_octant(4)), Box::new(make_octant(5)),
            Box::new(make_octant(6)), Box::new(make_octant(7)),
        ]))
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_data(&self) -> Option<VoxelData> {
        match *self {
            SVO::Voxel { data, .. } => Some(data),
            _ => None
        }
    }

    // If the SVO is Octants, return its contents.
    fn get_octants(&self) -> Option<&ArrayVec<[Box<SVO>; 8]>> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }

    fn get_mut_octants(&mut self) -> Option<&mut ArrayVec<[Box<SVO>; 8]>> {
        match *self {
            SVO::Octants(ref mut octants) => Some(octants),
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
