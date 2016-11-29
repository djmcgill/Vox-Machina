use nalgebra::*;

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

    pub fn pan_rot_mut(&mut self, dt: f32, pan: Vector2<f32>, rot: f32) {
        if pan.x != 0.0 { self.iso.translation += Vector3::x() * dt * pan.x; }
        if pan.y != 0.0 { self.iso.translation += Vector3::z() * dt * pan.y * self.iso.rotation; }
        if rot != 0.0 { self.iso.rotation.prepend_rotation_mut(&(Vector3::z() * rot * dt)); }
        if pan.x != 0.0 || pan.y != 0.0 || rot != 0.0 { self.view = self.iso.to_homogeneous(); }
    }

    pub fn rot_mut(&mut self, dt: f32, rot: f32) {
        if rot != 0.0 {
            self.iso.rotation.prepend_rotation_mut(&(Vector3::z() * rot * dt));
            self.view = self.iso.to_homogeneous();
        }
    }
}
