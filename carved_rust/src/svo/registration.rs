use svo::voxel_data::VoxelData;
use nalgebra::Vec3;

pub type RegisterExtern = extern "stdcall" fn(Vec3<f32>, i32, VoxelData) -> u32;
pub type DeregisterExtern = extern "stdcall" fn(u32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Registered { pub external_id: u32 }
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Unregistered { pub _padding: u32 }
impl Unregistered {
    pub fn new() -> Unregistered { Unregistered { _padding: 0 } }
}

pub trait RegistrationState {}
impl RegistrationState for Registered {}
impl RegistrationState for Unregistered {}

pub struct RegistrationFunctions<'a> {
    pub register: Box<Fn(Vec3<f32>, i32, VoxelData) -> Registered + 'a>,
    pub deregister: Box<Fn(Registered) + 'a>
}

impl<'a> RegistrationFunctions<'a> {
	pub fn dummy() -> RegistrationFunctions<'a> {
		RegistrationFunctions {
			register: Box::new(|_, _, _| Registered { external_id: 0}),
			deregister: Box::new(|_| {})
		}
	}

	pub fn external(
			ext_register: RegisterExtern,
		    ext_deregister: DeregisterExtern) -> RegistrationFunctions<'a> {
		RegistrationFunctions {
			register: Box::new(move |origin, depth, data|
				Registered{ external_id: ext_register(origin, depth, data) }),
			deregister: Box::new(move |Registered{external_id}| ext_deregister(external_id))
		}
	}
}
