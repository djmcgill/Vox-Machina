#![allow(dead_code)]

#[macro_use]
extern crate nalgebra;
extern crate byteorder; // could switch to bincode if needed

#[cfg(test)]
extern crate quickcheck;

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

macro_rules! guard(
	($e:expr) => (if !$e { return None })
);

mod svo;
pub mod ffi;