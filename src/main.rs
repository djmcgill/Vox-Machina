// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Modified by David McGillicuddy
#![allow(dead_code)]

extern crate env_logger;
#[macro_use]
extern crate gfx;
#[macro_use]
extern crate nalgebra;
extern crate byteorder;
#[cfg(test)]
extern crate quickcheck;
extern crate glutin;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate arrayvec;


macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

macro_rules! guard(
	($e:expr) => (if !$e { return None })
);

mod app;
mod svo;
mod graphics;
mod camera;

use app::App;

pub fn main() {
    App::launch("Vox Machina", app::DEFAULT_CONFIG);
}
