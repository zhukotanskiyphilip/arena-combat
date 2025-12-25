/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/rendering/mesh.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Mesh система - рендеринг 3D об'єктів (куби, моделі, тощо).

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - MeshVertex struct (position + normal + color)
   - Генерація простих примітивів (cube, sphere, plane)
   - Mesh struct з vertex/index buffers
   - Render pipeline для 3D об'єктів
   - Transform support (Model matrix)

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - wgpu - GPU rendering
   - bytemuck - GPU data conversion
   - transform - Transform, TransformUniform

   Експортує для:
   - renderer.rs - інтеграція в render loop

⚠️  ВАЖЛИВІ ДЕТАЛІ:
   - Coordinate system: Y-up, right-handed
   - Normals: outward facing for lighting
   - Winding order: counter-clockwise (CCW) for front faces
   - Index format: u16 (max 65535 vertices per mesh)
   - Transform: Model matrix в group(1) binding(0)

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базовий mesh rendering з cube primitive
   2025-12-14: Додано Transform support (Model matrix)

═══════════════════════════════════════════════════════════════════════════════
*/

use wgpu::util::DeviceExt;
use crate::transform::{Transform, TransformUniform};
use crate::debug_log::log_debug;

/// Vertex структура для 3D mesh
///
/// Містить:
/// - position: позиція в local space
/// - normal: нормаль для освітлення
/// - color: колір вершини (для debug або vertex coloring)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl MeshVertex {
    /// Vertex buffer layout для wgpu pipeline
    pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position: location 0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal: location 1
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color: location 2
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Генерує циліндр вздовж Y-осі
///
/// # Аргументи
/// * `radius` - радіус циліндра
/// * `height` - висота циліндра
/// * `segments` - кількість сегментів по колу (більше = гладкіший)
/// * `color` - колір всіх вершин
///
/// # Повертає
/// (vertices, indices) - вершини та індекси для rendering
pub fn generate_cylinder(radius: f32, height: f32, segments: u32, color: [f32; 3]) -> (Vec<MeshVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let half_height = height / 2.0;

    // Генеруємо бокову поверхню
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let nx = angle.cos();
        let nz = angle.sin();

        // Bottom vertex
        vertices.push(MeshVertex {
            position: [x, -half_height, z],
            normal: [nx, 0.0, nz],
            color,
        });

        // Top vertex
        vertices.push(MeshVertex {
            position: [x, half_height, z],
            normal: [nx, 0.0, nz],
            color,
        });
    }

    // Індекси для бокової поверхні
    for i in 0..segments {
        let base = i * 2;
        // Two triangles per quad
        indices.push(base as u16);
        indices.push((base + 1) as u16);
        indices.push((base + 2) as u16);

        indices.push((base + 2) as u16);
        indices.push((base + 1) as u16);
        indices.push((base + 3) as u16);
    }

    // Top cap
    let top_center_idx = vertices.len() as u16;
    vertices.push(MeshVertex {
        position: [0.0, half_height, 0.0],
        normal: [0.0, 1.0, 0.0],
        color,
    });

    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        vertices.push(MeshVertex {
            position: [x, half_height, z],
            normal: [0.0, 1.0, 0.0],
            color,
        });
    }

    // Top cap indices
    for i in 0..segments {
        let base = top_center_idx + 1 + i as u16;
        indices.push(top_center_idx);
        indices.push(base + 1);
        indices.push(base);
    }

    // Bottom cap
    let bottom_center_idx = vertices.len() as u16;
    vertices.push(MeshVertex {
        position: [0.0, -half_height, 0.0],
        normal: [0.0, -1.0, 0.0],
        color,
    });

    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        vertices.push(MeshVertex {
            position: [x, -half_height, z],
            normal: [0.0, -1.0, 0.0],
            color,
        });
    }

    // Bottom cap indices (reversed winding)
    for i in 0..segments {
        let base = bottom_center_idx + 1 + i as u16;
        indices.push(bottom_center_idx);
        indices.push(base);
        indices.push(base + 1);
    }

    (vertices, indices)
}

