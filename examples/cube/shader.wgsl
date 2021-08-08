struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.color = input.color;
    output.clip_position = vec4<f32>(input.position, 1.0);
    return output;
}

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}