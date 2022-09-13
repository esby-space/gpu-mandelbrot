use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod state;
mod vertex;
mod view;

use state::State;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("mandelbrot!")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 1600,
            height: 1600,
        })
        .build(&event_loop)?;
    let state = pollster::block_on(State::new(&window))?;
    run_loop(state, event_loop, window)?;
    Ok(())
}

fn run_loop(mut state: State, event_loop: EventLoop<()>, window: Window) -> Result<()> {
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } if !state.input(&event) => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(new_size) => state.resize(&new_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(new_inner_size),
            _ => {}
        },

        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(..) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(_) => {}
            }
        }

        _ => {}
    });
}