/// Генерує сферу з центром в (0, 0, 0)
///
/// # Аргументи
/// * `radius` - радіус сфери
/// * `h_segments` - горизонтальні сегменти (longitude)
/// * `v_segments` - вертикальні сегменти (latitude)
/// * `color` - колір всіх вершин
///
/// # Повертає
/// (vertices, indices) - вершини та індекси для rendering
pub fn generate_sphere(radius: f32, h_segments: u32, v_segments: u32, color: [f32; 3]) -> (Vec<MeshVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for v in 0..=v_segments {
        let v_angle = (v as f32 / v_segments as f32) * std::f32::consts::PI;
        let y = v_angle.cos();
        let ring_radius = v_angle.sin();

        for h in 0..=h_segments {
            let h_angle = (h as f32 / h_segments as f32) * std::f32::consts::TAU;
            let x = ring_radius * h_angle.cos();
            let z = ring_radius * h_angle.sin();

            vertices.push(MeshVertex {
                position: [x * radius, y * radius, z * radius],
                normal: [x, y, z], // Normalized (unit sphere)
                color,
            });
        }
    }

    // Generate indices
    for v in 0..v_segments {
        for h in 0..h_segments {
            let current = v * (h_segments + 1) + h;
            let next = current + h_segments + 1;

            // Two triangles per quad
            indices.push(current as u16);
            indices.push(next as u16);
            indices.push((current + 1) as u16);

            indices.push((current + 1) as u16);
            indices.push(next as u16);
            indices.push((next + 1) as u16);
        }
    }

    (vertices, indices)
}

/// Генерує манекен гравця (капсулоподібна фігура)
///
/// Складається з:
/// - Тіло (циліндр)
/// - Голова (сфера зверху)
///
/// # Аргументи
/// * `body_radius` - радіус тіла
/// * `body_height` - висота тіла (без голови)
/// * `head_radius` - радіус голови
/// * `body_color` - колір тіла
/// * `head_color` - колір голови
///
/// # Повертає
/// (vertices, indices) - вершини та індекси для rendering
pub fn generate_player_mannequin(
    body_radius: f32,
    body_height: f32,
    head_radius: f32,
    body_color: [f32; 3],
    head_color: [f32; 3],
) -> (Vec<MeshVertex>, Vec<u16>) {
    let segments = 12; // Достатньо для гладкого вигляду

    // Генеруємо тіло (циліндр)
    let (mut vertices, mut indices) = generate_cylinder(body_radius, body_height, segments, body_color);

    // Генеруємо голову (сфера)
    let (head_vertices, head_indices) = generate_sphere(head_radius, segments, segments / 2, head_color);

    // Offset голови вгору (на верх тіла + радіус голови)
    let head_y_offset = body_height / 2.0 + head_radius * 0.8; // Трохи втоплена в тіло

    // Додаємо голову з offset
    let vertex_offset = vertices.len() as u16;
    for mut v in head_vertices {
        v.position[1] += head_y_offset;
        vertices.push(v);
    }

    for idx in head_indices {
        indices.push(idx + vertex_offset);
    }

    (vertices, indices)
}

