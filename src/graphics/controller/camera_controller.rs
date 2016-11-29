use graphics::model::camera::OverheadCamera;
use std::collections::HashSet;
use graphics::model::KeyDown;

#[derive(Debug)]
pub struct CameraController {
    pub camera: OverheadCamera,
}

const CAMERA_PAN_MULT: f32 = 1.0/32.0;
const CAMERA_ROT_MULT: f32 = 1.0/64.0;
impl CameraController {
    pub fn new() -> Self {
        CameraController { camera: OverheadCamera::new() }
    }

    pub fn mouse_moved_mut(&mut self, dt: f32, (old_x, old_y): (i32, i32), (new_x, new_y): (i32, i32)) {
        let _dx = new_x - old_x;
        let dy = (new_y - old_y) as f32;
        if dy != 0.0 { self.camera.rot_mut(dt, dy); };
    }

    pub fn update_with_keys_mut(&mut self, dt: f32, keys_down: &HashSet<KeyDown>) {
        use nalgebra::Vector2;
        use glutin::VirtualKeyCode;
        let mut pan = Vector2 { x: 0.0, y: 0.0 };
        if keys_down.contains(&VirtualKeyCode::Left.into()) {
            pan.x += 1.0;
        }
        if keys_down.contains(&VirtualKeyCode::Right.into()) {
            pan.x -= 1.0;
        }
        if keys_down.contains(&VirtualKeyCode::Up.into()) {
            pan.y += 1.0;
        }
        if keys_down.contains(&VirtualKeyCode::Down.into()) {
            pan.y -= 1.0;
        }

        let mut rot = 0.0;
        if keys_down.contains(&VirtualKeyCode::Q.into()) {
            rot += 1.0;
        }
        if keys_down.contains(&VirtualKeyCode::E.into()) {
            rot -= 1.0;
        }
        self.camera.pan_rot_mut(dt, pan * CAMERA_PAN_MULT, rot * CAMERA_ROT_MULT);
    }
}
