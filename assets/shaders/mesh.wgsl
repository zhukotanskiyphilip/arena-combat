/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: assets/shaders/mesh.wgsl
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   WGSL shader для рендерингу 3D mesh об'єктів з базовим освітленням.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Vertex shader: transform position, pass normal та color
   - Fragment shader: базове diffuse освітлення (directional light)

🔗 ЗВ'ЯЗКИ:
   Використовується в: src/rendering/mesh.rs

⚠️  ВАЖЛИВІ ДЕТАЛІ:
   - Directional light: фіксований напрямок (зверху-спереду)
   - Ambient light: 0.3 (щоб тіні не були повністю чорними)
   - Diffuse: dot(normal, light_dir) для освітлення граней

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базовий mesh shader з diffuse lighting

═══════════════════════════════════════════════════════════════════════════════
*/

// Camera uniform (той самий що і для grid)
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

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

    // Transform position to clip space
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);

    // Pass normal (в майбутньому треба трансформувати через normal matrix)
    // Поки що без model transform, тому normal залишається як є
    output.world_normal = input.normal;

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
