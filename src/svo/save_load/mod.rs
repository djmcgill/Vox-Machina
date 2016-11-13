use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write, Result, Error, ErrorKind};
use svo::*;

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
        let mut b = [0];
        let bytes_read = try!{ self.read(&mut b) };
        if bytes_read == 0 {
            let msg = "Unexpected end of input stream.";
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }

        match b[0] {
            VOXEL_TAG => {
                self.read_voxel_data().map(|data| SVO::new_voxel(data))
            },
            OCTANT_TAG => {
                SVO::new_octants_mut_err(|_| self.read_svo())
            },
            other => {
                let msg = format!("Invalid SVO type specifier '{}' found", other);
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}

pub trait WriteSVO: Write {
    fn write_voxel(&mut self, voxel: VoxelData) -> Result<()> {
        let VoxelData { voxel_type } = voxel;
        self.write_i32::<LittleEndian>(voxel_type)
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

// Yes these are weird, but they really need to be here! Things don't implement ReadSVO by default!
impl<R: ReadBytesExt> ReadSVO for R {}
impl<W: WriteBytesExt> WriteSVO for W {}
