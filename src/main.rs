extern crate sdl2;
extern crate gfx_gl;
extern crate time;
extern crate chunk_protocol as protocol;
extern crate cgmath;
extern crate collision;

use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use gfx_gl::*;
// use gfx_gl::types::*;
use std::net::{UdpSocket};
use protocol::enums::MessageType;
use std::rc::Rc;
use std::cell::RefCell;
use context::*;

mod shaders;
mod timers;
// mod skills;
// mod threads;
mod rooms_ui;
mod graphics;
mod context;
mod objects;

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

    let mut ft = timer.frame_time();

    let socket = UdpSocket::bind("127.0.0.1:45001").expect("couldn't bind to address");
    socket.set_nonblocking(true).expect("couldn't set nonblocking");
    let network_source = Rc::new(socket);

    // Connect to the server
    {
        let msg = MessageType::AddToListenersRequest;
        let buf = protocol::pack(&msg);
        network_source.send_to(&buf, "127.0.0.1:45000").unwrap();
    }

    let mut active_scene_context: RefSceneContext = Rc::new(RefCell::new(MainSceneContext::new(&gl, &network_source)));

    while !exit {

        active_scene_context.borrow_mut().update();
        active_scene_context.borrow_mut().render();

        ft = timer.frame_time();

        match event_pump.poll_event() {
            Some(event) => {

                match event {

                    Event::Quit {..} => { exit = true; },

                    _ => ()
                }

                active_scene_context.borrow_mut().user_input(event);
            },
            None => ()
        }

        window.gl_swap_window();

        let context = active_scene_context.borrow().switch_context();
        if context.is_some() {
            active_scene_context = context.unwrap();
        }

    }

    // Disonnect from the server
    let msg = MessageType::RemoveFromListeners;
    let buf = protocol::pack(&msg);
    network_source.send_to(&buf, "127.0.0.1:45000").unwrap();
}
