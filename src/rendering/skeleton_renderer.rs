/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/rendering/skeleton_renderer.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Візуалізація фізичного скелета - малює капсули для кожної кістки.
   Використовує instanced rendering для ефективності.

═══════════════════════════════════════════════════════════════════════════════
*/

use wgpu::util::DeviceExt;
use glam::{Vec3, Quat, Mat4};

use crate::physics::{BoneId, PhysicsWorld};
use crate::transform::TransformUniform;

/// Кольори для різних частин тіла (оптимізовано для 11 кісток)
pub fn get_bone_color(bone_id: BoneId) -> [f32; 3] {
    match bone_id {
        // Торс - синій
        BoneId::Pelvis => [0.2, 0.3, 0.8],
        BoneId::Spine => [0.3, 0.4, 0.9],

        // Голова - тілесний
        BoneId::Head => [0.9, 0.75, 0.6],

        // Ліва рука - зелений
        BoneId::LeftUpperArm | BoneId::LeftLowerArm => {
            [0.3, 0.8, 0.3]
        }

        // Права рука - червоний (зброя)
        BoneId::RightUpperArm | BoneId::RightLowerArm => {
            [0.8, 0.3, 0.3]
        }

        // Ліва нога - жовтий
        BoneId::LeftUpperLeg | BoneId::LeftLowerLeg => {
            [0.8, 0.8, 0.3]
        }

        // Права нога - помаранчевий
        BoneId::RightUpperLeg | BoneId::RightLowerLeg => {
            [0.9, 0.5, 0.2]
        }
    }
}

/// Розміри кісток (довжина, радіус) - оптимізовано для 11 кісток
pub fn get_bone_dimensions(bone_id: BoneId) -> (f32, f32) {
    match bone_id {
        BoneId::Pelvis => (0.2, 0.12),
        BoneId::Spine => (0.45, 0.11),  // Довший - об'єднує spine+chest
        BoneId::Head => (0.25, 0.09),   // Включає шию
        BoneId::LeftUpperArm | BoneId::RightUpperArm => (0.28, 0.04),
        BoneId::LeftLowerArm | BoneId::RightLowerArm => (0.25, 0.035),
        BoneId::LeftUpperLeg | BoneId::RightUpperLeg => (0.42, 0.06),
        BoneId::LeftLowerLeg | BoneId::RightLowerLeg => (0.40, 0.045),
    }
}

