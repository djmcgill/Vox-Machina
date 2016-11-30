use glutin::ElementState;

pub struct MousePositionController {
    pub current_mouse_position: (i32, i32),
    pub drag_start_position: Option<(i32, i32)>,
}

impl MousePositionController {
    pub fn new() -> Self {
        MousePositionController {
            current_mouse_position: (0, 0),
            drag_start_position: None,
        }
    }

    pub fn update_position_mut(&mut self, new_position: (i32, i32)) {
        self.current_mouse_position = new_position; 
    }

    pub fn is_dragging(&self) -> bool {
        self.drag_start_position.is_some()
    }

    pub fn update_drag_position_mut(&mut self, press_state: ElementState) {
        self.drag_start_position = match press_state {
            ElementState::Pressed => Some(self.current_mouse_position),
            ElementState::Released => None,
        };
    }
}
