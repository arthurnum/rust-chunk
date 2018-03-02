use gfx_gl::*;
use shaders::{self, Shader};
use shaders::smpl;
use std::ffi::CString;
use cgmath::*;
use rooms_ui::RoomUICollection;
use graphics;
use std::net::{UdpSocket};
use protocol;
use protocol::enums::MessageType;
use sdl2::event::Event;
use std::rc::Rc;
use std::cell::RefCell;
use objects::*;

type RefSceneContext = Rc<RefCell<SceneContext>>;

pub trait SceneContext {
    fn render(&self);
    fn update(&mut self, network: &UdpSocket);
    fn user_input(&mut self, event: Event, network: &UdpSocket);
    fn switch_context(&self) -> Option<RefSceneContext>;
}

pub struct MainSceneContext {
    program: Box<Shader>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>,
    rooms: Box<RoomUICollection>,
    switch_context: Option<RefSceneContext>
}

impl MainSceneContext {
    pub fn new(gl: &Gl) -> MainSceneContext {
        let mut program = shaders::new(&gl);

        let vsource = CString::new(smpl::DEFAULT_VERTEX).unwrap();
        let vsb = vsource.to_bytes();

        let fsource = CString::new(smpl::YELLOW_FRAGMENT).unwrap();
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
            rooms: rooms,
            switch_context: None
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

    fn user_input(&mut self, event: Event, network: &UdpSocket) {
        match event {

            Event::MouseButtonUp { x, y, .. } => {
                match self.rooms.find_by_coords(x as u32, 400 - y as u32) {
                    Some(room) => {
                        println!("Room {:?}", room.number());
                        let msg = MessageType::MemberIn(room.number());
                        let buf = protocol::pack(&msg);
                        network.send_to(&buf, "127.0.0.1:45000").unwrap();

                        self.switch_context = Some(Rc::new(RefCell::new(RoomSceneContext::new(&self.gl))));
                    }

                    None => ()
                }
            }

            _ => ()
        }
    }

    fn switch_context(&self) -> Option<RefSceneContext> {
        match self.switch_context {
            Some(ref context) => { Some(context.clone()) }
            None => None
        }
    }
}

pub struct RoomSceneContext {
    circle: Circle,
    program: Box<Shader>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>
}

impl RoomSceneContext {
    pub fn new(gl: &Gl) -> RoomSceneContext {
        let circle = Circle::new(gl, 0f32, 0f32, 10f32);

        let mut program = shaders::new(&gl);

        let vsource = CString::new(smpl::DEFAULT_VERTEX).unwrap();
        let vsb = vsource.to_bytes();

        let fsource = CString::new(smpl::YELLOW_FRAGMENT).unwrap();
        let fsb = fsource.to_bytes();

        program.vertex_shader(vsb)
               .fragment_shader(fsb)
               .link();

        let translation = Matrix4::from_translation(Vector3 { x: 100f32, y: 100f32, z: 0f32 });

        RoomSceneContext {
            circle: circle,
            program: program,
            matrix: ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0) * translation,
            gl: Box::new(gl.clone())
        }
    }
}

impl SceneContext for RoomSceneContext {
    fn render(&self) {
        let matrix_uniform_name = CString::new("supermatrix").unwrap();

        unsafe {
            self.gl.ClearColor(0.05, 0.05, 0.1, 1.0);
            self.gl.Clear(COLOR_BUFFER_BIT);

            self.program.use_program();
            let ul = self.gl.GetUniformLocation(self.program.id, matrix_uniform_name.to_bytes().as_ptr() as *const i8);
            let matrix_slice: &[f32; 16] = self.matrix.as_ref();
            self.gl.UniformMatrix4fv(ul, 1, FALSE, matrix_slice.as_ptr());

            self.circle.draw();
        }
    }

    fn update(&mut self, network: &UdpSocket) {}
    fn user_input(&mut self, event: Event, network: &UdpSocket) {}
    fn switch_context(&self) -> Option<RefSceneContext> { None }
}
