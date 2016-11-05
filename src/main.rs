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

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;
#[macro_use]
extern crate nalgebra;
extern crate byteorder;
#[cfg(test)]
extern crate quickcheck;

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

macro_rules! guard(
	($e:expr) => (if !$e { return None })
);

mod svo;
mod graphics;

use gfx::{Bundle, texture};
use svo::{SVO, VoxelData};
use graphics::*;

struct App<R: gfx::Resources>{
    bundle: Bundle<R, pipe::Data<R>>,
    mapping: gfx::mapping::RWable<R, Instance>,
    svo: SVO
}

fn update_instances(instances: &mut [Instance]) {} // TODO
const MAX_INSTANCE_COUNT: usize = 2048;

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
        use cgmath::{Point3, Vector3};
        use cgmath::{Transform, AffineMatrix3};
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/cube_150.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/cube_150.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let svo = SVO::new_voxel(VoxelData::new(1));

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

        let pso = factory.create_pipeline_simple(
            vs.select(init.backend).unwrap(),
            ps.select(init.backend).unwrap(),
            pipe::new()
        ).unwrap();

        let view: AffineMatrix3<f32> = Transform::look_at(
            Point3::new(1.5f32, -5.0, 3.0),
            Point3::new(0f32, 0.0, 0.0),
            Vector3::unit_z(),
        );
        let proj = cgmath::perspective(cgmath::deg(45.0f32), init.aspect_ratio, 1.0, 10.0);

        let data = pipe::Data {
            vbuf: quad_vertices,
            instance: instance_buffer,
            transform: (proj * view.mat).into(),
            locals: factory.create_constant_buffer(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            out_color: init.color,
            out_depth: init.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            mapping: instance_mapping,
            svo: svo,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        let mut instances = self.mapping.read_write();
        update_instances(&mut instances);
        let locals = Locals { transform: self.bundle.data.transform };
        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        self.bundle.encode(encoder);
    }
}

pub fn main() {
    use gfx_app::Application;
    App::launch_default("Cube example");
}
