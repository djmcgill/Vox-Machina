use svo::voxel_data::VoxelData;
use nalgebra::Vec3;
use std::marker::PhantomData;

pub type RegisterExtern = extern "stdcall" fn(Vec3<f32>, i32, VoxelData) -> u32;
pub type DeregisterExtern = extern "stdcall" fn(u32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Registered { pub external_id: u32 }
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Unregistered { pub _padding: u32 }
impl Unregistered {
    pub fn new() -> Unregistered { Unregistered { _padding: 0 } }
}

pub trait RegistrationTrait {}
impl RegistrationTrait for Registered {}
impl RegistrationTrait for Unregistered {}

pub struct RegistrationFunctions<T: RegistrationTrait> {
    pub register: Box<Fn(Vec3<f32>, i32, VoxelData) -> u32>,
    pub deregister: Box<Fn(u32)>,
    registration_trait: PhantomData<T>
}

impl<T: RegistrationTrait> RegistrationFunctions<T> {
	pub fn dummy_registration() -> RegistrationFunctions<Unregistered> {
		RegistrationFunctions {
			register: Box::new(|_, _, _| 0),
			deregister: Box::new(|_| {}),
			registration_trait: PhantomData
		}
	}

	pub fn external_registration(ext_register: RegisterExtern,
		                         ext_deregister: DeregisterExtern
		                         ) -> RegistrationFunctions<Registered> {
		RegistrationFunctions {
			register: Box::new(move |origin, depth, data| ext_register(origin, depth, data)),
			deregister: Box::new(move |uid| ext_deregister(uid)),
			registration_trait: PhantomData
		}
	}
}
