/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/rendering/renderer.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   WgpuRenderer - основний клас для рендерінгу через wgpu.

   На даному етапі (Phase 1, Week 1-2): Просто очищує екран кольором.
   Майбутнє: 3D рендерінг, камера, моделі, освітлення.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Ініціалізація wgpu (instance, adapter, device, queue, surface)
   - Налаштування surface configuration
   - Рендеринг кадру (зараз - clear color, потім - 3D сцена)
   - Обробка resize вікна

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - wgpu - graphics API
   - winit::window::Window - для створення surface

   Експортує для:
   - main.rs - створення і використання renderer

📦 ЗАЛЕЖНОСТІ:
   - wgpu = "22.1" - graphics API (Vulkan/DX12/Metal backend)
   - pollster = "0.4" - для async/await в sync контексті

⚠️  ВАЖЛИВІ ОБМЕЖЕННЯ:
   1. Renderer ПОВИНЕН бути створений ПІСЛЯ вікна (surface залежить від window)
   2. При resize вікна треба оновити surface configuration
   3. wgpu працює асинхронно - використовуємо pollster::block_on

🧪 ТЕСТУВАННЯ:
   Запуск:
   ```bash
   cargo run
   ```

   Очікуваний результат:
   - Вікно 800x600 з темно-синім кольором (RGB: 0.1, 0.2, 0.3)

📝 ПРИКЛАД ВИКОРИСТАННЯ:
   ```rust
   // В main.rs
   let renderer = pollster::block_on(WgpuRenderer::new(&window));

   // В event loop
   match event {
       WindowEvent::RedrawRequested => {
           renderer.render().unwrap();
       }
   }
   ```

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базова ініціалізація wgpu + clear color

═══════════════════════════════════════════════════════════════════════════════
*/

use std::sync::Arc;
use wgpu;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::camera::{Camera, CameraUniform};
use crate::transform::Transform;
use super::grid::Grid;
use super::mesh::{Mesh, generate_cube};
use glam::Vec3;

/// Основний renderer на базі wgpu
///
/// Структура містить всі необхідні wgpu об'єкти для рендерінгу.
pub struct WgpuRenderer {
    /// wgpu surface - зв'язок з вікном ОС
    surface: wgpu::Surface<'static>,

    /// Збережене вікно (Arc для 'static lifetime)
    #[allow(dead_code)]
    window: Arc<Window>,

    /// wgpu device - логічний GPU пристрій
    device: wgpu::Device,

    /// wgpu queue - черга команд для GPU
    queue: wgpu::Queue,

    /// Конфігурація surface (формат, розмір, режим презентації)
    config: wgpu::SurfaceConfiguration,

    /// Розмір вікна
    size: winit::dpi::PhysicalSize<u32>,

    /// 3D Camera
    pub camera: Camera,

    /// Camera uniform buffer
    camera_uniform: CameraUniform,

    /// Camera uniform buffer на GPU
    camera_buffer: wgpu::Buffer,

    /// Bind group для camera
    camera_bind_group: wgpu::BindGroup,

    /// Grid (координатна сітка)
    grid: Grid,

    /// Depth texture для правильного z-ordering
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    /// Cubes (тестові об'єкти)
    cubes: Vec<Mesh>,

