use context::scene_context::*;
use context::room_scene_context::*;
use objects::*;
use shaders;
use shaders::*;
use graphics;
use cgmath::*;
use gfx_gl::*;
use rooms_ui::*;
use timers;
use protocol;
use protocol::enums::MessageType;
use sdl2::event::Event;
use std::ffi::CString;
use std::net::UdpSocket;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MainSceneContext {
    program: Box<Shader>,
    background_program: Box<Shader>,
    background: Box<Rectangle>,
    matrix: Matrix4<f32>,
    gl: Box<Gl>,
    rooms: Box<RoomUICollection>,
    switch_context: Option<RefSceneContext>,
    timer: Box<timers::Timer>,
    network: Rc<UdpSocket>,
}

impl MainSceneContext {
    pub fn new(gl: &Gl, network: &Rc<UdpSocket>) -> MainSceneContext {
        let mut program = shaders::new(&gl);

        let vsource = CString::new(smpl::DEFAULT_VERTEX).unwrap();
        let vsb = vsource.to_bytes();

        let fsource = CString::new(smpl::YELLOW_FRAGMENT).unwrap();
        let fsb = fsource.to_bytes();

        program.vertex_shader(vsb).fragment_shader(fsb).link();

        let mut background_program = shaders::new(&gl);

        let fsource = CString::new(smpl::BACKGROUND_FRAGMENT).unwrap();
        let fsb = fsource.to_bytes();

        background_program
            .vertex_shader(vsb)
            .fragment_shader(fsb)
            .link();

        let mut rooms = RoomUICollection::new(1);
        for mut room in rooms.each_mut() {
            let gfx = graphics::Gfx::build_rectangle_sample(&gl, &room.calc_vertices());
            room.gfx = Some(gfx);
        }

        MainSceneContext {
            program: program,
            background_program: background_program,
            background: Box::new(Rectangle::new(gl, 300f32, 200f32, 600f32, 400f32)),
            matrix: ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0),
            gl: Box::new(gl.clone()),
            rooms: rooms,
            switch_context: None,
            timer: timers::new(),
            network: network.clone(),
        }
    }
}

impl SceneContext for MainSceneContext {
    fn render(&self) {
        unsafe {
            self.gl.ClearColor(0.05, 0.05, 0.1, 1.0);
            self.gl.Clear(COLOR_BUFFER_BIT);

            self.background_program.use_program();
            self.background_program
                .uniform_matrix4fv("supermatrix", &self.matrix);
            self.background_program
                .uniform1f("time", self.timer.elapsed() as f32 / 10000f32);
            self.background.draw();

            self.program.use_program();
            self.program.uniform_matrix4fv("supermatrix", &self.matrix);

            for room in self.rooms.each() {
                if room.is_active() {
                    room.draw();
                }
            }
        }
    }

    fn update(&mut self) {
        let mut buf: Vec<u8> = vec![0; 128];
        let recr = self.network.recv_from(&mut buf);

        if recr.is_ok() {
            match protocol::unpack(&buf) {
                MessageType::ServerOn => for mut room in self.rooms.each_mut() {
                    room.activate();
                },

                _ => (),
            }
        }
    }

    fn user_input(&mut self, event: Event) {
        match event {
            Event::MouseButtonUp { x, y, .. } => {
                match self.rooms.find_by_coords(x as u32, 400 - y as u32) {
                    Some(room) => {
                        println!("Room {:?}", room.number());
                        let msg = MessageType::MemberIn;
                        let buf = protocol::pack(&msg);
                        self.network.send_to(&buf, "127.0.0.1:45000").unwrap();

                        self.switch_context = Some(Rc::new(RefCell::new(RoomSceneContext::new(
                            &self.gl,
                            &self.network,
                        ))));
                    }

                    None => (),
                }
            }

            _ => (),
        }
    }

    fn switch_context(&self) -> Option<RefSceneContext> {
        match self.switch_context {
            Some(ref context) => Some(context.clone()),
            None => None,
        }
    }
}
