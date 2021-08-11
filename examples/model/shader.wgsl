[[block]]
struct Uniforms {
    view_matrix: mat4x4<f32>;
    projection_matrix: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[block]]
struct Light {
    position: vec3<f32>;
    color: vec3<f32>;
};
[[group(1), binding(0)]]
var<uniform> light: Light;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] normal: vec3<f32>;
};
struct InstanceInput {
    [[location(3)]] model_matrix_0: vec4<f32>;
    [[location(4)]] model_matrix_1: vec4<f32>;
    [[location(5)]] model_matrix_2: vec4<f32>;
    [[location(6)]] model_matrix_3: vec4<f32>;
    [[location(7)]] normal_matrix_0: vec3<f32>;
    [[location(8)]] normal_matrix_1: vec3<f32>;
    [[location(9)]] normal_matrix_2: vec3<f32>;
    [[location(10)]] ambient_color: vec3<f32>;
    [[location(11)]] diffuse_color: vec3<f32>;
    [[location(12)]] specular_color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] ambient_color: vec3<f32>;
    [[location(1)]] diffuse_color: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn main(vertex_input: VertexInput, instance_input: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance_input.model_matrix_0,
        instance_input.model_matrix_1,
        instance_input.model_matrix_2,
        instance_input.model_matrix_3,
    );
    let normal_matrix = mat3x3<f32>(
        instance_input.normal_matrix_0,
        instance_input.normal_matrix_1,
        instance_input.normal_matrix_2,
    );

    let model_view_projection = uniforms.projection_matrix * uniforms.view_matrix * model_matrix;

    var output: VertexOutput;
    output.position = model_view_projection * vec4<f32>(vertex_input.position, 1.0);
    output.ambient_color = instance_input.ambient_color;
    output.diffuse_color = instance_input.diffuse_color;
    output.normal = normal_matrix * vertex_input.normal;
    return output;
}

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    let ambient_strength = 0.1;
    let ambient_color = input.ambient_color * ambient_strength;

    let light_dir = normalize(light.position - input.normal);

    let diffuse_strength = max(dot(input.normal, light_dir), 0.0);
    let diffuse_color = input.diffuse_color * diffuse_strength;

    let color = ambient_color + diffuse_color;

    return vec4<f32>(color, 1.0);
}