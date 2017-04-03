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
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate quickcheck;
extern crate glutin;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate arrayvec;
extern crate num;
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! { }
}

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

macro_rules! guard(
	($e:expr) => (if !$e { return None })
);

mod app;
mod svo;
mod graphics;

use app::App;

pub fn main() {
    if let Err(ref e) = App::launch("Vox Machina", app::DEFAULT_CONFIG) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
