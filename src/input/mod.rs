mod mouse;
mod keyboard;
mod input_event;

pub use mouse::{MouseState, MouseStateSnapshot};
pub use keyboard::{KeyboardState, KeyboardStateSnapshot};
pub use input_event::InputEvent;