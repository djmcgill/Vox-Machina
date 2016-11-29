use glutin::{MouseButton, VirtualKeyCode};
use std::convert::From;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum KeyDown {
    Key(VirtualKeyCode),
    Mouse(MouseButton)
}

impl From<VirtualKeyCode> for KeyDown {
    fn from(key: VirtualKeyCode) -> Self { KeyDown::Key(key) }
}

impl From<MouseButton> for KeyDown {
    fn from(button: MouseButton) -> Self { KeyDown::Mouse(button) }
}
