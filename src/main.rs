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

mod svo;
mod graphics;

use gfx::{Bundle, Factory, texture};
use svo::SVO;
use graphics::*;

pub struct Config {
    pub size: (u16, u16),
}

pub const DEFAULT_CONFIG: Config = Config {
    size: (800, 520),
};

type R = gfx_device_gl::Resources;
type C = gfx_device_gl::CommandBuffer;
type F = gfx_device_gl::Factory;

struct App {
    bundle: Bundle<R, pipe::Data<R>>,
    mapping: gfx::mapping::RWable<R, Instance>,
    svo: SVO,
    encoder: gfx::Encoder<R, C>,
}

fn update_instances(svo: &SVO, instances: &mut [Instance]) -> u32 {
    svo.fill_instances(instances)
}
const MAX_INSTANCE_COUNT: usize = 2048;

pub struct Init {
    pub color: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub depth: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub aspect_ratio: f32,
}

impl App {
    fn launch(title: &str, config: Config) {
        use gfx::traits::Device;

        env_logger::init().unwrap();
        let gl_version = glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (2, 0),
        };
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(config.size.0 as u32, config.size.1 as u32)
            .with_gl(gl_version)
            .with_vsync();
        let (window, mut device, factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
        let (width, height) = window.get_inner_size().unwrap();
        let init = Init {
            color: main_color,
            depth: main_depth,
            aspect_ratio: width as f32 / height as f32,
        };

        let mut app = Self::new(factory, init);

        'main: loop {
            // quit when Esc is pressed.
            for event in window.poll_events() {
                match event {
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::Closed => break 'main,
                    _ => {}
                }
            }
            // draw a frame
            app.render(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        }
    }

    fn new(mut factory: F, init: Init) -> Self {
        use gfx::traits::FactoryExt;
        use nalgebra;
        use nalgebra::ToHomogeneous;

        let svo = SVO::example();
        let (instance_buffer, mut instance_mapping) = factory
            .create_buffer_persistent_rw(MAX_INSTANCE_COUNT,
                                         gfx::buffer::Role::Vertex,
                                         gfx::Bind::empty());
        let instance_count = {
            let mut instances = instance_mapping.read_write();
            svo.fill_instances(&mut instances)
        };
        assert!(instance_count as usize <= MAX_INSTANCE_COUNT);

        let (quad_vertices, mut slice) = factory
            .create_vertex_buffer_with_slice(&svo_graphics::CUBE_VERTS, &svo_graphics::CUBE_INDICES[..]);
        slice.instances = Some((instance_count, 0));

        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
            ).unwrap();

        let sinfo = texture::SamplerInfo::new(
            texture::FilterMethod::Bilinear,
            texture::WrapMode::Clamp);

        let vs = include_bytes!("shader/cube_150.glslv");
        let ps = include_bytes!("shader/cube_150.glslf");
        let pso = factory.create_pipeline_simple(vs, ps, pipe::new()).unwrap();

        let eye = nalgebra::Point3::<f32>::new(1.5, -5.0, 3.0);
        let target = nalgebra::Point3::<f32>::new(0.0, 0.0, 0.0);
        let up = nalgebra::Vector3::<f32>::z();
        let view = nalgebra::Isometry3::<f32>::look_at_rh(&eye, &target, &up); // TODO: build from Rotation3::look_at_rh
        let proj = nalgebra::PerspectiveMatrix3::<f32>::new(init.aspect_ratio, 45.0f32.to_radians(), 1.0, 10.0);
        let transform = proj.to_matrix() * view.to_homogeneous();

        let data = pipe::Data {
            vbuf: quad_vertices,
            instance: instance_buffer,
            transform: transform.as_ref().clone(),
            locals: factory.create_constant_buffer(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            out_color: init.color,
            out_depth: init.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            mapping: instance_mapping,
            svo: svo,
            encoder: factory.create_command_buffer().into(),
        }
    }

    fn render<D>(&mut self, device: &mut D) where D: gfx::Device<Resources = R, CommandBuffer = C> {
        {
            let mut instances = self.mapping.read_write();
            let instance_count = update_instances(&self.svo, &mut instances);
            self.bundle.slice.instances = Some((instance_count, 0));
        }
        let locals = Locals { transform: self.bundle.data.transform };
        self.encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        self.encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        self.encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        self.encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        self.bundle.encode(&mut self.encoder);
        self.encoder.flush(device);
    }
}

pub fn main() {
    App::launch("Cube example", DEFAULT_CONFIG);
}