/// Генерує тіло гравця (без руки зі зброєю)
///
/// Складається з:
/// - Тіло (циліндр)
/// - Голова (сфера)
/// - Груди (випуклість спереду для орієнтації)
/// - Обличчя (плоска частина голови спереду)
///
/// Forward direction = -Z (коли yaw=0)
/// Рука з мечем генерується окремо для анімації
pub fn generate_player_body(
    body_color: [f32; 3],
    head_color: [f32; 3],
) -> (Vec<MeshVertex>, Vec<u16>) {
    let segments = 12;

    // Body parameters
    let body_radius: f32 = 0.3;
    let body_height: f32 = 1.2;
    let head_radius: f32 = 0.25;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // === BODY ===
    let (body_verts, body_idx) = generate_cylinder(body_radius, body_height, segments, body_color);
    vertices.extend(body_verts);
    indices.extend(body_idx);

    // === ARROW (довга стрілка вперед для наочності напрямку) ===
    // Яскраво-червона стрілка в напрямку -Z
    let arrow_color = [1.0, 0.0, 0.0]; // Яскраво-червоний
    let (arrow_verts, arrow_idx) = generate_box(0.1, 0.1, 1.5, arrow_color); // Довга коробка
    let arrow_z = -0.75 - body_radius; // Центр стрілки попереду тіла
    let arrow_y = 0.3;
    let vertex_offset = vertices.len() as u16;
    for mut v in arrow_verts {
        v.position[1] += arrow_y;
        v.position[2] += arrow_z;
        vertices.push(v);
    }
    for idx in arrow_idx {
        indices.push(idx + vertex_offset);
    }

    // === HEAD ===
    let (head_verts, head_idx) = generate_sphere(head_radius, segments, segments / 2, head_color);
    let head_y_offset = body_height / 2.0 + head_radius * 0.8;
    let vertex_offset = vertices.len() as u16;
    for mut v in head_verts {
        v.position[1] += head_y_offset;
        vertices.push(v);
    }
    for idx in head_idx {
        indices.push(idx + vertex_offset);
    }

    // === FACE (ніс/обличчя спереду голови) ===
    // Маленька піраміда/конус як ніс
    let face_color = [0.9, 0.75, 0.6]; // Тілесний колір
    let nose_size = 0.08;
    let nose_z = -(head_radius + nose_size * 0.5);
    let nose_y = head_y_offset;

    // Простий "ніс" - маленький box
    let (nose_verts, nose_idx) = generate_box(nose_size, nose_size * 0.8, nose_size, face_color);
    let vertex_offset = vertices.len() as u16;
    for mut v in nose_verts {
        v.position[1] += nose_y;
        v.position[2] += nose_z;
        vertices.push(v);
    }
    for idx in nose_idx {
        indices.push(idx + vertex_offset);
    }

    (vertices, indices)
}

/// Генерує руку з мечем (для анімації)
///
/// Pivot point (центр обертання) - на плечі (0, 0, 0).
/// Рука йде вправо (+X), меч направлений вперед (-Z)
pub fn generate_weapon_arm(
    arm_color: [f32; 3],
    weapon_color: [f32; 3],
) -> (Vec<MeshVertex>, Vec<u16>) {
    // Arm parameters
    let arm_radius = 0.08;
    let arm_length = 0.6;

    // Weapon parameters
    let weapon_width = 0.08;
    let weapon_length = 1.0;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // === ARM ===
    // Pivot at (0, 0, 0), arm extends in +X direction
    let (arm_verts, arm_idx) = generate_cylinder(arm_radius, arm_length, 8, arm_color);
    for mut v in arm_verts {
        // Повертаємо циліндр: Y-axis → X-axis
        let old_y = v.position[1];
        v.position[1] = v.position[0];
        v.position[0] = old_y + arm_length / 2.0;  // Зсув щоб початок був на pivot

        let old_ny = v.normal[1];
        v.normal[1] = v.normal[0];
        v.normal[0] = old_ny;

        vertices.push(v);
    }
    indices.extend(arm_idx);

    // === WEAPON (sword) ===
    // Attached at end of arm, pointing forward (-Z)
    let weapon_x = arm_length;           // Кінець руки
    let weapon_z = -weapon_length / 2.0; // Центр меча попереду

    let (weapon_verts, weapon_idx) = generate_box(weapon_width, weapon_width, weapon_length, weapon_color);
    let vertex_offset = vertices.len() as u16;
    for mut v in weapon_verts {
        v.position[0] += weapon_x;
        v.position[2] += weapon_z;
        vertices.push(v);
    }
    for idx in weapon_idx {
        indices.push(idx + vertex_offset);
    }

    (vertices, indices)
}

