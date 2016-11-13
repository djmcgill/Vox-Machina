use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write, Result, Error, ErrorKind};
use svo::*;
use std::mem;
use arrayvec::ArrayVec;

#[cfg(test)]
mod test;

const VOXEL_TAG: u8 = 1;
const OCTANT_TAG: u8 = 2;

pub trait ReadSVO: Read {
    fn read_voxel_data(&mut self) -> Result<VoxelData> {
        let voxel_type = try! { self.read_i32::<LittleEndian>() };
        Ok(VoxelData::new(voxel_type))
    }

    fn read_svo(&mut self) -> Result<SVO> {
        self.read_svo_from()
    }

    fn read_svo_from(&mut self) -> Result<SVO> {
        let mut b = [0];
        let bytes_read = try!{ self.read(&mut b) };
        if bytes_read == 0 {
            let msg = "Unexpected end of input stream.";
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }

        match b[0] {
            VOXEL_TAG => {
                let data = try!{ self.read_voxel_data() };
                // let data = VoxelData::new(1);
                Ok(SVO::new_voxel(data))
            },
            OCTANT_TAG => {
                // TODO: use mem::uninitialized here
                let mut octants: ArrayVec<[Box<SVO>; 8]> =
                    ArrayVec::from (
                        [Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))),
                        Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1))), Box::new(SVO::new_voxel(VoxelData::new(1)))
                    ]);

                for ix in 0..8 {
                    let result: SVO = try!{ self.read_svo_from() };
                    mem::replace(&mut octants[ix as usize], Box::new(result));
                }
                Ok(SVO::Octants(octants))
            },
            other => {
                let msg = format!("Invalid SVO type specifier '{}' found", other);
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}

impl<R: ReadBytesExt> ReadSVO for R {}

pub trait WriteSVO: Write {
    fn write_voxel(&mut self, voxel: VoxelData) -> Result<()> {
        self.write_i32::<LittleEndian>(voxel.voxel_type)
    }

    fn write_svo(&mut self, svo: &SVO) -> Result<()> {
        match *svo {
            SVO::Voxel { data, .. } => {
                try!{ self.write(&[VOXEL_TAG]) };
                try!{ self.write_voxel(data) };
                Ok(())
            },
            SVO::Octants (ref octants) => {
                try! { self.write(&[OCTANT_TAG]) }; // Tag the next 8 SVOs as a voxel
                for octant in octants { try! { self.write_svo(octant) }; }
                Ok(())
            }
        }
    }
}
impl<W: WriteBytesExt> WriteSVO for W {}
