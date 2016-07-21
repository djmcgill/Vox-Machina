use quickcheck::*;

// use carved_rust;
use nalgebra::{ApproxEq, Vec3, zero};
use std::cell::{Cell, RefCell};
use svo::*;

fn register(_: Vec3<f32>, _: i32, _: VoxelData) -> u32 { 0 }
fn deregister(_: u32) {}

// WARNING: Clone is for non-FFI testing purposes only.
// Cloning a SVO that has been registered externally is BAD.
impl Clone for SVO {
    fn clone(&self) -> SVO {
        match *self {
            SVO::Voxel { data, external_id } => SVO::new_voxel(data, external_id),
            SVO::Octants(ref octants) => SVO::new_octants(|ix| *octants[ix as usize].clone())
        }
    }
}

impl Arbitrary for SVO {
    fn arbitrary<G: Gen>(g: &mut G) -> SVO {
        fn fixed_size_arbitrary<G: Gen>(g: &mut G, size: usize) -> SVO {
            match size {
                0 => {
                    let data_type = g.gen::<bool>() as i32;
                    SVO::new_voxel(VoxelData::new(data_type), Arbitrary::arbitrary(g))
                },
                n => {
                    let new_size = n/8;
                    SVO::Octants([
                        Box::new(fixed_size_arbitrary(g, new_size)), Box::new(fixed_size_arbitrary(g, new_size)),
                        Box::new(fixed_size_arbitrary(g, new_size)), Box::new(fixed_size_arbitrary(g, new_size)),
                        Box::new(fixed_size_arbitrary(g, new_size)), Box::new(fixed_size_arbitrary(g, new_size)),
                        Box::new(fixed_size_arbitrary(g, new_size)), Box::new(fixed_size_arbitrary(g, new_size))
                    ])
                }
            }
        }
        let height = g.size();
        fixed_size_arbitrary(g, height)
    }

    // fn shrink(&self) -> Box<Iterator<Item=SVO>> {
    //     match *self {
    //         SVO::Voxel { data, external_id } => {
    //             Box::new((data, external_id).shrink().map(|(new_data, new_external_id)| SVO::new_voxel(new_data, new_external_id)))
    //         },
    //         SVO::Octants(ref octants) => Box::new(vec![
    //             *octants[0].clone(), *octants[1].clone(),
    //             *octants[2].clone(), *octants[3].clone(),
    //             *octants[4].clone(), *octants[5].clone(),
    //             *octants[6].clone(), *octants[7].clone()
    //         ].into_iter())
    //     }
    // }
}

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

#[test]
fn above_axis_index_cancel() {
    fn check_above_axis_index_cancel(ix: u8) -> TestResult {
        if ix >= 8 { return TestResult::discard(); }
        TestResult::from_bool(index(above_axis(ix)) == ix)
    }
    quickcheck(check_above_axis_index_cancel as fn(u8) -> TestResult)
}

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
