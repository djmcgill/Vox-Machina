//extern crate test;

// use carved_rust;
use nalgebra::{ApproxEq, Vec3, zero};
use std::cell::{Cell, RefCell};
use svo::*;

fn register(_: Vec3<f32>, _: i32, _: VoxelData) -> u32 { 0 }
fn deregister(_: u32) {}

#[test]
fn on_blocks() {
    let svo = SVO::floor();
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);
}

#[test]
fn register_blocks() {
	let counter = Cell::new(0u32);
	let deregistered_vec: RefCell<Vec<u32>> = RefCell::new(vec![]);

    let svo = {
    	let registration_fns = RegistrationFunctions {
	    		register: Box::new(move |_, _, _| {
	            	let old_count = counter.get();
	            	counter.set(old_count + 1);
	            	old_count}),
	    		deregister: Box::new(|external_id| {
	    			deregistered_vec.borrow_mut().push(external_id)})
    	};

        let data = VoxelData::new(1);
        let external_id = (registration_fns.register)(zero(), 0, data);
    	let mut svo = SVO::new_voxel(data, external_id);
        svo.set_block(&registration_fns, &[2], VoxelData::new(0));
        svo.set_block(&registration_fns, &[3], VoxelData::new(0));
        svo.set_block(&registration_fns, &[6], VoxelData::new(0));
        svo.set_block(&registration_fns, &[7], VoxelData::new(0));
        svo.set_block(&registration_fns, &[1, 3], VoxelData::new(2));
        svo
    };
    let deregistered_vec = deregistered_vec.into_inner();

    match svo {
        SVO::Voxel{ .. } => panic!("Unexpected Voxel found"),
        SVO::Octants(ref octants) => {
        	octants[0].assert_is_voxel(VoxelData::new(1), 1);

            match *octants[1] {
                SVO::Voxel{ .. } => panic!("Unexpected Voxel found"),
                SVO::Octants(ref sub_octants) => {
                    sub_octants[0].assert_is_voxel(VoxelData::new(1), 13);
                    sub_octants[1].assert_is_voxel(VoxelData::new(1), 14);
                    sub_octants[2].assert_is_voxel(VoxelData::new(1), 15);
                    sub_octants[3].assert_is_voxel(VoxelData::new(2), 21);
                    sub_octants[4].assert_is_voxel(VoxelData::new(1), 17);
                    sub_octants[5].assert_is_voxel(VoxelData::new(1), 18);
                    sub_octants[6].assert_is_voxel(VoxelData::new(1), 19);
                    sub_octants[7].assert_is_voxel(VoxelData::new(1), 20);
                }
            }
            octants[2].assert_is_voxel(VoxelData::new(0), 9);
            octants[3].assert_is_voxel(VoxelData::new(0), 10);
            octants[4].assert_is_voxel(VoxelData::new(1), 5);
            octants[5].assert_is_voxel(VoxelData::new(1), 6);
            octants[6].assert_is_voxel(VoxelData::new(0), 11);
            octants[7].assert_is_voxel(VoxelData::new(0), 12);
        }
    }
    assert_eq!(deregistered_vec, vec![0, 3, 4, 7, 8, 2, 16]);
}

// #[test]
// fn flat_height_map() {
//     let width = 4;
//     let height = 4;

//     let image: [u8; 16] = [127u8; 16];
//     let svo = SVO::height_map(1, &image, width, height);
//     svo.assert_contains(vec![
//         (0. , 0. , 0. , 1, 1),
//         (0.5, 0. , 0. , 1, 1),
//         (0. , 0.5, 0. , 1, 0),
//         (0.5, 0.5, 0. , 1, 0),
//         (0. , 0. , 0.5, 1, 1),
//         (0.5, 0. , 0.5, 1, 1),
//         (0. , 0.5, 0.5, 1, 0),
//         (0.5, 0.5, 0.5, 1, 0)
//     ]);
// }

// #[test]
// fn full_height_map() {
//     let width = 4;
//     let height = 4;

//     let image: [u8; 16] = [u8::MAX; 16];
//     let svo = SVO::height_map(1, &image, width, height);
//     svo.assert_contains(vec![(0., 0., 0., 0, 1)]);
// }

// #[test]
// fn empty_height_map() {
//     let width = 4;
//     let height = 4;

//     let image: [u8; 16] = [0u8; 16];
//     let svo = SVO::height_map(1, &image, width, height);
//     svo.assert_contains(vec![(0., 0., 0., 0, 0)]);
// }

