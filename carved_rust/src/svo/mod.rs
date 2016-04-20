use std::mem;
use nalgebra::{Vec3, zero};

// pub mod cast_ray;
// pub mod set_block;
// pub mod save_load;
// pub mod generator;

// #[cfg(test)]
// mod svo_tests;

pub trait Register: Fn(Vec3<f32>, i32, VoxelData) -> u32 {}
impl<R: Fn(Vec3<f32>, i32, VoxelData) -> u32> Register for R {}

pub trait Deregister: Fn(u32) {}
impl<D: Fn(u32)> Deregister for D {}

#[repr(C)] #[derive(Debug, PartialEq, Copy, Clone)]
pub struct VoxelData {
    pub voxel_type: i32
}

impl VoxelData {
    pub fn new(voxel_type: i32) -> VoxelData {
        VoxelData { voxel_type: voxel_type }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Registered { pub external_id: u32 }
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Unregistered;

pub trait RegistrationTrait {}
impl RegistrationTrait for Registered {}
impl RegistrationTrait for Unregistered {}

// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
#[derive(Debug, PartialEq)]
pub enum SVO<R: RegistrationTrait> {
    Voxel { data: VoxelData, registration: R },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO<R>>; 8])
}

impl SVO<Unregistered> {
    pub fn new_voxel(voxel_data: VoxelData) -> SVO<Unregistered> {
        SVO::Voxel { data: voxel_data, registration: Unregistered }
    }

    pub fn register<R: Register>(self, register: R) -> SVO<Registered> {
        self.register_from(&register, zero(), 0)
    }

    fn register_from<R: Register>(self, register: &R, origin: Vec3<f32>, depth: i32) -> SVO<Registered> {
        match self {
            SVO::Octants (octants) => SVO::new_octants(|ix| {
                let new_origin = origin + offset(ix, depth);
                panic!() //octants[ix as usize].register_from(register, new_origin, depth+1)


            }),
            SVO::Voxel { data, .. } => {
                let uid = register(origin, depth, data);
                SVO::Voxel { data: data, registration: Registered { external_id: uid } }
            }
        }
    }
}

impl SVO<Registered> {
    fn deregister<D: Deregister>(self, deregister_voxel: &D) -> SVO<Unregistered> {
        match self {
            SVO::Voxel { registration: Registered { external_id }, data } => {
                deregister_voxel(external_id);
                SVO::Voxel { data: data, registration: Unregistered }
            },
            SVO::Octants (mut octants) => {
                let new0 = unsafe { mem::replace(&mut octants[0], mem::uninitialized()).deregister(deregister_voxel) };
                let new1 = unsafe { mem::replace(&mut octants[1], mem::uninitialized()).deregister(deregister_voxel) };
                let new2 = unsafe { mem::replace(&mut octants[2], mem::uninitialized()).deregister(deregister_voxel) };
                let new3 = unsafe { mem::replace(&mut octants[3], mem::uninitialized()).deregister(deregister_voxel) };
                let new4 = unsafe { mem::replace(&mut octants[4], mem::uninitialized()).deregister(deregister_voxel) };
                let new5 = unsafe { mem::replace(&mut octants[5], mem::uninitialized()).deregister(deregister_voxel) };
                let new6 = unsafe { mem::replace(&mut octants[6], mem::uninitialized()).deregister(deregister_voxel) };
                let new7 = unsafe { mem::replace(&mut octants[7], mem::uninitialized()).deregister(deregister_voxel) };
                SVO::Octants([
                    Box::new(new0), Box::new(new1), Box::new(new2), Box::new(new3),
                    Box::new(new4), Box::new(new5), Box::new(new6), Box::new(new7)
                ])
            }
        }
    }
}

impl<R: RegistrationTrait> SVO<R> {
    pub fn new_octants<F: Fn(u8) -> SVO<R>>(make_octant: F) -> SVO<R> {
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
    fn get_octants(&self) -> Option<&[Box<SVO<R>>; 8]> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }

    fn get_mut_octants(&mut self) -> Option<&mut [Box<SVO<R>>; 8]> {
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
    above_axis(ix) / ((1 << (depth+1)) as f32)
}
