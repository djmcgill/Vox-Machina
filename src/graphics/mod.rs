pub mod svo_graphics;
pub mod camera;
pub mod key_down;

use gfx;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    #[derive(PartialEq)]
    vertex Instance {
        translate: [f32; 3] = "a_Translate",
        side_width: f32 = "a_SideWidth",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
