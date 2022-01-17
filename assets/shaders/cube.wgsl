struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};

struct VP {
    matrix: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> vp: VP;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vp.matrix* position;
    return out;
}

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
