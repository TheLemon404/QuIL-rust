use std::os::windows::ffi;

use glfw::{
    self,
    ffi::{GLFW_PRESS, GLFW_RELEASE},
    Glfw, Key, MouseButton,
};

pub enum QuilInputState {
    QUIL_RELEASED,
    QUIL_JUST_RELEASED,
    QUIL_PRESSED,
    QUIL_JUST_PRESSED,
}

pub struct Quil {
    keys: [i32; 349],
    buttons: [i32; 8],
}

impl Quil {
    pub fn new() -> Quil {
        Quil {
            keys: [0; 349],
            buttons: [0; 8],
        }
    }

    pub fn get_key_state(&mut self, key: Key, glfw_window: &glfw::PWindow) -> QuilInputState {
        let mut result: QuilInputState = QuilInputState::QUIL_RELEASED;
        let glfw_key_state: i32 = glfw_window.get_key(key) as i32;

        match glfw_key_state {
            GLFW_PRESS => {
                if (self.keys[key as usize] == GLFW_PRESS) {
                    result = QuilInputState::QUIL_PRESSED;
                } else if (self.keys[key as usize] == GLFW_RELEASE) {
                    result = QuilInputState::QUIL_JUST_PRESSED;
                }
            }
            GLFW_RELEASE => {
                if self.keys[key as usize] == GLFW_RELEASE {
                    result = QuilInputState::QUIL_RELEASED
                } else if self.keys[key as usize] == GLFW_PRESS {
                    result = QuilInputState::QUIL_JUST_RELEASED;
                }
            }
            _ => {}
        }

        self.keys[key as usize] = glfw_key_state;

        return result;
    }

    pub fn get_mouse_button_state(
        &mut self,
        button: MouseButton,
        glfw_window: &glfw::PWindow,
    ) -> QuilInputState {
        let mut result: QuilInputState = QuilInputState::QUIL_RELEASED;
        let glfw_button_state: i32 = glfw_window.get_mouse_button(button) as i32;

        match glfw_button_state {
            GLFW_PRESS => {
                if self.keys[button as usize] == GLFW_PRESS {
                    result = QuilInputState::QUIL_PRESSED;
                } else if self.keys[button as usize] == GLFW_RELEASE {
                    result = QuilInputState::QUIL_JUST_PRESSED;
                }
            }
            GLFW_RELEASE => {
                if self.keys[button as usize] == GLFW_RELEASE {
                    result = QuilInputState::QUIL_RELEASED
                } else if self.keys[button as usize] == GLFW_PRESS {
                    result = QuilInputState::QUIL_JUST_RELEASED;
                }
            }
            _ => {}
        }

        self.buttons[button as usize] = glfw_button_state;

        return result;
    }

    pub fn key_to_string(&self, key: QuilInputState) -> String {
        match key {
            QuilInputState::QUIL_PRESSED => {
                return "PRESSED".to_string();
            }
            QuilInputState::QUIL_JUST_PRESSED => {
                return "JUST_PRESSED".to_string();
            }
            QuilInputState::QUIL_RELEASED => {
                return "RELEASED".to_string();
            }
            QuilInputState::QUIL_JUST_RELEASED => {
                return "JUST_RELEASED".to_string();
            }
            _ => {
                return "UNKNOWN_KEY_STATE".to_string();
            }
        }
    }

    pub fn is_key_released(&mut self, key: Key, glfw_window: &glfw::PWindow) -> bool {
        return matches!(
            self.get_key_state(key, glfw_window),
            QuilInputState::QUIL_RELEASED
        );
    }

    pub fn is_key_just_released(&mut self, key: Key, glfw_window: &glfw::PWindow) -> bool {
        return matches!(
            self.get_key_state(key, glfw_window),
            QuilInputState::QUIL_JUST_RELEASED
        );
    }

    pub fn is_key_pressed(&mut self, key: Key, glfw_window: &glfw::PWindow) -> bool {
        return matches!(
            self.get_key_state(key, glfw_window),
            QuilInputState::QUIL_PRESSED
        );
    }

    pub fn is_key_just_pressed(&mut self, key: Key, glfw_window: &glfw::PWindow) -> bool {
        return matches!(
            self.get_key_state(key, glfw_window),
            QuilInputState::QUIL_JUST_PRESSED
        );
    }

    pub fn is_mouse_button_released(
        &mut self,
        button: MouseButton,
        glfw_window: &glfw::PWindow,
    ) -> bool {
        return matches!(
            self.get_mouse_button_state(button, glfw_window),
            QuilInputState::QUIL_RELEASED
        );
    }

    pub fn is_mouse_button_just_released(
        &mut self,
        button: MouseButton,
        glfw_window: &glfw::PWindow,
    ) -> bool {
        return matches!(
            self.get_mouse_button_state(button, glfw_window),
            QuilInputState::QUIL_JUST_RELEASED
        );
    }

    pub fn is_mouse_button_pressed(
        &mut self,
        button: MouseButton,
        glfw_window: &glfw::PWindow,
    ) -> bool {
        return matches!(
            self.get_mouse_button_state(button, glfw_window),
            QuilInputState::QUIL_PRESSED
        );
    }

    pub fn is_mouse_button_just_pressed(
        &mut self,
        button: MouseButton,
        glfw_window: &glfw::PWindow,
    ) -> bool {
        return matches!(
            self.get_mouse_button_state(button, glfw_window),
            QuilInputState::QUIL_JUST_PRESSED
        );
    }
}
