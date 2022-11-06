use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    tracing_subscriber::fmt::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    /*
    NOTE:
        Winit's event_loop can be hard to read, but if you already understand basic pattern matching in rust, this is just that, but nested.
        If you are any familiar with languages that use '{}', just follow those to see the flow of the match.
    */
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event { // Here's your nested pattern matching.
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                // You're just checking for the use of the 'esc' key on your keyboard.
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}