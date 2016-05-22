// #[cfg(test)]
// mod tests;

use nalgebra::{Vec3, zero};
use svo::*;
use std::ptr;

impl SVO<Registered> {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block(
            self,
            registration_fns: &RegistrationFunctions,
            index: &[u8],
            new_data: VoxelData) -> SVO<Registered> {

        self.set_voxel_from(registration_fns, index, &new_data, zero(), 0)
    }

    fn set_voxel_from(
            self,
            registration_fns: &RegistrationFunctions,
            index: &[u8],
            new_data: &VoxelData,
            origin: Vec3<f32>,
            depth: i32) -> SVO<Registered> {

        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == *new_data {return self;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister(registration_fns);
                SVO::new_voxel(*new_data).register_from(registration_fns, origin, depth)
            },

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                let split_self = match self.get_voxel_data() {
                    None => self,
                    Some(_) => self.subdivide_voxel(registration_fns, origin, depth)
                };

                // Insert destructively into the sub_octant
                let inserted_self = match split_self {
                    SVO::Voxel {..} => unreachable!(),
                    SVO::Octants(mut octants) => unsafe {
                        let new_origin = origin + offset(ix, depth);
                        {
                            let octant_ptr = &mut *octants[ix as usize];
                            let new_octant = ptr::read(octant_ptr).set_voxel_from(registration_fns, rest, new_data, new_origin, depth+1);
                            ptr::write(octant_ptr, new_octant);
                        }
                        SVO::Octants(octants)
                    }
                };

                inserted_self.recombine_svo(registration_fns, origin, depth)
            }
        }
    }

    fn subdivide_voxel(
            self,
            registration_fns: &RegistrationFunctions,
            origin: Vec3<f32>,
            depth: i32) -> SVO<Registered> {
        match self {
            SVO::Octants(_) => panic!("subdivide_voxel called on a non-voxel!"),
            SVO::Voxel { data, registration } => {
                (registration_fns.deregister)(registration);
                SVO::new_octants(|ix| {
                    let new_origin = origin + offset(ix, depth);
                    SVO::new_voxel(data).register_from(registration_fns, new_origin, depth+1)
                })
            }
        }
    }

    pub fn recombine_svo(
            self,
            registration_fns: &RegistrationFunctions,
            origin: Vec3<f32>,
            depth: i32) -> SVO<Registered> {
        match self.get_octants().and_then(combine_voxels) {
            None => self,
            Some(combined_voxel_data) => {
                self.deregister(registration_fns);
                SVO::new_voxel(combined_voxel_data).register_from(registration_fns, origin, depth)
            }
        }
    }
}

// Return the voxel_data that all of the octants share, or None.
fn combine_voxels(octants: &[Box<SVO<Registered>>; 8]) -> Option<VoxelData> {
        octants[0].get_voxel_data().and_then(|voxel_data| {
            let first_data = Some(voxel_data);
            for octant in &octants[1..] { guard!(octant.get_voxel_data() == first_data) }
            first_data
        })
    }