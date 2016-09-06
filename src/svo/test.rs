use quickcheck::*;

use nalgebra::{ApproxEq, Vec3, zero};
use svo::*;

impl Clone for SVO {
    fn clone(&self) -> SVO {
        match *self {
            SVO::Voxel { data } => SVO::new_voxel(data),
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
                    SVO::new_voxel(VoxelData::new(data_type))
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
fn create_split() {
    let svo = {
        let data = VoxelData::new(1);
    	let mut svo = SVO::new_voxel(data);
        svo.set_block(&[2], VoxelData::new(0));
        svo.set_block(&[3], VoxelData::new(0));
        svo.set_block(&[6], VoxelData::new(0));
        svo.set_block(&[7], VoxelData::new(0));
        svo.set_block(&[1, 3], VoxelData::new(2));
        svo
    };

    match svo {
        SVO::Voxel{ .. } => panic!("Unexpected Voxel found"),
        SVO::Octants(ref octants) => {
        	octants[0].assert_is_voxel(VoxelData::new(1));

            match *octants[1] {
                SVO::Voxel{ .. } => panic!("Unexpected Voxel found"),
                SVO::Octants(ref sub_octants) => {
                    sub_octants[0].assert_is_voxel(VoxelData::new(1));
                    sub_octants[1].assert_is_voxel(VoxelData::new(1));
                    sub_octants[2].assert_is_voxel(VoxelData::new(1));
                    sub_octants[3].assert_is_voxel(VoxelData::new(2));
                    sub_octants[4].assert_is_voxel(VoxelData::new(1));
                    sub_octants[5].assert_is_voxel(VoxelData::new(1));
                    sub_octants[6].assert_is_voxel(VoxelData::new(1));
                    sub_octants[7].assert_is_voxel(VoxelData::new(1));
                }
            }
            octants[2].assert_is_voxel(VoxelData::new(0));
            octants[3].assert_is_voxel(VoxelData::new(0));
            octants[4].assert_is_voxel(VoxelData::new(1));
            octants[5].assert_is_voxel(VoxelData::new(1));
            octants[6].assert_is_voxel(VoxelData::new(0));
            octants[7].assert_is_voxel(VoxelData::new(0));
        }
    }
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
	pub fn assert_is_voxel(&self, expected_data: VoxelData) {
		match *self {
			SVO::Octants(_) => panic!("Found Octants when expecting a Voxel!"),
			SVO::Voxel{ data } => assert_eq!(data, expected_data)
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
            SVO::Voxel { data: VoxelData { voxel_type, .. } } =>
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
        let data = VoxelData::new(1);
        let mut svo = SVO::new_voxel(data);
        svo.set_block(&[2], VoxelData::new(0));
        svo.set_block(&[3], VoxelData::new(0));
        svo.set_block(&[6], VoxelData::new(0));
        svo.set_block(&[7], VoxelData::new(0));
        svo
    }
}
