[[block]]
struct Uniforms {
    view: mat4x4<f32>;
    projection: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
};
struct InstanceInput {
    [[location(1)]] model_matrix_0: vec4<f32>;
    [[location(2)]] model_matrix_1: vec4<f32>;
    [[location(3)]] model_matrix_2: vec4<f32>;
    [[location(4)]] model_matrix_3: vec4<f32>;
    [[location(5)]] color: vec3<f32>;
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

    let model_view_projection = uniforms.projection * uniforms.view * model_matrix;

    var output: VertexOutput;
    output.color = instance_input.color;
    output.position = model_view_projection * vec4<f32>(vertex_input.position, 1.0);
    return output;
}

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}