    /// Camera bind group layout (зберігаємо для створення нових mesh)
    camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl WgpuRenderer {
    /// Створює новий WgpuRenderer
    ///
    /// # Аргументи
    /// * `window` - Winit window (Arc) для створення surface
    ///
    /// # Повертає
    /// Новий екземпляр WgpuRenderer, готовий до рендерінгу
    ///
    /// # Приклад
    /// ```
    /// let window = Arc::new(window);
    /// let renderer = pollster::block_on(WgpuRenderer::new(window));
    /// ```
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        log::info!("Ініціалізація wgpu renderer...");
        log::debug!("Розмір вікна: {}x{}", size.width, size.height);

        // 1. Створити wgpu Instance (точка входу в wgpu)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(), // Автовибір: Vulkan/DX12/Metal
            ..Default::default()
        });
        log::debug!("wgpu Instance створено");

        // 2. Створити Surface (зв'язок з вікном)
        let surface = instance.create_surface(window.clone()).unwrap();
        log::debug!("wgpu Surface створено");

        // 3. Запитати Adapter (фізичний GPU)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let adapter_info = adapter.get_info();
        log::info!(
            "Використовується GPU: {} ({:?})",
            adapter_info.name,
            adapter_info.backend
        );

        // 4. Запитати Device і Queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();
        log::debug!("wgpu Device і Queue створені");

        // 5. Налаштувати Surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        log::debug!("Surface format: {:?}", surface_format);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // VSync
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // 6. Створити Camera
        use glam::Vec3;
        let camera = Camera::new(
            Vec3::new(0.0, 3.0, 8.0),  // Позиція: трохи вище та назад
            Vec3::new(0.0, 0.0, 0.0),  // Дивимось на центр
            size.width as f32 / size.height as f32, // Aspect ratio
        );

        // 7. Створити Camera Uniform Buffer
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 8. Створити Bind Group Layout для Camera
        let camera_bind_group_layout =
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
                label: Some("camera_bind_group_layout"),
            });

        // 9. Створити Bind Group для Camera
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // 10. Створити Grid
        let grid = Grid::new(&device, &config, &camera_bind_group_layout, 20);

        // 11. Створити Depth Texture
        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &config);

        // 12. Створити кілька Cube meshes з різними позиціями
        let mut cubes = Vec::new();

        // Центральний червоний куб (трохи підняти над grid)
        let (cube_vertices, cube_indices) = generate_cube(1.0, [0.8, 0.3, 0.3]);
        let cube1 = Mesh::new(
            &device,
            &config,
            &cube_vertices,
            &cube_indices,
            &camera_bind_group_layout,
            Transform::new(Vec3::new(0.0, 0.5, 0.0)), // Center, lifted by 0.5 (half of cube height)
        );
        cubes.push(cube1);

        // Зелений куб зліва
        let (cube_vertices, cube_indices) = generate_cube(1.0, [0.3, 0.8, 0.3]);
        let cube2 = Mesh::new(
            &device,
            &config,
            &cube_vertices,
            &cube_indices,
            &camera_bind_group_layout,
            Transform::new(Vec3::new(-3.0, 0.5, 0.0)),
        );
        cubes.push(cube2);

        // Синій куб справа
        let (cube_vertices, cube_indices) = generate_cube(1.0, [0.3, 0.3, 0.8]);
        let cube3 = Mesh::new(
            &device,
            &config,
            &cube_vertices,
            &cube_indices,
            &camera_bind_group_layout,
            Transform::new(Vec3::new(3.0, 0.5, 0.0)),
        );
        cubes.push(cube3);

        // Жовтий куб позаду
        let (cube_vertices, cube_indices) = generate_cube(1.5, [0.9, 0.8, 0.2]); // Bigger cube
        let cube4 = Mesh::new(
            &device,
            &config,
            &cube_vertices,
            &cube_indices,
            &camera_bind_group_layout,
            Transform::new(Vec3::new(0.0, 0.75, -4.0)),
        );
        cubes.push(cube4);

        log::info!("wgpu renderer готовий до роботи!");
        log::info!("Camera: position={:?}, target={:?}", camera.position, camera.target);
        log::info!("Створено {} кубів з різними позиціями", cubes.len());

        Self {
            surface,
            window,
            device,
            queue,
            config,
            size,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            grid,
            depth_texture,
            depth_view,
            cubes,
            camera_bind_group_layout,
        }
    }

    /// Створює depth texture для z-ordering
    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        (texture, view)
    }

    /// Оновлює розмір вікна
    ///
    /// Викликається при WindowEvent::Resized
    ///
    /// # Аргументи
    /// * `new_size` - Новий розмір вікна
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            log::debug!("Resize: {}x{}", new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // Оновлюємо aspect ratio камери
            self.camera.update_aspect(new_size.width, new_size.height);

            // Пересоздаємо depth texture з новим розміром
            let (depth_texture, depth_view) = Self::create_depth_texture(&self.device, &self.config);
            self.depth_texture = depth_texture;
            self.depth_view = depth_view;
        }
    }

    /// Рендерить один кадр
    ///
    /// На даному етапі: просто очищує екран кольором.
    /// Майбутнє: рендерінг 3D сцени.
    ///
    /// # Повертає
    /// `Ok(())` якщо рендерінг успішний
    /// `Err(wgpu::SurfaceError)` при помилці
    ///
    /// # Помилки
    /// - `SurfaceError::Lost` - surface втрачено, треба пересоздать
    /// - `SurfaceError::OutOfMemory` - не вистачає пам'яті
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Оновити camera uniform buffer
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // 2. Отримати поточний frame з surface
        let output = self.surface.get_current_texture()?;

        // 3. Створити view для текстури
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 4. Створити command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // 5. Створити render pass з depth buffer
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, // Темно-синій колір для арени
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), // Clear depth to 1.0 (far)
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Малюємо 3D об'єкти (cubes)
            for cube in &self.cubes {
                cube.render(&mut render_pass, &self.camera_bind_group);
            }

            // Малюємо grid (після mesh щоб правильно відображався поверх через alpha)
            self.grid.render(&mut render_pass, &self.camera_bind_group);
            // render_pass автоматично завершується при drop
        }

        // 5. Відправити команди в queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // 6. Презентувати frame
        output.present();

        Ok(())
    }

    /// Повертає поточний розмір вікна
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
}
