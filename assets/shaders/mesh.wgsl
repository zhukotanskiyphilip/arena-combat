/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: assets/shaders/mesh.wgsl
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   WGSL shader для рендерингу 3D mesh об'єктів з базовим освітленням.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Vertex shader: transform position через Model matrix, pass normal та color
   - Fragment shader: базове diffuse освітлення (directional light)

🔗 ЗВ'ЯЗКИ:
   Використовується в: src/rendering/mesh.rs

⚠️  ВАЖЛИВІ ДЕТАЛІ:
   - Model matrix: local space → world space
   - Normal matrix: для коректної трансформації нормалей (inverse transpose)
   - Directional light: фіксований напрямок (зверху-спереду)
   - Ambient light: 0.3 (щоб тіні не були повністю чорними)

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базовий mesh shader з diffuse lighting
   2025-12-14: Додано Model matrix та Normal matrix

═══════════════════════════════════════════════════════════════════════════════
*/

// Camera uniform (View-Projection matrix)
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Transform uniform (Model matrix + Normal matrix)
struct TransformUniform {
    model: mat4x4<f32>,
    normal_matrix_0: vec4<f32>,
    normal_matrix_1: vec4<f32>,
    normal_matrix_2: vec4<f32>,
    _padding: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> transform: TransformUniform;

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// ============================================================================
// VERTEX SHADER
// ============================================================================

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Transform position: local → world → clip
    let world_position = transform.model * vec4<f32>(input.position, 1.0);
    output.clip_position = camera.view_proj * world_position;

    // Transform normal using normal matrix (3x3 upper-left of inverse transpose)
    let normal_matrix = mat3x3<f32>(
        transform.normal_matrix_0.xyz,
        transform.normal_matrix_1.xyz,
        transform.normal_matrix_2.xyz
    );
    output.world_normal = normal_matrix * input.normal;

    // Pass color
    output.color = input.color;

    return output;
}

// ============================================================================
// FRAGMENT SHADER
// ============================================================================

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Directional light (normalized direction FROM light TO surface)
    // Light comes from top-front-right (typical 3-point lighting key light position)
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));

    // Normalize the interpolated normal
    let normal = normalize(input.world_normal);

    // Ambient light (base illumination so shadows aren't pitch black)
    let ambient = 0.3;

    // Diffuse lighting (Lambert)
    // dot(N, L) gives cosine of angle between normal and light
    // max(0, ...) clamps negative values (surfaces facing away from light)
    let diffuse = max(dot(normal, light_dir), 0.0);

    // Final lighting = ambient + diffuse
    // Clamped to 1.0 to prevent over-brightening
    let lighting = min(ambient + diffuse, 1.0);

    // Apply lighting to color
    let final_color = input.color * lighting;

    return vec4<f32>(final_color, 1.0);
}
