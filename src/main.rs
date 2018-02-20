extern crate sdl2;
extern crate gfx_gl;
extern crate time;
extern crate chunk_protocol as protocol;
extern crate cgmath;
extern crate collision;

use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use gfx_gl::*;
use gfx_gl::types::*;
use std::net::{UdpSocket};
use protocol::enums::MessageType;

mod shaders;
mod timers;
mod skills;
mod threads;
mod rooms_ui;
mod graphics;
mod scene_context;

use scene_context::{SceneContext, MainSceneContext};

fn build_circle_sample(gl: &Gl) -> (GLuint, usize) {
    let mut counter = 2.0 * std::f32::consts::PI;
    let mut data: Vec<f32> = Vec::new();
    loop {
        data.append(&mut vec![
            300.0 + counter.sin() * 100.0,
            200.0 + counter.cos() * 100.0,
            1.0
            ]);
        counter -= 0.15;
        if counter < 0.0 {
            data.append(&mut vec![
                300.0 + f32::sin(0.0) * 100.0,
                200.0 + f32::cos(0.0) * 100.0,
                1.0
                ]);
            break;
        }
    }
    unsafe {
        let mut vao: GLuint = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(ARRAY_BUFFER, vbo);

        gl.BufferData(ARRAY_BUFFER, 4*data.len() as isize, std::mem::transmute(data.as_ptr()), STATIC_DRAW);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 3, FLOAT, FALSE, 0, std::ptr::null());
        (vao, data.len() / 3)
    }
}

fn ortho2d(left: f32, right: f32, bottom: f32, top: f32) -> Vec<f32> {
    let a1 = 2.0 / (right - left);
    let a2 = 2.0 / (top - bottom);
    let a3 = -1.0;
    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = 0.0;
    vec![a1, 0.0, 0.0, 0.0,
         0.0, a2, 0.0, 0.0,
         0.0, 0.0, a3, 0.0,
         tx, ty, tz, 1.0]
}

fn build_dynamic_line_sample(gl: &Gl) -> (GLuint, GLuint, Vec<f32>) {
    unsafe {
        let mut vao: GLuint = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(ARRAY_BUFFER, vbo);

        let vertices = vec![
            300.0, 200.0, 1.0,
            300.0, 200.0, 1.0,
        ];

        gl.BufferData(ARRAY_BUFFER, 4*6, std::mem::transmute(vertices.as_ptr()), DYNAMIC_DRAW);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 3, FLOAT, FALSE, 0, std::ptr::null());
        (vao, vbo, vertices)
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys.window("Title", 600, 400)
                             .opengl()
                             .resizable()
                             .build()
                             .unwrap();

    video_subsys.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
    video_subsys.gl_attr().set_double_buffer(true);
    video_subsys.gl_attr().set_multisample_buffers(2);

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    let gl = Gl::load_with(|s| unsafe {
        std::mem::transmute(video_subsys.gl_get_proc_address(s))
    });

    unsafe {
        let mut major = 102i32;
        gl.GetIntegerv(MAJOR_VERSION, &mut major);
        let mut minor = 304i32;
        gl.GetIntegerv(MINOR_VERSION, &mut minor);
        println!("OpenGL v. {:?}.{:?}", major, minor);
        gl.Enable(MULTISAMPLE);
    }

    let mut exit = false;

    let mut timer = timers::new();
    let death_ray = skills::DeathRay { damage: 150000.0, freq: 500.0 };

    let mut ft = timer.frame_time();

    let network_source = UdpSocket::bind("127.0.0.1:45001").expect("couldn't bind to address");
    network_source.set_nonblocking(true).expect("couldn't set nonblocking");

    // Connect to the server
    {
        let msg = MessageType::AddToListenersRequest;
        let buf = protocol::pack(&msg);
        network_source.send_to(&buf, "127.0.0.1:45000").unwrap();
    }

    let mut active_scene_context: Box<SceneContext> = Box::new(MainSceneContext::new(&gl));

    while !exit {

        active_scene_context.update(&network_source);
        active_scene_context.render();

        ft = timer.frame_time();

        match event_pump.poll_event() {
            Some(event) => {

                match event {

                    Event::Quit {..} => { exit = true; },

                    _ => ()
                }

                active_scene_context.user_input(event);
            },
            None => ()
        }

        window.gl_swap_window();

    }

    // Disonnect from the server
    let msg = MessageType::RemoveFromListeners;
    let buf = protocol::pack(&msg);
    network_source.send_to(&buf, "127.0.0.1:45000").unwrap();
}