/// Генерує box (паралелепіпед) з центром в (0, 0, 0)
///
/// # Аргументи
/// * `width` - розмір по X
/// * `height` - розмір по Y
/// * `depth` - розмір по Z
/// * `color` - колір
pub fn generate_box(width: f32, height: f32, depth: f32, color: [f32; 3]) -> (Vec<MeshVertex>, Vec<u16>) {
    let hx = width / 2.0;
    let hy = height / 2.0;
    let hz = depth / 2.0;

    let vertices = vec![
        // Front face (Z+)
        MeshVertex { position: [-hx, -hy,  hz], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [ hx, -hy,  hz], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [ hx,  hy,  hz], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [-hx,  hy,  hz], normal: [0.0, 0.0, 1.0], color },
        // Back face (Z-)
        MeshVertex { position: [ hx, -hy, -hz], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [-hx, -hy, -hz], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [-hx,  hy, -hz], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [ hx,  hy, -hz], normal: [0.0, 0.0, -1.0], color },
        // Top face (Y+)
        MeshVertex { position: [-hx,  hy,  hz], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [ hx,  hy,  hz], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [ hx,  hy, -hz], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [-hx,  hy, -hz], normal: [0.0, 1.0, 0.0], color },
        // Bottom face (Y-)
        MeshVertex { position: [-hx, -hy, -hz], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [ hx, -hy, -hz], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [ hx, -hy,  hz], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [-hx, -hy,  hz], normal: [0.0, -1.0, 0.0], color },
        // Right face (X+)
        MeshVertex { position: [ hx, -hy,  hz], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ hx, -hy, -hz], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ hx,  hy, -hz], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ hx,  hy,  hz], normal: [1.0, 0.0, 0.0], color },
        // Left face (X-)
        MeshVertex { position: [-hx, -hy, -hz], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-hx, -hy,  hz], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-hx,  hy,  hz], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-hx,  hy, -hz], normal: [-1.0, 0.0, 0.0], color },
    ];

    let indices: Vec<u16> = vec![
        0, 1, 2,  2, 3, 0,     // Front
        4, 5, 6,  6, 7, 4,     // Back
        8, 9, 10,  10, 11, 8,  // Top
        12, 13, 14,  14, 15, 12, // Bottom
        16, 17, 18,  18, 19, 16, // Right
        20, 21, 22,  22, 23, 20, // Left
    ];

    (vertices, indices)
}

