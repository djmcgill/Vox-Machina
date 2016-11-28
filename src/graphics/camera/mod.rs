use nalgebra::{Isometry3, Matrix4, Point3, ToHomogeneous, Vector3, Rotation};
use std::collections::HashSet;
use glutin::VirtualKeyCode;

pub struct OverheadCamera {
    iso: Isometry3<f32>,
    view: Matrix4<f32>, // Cached from the other parameters
}

const INITAL_TARGET: Point3<f32> = Point3 { x: 0.0, y: 0.0, z: 0.0 };
const INITIAL_EYE: Point3<f32> = Point3 { x: 4.5, y: -15.0, z: 9.0 };

impl OverheadCamera {
    pub fn new() -> OverheadCamera {
        let up = Vector3::z();
        let iso = Isometry3::<f32>::look_at_rh(&INITIAL_EYE, &INITAL_TARGET, &up);
        OverheadCamera {
            iso: iso,
            view: iso.to_homogeneous(),
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn update_from_keys(&mut self, dt: f32, keys_down: &HashSet<VirtualKeyCode>) {
        if keys_down.contains(&VirtualKeyCode::Left) {
            self.iso.translation += Vector3::x() * dt;
        }
        if keys_down.contains(&VirtualKeyCode::Right) {
            self.iso.translation -= Vector3::x() * dt;
        }
        if keys_down.contains(&VirtualKeyCode::Up) {
            self.iso.translation += Vector3::z() * dt * self.iso.rotation;
        }
        if keys_down.contains(&VirtualKeyCode::Down) {
            self.iso.translation -= Vector3::z() * dt * self.iso.rotation;
        }

        self.view = self.iso.to_homogeneous();
    }

    pub fn rotate(&mut self, dt: f32, dist: f32) {
        debug!("rotating by {}", dt * dist);
        self.iso.rotation.prepend_rotation_mut(&(Vector3::z() * dist * dt));
        self.view = self.iso.to_homogeneous();
    }
}
