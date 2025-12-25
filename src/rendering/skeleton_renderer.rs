/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/rendering/skeleton_renderer.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Візуалізація фізичного скелета - малює TAPERED CAPSULES для кожної кістки.

   ПІДХІД: Pre-generated meshes
   - Для кожного ТИПУ кістки генерується окремий mesh з реальними розмірами
   - Однакові кістки (ліва/права рука) використовують той самий mesh
   - Shader НЕ масштабує геометрію, тільки застосовує position/rotation
   - Це гарантує правильні пропорції без спотворення caps

═══════════════════════════════════════════════════════════════════════════════
*/

use wgpu::util::DeviceExt;
use glam::{Vec3, Quat, Mat4};
use std::collections::HashMap;

use crate::physics::BoneId;
use crate::debug_log::log_debug;

/// Кольори для різних частин тіла
pub fn get_bone_color(bone_id: BoneId) -> [f32; 3] {
    match bone_id {
        // Торс - синій
        BoneId::Pelvis => [0.2, 0.3, 0.8],
        BoneId::Spine => [0.3, 0.4, 0.9],

        // Голова - тілесний
        BoneId::Head => [0.9, 0.75, 0.6],

        // Ліва рука - зелений
        BoneId::LeftUpperArm | BoneId::LeftLowerArm => [0.3, 0.8, 0.3],

        // Права рука - червоний (зброя)
        BoneId::RightUpperArm | BoneId::RightLowerArm => [0.8, 0.3, 0.3],

        // Ліва нога - жовтий
        BoneId::LeftUpperLeg | BoneId::LeftLowerLeg => [0.8, 0.8, 0.3],

        // Права нога - помаранчевий
        BoneId::RightUpperLeg | BoneId::RightLowerLeg => [0.9, 0.5, 0.2],
    }
}

/// Розміри кісток (довжина, радіус_верх, радіус_низ) - TAPERED для анатомічної коректності
///
/// Людські кінцівки мають різну товщину на різних кінцях:
/// - Стегно: товще біля тазу (~0.10м), тонше біля коліна (~0.06м)
/// - Гомілка: товще біля коліна (~0.055м), тонше біля щиколотки (~0.035м)
/// - Плече: товще біля плеча (~0.055м), тонше біля ліктя (~0.04м)
/// - Передпліччя: товще біля ліктя (~0.04м), тонше біля зап'ястя (~0.025м)
pub fn get_bone_dimensions(bone_id: BoneId) -> (f32, f32, f32) {
    // Повертає (length, radius_top, radius_bottom)
    // top = ближче до центру тіла (+Y в локальних координатах кістки)
    // bottom = далі від центру тіла (-Y)
    match bone_id {
        // === ТОРС (симетричний) ===
        BoneId::Pelvis => (0.15, 0.14, 0.14),   // Таз - широкий, симетричний
        BoneId::Spine => (0.46, 0.12, 0.16),    // Груди ширші зверху ніж живіт
        BoneId::Head => (0.29, 0.09, 0.06),     // Голова + шия: голова широка, шия тонка

        // === РУКИ (tapered - товще біля тіла) ===
        BoneId::LeftUpperArm | BoneId::RightUpperArm => (0.32, 0.055, 0.038),
        BoneId::LeftLowerArm | BoneId::RightLowerArm => (0.29, 0.042, 0.028),

        // === НОГИ (tapered - товще біля тіла) ===
        BoneId::LeftUpperLeg | BoneId::RightUpperLeg => (0.45, 0.10, 0.065),
        BoneId::LeftLowerLeg | BoneId::RightLowerLeg => (0.40, 0.058, 0.038),
    }
}

/// Типи кісток для групування однакових meshes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoneType {
    Pelvis,
    Spine,
    Head,
    UpperArm,
    LowerArm,
    UpperLeg,
    LowerLeg,
}

impl BoneType {
    fn from_bone_id(bone_id: BoneId) -> Self {
        match bone_id {
            BoneId::Pelvis => BoneType::Pelvis,
            BoneId::Spine => BoneType::Spine,
            BoneId::Head => BoneType::Head,
            BoneId::LeftUpperArm | BoneId::RightUpperArm => BoneType::UpperArm,
            BoneId::LeftLowerArm | BoneId::RightLowerArm => BoneType::LowerArm,
            BoneId::LeftUpperLeg | BoneId::RightUpperLeg => BoneType::UpperLeg,
            BoneId::LeftLowerLeg | BoneId::RightLowerLeg => BoneType::LowerLeg,
        }
    }

