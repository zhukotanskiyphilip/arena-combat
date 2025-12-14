// ═══════════════════════════════════════════════════════════════════════════
// ФАЙЛ: assets/shaders/grid.wgsl
// ═══════════════════════════════════════════════════════════════════════════
//
// 📋 ПРИЗНАЧЕННЯ:
//    Grid shader для відображення координатної сітки на підлозі арени.
//    Використовується для debug та візуалізації простору.
//
// 🎯 ВІДПОВІДАЛЬНІСТЬ:
//    - Vertex shader: трансформація вершин grid з world space в clip space
//    - Fragment shader: малювання ліній сітки з fade-out на відстані
//
// 🔗 ЗВ'ЯЗКИ:
//    Використовується в: src/rendering/renderer.rs
//    Uniform buffer: CameraUniform (view-projection матриця)
//
// ⚠️  ВАЖЛИВІ ДЕТАЛІ:
//    - Coordinate system: Y-up, right-handed
//    - Grid розміщується на Y=0 (XZ plane)
//    - Лінії сітки кожні 1.0 unit
//    - Центральні осі (X, Z) виділені іншим кольором
//
// 🕐 ІСТОРІЯ:
//    2025-12-14: Створено - базовий grid shader з fade-out
//
// ═══════════════════════════════════════════════════════════════════════════

// Uniform buffer з camera матрицею
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Вхідні дані для vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// Вихідні дані vertex shader → вхідні для fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// ============================================================================
// VERTEX SHADER
// ============================================================================

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Трансформуємо позицію з world space в clip space
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);

    // Передаємо world position для обчислення fade-out
    output.world_position = input.position;

    // Передаємо колір
    output.color = input.color;

    return output;
}

// ============================================================================
// FRAGMENT SHADER
// ============================================================================

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Базовий колір ліній
    var color = input.color;

    // Обчислюємо відстань від центру (0, 0, 0)
    let distance_from_center = length(input.world_position.xz);

    // Fade-out на відстані (зникає після 20 одиниць)
    let fade_start = 15.0;
    let fade_end = 25.0;
    var alpha = 1.0 - smoothstep(fade_start, fade_end, distance_from_center);

    // Альфа для ліній сітки (трохи прозорі)
    alpha = alpha * 0.3;

    // Якщо це центральна лінія (X або Z осі), робимо яскравіше
    let is_center_x = abs(input.world_position.x) < 0.05;
    let is_center_z = abs(input.world_position.z) < 0.05;

    if (is_center_x || is_center_z) {
        alpha = alpha * 2.0; // Центральні лінії яскравіші
    }

    return vec4<f32>(color, alpha);
}
