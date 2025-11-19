# QuIL
### Quick Input Layer

Oftentimes, when writing a project containing keyboard and mouse input, I find myself rewriting the same code over and over again between projects. QuIL is meant to be an extremely small code layer that sits on top of GLFW and implements the basic input logic needed to get a prototype project off the ground. Using QuIL, you can have out-of-the-box input functionality with zero setup, making early project development a tiny bit easier.

## Features:
* QUIL_PRESSED, QUIL_JUST_PRESSED, QUIL_RELEASED, QUIL_JUST_RELEASED input states for keyboard and mouse
* input state checking functionality through reading the current state of a key, or simply checking is_key_down(key, glfw_window)

## Limitations:
* The Rust implementation of Quil cannot yet use function callbacks, as was implemented in the C version of Quil.

## Basic Input State Usage
```Rust
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
```
