use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write, Result};

use svo::*;

const VOXEL_TAG: u8 = 1;
const OCTANT_TAG: u8 = 2;

// TODO: put this into the rust Read/Write traits
impl VoxelData {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32::<LittleEndian>(self.voxel_type)
    }

    fn read_from<R: Read>(reader: &mut R) -> Result<VoxelData> {
        let voxel_type = try! { reader.read_i32::<LittleEndian>() };
        Ok(VoxelData::new(voxel_type))
    }
}

impl SVO {
    /// The data is encoded like a stack. Each set of bytes is tagged with VOXEL_TAG or OCTANT_TAG.
    /// If you find a VOXEL_TAG, then read a VoxelData and add this voxel to the stack.
    /// If you find a OCTANT_TAG, then pop previous 8 SVOs (which may or may not be Voxels) and add an Octant.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            SVO::Voxel { data, .. } => {
                try!{ writer.write(&[VOXEL_TAG]) };
                try!{ data.write_to(writer) };
                Ok(())
            },
            SVO::Octants (ref octants) => {
                for octant in octants { try! { octant.write_to(writer) }; }
                try! { writer.write(&[OCTANT_TAG]) }; // Tag the previous 8 SVOs as a voxel
                Ok(())
            }
        }
    }

    // TODO: implement Result for this properly.
    pub fn read_from<R: Read>(reader: &mut R) -> Result<SVO> {
        let mut stack: Vec<SVO> = vec![];

        let mut b = [0];
        while try!{ reader.read(&mut b) } > 0 {
            match b[0] {
                VOXEL_TAG => {
                    let data = try!{ VoxelData::read_from(reader) };
                    stack.push(SVO::new_voxel(data, 0));
                },
                OCTANT_TAG if stack.len() < 8 => panic!("Cannot interpret bytes as SVO; found an Octant when there weren't enough children."),
                OCTANT_TAG => {
                    // TODO: sort this out.
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
                    ]));
                }
                other => panic!("Invalid SVO type specifier '{}' found", other)
            }
        }

        if stack.len() != 1 {
            panic!("Finished reading the bytes and found {} root SVOs when there should only be one.", stack.len())
        };
        // TODO: register_all the SVO
        Ok(stack.pop().unwrap())
    }
}






