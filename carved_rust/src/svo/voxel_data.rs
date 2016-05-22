#[repr(C)] #[derive(Debug, PartialEq, Copy, Clone)]
pub struct VoxelData {
    pub voxel_type: i32
}

impl VoxelData {
    pub fn new(voxel_type: i32) -> VoxelData {
        VoxelData { voxel_type: voxel_type }
    }
}