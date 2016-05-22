// #[cfg(test)]
// mod set_block_tests;

use nalgebra::{Vec3, zero};
use svo::*;
use std::mem;
use std::ptr;

impl SVO<Unregistered> {
    // This is pretty gross but in the interests of not spending a million years setting up
    // a properly typed and safe version I don't really care.
    pub fn set_block(self, index: &[u8], new_voxel_data: VoxelData) -> SVO<Unregistered> {
        let fake_registered_svo: SVO<Registered> = unsafe { mem::transmute(self) };
        let new_svo = fake_registered_svo.set_block(|_,_,_| 0, |_| {}, index, new_voxel_data);
        unsafe { mem::transmute(new_svo) }
    }
}

impl SVO<Registered> {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block<R, D>(self, register_voxel: R, deregister_voxel: D,
                           index: &[u8], new_voxel_data: VoxelData)
                        -> SVO<Registered> where R: Register, D: Deregister {
        let env = SetBlockEnv {
            register_voxel: register_voxel,
            deregister_voxel: deregister_voxel,
            new_voxel_data: new_voxel_data
        };
        self.set_voxel_from(&env, index, zero(), 0)
    }

    // TODO: could this recursion pattern be generalised?
    fn set_voxel_from<R, D>(self, env: &SetBlockEnv<R, D>,
                            index: &[u8], origin: Vec3<f32>, depth: i32)
                         -> SVO<Registered> where R: Register, D: Deregister {

        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == env.new_voxel_data {return self;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister(&env.deregister_voxel);
                SVO::new_voxel(env.new_voxel_data).register_from(&env.register_voxel, origin, depth)
            },

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                let split_self = match self.get_voxel_data() {
                    None => self,
                    Some(_) => self.subdivide_voxel(&env.register_voxel, &env.deregister_voxel, origin, depth)
                };

                // Insert destructively into the sub_octant
                let inserted_self = match split_self {
                    SVO::Voxel {..} => unreachable!(),
                    SVO::Octants(mut octants) => unsafe {
                        let new_origin = origin + offset(ix, depth);
                        {
                            let octant_ptr = &mut *octants[ix as usize];
                            let new_octant = ptr::read(octant_ptr).set_voxel_from(env, rest, new_origin, depth+1);
                            ptr::write(octant_ptr, new_octant);
                        }
                        SVO::Octants(octants)
                    }
                };

                inserted_self.recombine_svo(&env.register_voxel, &env.deregister_voxel, origin, depth)
            }
        }
    }

    fn subdivide_voxel<R, D>(self, register_voxel: &R, deregister_voxel: &D,
                             origin: Vec3<f32>, depth: i32)
                          -> SVO<Registered> where R: Register, D: Deregister {
        match self {
            SVO::Octants(_) => panic!("subdivide_voxel called on a non-voxel!"),
            SVO::Voxel { data, registration: Registered{ external_id }} => {
                deregister_voxel(external_id);
                SVO::new_octants(|ix| {
                    let new_origin = origin + offset(ix, depth);
                    SVO::new_voxel(data).register_from(register_voxel, new_origin, depth+1)
                })
            }
        }
    }

    pub fn recombine_svo<R, D>(self, register_voxel: &R, deregister_voxel: &D, origin: Vec3<f32>, depth: i32) -> SVO<Registered>
            where R: Register, D: Deregister {
        match self.get_octants().and_then(combine_voxels) {
            None => self,
            Some(combined_voxel_data) => {
                self.deregister(deregister_voxel);
                SVO::new_voxel(combined_voxel_data).register_from(register_voxel, origin, depth)
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