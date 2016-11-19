use nalgebra::{Isometry3, Matrix4, Point3, ToHomogeneous, Vector3};
use std::collections::HashSet;
use glutin::VirtualKeyCode;

pub struct OverheadCamera {
    eye: Point3<f32>,
    target: Point3<f32>,
    view: Matrix4<f32>, // Cached from the other parameters
}

impl OverheadCamera {
    pub fn new() -> OverheadCamera {
        let eye = Point3::new(1.5, -5.0, 3.0) * 3.0;
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::z();
        // TODO: build from Rotation3::look_at_rh
        OverheadCamera {
            eye: eye,
            target: target,
            view: Isometry3::<f32>::look_at_rh(&eye, &target, &up).to_homogeneous(),
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn pan(&mut self, dv: Vector3<f32>) {
        let up = Vector3::z();
        self.target += dv;
        self.eye += dv;
        self.view = Isometry3::<f32>::look_at_rh(&self.eye, &self.target, &up).to_homogeneous();
    }

    pub fn update(&mut self, dt: f32, keys_down: &HashSet<VirtualKeyCode>) {
        let mut dv = Vector3::new(0.0, 0.0, 0.0);
        if keys_down.contains(&VirtualKeyCode::Left) {
            dv -= Vector3::x() * dt;
        }
        if keys_down.contains(&VirtualKeyCode::Right) {
            dv += Vector3::x() * dt;
        }
        if keys_down.contains(&VirtualKeyCode::Up) {
            dv += Vector3::y() * dt;
        }
        if keys_down.contains(&VirtualKeyCode::Down) {
            dv -= Vector3::y() * dt;
        }

        self.pan(dv);
    }
}
