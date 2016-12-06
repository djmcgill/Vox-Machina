use nalgebra::*;
use num::{One, Zero};

#[derive(Debug)]
// A translation followed by a rotation.
pub struct OverheadCamera {
    translation: Vector3<f32>,
    rotation: Rotation3<f32>,
    view_cache: Matrix4<f32>,
}

const INITIAL_CAMERA_START: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: -50.0 };

const PAN_MULT: f32 = 1.0/32.0;
const ROT_MULT: f32 = 1.0/256.0;

impl OverheadCamera {
    pub fn new() -> OverheadCamera {
        let translation = INITIAL_CAMERA_START;
        let rotation = Rotation3::look_at_rh(&translation, &Vector3::y());
        
        OverheadCamera {
            translation: translation,
            rotation: rotation,
            view_cache: OverheadCamera::generate_view(&translation, &rotation),
        }
    }

    fn generate_view(translation: &Vector3<f32>, rotation: &Rotation3<f32>) -> Matrix4<f32> {
        let mut result = Matrix4::one();
        let col = Vector4::new(translation.x, translation.y, translation.z, 1.0);
        result.set_column(3, col);
        rotation.to_homogeneous() * result
    }

    fn regenerate_view(&mut self) {
        self.view_cache = OverheadCamera::generate_view(&self.translation, &self.rotation);
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view_cache
    }

    pub fn pan_rot_mut(&mut self, dt: f32, pan: Vector3<f32>, rot: f32) {
        if pan != Vector3::zero() { self.translation.prepend_translation_mut(&(pan * dt * PAN_MULT)); }
        if rot != 0.0 { self.rotation.prepend_rotation_mut(&(Vector3::y() * -rot * dt * ROT_MULT)); }
        if pan != Vector3::zero() || rot != 0.0 {
            self.regenerate_view();
        }
    }

    pub fn rot_mut(&mut self, dt: f32, rot: f32) {
        if rot != 0.0 {
            self.rotation.prepend_rotation_mut(&(Vector3::y() * -rot * dt * ROT_MULT));
            self.regenerate_view();
        }
    }
}
