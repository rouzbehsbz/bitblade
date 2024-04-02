use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct Keyboard {
    device_state: DeviceState,
    keys: Vec<Keycode>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            keys: Vec::new(),
        }
    }

    pub fn poll_keys(&mut self) {
        self.keys.clear();

        for key in self.device_state.get_keys() {
            self.keys.push(key);
        }
    }

    pub fn is_key_pressed(&self, key: &Keycode) -> bool {
        self.keys.contains(key)
    }
}
