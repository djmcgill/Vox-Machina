use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Read;

use svo::*;

// TODO: put this into the rust Read/Write traits
impl VoxelData {
    fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.write_i32::<LittleEndian>(self.voxel_type).unwrap();
        vec
    }

    fn read(mut bytes: &[u8]) -> VoxelData {
        let voxel_type = bytes.read_i32::<LittleEndian>().unwrap();
        VoxelData::new(voxel_type)
    }
}

impl SVO {
    /// The data is encoded like a stack. Each set of bytes is tagged with 0u8 or 1u8.
    /// If you find a 0u8, then read a VoxelData and add this voxel to the stack.
    /// If you find a 1u8, then pop previous 8 SVOs (which may or may not be Voxels) and add an Octant.
    pub fn as_bytes(&self) -> Box<[u8]> {
        let mut bytes = vec![];
        self.as_bytes_in(&mut bytes);
        bytes.into_boxed_slice()
    }

    fn as_bytes_in(&self, vec: &mut Vec<u8>) {
        match *self {
            SVO::Voxel { data, .. } => {
                vec.push(0u8); // Tag the bytes as a voxel
                vec.append(&mut data.as_bytes())
            },
            SVO::Octants (ref octants) => {

                for octant in octants {
                    octant.as_bytes_in(vec);
                }

                vec.push(1u8); // Tag the previous 8 SVOs as a voxel
            }
        }
    }

    pub fn from_bytes(mut bytes: &[u8]) -> SVO {
        let mut stack: Vec<SVO> = vec![];
        let mut b = [0];
        bytes.read(&mut b).unwrap();
        match b[0] {
            0u8 => stack.push(SVO::new_voxel(VoxelData::read(bytes), 0)),
            1u8 if stack.len() < 8 => panic!("Cannot interpret bytes as SVO; found an Octant when there weren't enough children."),
            1u8 => {
                let octant7 = Box::new(stack.pop().unwrap());
                let octant6 = Box::new(stack.pop().unwrap());
                let octant5 = Box::new(stack.pop().unwrap());
                let octant4 = Box::new(stack.pop().unwrap());
                let octant3 = Box::new(stack.pop().unwrap());
                let octant2 = Box::new(stack.pop().unwrap());
                let octant1 = Box::new(stack.pop().unwrap());
                let octant0 = Box::new(stack.pop().unwrap());

                stack.push(SVO::Octants([
                    octant0, octant1, octant2, octant3,
                    octant4, octant5, octant6, octant7
                ]))
            }
            other => panic!(format!("Invalid SVO type specifier '{}' found", other))
        }

        if stack.len() != 1 {
            panic!(format!("Finished reading the bytes and found {} root SVOs when there should only be one.", stack.len()))
        };
        // TODO: register_all the SVO
        stack.pop().unwrap()
    }
}






