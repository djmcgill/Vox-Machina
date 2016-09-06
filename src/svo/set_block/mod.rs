#[cfg(test)]
mod test;

use svo::*;

impl SVO {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block(
            &mut self,
            index: &[u8],
            new_data: VoxelData) {

        self.set_voxel_from(index, &new_data);
    }

    fn set_voxel_from(
            &mut self,
            index: &[u8],
            new_data: &VoxelData) {

        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == *new_data {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => *self = SVO::new_voxel(*new_data),

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                if self.get_voxel_data().is_some() {
                    self.subdivide_voxel();
                }

                // Insert destructively into the sub_octant
                if let SVO::Octants(ref mut octants) = *self {
                    octants[ix as usize].set_voxel_from(rest, new_data);
                };

                self.recombine_svo();
            }
        }
    }

    fn subdivide_voxel(&mut self) {
        *self = match *self {
            SVO::Octants(_) => panic!("subdivide_voxel called on a non-voxel!"),
            SVO::Voxel { data, .. } => SVO::new_octants(|_| { SVO::new_voxel(data) })
        };
    }

    pub fn recombine_svo(&mut self) {
    if let Some(combined_voxel_data) = self.get_octants().and_then(combine_voxels) {
            *self = SVO::new_voxel(combined_voxel_data);
        }
    }
}

// Return the voxel_data that all of the octants share, or None.
fn combine_voxels(octants: &[Box<SVO>; 8]) -> Option<VoxelData> {
        octants[0].get_voxel_data().and_then(|voxel_data| {
            let first_data = Some(voxel_data);
            for octant in &octants[1..] { guard!(octant.get_voxel_data() == first_data) }
            first_data
        })
    }