    /// Повертає реальні розміри для цього типу кістки
    fn dimensions(&self) -> (f32, f32, f32) {
        match self {
            BoneType::Pelvis => (0.15, 0.14, 0.14),
            BoneType::Spine => (0.46, 0.12, 0.16),
            BoneType::Head => (0.29, 0.09, 0.06),
            BoneType::UpperArm => (0.32, 0.055, 0.038),
            BoneType::LowerArm => (0.29, 0.042, 0.028),
            BoneType::UpperLeg => (0.45, 0.10, 0.065),
            BoneType::LowerLeg => (0.40, 0.058, 0.038),
        }
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

/// Генерує TAPERED CAPSULE з реальними розмірами
///
/// # Аргументи
/// * `length` - довжина кістки (від суглоба до суглоба)
/// * `radius_top` - радіус на верхньому кінці (+Y)
/// * `radius_bottom` - радіус на нижньому кінці (-Y)
/// * `segments` - кількість сегментів по колу
///
/// Capsule складається з:
/// - Top hemisphere (радіус = radius_top)
/// - Tapered cylinder (від radius_top до radius_bottom)
/// - Bottom hemisphere (радіус = radius_bottom)
///
/// Центр капсули в (0, 0, 0), орієнтована вздовж Y осі
pub fn generate_tapered_capsule_real(
    length: f32,
    radius_top: f32,
    radius_bottom: f32,
    segments: u32,
) -> (Vec<CapsuleVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Cylinder half-height (частина БЕЗ caps)
    // total_length = cylinder_height + radius_top + radius_bottom
    // cylinder_height = length - radius_top - radius_bottom
    let cylinder_half_height = (length - radius_top - radius_bottom).max(0.01) / 2.0;

    let rings = segments / 2;
    let cylinder_rings = 4;

    // === TOP HEMISPHERE (at Y = +cylinder_half_height) ===
    for ring in 0..=rings {
        let phi = (ring as f32 / rings as f32) * std::f32::consts::FRAC_PI_2;
        let y = cylinder_half_height + radius_top * phi.sin();
        let ring_radius = radius_top * phi.cos();

        for seg in 0..=segments {
            let theta = (seg as f32 / segments as f32) * std::f32::consts::TAU;
            let x = ring_radius * theta.cos();
            let z = ring_radius * theta.sin();

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

    // === TAPERED CYLINDER ===
    for ring in 0..=cylinder_rings {
        let t = ring as f32 / cylinder_rings as f32; // 0 = top, 1 = bottom
        let y = cylinder_half_height - t * 2.0 * cylinder_half_height;
        let radius = radius_top + t * (radius_bottom - radius_top);

        for seg in 0..=segments {
            let theta = (seg as f32 / segments as f32) * std::f32::consts::TAU;
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            // Normal for tapered cylinder - account for slope
            let slope = (radius_top - radius_bottom) / (2.0 * cylinder_half_height);
            let ny = slope / (1.0 + slope * slope).sqrt();
            let nxz = 1.0 / (1.0 + slope * slope).sqrt();
            let nx = nxz * theta.cos();
            let nz = nxz * theta.sin();

            vertices.push(CapsuleVertex {
                position: [x, y, z],
                normal: [nx, ny, nz],
            });
        }
    }

    // === BOTTOM HEMISPHERE (at Y = -cylinder_half_height) ===
    for ring in 0..=rings {
        let phi = (ring as f32 / rings as f32) * std::f32::consts::FRAC_PI_2;
        let y = -cylinder_half_height - radius_bottom * phi.sin();
        let ring_radius = radius_bottom * phi.cos();

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

    // === INDICES ===
    let verts_per_ring = segments + 1;

    // Top hemisphere
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

    // Tapered cylinder
    let cylinder_start = (rings + 1) * verts_per_ring;
    for ring in 0..cylinder_rings {
        for seg in 0..segments {
            let current = cylinder_start + ring * verts_per_ring + seg;
            let next = current + verts_per_ring;

            indices.push(current as u16);
            indices.push(next as u16);
            indices.push((current + 1) as u16);

            indices.push((current + 1) as u16);
            indices.push(next as u16);
            indices.push((next + 1) as u16);
        }
    }

    // Bottom hemisphere
    let bottom_start = cylinder_start + (cylinder_rings + 1) * verts_per_ring;
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
    /// Color (RGB) + padding (W unused, set to 1.0)
    pub color: [f32; 4],
}

impl BoneInstance {
    pub fn instance_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BoneInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // model_matrix - 4 slots
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
                // color (vec4)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Mesh data для одного типу кістки
struct BoneMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

/// Renderer для скелета
pub struct SkeletonRenderer {
    /// Pre-generated meshes для кожного типу кістки
    bone_meshes: HashMap<BoneType, BoneMesh>,

    /// Instance buffers per bone type (для batching)
    instance_buffers: HashMap<BoneType, wgpu::Buffer>,
    instance_counts: HashMap<BoneType, u32>,

    render_pipeline: wgpu::RenderPipeline,
}

impl SkeletonRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        // === GENERATE MESHES FOR EACH BONE TYPE ===
        let mut bone_meshes = HashMap::new();
        let mut instance_buffers = HashMap::new();
        let instance_counts = HashMap::new();

        for bone_type in [
            BoneType::Pelvis,
            BoneType::Spine,
            BoneType::Head,
            BoneType::UpperArm,
            BoneType::LowerArm,
            BoneType::UpperLeg,
            BoneType::LowerLeg,
        ] {
            let (length, radius_top, radius_bottom) = bone_type.dimensions();
            let (vertices, indices) = generate_tapered_capsule_real(length, radius_top, radius_bottom, 12);

            log_debug(&format!(
                "Generated mesh for {:?}: len={:.3}, r_top={:.3}, r_bot={:.3}, verts={}, indices={}",
                bone_type, length, radius_top, radius_bottom, vertices.len(), indices.len()
            ));

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", bone_type)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", bone_type)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            bone_meshes.insert(bone_type, BoneMesh {
                vertex_buffer,
                index_buffer,
                index_count: indices.len() as u32,
            });

            // Instance buffer (max 4 instances per type - left/right pairs)
            let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{:?} Instance Buffer", bone_type)),
                size: (std::mem::size_of::<BoneInstance>() * 4) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            instance_buffers.insert(bone_type, instance_buffer);
        }

        // === SHADER ===
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
                entry_point: Some("vs_main"),
                buffers: &[
                    CapsuleVertex::vertex_buffer_layout(),
                    BoneInstance::instance_buffer_layout(),
                ],
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
            bone_meshes,
            instance_buffers,
            instance_counts,
            render_pipeline,
        }
    }

    /// Оновлює instances на основі позицій кісток
    pub fn update_bones(
        &mut self,
        queue: &wgpu::Queue,
        bone_transforms: &[(BoneId, Vec3, Quat)],
    ) {
        // Group bones by type
        let mut instances_by_type: HashMap<BoneType, Vec<BoneInstance>> = HashMap::new();

        // Debug logging
        static mut FRAME_COUNT: u32 = 0;
        let should_log = unsafe {
            FRAME_COUNT += 1;
            FRAME_COUNT % 60 == 1
        };

        if should_log {
            log_debug("=== SKELETON RENDERER UPDATE ===");
        }

        for (bone_id, position, rotation) in bone_transforms {
            let bone_type = BoneType::from_bone_id(*bone_id);
            let color = get_bone_color(*bone_id);

            // NO SCALING - mesh already has correct dimensions!
            // Just position and rotation
            let model_matrix = Mat4::from_rotation_translation(*rotation, *position);

            if should_log {
                log_debug(&format!(
                    "{:?} ({:?}): pos=({:.2}, {:.2}, {:.2})",
                    bone_id, bone_type, position.x, position.y, position.z
                ));
            }

            instances_by_type
                .entry(bone_type)
                .or_insert_with(Vec::new)
                .push(BoneInstance {
                    model_matrix: model_matrix.to_cols_array_2d(),
                    color: [color[0], color[1], color[2], 1.0],
                });
        }

        // Update instance buffers
        self.instance_counts.clear();
        for (bone_type, instances) in instances_by_type {
            if let Some(buffer) = self.instance_buffers.get(&bone_type) {
                self.instance_counts.insert(bone_type, instances.len() as u32);
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&instances));
            }
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, camera_bind_group: &'a wgpu::BindGroup) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);

        // Render each bone type
        for (bone_type, mesh) in &self.bone_meshes {
            let instance_count = self.instance_counts.get(bone_type).copied().unwrap_or(0);
            if instance_count == 0 {
                continue;
            }

            if let Some(instance_buffer) = self.instance_buffers.get(bone_type) {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..instance_count);
            }
        }
    }
}
