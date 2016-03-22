use byteorder::{LittleEndian, WriteBytesExt};

use svo::*;

impl VoxelData {
    fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.write_i32::<LittleEndian>(self.voxel_type);
        vec
    }
}

impl SVO {
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

                let octant_indices = octants.iter().map(|octant| {
                    octant.as_bytes_in(vec);
                    vec.len() as u64 - 1
                });

                // error: cannot borrow `*vec` as mutable because previous closure requires unique access [E0501]
                //vec.push(1u8); // Tag the bytes as an octant
                for octant_index in octant_indices {
                    //vec.write_u64::<LittleEndian>(octant_index).unwrap();
                }
            }
        };

    }
}