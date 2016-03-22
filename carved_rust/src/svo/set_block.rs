use nalgebra::{Vec3, zero};
use svo::*;

impl SVO {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                           index: &[u8], new_voxel_data: VoxelData)
        	where D : Fn(u32), R : Fn(Vec3<f32>, i32, VoxelData) -> u32 {
        self.set_voxel_from(deregister_voxel, register_voxel, index, new_voxel_data, zero(), 0);
    }

    // TODO: could this recursion pattern be generalised?
    fn set_voxel_from<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
    	                    index: &[u8], new_voxel_data: VoxelData, origin: Vec3<f32>, depth: i32)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, VoxelData) -> u32 {
        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == new_voxel_data {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister_all(deregister_voxel);
                let uid = register_voxel(origin, depth, new_voxel_data);
                *self = SVO::new_voxel(new_voxel_data, uid);
            },

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                if self.get_voxel_data().is_some() { self.subdivide_voxel(deregister_voxel, register_voxel, origin, depth); }

                {
                    // Insert into the sub_octant
                    let ref mut octants = self.get_mut_octants().unwrap();
                    let new_origin = origin + offset(ix, depth);
                    octants[ix as usize].set_voxel_from(deregister_voxel, register_voxel,
                                                        rest, new_voxel_data, new_origin, depth+1);
                }

                // Then if we have 8 voxels of the same type, combine them.
                if let Some(combined_voxel_data) = self.get_octants().and_then(combine_voxels) {
                    self.recombine_octants(deregister_voxel, register_voxel, origin, depth, combined_voxel_data);
                }
            }
        }
    }

    fn subdivide_voxel<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                             origin: Vec3<f32>, depth: i32)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, VoxelData) -> u32 {
        *self = match *self {
            SVO::Voxel { data, external_id } => {
                deregister_voxel(external_id);
                SVO::new_octants(&|ix| {
                    let uid = register_voxel(origin + offset(ix, depth), depth+1, data);
                    SVO::new_voxel(data, uid)
                })
            },
            _ => panic!("subdivide_voxel called on a non-voxel!")
        };
    }

    fn recombine_octants<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                               origin: Vec3<f32>, depth: i32, voxel_data: VoxelData)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, VoxelData) -> u32 {
        *self = match *self {
            SVO::Octants(ref mut octants) => {
                for octant in octants { octant.deregister_all(deregister_voxel); }
                let uid = register_voxel(origin, depth, voxel_data);
                SVO::new_voxel(voxel_data, uid)
            },
            _ => panic!("recombine_octants called on non-octants!")
        }
    }

    fn deregister_all<D>(&mut self, deregister_voxel: &D) where D: Fn(u32) {
        match *self {
            SVO::Voxel { external_id, .. } => deregister_voxel(external_id),
            SVO::Octants (ref mut octants) =>
                for octant in octants { octant.deregister_all(deregister_voxel); }
        }
    }
}

// Return the voxel_data that all of the octants share, or None.
fn combine_voxels(octants: &[Box<SVO>; 8]) -> Option<VoxelData> {
        octants[0].get_voxel_data().and_then( |voxel_data| {
            let mut tail = octants.iter().skip(1);
            if tail.all(|octant| octant.get_voxel_data() == Some(voxel_data)) {
                Some(voxel_data)
            } else {
                None
            }
        })
    }