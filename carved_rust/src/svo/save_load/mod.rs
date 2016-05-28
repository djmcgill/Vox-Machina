use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use nalgebra::{Vec3, zero};
use std::io::{Read, Write, Result, Error, ErrorKind};
use svo::*;

const VOXEL_TAG: u8 = 1;
const OCTANT_TAG: u8 = 2;

pub trait ReadSVO: Read {
    fn read_voxel_data(&mut self) -> Result<VoxelData> {
        let voxel_type = try! { self.read_i32::<LittleEndian>() };
        Ok(VoxelData::new(voxel_type))
    }

    fn read_svo(&mut self, registration_fns: &RegistrationFunctions) -> Result<SVO> {
        self.read_svo_from(registration_fns, zero(), 0)
    }

    fn read_svo_from(
            &mut self,
            registration_fns: &RegistrationFunctions,
            origin: Vec3<f32>,
            depth: i32) -> Result<SVO> {
        let mut stack: Vec<SVO> = vec![];

        let mut b = [0];
        while try!{ self.read(&mut b) } > 0 { match b[0] {
            VOXEL_TAG => {
                let data = try!{ self.read_voxel_data() };
                stack.push(SVO::new_voxel(data, 0));
            },
            OCTANT_TAG if stack.len() < 8 => {
                let msg = "Cannot interpret bytes as SVO; found an Octant when there weren't enough children.";
                return Err(Error::new(ErrorKind::InvalidData, msg));
            },
            OCTANT_TAG => {
                // TODO: surely there's a better way
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
            },
            other => {
                let msg = format!("Invalid SVO type specifier '{}' found", other);
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
        }}

        if stack.len() != 1 {
            let msg = format!("Finished reading the bytes and found {} root SVOs when there should only be one.", stack.len());
            return Err(Error::new(ErrorKind::InvalidData, msg));
        };
        // TODO: register_all the SVO
        Ok(stack.pop().unwrap())
    }
}

impl<R: ReadBytesExt> ReadSVO for R {}

pub trait WriteSVO: Write {
    fn write_voxel(&mut self, voxel: VoxelData) -> Result<()> {
        self.write_i32::<LittleEndian>(voxel.voxel_type)
    }

    /// The data is encoded like a stack. Each set of bytes is tagged with VOXEL_TAG or OCTANT_TAG.
    /// If you find a VOXEL_TAG, then read a VoxelData and add this voxel to the stack.
    /// If you find a OCTANT_TAG, then pop previous 8 SVOs (which may or may not be Voxels) and add an Octant.
    fn write_svo(&mut self, svo: &SVO) -> Result<()> {
        match *svo {
            SVO::Voxel { data, .. } => {
                try!{ self.write(&[VOXEL_TAG]) };
                try!{ self.write_voxel(data) };
                Ok(())
            },
            SVO::Octants (ref octants) => {
                for octant in octants { try! { self.write_svo(octant) }; }
                try! { self.write(&[OCTANT_TAG]) }; // Tag the previous 8 SVOs as a voxel
                Ok(())
            }
        }
    }
}
impl<W: WriteBytesExt> WriteSVO for W {}
