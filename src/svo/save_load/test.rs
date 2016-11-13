use svo::*;
use svo::save_load::{ReadSVO, WriteSVO};
use std::io::Cursor;

#[test]
fn save_load() {
    let mut svo = SVO::floor();

    svo.set_block(&[1, 3], VoxelData::new(2));
    svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
            (0.5 , 0.  , 0.  , 2, 1),
            (0.75, 0.  , 0.  , 2, 1),
            (0.5 , 0.25, 0.  , 2, 1),
            (0.75, 0.25, 0.  , 2, 2),
            (0.5 , 0.  , 0.25, 2, 1),
            (0.75, 0.  , 0.25, 2, 1),
            (0.5 , 0.25, 0.25, 2, 1),
            (0.75, 0.25, 0.25, 2, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);

    println!("==== SAVING ====");
    let mut bytes: Vec<u8> = vec![];
    bytes.write_svo(&svo).unwrap();
    println!("\n\n");

    // Just to make sure that we don't reuse the same memory
    let dummy_vec = vec![0u8; 500];
    println!("{:?}", dummy_vec[2]);

    println!("==== LOADING ====");
    let reader = &mut Cursor::new(bytes);
    let new_svo = reader.read_svo().unwrap();
    new_svo.assert_contains(vec![
        (0. , 0. , 0. , 1, 1),
            (0.5 , 0.  , 0.  , 2, 1),
            (0.75, 0.  , 0.  , 2, 1),
            (0.5 , 0.25, 0.  , 2, 1),
            (0.75, 0.25, 0.  , 2, 2),
            (0.5 , 0.  , 0.25, 2, 1),
            (0.75, 0.  , 0.25, 2, 1),
            (0.5 , 0.25, 0.25, 2, 1),
            (0.75, 0.25, 0.25, 2, 1),
        (0. , 0.5, 0. , 1, 0),
        (0.5, 0.5, 0. , 1, 0),
        (0. , 0. , 0.5, 1, 1),
        (0.5, 0. , 0.5, 1, 1),
        (0. , 0.5, 0.5, 1, 0),
        (0.5, 0.5, 0.5, 1, 0)]);
}
