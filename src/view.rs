use bytemuck::{Pod, Zeroable};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent, MouseScrollDelta},
};

const MOVE_STEP: f32 = 0.05;
const SCALE_STEP: f32 = 1.05;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct View {
    offset: [f32; 2],
    scale: f32,
    _padding: u32,
}

pub struct ViewController {
    panning: bool,
    cursor: PhysicalPosition<f64>,
}

impl View {
    pub fn new() -> (Self, ViewController) {
        (
            Self {
                offset: [-0.5, 0.0],
                scale: 1.0,
                _padding: 0,
            },
            ViewController {
                panning: false,
                cursor: PhysicalPosition { x: 0.0, y: 0.0 },
            },
        )
    }
}

impl ViewController {
    pub fn input(&mut self, event: &WindowEvent, view: &mut View) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                VirtualKeyCode::Equals => view.scale *= SCALE_STEP,
                VirtualKeyCode::Minus => view.scale /= SCALE_STEP,
                VirtualKeyCode::W => view.offset[1] += MOVE_STEP,
                VirtualKeyCode::A => view.offset[0] -= MOVE_STEP,
                VirtualKeyCode::S => view.offset[1] -= MOVE_STEP,
                VirtualKeyCode::D => view.offset[0] += MOVE_STEP,
                _ => return false,
            },

            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => self.panning = *state == ElementState::Pressed,

            WindowEvent::CursorMoved { position, .. } => {
                if self.panning {
                    // TODO: replace 1600.0 with width and height of the window
                    view.offset[0] += (self.cursor.x - position.x) as f32 / view.scale / 1600.0;
                    view.offset[1] -= (self.cursor.y - position.y) as f32 / view.scale / 1600.0;
                }

                self.cursor = *position;
            },

            WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, y), .. } => {
                // FIXME: doesn't work
                if *y > 0.0 {
                    view.scale *= SCALE_STEP;
                } else {
                    view.scale /= SCALE_STEP;
                }
            }

            _ => return false,
        }

        true
    }
}
