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
                entry_point: "vs_main",
                buffers: &[MeshVertex::vertex_buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
