/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/rendering/grid.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Grid - генерація та рендеринг координатної сітки на підлозі.

   Сітка допомагає орієнтуватися в 3D просторі та бачити масштаб.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Генерація вершин для grid (лінії на XZ plane)
   - Створення vertex/index buffers
   - Налаштування render pipeline для grid shader
   - Рендеринг сітки кожен кадр

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - wgpu - для buffers та pipeline
   - bytemuck - для конвертації даних у байти

   Експортує для:
   - rendering/renderer.rs - рендеринг grid

📦 ЗАЛЕЖНОСТІ:
   - wgpu = "22.0"
   - bytemuck = "1.14"

⚠️  ВАЖЛИВІ ОБМЕЖЕННЯ:
   1. Grid завжди на Y=0 (XZ plane)
   2. Розмір grid: -size..+size по X та Z
   3. Інтервал між лініями: 1.0 unit

🧪 ТЕСТУВАННЯ:
   Grid має бути видимий при camera.position = Vec3::new(0.0, 2.0, 5.0)
   та camera.target = Vec3::ZERO

🕐 ІСТОРІЯ:
   2025-12-14: Створено - генерація grid mesh та render pipeline

═══════════════════════════════════════════════════════════════════════════════
*/

use bytemuck::{Pod, Zeroable};
use wgpu;
use wgpu::util::DeviceExt;

/// Вершина для grid (позиція + колір)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GridVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl GridVertex {
    /// Descriptor для vertex buffer layout
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GridVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Grid - координатна сітка на підлозі
pub struct Grid {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Grid {
    /// Створює новий Grid
    ///
    /// # Аргументи
    /// * `device` - wgpu device
    /// * `config` - surface configuration (для format)
    /// * `camera_bind_group_layout` - layout для camera uniform buffer
    /// * `size` - розмір grid (від -size до +size по X та Z)
    ///
    /// # Повертає
    /// Новий Grid готовий до рендерінгу
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        size: i32,
    ) -> Self {
        // Генеруємо вершини та індекси
        let (vertices, indices) = Self::generate_grid_mesh(size);

        // Створюємо vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Створюємо index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;

        // Завантажуємо shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Grid Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shaders/grid.wgsl").into()),
        });

        // Створюємо render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Grid Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Grid Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[GridVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Для прозорості
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList, // Малюємо лінії
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Без culling для ліній
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
            }), // Depth buffer для правильного z-ordering
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
            num_indices,
            pipeline,
        }
    }

    /// Генерує вершини та індекси для grid mesh
    ///
    /// Створює лінії паралельні до X та Z осей на площині Y=0
    ///
    /// # Аргументи
    /// * `size` - розмір grid (від -size до +size)
    ///
    /// # Повертає
    /// (vertices, indices) для grid
    fn generate_grid_mesh(size: i32) -> (Vec<GridVertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Колір для звичайних ліній (світло-сірий)
        let grid_color = [0.5, 0.5, 0.5];

        // Лінії паралельні до X осі (вздовж Z)
        for z in -size..=size {
            let z_pos = z as f32;

            // Початок лінії
            vertices.push(GridVertex {
                position: [-size as f32, 0.0, z_pos],
                color: grid_color,
            });

            // Кінець лінії
            vertices.push(GridVertex {
                position: [size as f32, 0.0, z_pos],
                color: grid_color,
            });
        }

        // Лінії паралельні до Z осі (вздовж X)
        for x in -size..=size {
            let x_pos = x as f32;

            // Початок лінії
            vertices.push(GridVertex {
                position: [x_pos, 0.0, -size as f32],
                color: grid_color,
            });

            // Кінець лінії
            vertices.push(GridVertex {
                position: [x_pos, 0.0, size as f32],
                color: grid_color,
            });
        }

        // Генеруємо індекси (кожна пара вершин = одна лінія)
        for i in 0..vertices.len() as u16 {
            indices.push(i);
        }

        (vertices, indices)
    }

    /// Рендерить grid
    ///
    /// # Аргументи
    /// * `render_pass` - активний render pass
    /// * `camera_bind_group` - bind group з camera uniform
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
