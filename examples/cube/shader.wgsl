[[block]]
struct Uniforms {
    view_projection: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};
struct InstanceInput {
    [[location(5)]] model_matrix_0: vec4<f32>;
    [[location(6)]] model_matrix_1: vec4<f32>;
    [[location(7)]] model_matrix_2: vec4<f32>;
    [[location(8)]] model_matrix_3: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn main(vertex_input: VertexInput, instance_input: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance_input.model_matrix_0,
        instance_input.model_matrix_1,
        instance_input.model_matrix_2,
        instance_input.model_matrix_3,
    );

    var output: VertexOutput;
    output.color = vertex_input.color;
    output.position = uniforms.view_projection * model_matrix * vec4<f32>(vertex_input.position, 1.0);
    return output;
}

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}