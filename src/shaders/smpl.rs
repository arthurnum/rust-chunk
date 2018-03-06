pub const DEFAULT_VERTEX: &'static str = "
    #version 410 core
    layout(location=0) in vec3 pos;

    uniform mat4 supermatrix;

    void main()
    {
        gl_Position = supermatrix * vec4(pos, 1.0);
    }
";

pub const YELLOW_FRAGMENT: &'static str = "
    #version 410 core

    out vec4 out_color;

    void main()
    {
        out_color = vec4(1.0, 1.0, 0.4, 1.0);
    }
";

pub const BACKGROUND_FRAGMENT: &'static str = "
    #version 410 core

    out vec4 out_color;

    uniform float time;

    void main()
    {
        vec2 uv = gl_FragCoord.xy / vec2(600.0, 400.0);
        out_color = vec4(uv.x / 2.0 + 0.5, uv.y / 2.0 + 0.5, sin(time), 1.0);
    }
";
