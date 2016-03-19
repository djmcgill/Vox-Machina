use nalgebra::Vec3;

pub mod cast_ray;
pub mod set_block;

#[derive(Debug, PartialEq)]
// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
pub enum SVO {
    Voxel { voxel_type: i32, external_id: u32 },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO>; 8])
}

impl SVO {
    pub fn new_voxel(voxel_type: i32, external_id: u32) -> SVO {
        SVO::Voxel { voxel_type: voxel_type, external_id: external_id }
    }

    pub fn new_octants<F>(make_octant: &F) -> SVO where F: Fn(u8) -> SVO {
        SVO::Octants([
            Box::new(make_octant(0)), Box::new(make_octant(1)),
            Box::new(make_octant(2)), Box::new(make_octant(3)),
            Box::new(make_octant(4)), Box::new(make_octant(5)),
            Box::new(make_octant(6)), Box::new(make_octant(7))])
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_type(&self) -> Option<i32> {
        match *self {
            SVO::Voxel { voxel_type, .. } => Some(voxel_type),
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
}

// Returns a vector with either 0. or 1. as its elements
fn above_axis(ix: u8) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}

// Returns the new origin of the child at the given index in global space.
pub fn offset(ix: u8, depth: i32) -> Vec3<f32> {
    above_axis(ix) / ((1 << depth+1) as f32)
}
