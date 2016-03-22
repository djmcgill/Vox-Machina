#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate nalgebra;
extern crate byteorder; // could switch to bincode if needed


mod svo;
pub mod carved_rust;

#[cfg(test)]
mod svo_tests;