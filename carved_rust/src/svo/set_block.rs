use nalgebra::{Vec3, zero};
use svo::*;

pub trait Register: Fn(Vec3<f32>, i32, VoxelData) -> u32 {}
impl<R: Fn(Vec3<f32>, i32, VoxelData) -> u32> Register for R {}

pub trait Deregister: Fn(u32) {}
impl<D: Fn(u32)> Deregister for D {}

struct SetBlockEnv<R: Register, D: Deregister> {
    pub register_voxel: R,
    pub deregister_voxel: D,
    pub new_voxel_data: VoxelData
}

impl SVO {
    // Follow an index, splitting voxels as necessary. The set the block at the target to a Voxel with the specified data.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block<R, D>(&mut self, register_voxel: R, deregister_voxel: D, index: &[u8], new_voxel_data: VoxelData)
            where R: Register, D: Deregister {
        let env = SetBlockEnv {
            register_voxel: register_voxel, deregister_voxel: deregister_voxel, new_voxel_data: new_voxel_data
        };
        self.set_voxel_from(&env, index, zero(), 0);
    }

    // TODO: could this recursion pattern be generalised?
    fn set_voxel_from<R, D>(&mut self, env: &SetBlockEnv<R, D>, index: &[u8], origin: Vec3<f32>, depth: i32)
            where R: Register, D: Deregister {

        println!("calling set_voxel_from with index {:?}", index);

        if let Some(voxel_data) = self.get_voxel_data() {
            if voxel_data == env.new_voxel_data {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister_all(&env.deregister_voxel);
                let uid = (env.register_voxel)(origin, depth, env.new_voxel_data);
                *self = SVO::new_voxel(env.new_voxel_data, uid);
            },

            // We need to go deeper.
            Some((&ix, rest)) => {
                // Voxels get split up
                if self.get_voxel_data().is_some() {
                    println!("Splitting a voxel");
                    self.subdivide_voxel(&env.register_voxel, &env.deregister_voxel, origin, depth);
                }

                {
                    // Insert into the sub_octant
                    let octants = self.get_mut_octants().unwrap();
                    let new_origin = origin + offset(ix, depth);
                    octants[ix as usize].set_voxel_from(env, rest, new_origin, depth+1);
                }

                // Then if we have 8 voxels of the same type, combine them.
                if let Some(combined_voxel_data) = self.get_octants().and_then(combine_voxels) {
                    self.recombine_octants(&env.register_voxel, &env.deregister_voxel, origin, depth, combined_voxel_data);
                }
            }
        }
    }

    fn subdivide_voxel<R, D>(&mut self, register_voxel: &R, deregister_voxel: &D, origin: Vec3<f32>, depth: i32) 
            where R: Register, D: Deregister {
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

    fn recombine_octants<R, D>(&mut self, register_voxel: &R, deregister_voxel: &D, origin: Vec3<f32>, depth: i32, voxel_data: VoxelData)
            where R: Register, D: Deregister {
        *self = match *self {
            SVO::Octants(ref mut octants) => {
                for octant in octants { octant.deregister_all(deregister_voxel); }
                let uid = register_voxel(origin, depth, voxel_data);
                SVO::new_voxel(voxel_data, uid)
            },
            _ => panic!("recombine_octants called on non-octants!")
        }
    }

    fn deregister_all<D: Deregister>(&mut self, deregister_voxel: &D) {
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