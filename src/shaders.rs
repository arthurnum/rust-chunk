use gfx_gl::*;

pub struct Shader<'a> {
    pub id: u32,
    pub vs: u32,
    pub fs: u32,
    gl: &'a Gl
}

pub fn new<'a>(gl: &'a Gl) -> Box<Shader> {
    Box::new(Shader {
        id: unsafe {
            gl.CreateProgram()
        },
        gl: gl,
        vs: 0,
        fs: 0
    })
}

fn shader_log(gl: &Gl, id: &u32) {
    unsafe {
        let mut compile_status: i32 = 0;
        gl.GetShaderiv(*id, COMPILE_STATUS, &mut compile_status);
        println!("Compile status: {:?}", compile_status > 0);
        let mut log: [i8; 512] = [0; 512];
        let log_ptr: *mut i8 = &mut log[0];
        let length: *mut i32 = &mut 0;
        gl.GetShaderInfoLog(*id, 512, length, log_ptr);
        println!("Log length: {:?}", *length);
        let s = String::from_raw_parts(log_ptr as *mut u8, *length as usize, 512);
        println!("Log: {:?}", s);
        println!("isShader: {:?} -> {:?}", *id, gl.IsShader(*id) > 0);
        println!();
    }
}

fn build_shader(gl: &Gl, type_: u32, source: &[u8]) -> u32 {
    unsafe {
        let id = gl.CreateShader(type_);
        gl.ShaderSource(id, 1, &(source.as_ptr() as *const i8), &(source.len() as i32));
        gl.CompileShader(id);
        id
    }
}

impl<'a> Shader<'a> {
    pub fn vertex_shader(&mut self, source: &[u8]) -> &mut Shader<'a> {
        println!("##### Vertex Shader #####");
        self.vs = build_shader(&self.gl, VERTEX_SHADER, &source);
        shader_log(&self.gl, &self.vs);
        self
    }

    pub fn fragment_shader(&mut self, source: &[u8]) -> &mut Shader<'a> {
        println!("##### Fragment Shader #####");
        self.fs = build_shader(&self.gl, FRAGMENT_SHADER, &source);
        shader_log(&self.gl, &self.fs);
        self
    }

    pub fn link(&self) {
        println!("##### Link Program #####");
        unsafe {
            self.gl.AttachShader(self.id, self.vs);
            self.gl.AttachShader(self.id, self.fs);
            self.gl.LinkProgram(self.id);

            let mut link_status: i32 = 0;
            self.gl.GetProgramiv(self.id, LINK_STATUS, &mut link_status);
            println!("Link status: {:?}", link_status > 0);
            let mut log: [i8; 512] = [0; 512];
            let log_ptr: *mut i8 = &mut log[0];
            let length: *mut i32 = &mut 0;
            self.gl.GetProgramInfoLog(self.id, 512, length, log_ptr);
            println!("Log length: {:?}", *length);
            let s = String::from_raw_parts(log_ptr as *mut u8, *length as usize, 512);
            println!("Log: {:?}", s);
            println!();
        }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}
