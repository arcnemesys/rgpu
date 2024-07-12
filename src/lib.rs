#![allow(unused_must_use)]
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        // Set up console deps in a web build, and env logger in a normal build.
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    #[cfg(target_arch="wasm32")]
{
    // Winit prevents CSS sizing, so it gets set manually for web builds.

    use winit::dpi::PhysicalSize;
    let _ = window.request_inner_size(PhysicalSize::new(450, 400));

    use winit::platform::web::WindowExtWebSys;

    web_sys::window()
        
}
    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    });
}

