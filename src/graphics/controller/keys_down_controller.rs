use glutin;
use graphics::model::KeyDown;
use std::collections::HashSet;

#[derive(Debug)]
pub struct KeysDownController {
    pub set: HashSet<KeyDown>
}

impl KeysDownController {
    pub fn new() -> Self {
        KeysDownController { set: HashSet::new(), }
    }

    pub fn update(&mut self, element_state: glutin::ElementState, key_code: glutin::VirtualKeyCode) {
        match element_state {
            glutin::ElementState::Pressed => {
                let _was_inserted = self.set.insert(KeyDown::Key(key_code));
                // assert!(was_inserted); Watch out for key repeat from the OS!
            },
            glutin::ElementState::Released => {
                let was_removed = self.set.remove(&KeyDown::Key(key_code));
                assert!(was_removed); // If false, weird things are happening
            },
        }
    }
}
