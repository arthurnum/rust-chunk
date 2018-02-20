use gfx_gl::*;
use shaders::{self, Shader};
use std::ffi::CString;
use cgmath::{ortho, Matrix4};
use rooms_ui::RoomUICollection;
use graphics;
use std::net::{UdpSocket};
use protocol;
use protocol::enums::MessageType;
use sdl2::event::Event;

pub trait SceneContext {
    fn render(&self);
    fn update(&mut self, network: &UdpSocket);
    fn user_input(&mut self, event: Event);
}

pub struct MainSceneContext {
    program: Box<Shader>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>,
    rooms: Box<RoomUICollection>
}

impl MainSceneContext {
    pub fn new(gl: &Gl) -> MainSceneContext {
        let mut program = shaders::new(&gl);
        let vsource = CString::new("
            #version 410 core
            layout(location=0) in vec3 pos;

            uniform mat4 supermatrix;

            void main()
            {
                gl_Position = supermatrix * vec4(pos, 1.0);
            }
        ").unwrap();
        let vsb = vsource.to_bytes();
        let fsource = CString::new("
            #version 410 core

            out vec4 out_color;

            void main()
            {
                out_color = vec4(1.0, 1.0, 0.4, 1.0);
            }
        ").unwrap();
        let fsb = fsource.to_bytes();
        program.vertex_shader(vsb)
               .fragment_shader(fsb)
               .link();

        let mut rooms = RoomUICollection::new(4);
        for mut room in rooms.each_mut() {
           let gfx = graphics::Gfx::build_rectangle_sample(&gl, &room.calc_vertices());
           room.gfx = Some(gfx);
        }

        MainSceneContext {
            program: program,
            matrix: ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0),
            gl: Box::new(gl.clone()),
            rooms: rooms
        }
    }
}

impl SceneContext for MainSceneContext {
    fn render(&self) {
        let matrix_uniform_name = CString::new("supermatrix").unwrap();

        unsafe {
            self.gl.ClearColor(0.05, 0.05, 0.1, 1.0);
            self.gl.Clear(COLOR_BUFFER_BIT);

            self.program.use_program();
            let ul = self.gl.GetUniformLocation(self.program.id, matrix_uniform_name.to_bytes().as_ptr() as *const i8);
            let matrix_slice: &[f32; 16] = self.matrix.as_ref();
            self.gl.UniformMatrix4fv(ul, 1, FALSE, matrix_slice.as_ptr());

            for room in self.rooms.each() {
                if room.is_active() { room.draw(); }
            }
        }
    }

    fn update(&mut self, network: &UdpSocket) {
        let mut buf: Vec<u8> = vec![0; 128];
        let recr = network.recv_from(&mut buf);

        if recr.is_ok() {
            match protocol::unpack(&buf) {
                MessageType::RoomStatus { number, is_active } => {
                    for mut room in self.rooms.each_mut() {
                        if room.number() == number {
                            if is_active { room.activate(); } else { room.deactivate(); }
                        }
                    }
                }

                _ => ()
            }
        }
    }

    fn user_input(&mut self, event: Event) {
        match event {

            Event::MouseButtonUp { x, y, .. } => {
                match self.rooms.find_by_coords(x as u32, 400 - y as u32) {
                    Some(room) => {
                        println!("Room {:?}", room.number());
                        let msg = MessageType::MemberIn(room.number());
                        let buf = protocol::pack(&msg);
                        // network_source.send_to(&buf, "127.0.0.1:45000").unwrap();
                    }

                    None => ()
                }
            }

            _ => ()
        }
    }
}
