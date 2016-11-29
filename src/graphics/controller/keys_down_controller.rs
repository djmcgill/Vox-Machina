use glutin::{ElementState, VirtualKeyCode};
use graphics::model::KeyDown;
use std::collections::HashSet;

#[derive(Debug)]
pub struct KeysDownController {
    pub set: HashSet<KeyDown>,
}

impl KeysDownController {
    pub fn new() -> Self {
        KeysDownController { set: HashSet::new(), }
    }

    pub fn update(&mut self, element_state: ElementState, key_code: VirtualKeyCode) {
        match element_state {
            ElementState::Pressed => {
                let _was_inserted = self.set.insert(key_code.into());
                // assert!(_was_inserted); Watch out for key repeat from the OS!
            },
            ElementState::Released => {
                let was_removed = self.set.remove(&key_code.into());
                assert!(was_removed); // If false, weird things are happening
            },
        }
    }
}