/// Генерує куб з центром в (0, 0, 0)
///
/// # Аргументи
/// * `size` - розмір куба (від -size/2 до +size/2 по кожній осі)
/// * `color` - колір всіх вершин
///
/// # Повертає
/// (vertices, indices) - вершини та індекси для rendering
///
/// # Деталі
/// - 24 вершини (4 на кожну грань, бо різні нормалі)
/// - 36 індексів (6 граней × 2 трикутники × 3 вершини)
/// - Нормалі направлені назовні
/// - CCW winding order
pub fn generate_cube(size: f32, color: [f32; 3]) -> (Vec<MeshVertex>, Vec<u16>) {
    let half = size / 2.0;

    // 6 граней куба, кожна з 4 вершинами (різні нормалі для кожної грані)
    let vertices = vec![
        // Front face (Z+) - дивиться на нас
        MeshVertex { position: [-half, -half,  half], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [ half, -half,  half], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [ half,  half,  half], normal: [0.0, 0.0, 1.0], color },
        MeshVertex { position: [-half,  half,  half], normal: [0.0, 0.0, 1.0], color },

        // Back face (Z-) - дивиться від нас
        MeshVertex { position: [ half, -half, -half], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [-half, -half, -half], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [-half,  half, -half], normal: [0.0, 0.0, -1.0], color },
        MeshVertex { position: [ half,  half, -half], normal: [0.0, 0.0, -1.0], color },

        // Top face (Y+) - дивиться вгору
        MeshVertex { position: [-half,  half,  half], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [ half,  half,  half], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [ half,  half, -half], normal: [0.0, 1.0, 0.0], color },
        MeshVertex { position: [-half,  half, -half], normal: [0.0, 1.0, 0.0], color },

        // Bottom face (Y-) - дивиться вниз
        MeshVertex { position: [-half, -half, -half], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [ half, -half, -half], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [ half, -half,  half], normal: [0.0, -1.0, 0.0], color },
        MeshVertex { position: [-half, -half,  half], normal: [0.0, -1.0, 0.0], color },

        // Right face (X+) - дивиться вправо
        MeshVertex { position: [ half, -half,  half], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ half, -half, -half], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ half,  half, -half], normal: [1.0, 0.0, 0.0], color },
        MeshVertex { position: [ half,  half,  half], normal: [1.0, 0.0, 0.0], color },

        // Left face (X-) - дивиться вліво
        MeshVertex { position: [-half, -half, -half], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-half, -half,  half], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-half,  half,  half], normal: [-1.0, 0.0, 0.0], color },
        MeshVertex { position: [-half,  half, -half], normal: [-1.0, 0.0, 0.0], color },
    ];

    // Індекси для 6 граней (2 трикутники на грань, CCW winding)
    let indices: Vec<u16> = vec![
        // Front
        0, 1, 2,  2, 3, 0,
        // Back
        4, 5, 6,  6, 7, 4,
        // Top
        8, 9, 10,  10, 11, 8,
        // Bottom
        12, 13, 14,  14, 15, 12,
        // Right
        16, 17, 18,  18, 19, 16,
        // Left
        20, 21, 22,  22, 23, 20,
    ];

    (vertices, indices)
}

/// Mesh struct для рендерингу 3D об'єктів
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    render_pipeline: wgpu::RenderPipeline,

    /// Transform для позиціонування mesh
    pub transform: Transform,

    /// Transform uniform buffer
    transform_uniform: TransformUniform,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
}

impl Mesh {
    /// Створює новий Mesh з вершин та індексів
    ///
    /// # Аргументи
    /// * `device` - wgpu Device
    /// * `config` - Surface configuration (для формату)
    /// * `vertices` - Вершини mesh
    /// * `indices` - Індекси для indexed drawing
    /// * `camera_bind_group_layout` - Layout для camera uniform
    /// * `transform` - Початковий transform для mesh
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        vertices: &[MeshVertex],
        indices: &[u16],
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        transform: Transform,
    ) -> Self {
        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Transform uniform
        let mut transform_uniform = TransformUniform::new();
        transform_uniform.update(&transform);

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Transform bind group layout
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("transform_bind_group_layout"),
            });

        // Transform bind group
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
            label: Some("transform_bind_group"),
        });

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mesh Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shaders/mesh.wgsl").into()),
        });

        // Pipeline layout (camera @ group(0), transform @ group(1))
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Mesh Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout, &transform_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[MeshVertex::vertex_buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back), // Back-face culling
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            render_pipeline,
            transform,
            transform_uniform,
            transform_buffer,
            transform_bind_group,
        }
    }

    /// Оновлює transform buffer на GPU
    ///
    /// Викликайте після зміни self.transform
    pub fn update_transform(&mut self, queue: &wgpu::Queue) {
        // DEBUG: log model matrix before upload
        let model = self.transform.model_matrix();
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            if COUNTER % 120 == 0 {
                log_debug(&format!("GPU upload model[0]: [{:.3}, {:.3}, {:.3}, {:.3}]",
                    model.x_axis.x, model.x_axis.y, model.x_axis.z, model.x_axis.w));
            }
        }

        self.transform_uniform.update(&self.transform);
        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[self.transform_uniform]),
        );
    }

    /// Рендерить mesh
    ///
    /// # Аргументи
    /// * `render_pass` - Активний render pass
    /// * `camera_bind_group` - Bind group з camera uniform
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, camera_bind_group: &'a wgpu::BindGroup) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
