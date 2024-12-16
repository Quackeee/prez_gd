use crate::input::InputEvent;

pub struct MouseState {
    keys_pressed: [bool; 9],
    pub scroll_delta: f64,
    pub position: (i32, i32),
    pub mouse_delta: (i32, i32),
    key_presses: Vec<i16>,
    key_releases: Vec<i16>
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 9],
            scroll_delta: 0.0,
            position: (0, 0),
            mouse_delta: (0, 0),
            key_presses: Vec::new(),
            key_releases: Vec::new(),
        }
    }

    pub fn process_events(&mut self, events: &Vec<InputEvent>) {
        self.key_presses.clear();
        self.key_releases.clear();
        self.scroll_delta = 0.0;
        let mouse_prev = self.position;
        for event in events.iter() {
            if let InputEvent::MouseKeyPressed(_, _, key) = event {
                self.keys_pressed[*key as usize] = true;
                self.key_presses.push(key.clone());
                continue;
            }
            if let InputEvent::MouseKeyReleased(_, _, key) = event {
                self.keys_pressed[*key as usize] = false;
                self.key_releases.push(key.clone());
                continue;
            }
            if let InputEvent::MouseMoved(x, y ) = event {
                self.position = (*x, *y)
            }
            if let InputEvent::ScrollMoved(delta) = event  {
                self.scroll_delta += delta
            }
        }

        self.mouse_delta = (self.position.0 - mouse_prev.0, self.position.1 - mouse_prev.1)
    }

    pub fn snapshot(&self) -> MouseStateSnapshot {
        MouseStateSnapshot::from(&self)
    }
}

#[allow(dead_code)]
pub struct MouseStateSnapshot {
    keys_pressed: [bool; 9],
    pub scroll_delta: f64,
    pub position: (i32, i32),
    pub mouse_delta: (i32, i32),
    key_presses: Vec<i16>,
    key_releases: Vec<i16>
}

#[allow(dead_code)]
impl MouseStateSnapshot {

    // Pretty sure this could be implemented in a way that avoids all the clones, but I don't care atm xd
    pub fn from(state: &MouseState) -> Self {
        Self {
            keys_pressed: state.keys_pressed.clone(),
            scroll_delta: state.scroll_delta.clone(),
            position: state.position.clone(),
            mouse_delta: state.mouse_delta.clone(),
            key_presses: state.key_presses.clone(),
            key_releases: state.key_releases.clone(),
        }
    }

    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 9],
            scroll_delta: 0.0,
            position: (0, 0),
            mouse_delta: (0, 0),
            key_presses: Vec::new(),
            key_releases: Vec::new(),
        }
    }

    pub fn is_button_pressed(&self, key: i16) -> bool {
        self.keys_pressed[key as usize]
    }

    pub fn was_button_pressed(&self, key: i16) -> bool {
        self.key_presses.contains(&key)
    }

    pub fn was_button_released(&self, key: i16) -> bool {
        self.key_releases.contains(&key)
    }

    pub fn is_lmb_pressed(&self) -> bool {
        self.is_button_pressed(0)
    }

    pub fn is_mmb_pressed(&self) -> bool {
        self.is_button_pressed(1)
    }

    pub fn is_rmb_pressed(&self) -> bool {
        self.is_button_pressed(2)
    }
}