mod quil;

use glfw::{
    fail_on_errors,
    ffi::{GLFW_KEY_Q, GLFW_MOUSE_BUTTON_1},
    Context,
};

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

        if (quil.is_mouse_button_just_pressed(glfw::MouseButtonLeft, &window)) {
            // custom logic
        }

        if (quil.is_mouse_button_pressed(glfw::MouseButtonLeft, &window)) {
            // custom logic
        }

        if (quil.is_key_just_released(glfw::Key::W, &window)) {
            // custom logic
        }

        if (quil.is_key_released(glfw::Key::W, &window)) {
            // custom logic
        }

        let mut space_key_state: QuilInputState = quil.get_key_state(glfw::Key::Space, &window);
        println!("{}", quil.key_to_string(space_key_state));

        glfw.poll_events();
    }
}
