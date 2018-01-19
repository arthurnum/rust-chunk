extern crate sdl2;
extern crate gfx_gl;
extern crate cgmath;

use sdl2::event::Event;
use gfx_gl::*;
use std::ffi::CString;
use gfx_gl::types::*;
use cgmath::{Deg, Matrix, Matrix3, Matrix4, Point3, Vector3, Vector4, SquareMatrix};

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

fn compile_shader(gl: &Gl, type_: GLenum, source: &[u8]) -> GLuint {
    unsafe {
        let shader = gl.CreateShader(type_);
        gl.ShaderSource(shader, 1, &(source.as_ptr() as *const i8), &(source.len() as i32));
        gl.CompileShader(shader);
        let mut compile_status: i32 = 0;
        gl.GetShaderiv(shader, COMPILE_STATUS, &mut compile_status);
        println!("compile status: {:?}", compile_status);
        let mut log: [i8; 512] = [0; 512];
        let log_ptr: *mut i8 = &mut log[0];
        let length: *mut i32 = &mut 0;
        gl.GetShaderInfoLog(shader, 512, length, log_ptr);
        println!("log length {:?}", *length);
        let s = String::from_raw_parts(log_ptr as *mut u8, *length as usize, 512);
        println!("{:?}", s);

                println!("isShader {:?} -> {:?}", shader, gl.IsShader(shader));
        if compile_status == 0 {
            0
        } else { shader }
    }
}

fn vertex_shader(gl: &Gl) -> GLuint {
    let source = CString::new("
        #version 410 core
        layout(location=0) in vec3 pos;

        uniform mat4 supermatrix;

        void main()
        {
            gl_Position = supermatrix * vec4(pos, 1.0);
        }
    ").unwrap();
    let sb = source.to_bytes();
    compile_shader(gl, VERTEX_SHADER, sb)
}

fn fragment_shader(gl: &Gl) -> GLuint {
    let source = CString::new("
        #version 410 core

        out vec4 out_color;

        void main()
        {
            out_color = vec4(1.0, 1.0, 0.5, 1.0);
        }
    ").unwrap();
    let sb = source.to_bytes();
    compile_shader(gl, FRAGMENT_SHADER, sb)
}

fn create_program(gl: &Gl, vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl.CreateProgram();
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);

        let mut link_status: i32 = 0;
        gl.GetShaderiv(program, LINK_STATUS, &mut link_status);
        println!("link status: {:?}", link_status);
        let mut log: [i8; 512] = [0; 512];
        let log_ptr: *mut i8 = &mut log[0];
        let length: *mut i32 = &mut 0;
        gl.GetProgramInfoLog(program, 512, length, log_ptr);
        println!("log length {:?}", *length);
        let s = String::from_raw_parts(log_ptr as *mut u8, *length as usize, 512);
        println!("{:?}", s);

        program
    }
}

fn draw_rectangle_sample(gl: &Gl) -> GLuint {
    unsafe {
        let mut vao: GLuint = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(ARRAY_BUFFER, vbo);

        let vertices: [f32; 12] = [
            10.0, 10.0, 1.0,
            110.0, 10.0, 1.0,
            10.0, 110.0, 1.0,
            110.0, 110.0, 1.0,
        ];

        gl.BufferData(ARRAY_BUFFER, 4*12, std::mem::transmute(&vertices[0]), STATIC_DRAW);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 3, FLOAT, FALSE, 0, std::ptr::null());
        vao
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
        println!("Version {:?}.{:?}", major, minor);
    }

    let orthomatrix = ortho2d(0.0, 600.0, 0.0, 400.0);
    // let mat:Matrix4<f32> = cgmath::ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0).into();
    // let orthomatrix = cgmath::ortho(0.0, 600.0, 0.0, 400.0, -1.0, 1.0);
    // println!("{:?}", ortho2d(0.0, 600.0, 0.0, 400.0));
    println!("{:?}", orthomatrix);

    let vs = vertex_shader(&gl);
    let fs = fragment_shader(&gl);
    let program = create_program(&gl, vs, fs);
    println!("Program {:?}", program);
    let vao = draw_rectangle_sample(&gl);
    println!("VAO {:?}", vao);

    let mut exit = false;

    while !exit {

        unsafe {
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(COLOR_BUFFER_BIT);

            gl.UseProgram(program);
            let uname = CString::new("supermatrix").unwrap();
            let ul = gl.GetUniformLocation(program, uname.as_ptr());
            gl.UniformMatrix4fv(ul, 1, FALSE, orthomatrix.as_ptr());

            gl.BindVertexArray(vao);
            gl.DrawArrays(TRIANGLE_STRIP, 0, 4);
        }

        match event_pump.poll_event() {
            Some(event) => {
                println!("{:?}", event);

                match event {
                    Event::Window {timestamp, ..} => {
                        println!("{:?}", timestamp);
                    },
                    Event::Quit {..} => { exit = true; }
                    _ => ()
                }

            },
            None => ()
        }

        window.gl_swap_window();

    }
}
