use svo::*;
use std::u8;

#[test]
fn flat_height_map() {
    let width = 4;
    let height = 4;

    let image: [u8; 16] = [127u8; 16];
    let svo = SVO::height_map(1, &image, width, height, &RegistrationFunctions::dummy());
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
        (0.5, 0. , 0. , 1, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)
    ]);
}

#[test]
fn full_height_map() {
    let width = 4;
    let height = 4;

    let image: [u8; 16] = [u8::MAX; 16];
    let svo = SVO::height_map(1, &image, width, height, &RegistrationFunctions::dummy());
    svo.assert_contains(vec![(0., 0., 0., 0, 1)]);
}

#[test]
fn empty_height_map() {
    let width = 4;
    let height = 4;

    let image: [u8; 16] = [0u8; 16];
    let svo = SVO::height_map(1, &image, width, height, &RegistrationFunctions::dummy());
    svo.assert_contains(vec![(0., 0., 0., 0, 0)]);
}