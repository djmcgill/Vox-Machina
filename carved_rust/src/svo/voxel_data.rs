#[repr(C)] #[derive(Debug, PartialEq, Copy, Clone)]
pub struct VoxelData {
    pub voxel_type: i32
}

impl VoxelData {
    pub fn new(voxel_type: i32) -> VoxelData {
        VoxelData { voxel_type: voxel_type }
    }
}

#[cfg(test)]
mod test {
	use quickcheck::{Arbitrary, Gen};
	use super::VoxelData;

	impl Arbitrary for VoxelData {
		fn arbitrary<G: Gen>(g: &mut G) -> VoxelData {
			VoxelData::new(Arbitrary::arbitrary(g))
		}
		fn shrink(&self) -> Box<Iterator<Item=VoxelData>> {
			Box::new(self.voxel_type.shrink().map(VoxelData::new))
		}
	}
}
