extern crate sdl2;
extern crate gfx_gl;
extern crate cgmath;
extern crate time;

use sdl2::event::Event;
use gfx_gl::*;
use std::ffi::CString;
use gfx_gl::types::*;
// use cgmath::{Deg, Matrix, Matrix3, Matrix4, Point3, Vector3, Vector4, SquareMatrix};

mod shaders;
mod timers;
mod skills;

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

fn build_rectangle_sample(gl: &Gl) -> GLuint {
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

    let orthomatrix = ortho2d(0.0, 600.0, 0.0, 400.0);

    let vao = build_rectangle_sample(&gl);
    let (line_vao, line_vbo, mut line_data) = build_dynamic_line_sample(&gl);
    let (cs_vao, cs_len) = build_circle_sample(&gl);

    let mut program = shaders::new(&gl);
    let vsource = CString::new("
        #version 410 core
        layout(location=0) in vec3 pos;

        out vec2 vpos;

        uniform mat4 supermatrix;

        void main()
        {
            vpos = pos.xy;
            gl_Position = supermatrix * vec4(pos, 1.0);
        }
    ").unwrap();
    let vsb = vsource.to_bytes();
    let fsource = CString::new("
        #version 410 core
        in vec2 vpos;

        out vec4 out_color;

        const vec2 origin = vec2(300.0, 200.0);
        uniform float death_ray_k;

        void main()
        {
            float s = length(vpos - origin) + death_ray_k;
            int a = (int(floor(s)) % 35);
            if (a < 0 * 5) {
                out_color = vec4(1.0, 0.0, 0.1, 1.0);
            } else if (a < 1 * 5) {
                out_color = vec4(1.0, 0.1, 0.2, 1.0);
            } else if (a < 2 * 5) {
                out_color = vec4(1.0, 0.25, 0.2, 1.0);
            } else if (a < 3 * 5) {
                out_color = vec4(1.0, 0.35, 0.3, 1.0);
            } else if (a < 4 * 5) {
                out_color = vec4(1.0, 0.45, 0.3, 1.0);
            } else if (a < 5 * 5) {
                out_color = vec4(1.0, 0.6, 0.3, 1.0);
            } else if (a < 6 * 5) {
                out_color = vec4(1.0, 0.8, 0.4, 1.0);
            } else if (a < 7 * 5) {
                out_color = vec4(1.0, 1.0, 0.4, 1.0);
            }
        }
    ").unwrap();
    let fsb = fsource.to_bytes();
    program.vertex_shader(vsb)
           .fragment_shader(fsb)
           .link();

    let mut exit = false;
    let mut on_fire = false;
    let uname = CString::new("supermatrix").unwrap();
    let death_ray_k_uniform = CString::new("death_ray_k").unwrap();

    let mut timer = timers::new();
    let mut target = 1_000_000.0;
    let death_ray = skills::DeathRay { damage: 150000.0, freq: 500.0 };

    // println!("{:?}", 23 as f32 / 11 as f32);
    let mut ft = timer.frame_time();

    while !exit {

        unsafe {
            gl.ClearColor(0.05, 0.05, 0.1, 1.0);
            gl.Clear(COLOR_BUFFER_BIT);

            program.use_program();
            let mut ul = gl.GetUniformLocation(program.id, uname.to_bytes().as_ptr() as *const i8);
            gl.UniformMatrix4fv(ul, 1, FALSE, orthomatrix.as_ptr());

            ul = gl.GetUniformLocation(program.id, death_ray_k_uniform.to_bytes().as_ptr() as *const i8);
            gl.Uniform1f(ul, timer.elapsed() as f32);

            gl.BindVertexArray(vao);
            gl.DrawArrays(TRIANGLE_STRIP, 0, 4);

            if on_fire {
                gl.BindVertexArray(line_vao);
                gl.DrawArrays(LINES, 0, 2);
            }

            gl.BindVertexArray(cs_vao);
            gl.DrawArrays(LINE_STRIP, 0, cs_len as i32);
        }

        ft = timer.frame_time();
        if on_fire && target > 0.0 {
            println!("{:?}", target);
            target = death_ray.apply(&target, &(ft as f32));
        }

        match event_pump.poll_event() {
            Some(event) => {

                match event {
                    Event::Window {timestamp, ..} => {
                        println!("{:?}", timestamp);
                    },

                    Event::Quit {..} => { exit = true; },

                    Event::MouseMotion {xrel, yrel, timestamp, ..} => {
                        if on_fire {
                            line_data[3] += xrel as f32;
                            line_data[4] -= yrel as f32;
                            unsafe {
                                gl.BindBuffer(ARRAY_BUFFER, line_vbo);
                                gl.BufferSubData(ARRAY_BUFFER, 4*3, 4*2, std::mem::transmute(line_data.get(3)));
                            }
                        }
                    },

                    Event::MouseButtonDown {x, y, ..} => {
                        on_fire = true;
                        if on_fire {
                            line_data[3] = x as f32;
                            line_data[4] = 400.0 - y as f32;
                            unsafe {
                                gl.BindBuffer(ARRAY_BUFFER, line_vbo);
                                gl.BufferSubData(ARRAY_BUFFER, 4*3, 4*2, std::mem::transmute(line_data.get(3)));
                            }
                        }
                    },

                    Event::MouseButtonUp { .. } => {
                        on_fire = false;
                    }

                    _ => { println!("{:?}", event); }
                }

            },
            None => ()
        }

        window.gl_swap_window();

    }
}
