// vertex

struct VertexIn {
    @location(0) position: vec3<f32>,
}

struct VertexOut {
    @builtin(position) pixel: vec4<f32>, // pixel coordinates (0 - 800, 0 - 600)
    @location(0) position: vec3<f32>, // position coordinates (-1 - 1, -1 - 1)
}

@vertex
fn vertex_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.pixel = vec4<f32>(in.position, 1.0);
    out.position = in.position.xyz;
    return out;
}

// fragment shader

struct View {
    offset: vec2<f32>,
    scale: f32,
}

@group(0) @binding(0)
var<uniform> view: View;

fn mandelbrot(c: vec2<f32>) -> i32 {
    var count = 0;
    var z = vec2<f32>(0.0, 0.0);

    while count < 256 && length(z) < 2.0 {
        let r = pow(z.x, 2.0) - pow(z.y, 2.0);
        let i = 2.0 * z.x * z.y;
        
        z.x = r;
        z.y = i;
        z += c;

        count++;
    }

    if count == 256 { return 0; }
    return count - 1;
}

fn color(iterations: i32) -> vec4<f32> {
    let color = f32(iterations) / 32.0;
    return vec4<f32>(color, color * 0.1, color * 0.5, 1.0);
}

@fragment
fn fragment_main(in: VertexOut) -> @location(0) vec4<f32> {
    let iterations = mandelbrot((in.position.xy / view.scale) + view.offset);
    if iterations == 0 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    return color(iterations);
}
