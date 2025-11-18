mod quil;

#[cfg(test)]
mod tests {
    use glfw::{fail_on_errors, ffi::GLFW_KEY_Q, Context};

    use crate::quil::{Quil, QuilInputState};

    use super::*;

    #[test]
    fn it_works() {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let mut window = glfw
            .create_window(400, 400, "Test of quil-r", glfw::WindowMode::Windowed)
            .expect("Failed to open window")
            .0;

        window.make_current();
        window.set_key_polling(true);

        let mut quil: Quil = Quil::new();

        while !window.should_close() {
            window.swap_buffers();
            glfw.poll_events();

            let state: QuilInputState = quil.get_key_state(glfw::Key::Space, &window);
            println!("{}", quil.key_to_string(state));
        }
    }
}
