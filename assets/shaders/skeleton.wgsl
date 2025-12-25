// Skeleton shader - instanced rendering для кісток
//
// ПІДХІД: Pre-generated geometry
// - Кожен тип кістки має свій mesh з правильними розмірами
// - Shader просто застосовує position/rotation (без scaling/taper)
// - Це гарантує правильні пропорції капсул

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) color: vec4<f32>, // rgb = color, a = unused (1.0)
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) color: vec3<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // Transform position
    let world_position = model_matrix * vec4<f32>(vertex.position, 1.0);

    // Normal matrix (upper-left 3x3 of model matrix)
    let normal_matrix = mat3x3<f32>(
        model_matrix[0].xyz,
        model_matrix[1].xyz,
        model_matrix[2].xyz,
    );

    var output: VertexOutput;
    output.clip_position = camera.view_proj * world_position;
    output.world_normal = normalize(normal_matrix * vertex.normal);
    output.color = instance.color.rgb;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ndotl = max(dot(input.world_normal, light_dir), 0.0);

    let ambient = 0.3;
    let diffuse = ndotl * 0.7;

    let final_color = input.color * (ambient + diffuse);

    return vec4<f32>(final_color, 1.0);
}