impl SVO {
	pub fn assert_is_voxel(&self, expected_data: VoxelData, expected_external_id: u32) {
		match *self {
			SVO::Octants(_) => panic!("Found Octants when expecting a Voxel!"),
			SVO::Voxel{ data, external_id } => {
				assert_eq!(data, expected_data);
				assert_eq!(external_id, expected_external_id);
			}
		}
	}

    pub fn assert_contains(&self, expected: Vec<(f32, f32, f32, i32, i32)>) {
        let mut results_vec: Vec<(f32, f32, f32, i32, i32)> = Vec::new();
        self.collect_svo(&mut results_vec, zero(), 0);
        let results = results_vec;

        println!("expected: {:?}", expected);
        println!("actual: {:?}", results);
        assert_eq!(results.len(), expected.len());

        for (actual_element, expected_element) in results.iter().zip(expected.iter()) {
            let &(x, y, z, depth, voxel_type) = actual_element;
            let &(x_, y_, z_, depth_, voxel_type_) = expected_element;

            assert_approx_eq_eps!(x, x_, 0.01);
            assert_approx_eq_eps!(y, y_, 0.01);
            assert_approx_eq_eps!(z, z_, 0.01);
            assert_eq!(depth, depth_);
            assert_eq!(voxel_type, voxel_type_);
        }
    }

    fn collect_svo(&self, results_vec: &mut Vec<(f32, f32, f32, i32, i32)>, origin: Vec3<f32>, depth: i32) {
        match *self {
            SVO::Voxel { data: VoxelData { voxel_type, .. }, .. } =>
                results_vec.push((origin.x, origin.y, origin.z, depth, voxel_type)),
            SVO::Octants (ref octants) => {
                for ix in 0..8 {
                    let new_origin = origin + offset(ix, depth);
                    octants[ix as usize].collect_svo(results_vec, new_origin, depth + 1);
                }
            }

        }
    }

    pub fn floor() -> SVO {
        let registration_fns = &RegistrationFunctions::dummy();
        let data = VoxelData::new(1);
        let external_id = (registration_fns.register)(zero(), 0, data);
        let mut svo = SVO::new_voxel(data, external_id);
        svo.set_block(registration_fns, &[2], VoxelData::new(0));
        svo.set_block(registration_fns, &[3], VoxelData::new(0));
        svo.set_block(registration_fns, &[6], VoxelData::new(0));
        svo.set_block(registration_fns, &[7], VoxelData::new(0));
        svo
    }
}

impl<'a> RegistrationFunctions<'a> {
    pub fn dummy() -> RegistrationFunctions<'a> {
        RegistrationFunctions {
            register: Box::new(|_,_,_| 0),
            deregister: Box::new(|_| {})
        }
    }
}

// //=== FFI tests ===
// extern "stdcall" fn ext_register(_: Vec3<f32>, _: i32, _: VoxelData) -> u32 { 0 }
// extern "stdcall" fn ext_deregister(_: u32) {}

// #[test]
// fn ffi_integration() {
//     let svo_ptr = carved_rust::svo_create(1, ext_register, ext_deregister);

//     let index = &[1u8];
//     carved_rust::svo_set_block(svo_ptr, index.as_ptr(), index.len(), 2);

//     {
//         let esvo: &carved_rust::ExternalSVO = unsafe { &*svo_ptr };
//         esvo.svo.assert_contains(vec![
//             (0. , 0. , 0. , 1, 1),
//             (0.5, 0. , 0. , 1, 2),
//             (0. , 0.5, 0. , 1, 1),
//             (0.5, 0.5, 0. , 1, 1),
//             (0. , 0. , 0.5, 1, 1),
//             (0.5, 0. , 0.5, 1, 1),
//             (0. , 0.5, 0.5, 1, 1),
//             (0.5, 0.5, 0.5, 1, 1)]);
//     }

//     let maybe_hit = carved_rust::svo_cast_ray(svo_ptr, Vec3::new(0.52, 2., 0.52), Vec3::new(0., -1., 0.));
//     assert!(maybe_hit.is_some != 0);
//     assert_approx_eq_eps!(maybe_hit.value, Vec3::new(0.52, 1., 0.52), 0.001);

//     let maybe_hit2 = carved_rust::svo_cast_ray(svo_ptr, Vec3::new(1.52, 2., 0.52), Vec3::new(0., -1., 0.));
//     assert!(maybe_hit2.is_some == 0);

//     carved_rust::svo_destroy(svo_ptr);
// }
