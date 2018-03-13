use context::scene_context::*;
use objects::*;
use shaders;
use shaders::*;
use cgmath::*;
use gfx_gl::*;
use sdl2::event::Event;
use std::ffi::CString;
use std::net::UdpSocket;
use std::rc::Rc;

pub struct RoomSceneContext {
    circle: Circle,
    program: Box<Shader>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>,
    network: Rc<UdpSocket>,
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
            x: 100f32,
            y: 100f32,
            z: 0f32,
        });

        RoomSceneContext {
            circle: circle,
            program: program,
            matrix: ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0) * translation,
            gl: Box::new(gl.clone()),
            network: network.clone(),
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

    fn update(&mut self) {}
    fn user_input(&mut self, event: Event) {}
    fn switch_context(&self) -> Option<RefSceneContext> {
        None
    }
}
