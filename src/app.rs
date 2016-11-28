use env_logger;
use gfx_device_gl;
use gfx_window_glutin;
use gfx;
use gfx::{Bundle, Factory, texture};
use glutin;
use graphics::*;
use graphics::camera::OverheadCamera;
use nalgebra;
use svo::SVO;
use std::collections::HashSet;
use std::time;

pub struct Config {
    pub size: (u16, u16),
}

pub const DEFAULT_CONFIG: Config = Config { size: (800, 520) };

type R = gfx_device_gl::Resources;
type C = gfx_device_gl::CommandBuffer;
type F = gfx_device_gl::Factory;

pub struct App {
    bundle: Bundle<R, pipe::Data<R>>,
    mapping: gfx::mapping::RWable<R, Instance>,
    svo: SVO,
    svo_max_height: i32,
    encoder: gfx::Encoder<R, C>,
    camera: OverheadCamera,
    proj: nalgebra::Matrix4<f32>,
    keys_down: HashSet<glutin::VirtualKeyCode>, // TODO: use EnumSet instead
    drag_mouse_position: Option<(i32, i32)>,
    current_cursor_position: (i32, i32),
    last_instant: time::Instant,
}

const MAX_INSTANCE_COUNT: u32 = 2048;

pub struct Init {
    pub color: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub depth: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub aspect_ratio: f32,
}

impl App {
    pub fn launch(title: &str, config: Config) {
        use nalgebra::PerspectiveMatrix3;
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
            let dt = app.update_dt();
            
            // quit when Esc is pressed.
            for event in window.poll_events() {
                // debug!("{:?}", event);
                match event {
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::Closed => break 'main,
                    glutin::Event::Resized(width, height) => {
                        let new_aspect_ratio = width as f32 / height as f32;
                        app.proj = PerspectiveMatrix3::<f32>::new(new_aspect_ratio, 45.0f32.to_radians(), 1.0, 100.0).to_matrix();
                    }
                    glutin::Event::KeyboardInput(element_state, _, Some(key_code)) => {
                        app.update_keys_down(element_state, key_code);
                    }
                    glutin::Event::MouseMoved(x, y) => {
                        if app.drag_mouse_position.is_some() {
                            let dy = (y - app.current_cursor_position.1) as f32;
                            if dy != 0.0 { app.camera.rotate(dt, dy); };
                        };
                        app.current_cursor_position = (x, y);
                        
                    },
                    glutin::Event::MouseInput(press_state, glutin::MouseButton::Left) => 
                        app.drag_mouse_position = match press_state {
                            glutin::ElementState::Pressed => Some(app.current_cursor_position),
                            glutin::ElementState::Released => None,
                        },
                    _ => {}
                }
            }

            app.camera.update_from_keys(dt, &app.keys_down);

            // draw a frame
            app.render(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        }
    }

    pub fn update_keys_down(&mut self,
                            element_state: glutin::ElementState,
                            key_code: glutin::VirtualKeyCode) {
        match element_state {
            glutin::ElementState::Pressed => {
                self.keys_down.insert(key_code);
            }
            glutin::ElementState::Released => {
                let was_removed = self.keys_down.remove(&key_code);
                assert!(was_removed); // If false, weird things are happening
            }
        }
    }

    fn update_dt(&mut self) -> f32 {
        let now = time::Instant::now();
        let duration = now.duration_since(self.last_instant);
        let dt = (duration.as_secs() * 1000) as f32 +
                        (duration.subsec_nanos() / 1000_000) as f32; 
        self.last_instant = now;
        dt
    }

    fn new(mut factory: F, init: Init) -> Self {
        use gfx::traits::FactoryExt;
        use nalgebra::*;

        let (instance_buffer, mut instance_mapping) =
            factory.create_buffer_persistent_rw(MAX_INSTANCE_COUNT as usize,
                                                gfx::buffer::Role::Vertex,
                                                gfx::Bind::empty());
        let svo = SVO::example();
        let max_height = 2;
        let instance_count = {
            let mut instances = instance_mapping.read_write();
            svo.fill_instances(&mut instances, max_height)
        };
        assert!(instance_count <= MAX_INSTANCE_COUNT);

        let (quad_vertices, mut slice) =
            factory.create_vertex_buffer_with_slice(&svo_graphics::CUBE_VERTS,
                                                    &svo_graphics::CUBE_INDICES[..]);
        slice.instances = Some((instance_count, 0));

        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
            ).unwrap();

        let sinfo = texture::SamplerInfo::new(texture::FilterMethod::Bilinear,
                                              texture::WrapMode::Clamp);

        let vs = include_bytes!("shader/cube_150.glslv");
        let ps = include_bytes!("shader/cube_150.glslf");
        let pso = factory.create_pipeline_simple(vs, ps, pipe::new()).unwrap();

        let data = pipe::Data {
            vbuf: quad_vertices,
            instance: instance_buffer,
            locals: factory.create_constant_buffer::<Locals>(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            out_color: init.color,
            out_depth: init.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            mapping: instance_mapping,
            svo: svo,
            svo_max_height: max_height,
            keys_down: HashSet::new(),
            encoder: factory.create_command_buffer().into(),
            camera: OverheadCamera::new(),
            drag_mouse_position: None,
            current_cursor_position: (0, 0),
            last_instant: time::Instant::now(),
            proj: PerspectiveMatrix3::<f32>::new(init.aspect_ratio,
                                                 45.0f32.to_radians(),
                                                 1.0,
                                                 100.0)
                      .to_matrix(),
        }
    }

    fn render<D>(&mut self, device: &mut D)
        where D: gfx::Device<Resources = R, CommandBuffer = C>
    {
        {
            let mut instances = self.mapping.read_write();
            let instance_count = self.svo.fill_instances(&mut instances, self.svo_max_height);
            self.bundle.slice.instances = Some((instance_count, 0));
        }

        let view = self.camera.view();
        let locals = Locals { transform: *(self.proj * view).as_ref() };
        self.encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

        self.encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        self.encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        self.encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        self.bundle.encode(&mut self.encoder);
        self.encoder.flush(device);
    }
}
