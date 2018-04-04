use context::scene_context::*;
use objects::*;
use shaders;
use shaders::*;
use timers;
use input_state::*;
use protocol;
use protocol::enums::MessageType;

use cgmath::*;
use gfx_gl::*;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use std::ffi::CString;
use std::net::UdpSocket;
use std::rc::Rc;

pub struct RoomSceneContext {
    circle: Circle,
    program: Box<Shader>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>,
    timer: Box<timers::Timer>,
    input_state: InputState,
    network: Rc<UdpSocket>,
    debug_move_start: i64,
    debug_move_stop: i64,
}

impl RoomSceneContext {
    pub fn new(gl: &Gl, network: &Rc<UdpSocket>) -> RoomSceneContext {
        let circle = Circle::new(gl, 0f32, 0f32, 10f32);

        let mut program = shaders::new(&gl);

        let vsource = CString::new(smpl::DEFAULT_VERTEX).unwrap();
        let vsb = vsource.to_bytes();

        let fsource = CString::new(smpl::YELLOW_FRAGMENT).unwrap();
        let fsb = fsource.to_bytes();

        program.vertex_shader(vsb).fragment_shader(fsb).link();

        let translation = Matrix4::from_translation(Vector3 {
            x: 300f32,
            y: 200f32,
            z: 0f32,
        });

        RoomSceneContext {
            circle: circle,
            program: program,
            matrix: ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0) * translation,
            gl: Box::new(gl.clone()),
            timer: timers::new(),
            input_state: InputState::default(),
            network: network.clone(),
            debug_move_start: 0,
            debug_move_stop: 0,
        }
    }
}

impl SceneContext for RoomSceneContext {
    fn render(&self) {
        unsafe {
            self.gl.ClearColor(0.05, 0.05, 0.1, 1.0);
            self.gl.Clear(COLOR_BUFFER_BIT);

            self.program.use_program();
            self.program.uniform_matrix4fv("supermatrix", &self.matrix);

            self.circle.draw();
        }
    }

    fn update(&mut self) {
        let dt = self.timer.frame_time();

        if self.input_state.mouse_rbtn_pressed {
            let d: Vector2<f32> = Vector2 {
                x: self.input_state.mouse_x as f32 - 300f32,
                y: 400f32 - self.input_state.mouse_y as f32 - 200f32,
            }.normalize();

            let msg = MessageType::MemberMove(d.x, d.y);
            self.network.send_to(&protocol::pack(&msg), "127.0.0.1:45000").unwrap();

            self.circle.gpos += d * dt as f32;
        }

        if self.input_state.mouse_rbtn_was_pressed {
            self.debug_move_start = self.timer.elapsed();
            self.input_state.mouse_rbtn_was_pressed = false;
        }

        if self.input_state.mouse_rbtn_was_released {
            self.debug_move_stop = self.timer.elapsed();
            println!("Moving elapsed time {:?}", self.debug_move_stop - self.debug_move_start);
            let msg = MessageType::MemberStopMove;
            self.network.send_to(&protocol::pack(&msg), "127.0.0.1:45000").unwrap();
            self.input_state.mouse_rbtn_was_released = false;
        }
    }

    fn user_input(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                match mouse_btn {
                    MouseButton::Right => {
                        self.input_state.mouse_rbtn_pressed = true;
                        self.input_state.mouse_rbtn_was_pressed = true;
                        self.input_state.mouse_x = x;
                        self.input_state.mouse_y = y;
                    }

                    _ => (),
                }
            }

            Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                match mouse_btn {
                    MouseButton::Right => {
                        self.input_state.mouse_rbtn_pressed = false;
                        self.input_state.mouse_rbtn_was_released = true;
                        self.input_state.mouse_x = x;
                        self.input_state.mouse_y = y;
                    }

                    _ => (),
                }
            }

            Event::MouseMotion { x, y, .. } => {
                self.input_state.mouse_x = x;
                self.input_state.mouse_y = y;
            }

            _ => (),
        }
    }

    fn switch_context(&self) -> Option<RefSceneContext> {
        None
    }
}
