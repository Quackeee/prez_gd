#[allow(dead_code)]
pub enum InputEvent {
    MouseMoved(i32, i32),
    KeyPressed(u32),
    KeyReleased(u32),
    MouseKeyPressed(i32, i32, i16),
    MouseKeyReleased(i32, i32, i16),
    ScrollMoved(f64)
}
