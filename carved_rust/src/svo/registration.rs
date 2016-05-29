use svo::voxel_data::VoxelData;
use nalgebra::Vec3;

pub type RegisterExtern = extern "stdcall" fn(Vec3<f32>, i32, VoxelData) -> u32;
pub type DeregisterExtern = extern "stdcall" fn(u32);

pub struct RegistrationFunctions<'a> {
    pub register: Box<Fn(Vec3<f32>, i32, VoxelData) -> u32 + 'a>,
    pub deregister: Box<Fn(u32) + 'a>
}

impl<'a> RegistrationFunctions<'a> {
	pub fn external(
			ext_register: RegisterExtern,
		    ext_deregister: DeregisterExtern) -> RegistrationFunctions<'a> {
		RegistrationFunctions {
			register: Box::new(move |origin, depth, data| ext_register(origin, depth, data)),
			deregister: Box::new(move |external_id| ext_deregister(external_id))
		}
	}
}
