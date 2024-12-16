use super::input_event::InputEvent;

pub struct KeyboardState {
    keys_pressed: [bool; 255],
    key_presses: Vec<u32>,
    key_releases: Vec<u32>
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 255],
            key_presses: Vec::new(),
            key_releases: Vec::new()
        }
    }

    pub fn process_events(&mut self, events: &Vec<InputEvent>) {
        self.key_presses.clear();
        self.key_releases.clear();

        for event in events.iter() {
            if let InputEvent::KeyPressed(key) = event {
                self.keys_pressed[*key as usize] = true;
                self.key_presses.push(key.clone());
                continue;
            }
            if let InputEvent::KeyReleased(key) = event {
                self.keys_pressed[*key as usize] = false;
                self.key_releases.push(key.clone());
                continue;
            }
        }
    }

    pub fn snapshot(&self) -> KeyboardStateSnapshot {
        KeyboardStateSnapshot::from(&self)
    }
}

#[allow(dead_code)]
pub struct KeyboardStateSnapshot {
    keys_pressed: [bool; 255],
    key_presses: Vec<u32>,
    key_releases: Vec<u32>
}

#[allow(dead_code)]
impl KeyboardStateSnapshot {
    // refer to the implementation of MouseStateSnapshot::from for thoughts
    pub fn from(state: &KeyboardState) -> Self {
        Self {
            key_presses: state.key_presses.clone(),
            key_releases: state.key_releases.clone(),
            keys_pressed: state.keys_pressed.clone()
        }
    }

    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 255],
            key_presses: Vec::new(),
            key_releases: Vec::new()
        }
    }

    pub fn is_key_pressed(&self, key: u32) -> bool {
        self.keys_pressed[key as usize]
    }

    pub fn was_key_pressed(&self, key: u32) -> bool {
        self.key_presses.contains(&key)
    }

    pub fn was_key_released(&self, key: u32) -> bool {
        self.key_releases.contains(&key)
    }
}

