use nalgebra::Vec3;

pub mod cast_ray;
pub mod set_block;
pub mod save_load;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VoxelData {
    pub voxel_type: i32
}

impl VoxelData {
    pub fn new(voxel_type: i32) -> VoxelData {
        VoxelData { voxel_type: voxel_type }
    }
}

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

    pub fn new_octants<F>(make_octant: &F) -> SVO where F: Fn(u8) -> SVO {
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

    pub fn for_voxels<F>(&self, f: &F) where F: Fn(VoxelData) {
        match *self {
            SVO::Voxel { data, ..} => f(data),
            SVO::Octants(ref octants) => for octant in octants { octant.for_voxels(f); }
        }
    }

    pub fn fold_voxels<A, F, G>(&self, f: &F, g: &G) -> A
            where F: Fn([&A; 8]) -> A, G: Fn(VoxelData) -> A {
        match *self {
            SVO::Voxel { data, ..} => g(data),
            SVO::Octants(ref octants) => {
                let map: Vec<A> = octants.iter().map(|svo| svo.fold_voxels(f, g)).collect();
                f([&map[0], &map[1], &map[2], &map[3], &map[4], &map[5], &map[6], &map[7]])
           },

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
