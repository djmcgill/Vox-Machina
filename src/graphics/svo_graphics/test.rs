use graphics::Instance;
use svo::{SVO, VoxelData};

impl Instance {
     fn zero() -> Instance {
        Instance {
            translate: [0.0, 0.0, 0.0],
            scale: 0.0,
        }
     }
}

#[test]
fn voxel_instance() {
    let svo = SVO::new_voxel(VoxelData::new(1));
    let mut instances: [Instance; 1] = [Instance::zero()];
    let count = svo.fill_instances(&mut instances);
    assert_eq!(count, 1);
    let expected_instance = Instance {
        translate: [0.0, 0.0, 0.0],
        scale: 1.0,
    };
    assert_eq!(expected_instance.translate, instances[0].translate);
    assert_eq!(expected_instance.scale, instances[0].scale);
}

#[test]
fn octants_instance() {
    let svo = SVO::new_octants(|_| SVO::new_voxel(VoxelData::new(1)));

    let mut instances: [Instance; 8] = [
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
    ];
    let count = svo.fill_instances(&mut instances);
    assert_eq!(count, 8);
    let expected_instance_0 = Instance {
        translate: [0.0, 0.0, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_0.translate, instances[0].translate);
    assert_eq!(expected_instance_0.scale, instances[0].scale);

    let expected_instance_1 = Instance {
        translate: [0.5, 0.0, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_1.translate, instances[1].translate);
    assert_eq!(expected_instance_1.scale, instances[1].scale);

    let expected_instance_2 = Instance {
        translate: [0.0, 0.5, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_2.translate, instances[2].translate);
    assert_eq!(expected_instance_2.scale, instances[2].scale);
    
    let expected_instance_3 = Instance {
        translate: [0.5, 0.5, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_3.translate, instances[3].translate);
    assert_eq!(expected_instance_3.scale, instances[3].scale);

    let expected_instance_4 = Instance {
        translate: [0.0, 0.0, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_4.translate, instances[4].translate);
    assert_eq!(expected_instance_4.scale, instances[4].scale);

    let expected_instance_5 = Instance {
        translate: [0.5, 0.0, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_5.translate, instances[5].translate);
    assert_eq!(expected_instance_5.scale, instances[5].scale);

    let expected_instance_6 = Instance {
        translate: [0.0, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_6.translate, instances[6].translate);
    assert_eq!(expected_instance_6.scale, instances[6].scale);
    
    let expected_instance_7 = Instance {
        translate: [0.5, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_7.translate, instances[7].translate);
    assert_eq!(expected_instance_7.scale, instances[7].scale);
}

#[test]
fn octants_instance_two_inner() {
    let svo = SVO::new_octants(|i|
        if i != 5 {
            SVO::new_voxel(VoxelData::new(1))
        } else {
            SVO::new_octants(|_| SVO::new_voxel(VoxelData::new(1)))
        }
    );

    let mut instances: [Instance; 24] = [
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
    ];

    let count = svo.fill_instances(&mut instances);
    assert_eq!(count, 15);
    let expected_instance_0 = Instance {
        translate: [0.5, 0.0, 0.5],
        scale: 0.25,
    };
    assert_eq!(expected_instance_0.translate, instances[5].translate);
    assert_eq!(expected_instance_0.scale, instances[5].scale);

    let expected_instance_1 = Instance {
        translate: [0.75, 0.0, 0.5],
        scale: 0.25,
    };
    assert_eq!(expected_instance_1.translate, instances[6].translate);
    assert_eq!(expected_instance_1.scale, instances[6].scale);

    let expected_instance_2 = Instance {
        translate: [0.5, 0.25, 0.5],
        scale: 0.25,
    };
    assert_eq!(expected_instance_2.translate, instances[7].translate);
    assert_eq!(expected_instance_2.scale, instances[7].scale);
    
    let expected_instance_3 = Instance {
        translate: [0.75, 0.25, 0.5],
        scale: 0.25,
    };
    assert_eq!(expected_instance_3.translate, instances[8].translate);
    assert_eq!(expected_instance_3.scale, instances[8].scale);

    let expected_instance_4 = Instance {
        translate: [0.5, 0.0, 0.75],
        scale: 0.25,
    };
    assert_eq!(expected_instance_4.translate, instances[9].translate);
    assert_eq!(expected_instance_4.scale, instances[9].scale);

    let expected_instance_5 = Instance {
        translate: [0.75, 0.0, 0.75],
        scale: 0.25,
    };
    assert_eq!(expected_instance_5.translate, instances[10].translate);
    assert_eq!(expected_instance_5.scale, instances[10].scale);

    let expected_instance_6 = Instance {
        translate: [0.5, 0.25, 0.75],
        scale: 0.25,
    };
    assert_eq!(expected_instance_6.translate, instances[11].translate);
    assert_eq!(expected_instance_6.scale, instances[11].scale);
    
    let expected_instance_7 = Instance {
        translate: [0.75, 0.25, 0.75],
        scale: 0.25,
    };
    assert_eq!(expected_instance_7.translate, instances[12].translate);
    assert_eq!(expected_instance_7.scale, instances[12].scale);
}

#[test]
fn octants_instance_two_outer() {
    let svo = SVO::new_octants(|i|
        if i != 5 {
            SVO::new_voxel(VoxelData::new(1))
        } else {
            SVO::new_octants(|_| SVO::new_voxel(VoxelData::new(1)))
        }
    );

    let mut instances: [Instance; 24] = [
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(), Instance::zero(),
    ];

    let count = svo.fill_instances(&mut instances);
    assert_eq!(count, 15);
    let expected_instance_0 = Instance {
        translate: [0.0, 0.0, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_0.translate, instances[0].translate);
    assert_eq!(expected_instance_0.scale, instances[0].scale);

    let expected_instance_1 = Instance {
        translate: [0.5, 0.0, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_1.translate, instances[1].translate);
    assert_eq!(expected_instance_1.scale, instances[1].scale);

    let expected_instance_2 = Instance {
        translate: [0.0, 0.5, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_2.translate, instances[2].translate);
    assert_eq!(expected_instance_2.scale, instances[2].scale);
    
    let expected_instance_3 = Instance {
        translate: [0.5, 0.5, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_3.translate, instances[3].translate);
    assert_eq!(expected_instance_3.scale, instances[3].scale);

    let expected_instance_4 = Instance {
        translate: [0.0, 0.0, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_4.translate, instances[4].translate);
    assert_eq!(expected_instance_4.scale, instances[4].scale);

    let expected_instance_6 = Instance {
        translate: [0.0, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_6.translate, instances[13].translate);
    assert_eq!(expected_instance_6.scale, instances[13].scale);
    
    let expected_instance_7 = Instance {
        translate: [0.5, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_7.translate, instances[14].translate);
    assert_eq!(expected_instance_7.scale, instances[14].scale);
}

#[test] #[should_panic]
fn panic_on_overflow() {
    let svo = SVO::new_voxel(VoxelData::new(1));
    let mut instances: [Instance; 0] = [];
    svo.fill_instances(&mut instances);
}

#[test]
fn octants_instance_empty() {
    let svo = SVO::new_octants(|i| {
        let data = [1, 0, 1, 0, 1, 1, 1, 1][i as usize];
        SVO::new_voxel(VoxelData::new(data))
    });

    let mut instances: [Instance; 8] = [
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
        Instance::zero(), Instance::zero(),
    ];
    let count = svo.fill_instances(&mut instances);
    assert_eq!(count, 6);
    let expected_instance_0 = Instance {
        translate: [0.0, 0.0, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_0.translate, instances[0].translate);
    assert_eq!(expected_instance_0.scale, instances[0].scale);

    let expected_instance_2 = Instance {
        translate: [0.0, 0.5, 0.0],
        scale: 0.5,
    };
    assert_eq!(expected_instance_2.translate, instances[1].translate);
    assert_eq!(expected_instance_2.scale, instances[1].scale);

    let expected_instance_4 = Instance {
        translate: [0.0, 0.0, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_4.translate, instances[2].translate);
    assert_eq!(expected_instance_4.scale, instances[2].scale);

    let expected_instance_5 = Instance {
        translate: [0.5, 0.0, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_5.translate, instances[3].translate);
    assert_eq!(expected_instance_5.scale, instances[3].scale);

    let expected_instance_6 = Instance {
        translate: [0.0, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_6.translate, instances[4].translate);
    assert_eq!(expected_instance_6.scale, instances[4].scale);
    
    let expected_instance_7 = Instance {
        translate: [0.5, 0.5, 0.5],
        scale: 0.5,
    };
    assert_eq!(expected_instance_7.translate, instances[5].translate);
    assert_eq!(expected_instance_7.scale, instances[5].scale);
}
