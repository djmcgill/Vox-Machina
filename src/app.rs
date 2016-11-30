use env_logger;
use gfx_device_gl;
use gfx_window_glutin;
use gfx;
use gfx::{Bundle, Factory, texture};
use glutin;
use graphics::*;
use graphics::controller::*;
use nalgebra;
use nalgebra::PerspectiveMatrix3;

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
    svo_controller: SvoController,
    encoder: gfx::Encoder<R, C>,
    camera_controller: CameraController,
    proj: nalgebra::Matrix4<f32>,
    keys_down_controller: KeysDownController,
    mouse_position_controller: MousePositionController,
    dt_controller: DtController,
}

const MAX_INSTANCE_COUNT: u32 = 2048;

pub struct Init {
    pub color: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub depth: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub aspect_ratio: f32,
}

impl App {
    pub fn launch(title: &str, config: Config) {
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
        app.main_loop(&window, &mut device);
        
    }

    fn main_loop<D>(&mut self, window: &glutin::Window, mut device: &mut D) 
            where D: gfx::Device<Resources=R, CommandBuffer=C> { loop {
        use glutin::Event::*;
        let dt = self.dt_controller.update_mut();
        
        // quit when Esc is pressed.
        for event in window.poll_events() {
            // debug!("{:?}", event);
            match event {
                KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                Closed => return,
                Resized(width, height) => {
                    let new_aspect_ratio = width as f32 / height as f32;
                    self.proj = PerspectiveMatrix3::<f32>::new(new_aspect_ratio, 45.0f32.to_radians(), 1.0, 100.0).to_matrix();
                },
                KeyboardInput(element_state, _, Some(key_code)) => {
                    self.keys_down_controller.update(element_state, key_code);
                },
                MouseMoved(x, y) => {
                    let new_position = (x, y);
                    if self.mouse_position_controller.is_dragging() {
                        let old_position = self.mouse_position_controller.current_mouse_position;
                        self.camera_controller.mouse_moved_mut(dt, old_position, new_position)
                    };
                    self.mouse_position_controller.update_position_mut(new_position);
                },
                MouseInput(press_state, glutin::MouseButton::Left) => 
                    self.mouse_position_controller.update_drag_position_mut(press_state),
                _ => {}
            }
        }

        self.camera_controller.update_with_keys_mut(dt, &self.keys_down_controller.set);

        // draw a frame
        self.render(&mut device as &mut D);
        window.swap_buffers().unwrap();
        device.cleanup();
    }}

    fn new(mut factory: F, init: Init) -> Self {
        use gfx::traits::FactoryExt;
        use nalgebra::*;

        let (instance_buffer, mut instance_mapping) =
            factory.create_buffer_persistent_rw(MAX_INSTANCE_COUNT as usize,
                                                gfx::buffer::Role::Vertex,
                                                gfx::Bind::empty());
        let svo_controller = SvoController::new();
        let instance_count = {
            let mut instances = instance_mapping.read_write();
            svo_controller.svo.fill_instances(&mut instances, svo_controller.max_height)
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

        let sinfo = texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/cube_150.glslv"), 
            include_bytes!("shader/cube_150.glslf"), 
            pipe::new()
        ).unwrap();

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
            svo_controller: svo_controller,
            keys_down_controller: KeysDownController::new(),
            encoder: factory.create_command_buffer().into(),
            camera_controller: CameraController::new(),
            mouse_position_controller: MousePositionController::new(),
            dt_controller: DtController::new(),
            proj: PerspectiveMatrix3::<f32>::new(init.aspect_ratio,
                                                 45.0f32.to_radians(),
                                                 1.0,
                                                 100.0)
                      .to_matrix(),
        }
    }

    fn render<D>(&mut self, device: &mut D)
            where D: gfx::Device<Resources = R, CommandBuffer = C> {
        {
            let mut instances = self.mapping.read_write();
            let instance_count = self.svo_controller.svo.fill_instances(&mut instances, self.svo_controller.max_height);
            self.bundle.slice.instances = Some((instance_count, 0));
        }

        let view = self.camera_controller.camera.view();
        let locals = Locals { transform: *(self.proj * view).as_ref() };
        self.encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

        self.encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        self.encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        self.encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
        self.bundle.encode(&mut self.encoder);
        self.encoder.flush(device);
    }
}
