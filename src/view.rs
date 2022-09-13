use bytemuck::{Pod, Zeroable};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent, MouseScrollDelta},
};

const MOVE_STEP: f32 = 0.05;
const SCALE_STEP: f32 = 1.05;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct View {
    offset: [f32; 2],
    scale: f32,
    aspect_ratio: f32,
}

pub struct ViewController {
    pub view: View,
    panning: bool,
    cursor: PhysicalPosition<f64>,
}

impl View {
    pub fn new() -> (Self, ViewController) {
        let view = Self {
            offset: [-0.5, 0.0],
            scale: 1.0,
            aspect_ratio: 1.0,
        };
        (
            view,
            ViewController {
                view,
                panning: false,
                cursor: PhysicalPosition { x: 0.0, y: 0.0 },
            },
        )
    }
}

impl ViewController {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
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
                VirtualKeyCode::Equals => self.view.scale *= SCALE_STEP,
                VirtualKeyCode::Minus => self.view.scale /= SCALE_STEP,
                VirtualKeyCode::W => self.view.offset[1] += MOVE_STEP,
                VirtualKeyCode::A => self.view.offset[0] -= MOVE_STEP,
                VirtualKeyCode::S => self.view.offset[1] -= MOVE_STEP,
                VirtualKeyCode::D => self.view.offset[0] += MOVE_STEP,
                _ => return false,
            },

            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => self.panning = *state == ElementState::Pressed,

            WindowEvent::CursorMoved { position, .. } => {
                if self.panning {
                    // TODO: replace 800.0 with width and height of the window / 2
                    self.view.offset[0] += (self.cursor.x - position.x) as f32 / self.view.scale / 800.0;
                    self.view.offset[1] -= (self.cursor.y - position.y) as f32 / self.view.scale / 800.0;
                }

                self.cursor = *position;
            },

            WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, y), .. } => {
                // FIXME: doesn't work
                if *y > 0.0 {
                    self.view.scale *= SCALE_STEP;
                } else {
                    self.view.scale /= SCALE_STEP;
                }
            }

            _ => return false,
        }

        true
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.view.aspect_ratio = size.width as f32 / size.height as f32;
    }
}
