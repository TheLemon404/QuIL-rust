mod quil;

use glfw::{fail_on_errors, ffi::GLFW_KEY_Q, Context};

use crate::quil::{Quil, QuilInputState};

fn main() {
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

        let state: QuilInputState = quil.get_key_state(glfw::Key::Space, &window);
        println!("{}", quil.key_to_string(state));

        glfw.poll_events();
    }
}
