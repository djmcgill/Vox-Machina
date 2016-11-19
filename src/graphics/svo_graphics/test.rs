use graphics::Instance;
use svo::{SVO, VoxelData};

impl Instance {
    fn zero() -> Instance {
        Instance {
            translate: [0.0, 0.0, 0.0],
            side_width: 0.0,
        }
    }
}

#[test]
fn voxel_instance() {
    let svo = SVO::new_voxel(VoxelData::new(1));
    let mut instances = vec![Instance::zero()];
    let count = svo.fill_instances(&mut instances, 2);
    assert_eq!(count, 1);
    let expected_instances = vec![Instance {
                                      translate: [0.0, 0.0, 0.0],
                                      side_width: 4.0,
                                  }];
    assert_eq!(expected_instances, instances);
}

#[test]
fn octants_instance() {
    let svo = SVO::new_octants(|_| SVO::new_voxel(VoxelData::new(1)));

    let mut instances = vec![Instance::zero(); 8];
    let count = svo.fill_instances(&mut instances, 2);
    assert_eq!(count, 8);
    let expected_instances = vec![
        Instance { translate: [0.0, 0.0, 0.0], side_width: 2.0 },
        Instance { translate: [2.0, 0.0, 0.0], side_width: 2.0 },
        Instance { translate: [0.0, 2.0, 0.0], side_width: 2.0 },
        Instance { translate: [2.0, 2.0, 0.0], side_width: 2.0 },
        Instance { translate: [0.0, 0.0, 2.0], side_width: 2.0 },
        Instance { translate: [2.0, 0.0, 2.0], side_width: 2.0 },
        Instance { translate: [0.0, 2.0, 2.0], side_width: 2.0 },
        Instance { translate: [2.0, 2.0, 2.0], side_width: 2.0 },
    ];
    instances.truncate(count as usize);
    assert_eq!(instances, expected_instances);
}

#[test]
fn octants_instance_two() {
    let svo = SVO::new_octants(|i| if i != 5 {
        SVO::new_voxel(VoxelData::new(1))
    } else {
        SVO::new_octants(|_| SVO::new_voxel(VoxelData::new(1)))
    });
    let mut instances = vec![Instance::zero(); 24];

    let count = svo.fill_instances(&mut instances, 2);
    assert_eq!(count, 15);

    let expected_instances = vec![
        Instance { translate: [0.0, 0.0, 0.0], side_width: 2.0 },
        Instance { translate: [2.0, 0.0, 0.0], side_width: 2.0 },
        Instance { translate: [0.0, 2.0, 0.0], side_width: 2.0 },
        Instance { translate: [2.0, 2.0, 0.0], side_width: 2.0 },
        Instance { translate: [0.0, 0.0, 2.0], side_width: 2.0 },

        Instance { translate: [2.0, 0.0, 2.0], side_width: 1.0 },
        Instance { translate: [3.0, 0.0, 2.0], side_width: 1.0 },
        Instance { translate: [2.0, 1.0, 2.0], side_width: 1.0 },
        Instance { translate: [3.0, 1.0, 2.0], side_width: 1.0 },
        Instance { translate: [2.0, 0.0, 3.0], side_width: 1.0 },
        Instance { translate: [3.0, 0.0, 3.0], side_width: 1.0 },
        Instance { translate: [2.0, 1.0, 3.0], side_width: 1.0 },
        Instance { translate: [3.0, 1.0, 3.0], side_width: 1.0 },

        Instance { translate: [0.0, 2.0, 2.0], side_width: 2.0 },
        Instance { translate: [2.0, 2.0, 2.0], side_width: 2.0 },
    ];
    instances.truncate(count as usize);
    assert_eq!(instances, expected_instances);
}

#[test]
#[should_panic]
fn panic_on_overflow() {
    let svo = SVO::new_voxel(VoxelData::new(1));
    let mut instances: [Instance; 0] = [];
    svo.fill_instances(&mut instances, 2);
}

#[test]
fn octants_instance_empty() {
    let svo = SVO::new_octants(|i| {
        let data = [1, 0, 1, 0, 1, 1, 1, 1][i as usize];
        SVO::new_voxel(VoxelData::new(data))
    });

    let mut instances = vec![Instance::zero(); 8];
    let count = svo.fill_instances(&mut instances, 3);
    assert_eq!(count, 6);

    let expected_instances = vec![
        Instance { translate: [0.0, 0.0, 0.0], side_width: 4.0 },
        Instance { translate: [0.0, 4.0, 0.0], side_width: 4.0 },
        Instance { translate: [0.0, 0.0, 4.0], side_width: 4.0 },
        Instance { translate: [4.0, 0.0, 4.0], side_width: 4.0 },
        Instance { translate: [0.0, 4.0, 4.0], side_width: 4.0 },
        Instance { translate: [4.0, 4.0, 4.0], side_width: 4.0 },
    ];
    instances.truncate(count as usize);
    assert_eq!(instances, expected_instances);
}
