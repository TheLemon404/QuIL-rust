mod quil;

#[cfg(test)]
mod tests {
    use glfw::{fail_on_errors, ffi::GLFW_KEY_Q};

    use crate::quil::{Quil, QuilInputState};

    use super::*;

    #[test]
    fn it_works() {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let mut window = glfw
            .create_window(400, 400, "Test of quil-r", glfw::WindowMode::Windowed)
            .expect("Failed to open window")
            .0;

        let mut quil: Quil = Quil::new(window, glfw);

        while !quil.is_key_just_pressed(glfw::Key::Q) {
            let state: QuilInputState = quil.get_key_state(glfw::Key::Space);
            println!("{}", quil.key_to_string(state));
        }
    }
}