/// Vertex для капсули
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CapsuleVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl CapsuleVertex {
    pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CapsuleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Генерує капсулу (циліндр з півсферами на кінцях)
pub fn generate_capsule(half_height: f32, radius: f32, segments: u32) -> (Vec<CapsuleVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let rings = segments / 2;

    // === ВЕРХНЯ ПІВСФЕРА ===
    for ring in 0..=rings {
        let phi = (ring as f32 / rings as f32) * std::f32::consts::FRAC_PI_2;
        let y = half_height + radius * phi.sin();
        let ring_radius = radius * phi.cos();

        for seg in 0..=segments {
            let theta = (seg as f32 / segments as f32) * std::f32::consts::TAU;
            let x = ring_radius * theta.cos();
            let z = ring_radius * theta.sin();

            // Normal points outward from sphere center
            let ny = phi.sin();
            let nxz = phi.cos();
            let nx = nxz * theta.cos();
            let nz = nxz * theta.sin();

            vertices.push(CapsuleVertex {
                position: [x, y, z],
                normal: [nx, ny, nz],
            });
        }
    }

    // === ЦИЛІНДР ===
    for i in 0..=1 {
        let y = if i == 0 { half_height } else { -half_height };

        for seg in 0..=segments {
            let theta = (seg as f32 / segments as f32) * std::f32::consts::TAU;
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            vertices.push(CapsuleVertex {
                position: [x, y, z],
                normal: [theta.cos(), 0.0, theta.sin()],
            });
        }
    }

    // === НИЖНЯ ПІВСФЕРА ===
    for ring in 0..=rings {
        let phi = (ring as f32 / rings as f32) * std::f32::consts::FRAC_PI_2;
        let y = -half_height - radius * phi.sin();
        let ring_radius = radius * phi.cos();

        for seg in 0..=segments {
            let theta = (seg as f32 / segments as f32) * std::f32::consts::TAU;
            let x = ring_radius * theta.cos();
            let z = ring_radius * theta.sin();

            let ny = -phi.sin();
            let nxz = phi.cos();
            let nx = nxz * theta.cos();
            let nz = nxz * theta.sin();

            vertices.push(CapsuleVertex {
                position: [x, y, z],
                normal: [nx, ny, nz],
            });
        }
    }

    // === ІНДЕКСИ ===
    let verts_per_ring = segments + 1;

    // Верхня півсфера
    for ring in 0..rings {
        for seg in 0..segments {
            let current = ring * verts_per_ring + seg;
            let next = current + verts_per_ring;

            indices.push(current as u16);
            indices.push(next as u16);
            indices.push((current + 1) as u16);

            indices.push((current + 1) as u16);
            indices.push(next as u16);
            indices.push((next + 1) as u16);
        }
    }

    // Циліндр
    let cylinder_start = (rings + 1) * verts_per_ring;
    for seg in 0..segments {
        let current = cylinder_start + seg;
        let next = current + verts_per_ring;

        indices.push(current as u16);
        indices.push(next as u16);
        indices.push((current + 1) as u16);

        indices.push((current + 1) as u16);
        indices.push(next as u16);
        indices.push((next + 1) as u16);
    }

    // Нижня півсфера
    let bottom_start = cylinder_start + 2 * verts_per_ring;
    for ring in 0..rings {
        for seg in 0..segments {
            let current = bottom_start + ring * verts_per_ring + seg;
            let next = current + verts_per_ring;

            indices.push(current as u16);
            indices.push((current + 1) as u16);
            indices.push(next as u16);

            indices.push((current + 1) as u16);
            indices.push((next + 1) as u16);
            indices.push(next as u16);
        }
    }

    (vertices, indices)
}

/// Instance data для кожної кістки
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoneInstance {
    pub model_matrix: [[f32; 4]; 4],
    pub color: [f32; 3],
    pub _padding: f32,
}

impl BoneInstance {
    pub fn instance_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BoneInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // model_matrix - потребує 4 слоти (mat4)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Renderer для скелета
pub struct SkeletonRenderer {
    capsule_vertex_buffer: wgpu::Buffer,
    capsule_index_buffer: wgpu::Buffer,
    capsule_index_count: u32,

    instance_buffer: wgpu::Buffer,
    instance_count: u32,

    render_pipeline: wgpu::RenderPipeline,
}

impl SkeletonRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        // Генеруємо стандартну капсулу (буде масштабуватись per-instance)
        let (vertices, indices) = generate_capsule(0.5, 1.0, 12);

        let capsule_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Capsule Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let capsule_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Capsule Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Instance buffer (для ~20 кісток)
        let max_bones = 32;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Bone Instance Buffer"),
            size: (std::mem::size_of::<BoneInstance>() * max_bones) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skeleton Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shaders/skeleton.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Skeleton Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skeleton Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    CapsuleVertex::vertex_buffer_layout(),
                    BoneInstance::instance_buffer_layout(),
                ],
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
                cull_mode: Some(wgpu::Face::Back),
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
            capsule_vertex_buffer,
            capsule_index_buffer,
            capsule_index_count: indices.len() as u32,
            instance_buffer,
            instance_count: 0,
            render_pipeline,
        }
    }

    /// Оновлює instances на основі позицій кісток
    pub fn update_bones(
        &mut self,
        queue: &wgpu::Queue,
        bone_transforms: &[(BoneId, Vec3, Quat)],
    ) {
        let mut instances: Vec<BoneInstance> = Vec::new();

        for (bone_id, position, rotation) in bone_transforms {
            let (length, radius) = get_bone_dimensions(*bone_id);
            let color = get_bone_color(*bone_id);

            // Scale: капсула 1.0 висоти масштабується до потрібного розміру
            // Капсула генерується з half_height=0.5, radius=1.0
            // Потрібно масштабувати Y по довжині, XZ по радіусу
            let scale = Vec3::new(radius, length, radius);

            let model_matrix = Mat4::from_scale_rotation_translation(scale, *rotation, *position);

            instances.push(BoneInstance {
                model_matrix: model_matrix.to_cols_array_2d(),
                color,
                _padding: 0.0,
            });
        }

        self.instance_count = instances.len() as u32;

        if !instances.is_empty() {
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, camera_bind_group: &'a wgpu::BindGroup) {
        if self.instance_count == 0 {
            return;
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.capsule_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.capsule_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.capsule_index_count, 0, 0..self.instance_count);
    }
}
