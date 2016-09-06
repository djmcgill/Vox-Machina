#[cfg(test)]
mod test;

use nalgebra::{Vec3, zero};
use svo::*;

impl SVO {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block(
            &mut self,
            registration_fns: &RegistrationFunctions,
            index: &[u8],
            new_data: VoxelData) {

        self.set_voxel_from(registration_fns, index, &new_data, zero(), 0);
    }

    fn set_voxel_from(
            &mut self,
            registration_fns: &RegistrationFunctions,
            index: &[u8],
            new_data: &VoxelData,
            origin: Vec3<f32>,
            depth: i32) {

        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == *new_data {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister(registration_fns);
                let external_id = (registration_fns.register)(origin, depth, *new_data);
                *self = SVO::new_voxel(*new_data, external_id);
            },

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                if self.get_voxel_data().is_some() {
                    self.subdivide_voxel(registration_fns, origin, depth);
                }

                // Insert destructively into the sub_octant
                if let SVO::Octants(ref mut octants) = *self {
                    let new_origin = origin + offset(ix, depth);
                    octants[ix as usize].set_voxel_from(registration_fns, rest, new_data, new_origin, depth+1);
                };

                self.recombine_svo(registration_fns, origin, depth);
            }
        }
    }

    fn subdivide_voxel(
            &mut self,
            registration_fns: &RegistrationFunctions,
            origin: Vec3<f32>,
            depth: i32) {
        *self = match *self {
            SVO::Octants(_) => panic!("subdivide_voxel called on a non-voxel!"),
            SVO::Voxel { data, external_id } => {
                (registration_fns.deregister)(external_id);
                SVO::new_octants(|ix| {
                    let new_origin = origin + offset(ix, depth);
                    let new_external_id = (registration_fns.register)(new_origin, depth+1, data);
                    SVO::new_voxel(data, new_external_id)
                })
            }
        };
    }

    pub fn recombine_svo(
            &mut self,
            registration_fns: &RegistrationFunctions,
            origin: Vec3<f32>,
            depth: i32) {
    if let Some(combined_voxel_data) = self.get_octants().and_then(combine_voxels) {
            self.deregister(registration_fns);
            let external_id = (registration_fns.register)(origin, depth, combined_voxel_data);
            *self = SVO::new_voxel(combined_voxel_data, external_id);
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
