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
use crate::player::Player;
use crate::combat::Combat;
use crate::enemy::Enemy;
use crate::debug_log::log_debug;
use crate::physics::BoneId;
use super::grid::Grid;
use super::mesh::{Mesh, generate_player_mannequin, generate_player_body, generate_weapon_arm};
use super::skeleton_renderer::SkeletonRenderer;
use super::screenshot::FirstFrameCapture;
use glam::{Vec3, Quat};

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

    /// Player mesh (тіло без руки)
    player_mesh: Mesh,

    /// Player weapon mesh (рука + меч) - окремий для анімації
    weapon_mesh: Mesh,

    /// Enemy meshes (вороги)
    enemy_meshes: Vec<Mesh>,

    /// Camera bind group layout (зберігаємо для створення нових mesh)
    camera_bind_group_layout: wgpu::BindGroupLayout,

    /// Skeleton renderer для фізичного ragdoll
    skeleton_renderer: SkeletonRenderer,

    /// Чи показувати скелет (для debug)
    pub show_skeleton: bool,

    /// Offscreen render texture (for screenshot support)
    render_texture: wgpu::Texture,
    render_texture_view: wgpu::TextureView,

    /// Screenshot capture for first frame (for AI analysis)
    first_frame_capture: FirstFrameCapture,
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
        // Використовуємо PRIMARY backends (D3D12 на Windows, Metal на macOS, Vulkan на Linux)
        // wgpu 24.0+ має виправлені D3D12 resource state transitions
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
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

        // Налаштовуємо обробник некритичних помилок wgpu
        device.on_uncaptured_error(Box::new(|error| {
            let msg = format!("WGPU UNCAPTURED ERROR: {:?}", error);
            log::error!("{}", msg);
            crate::debug_log::log_console(&msg);
        }));

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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Тільки RENDER_ATTACHMENT для swapchain
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

        // 12. Cubes (вимкнено для тестування ragdoll)
        let cubes = Vec::new();

        // 13. Створити Player body mesh (без руки)
        let (body_vertices, body_indices) = generate_player_body(
            [0.2, 0.6, 0.9],          // body_color (синій)
            [0.9, 0.8, 0.7],          // head_color (тілесний)
        );
        let player_mesh = Mesh::new(
            &device,
            &config,
            &body_vertices,
            &body_indices,
            &camera_bind_group_layout,
            Transform::new(Vec3::new(0.0, 0.75, 0.0)),
        );

        // 14. Створити Weapon/Arm mesh (окремо для анімації)
        let (weapon_vertices, weapon_indices) = generate_weapon_arm(
            [0.2, 0.5, 0.8],          // arm_color
            [0.7, 0.7, 0.75],         // weapon_color (світлий метал)
        );
        // Початкова позиція - на плечі (body_radius + offset, shoulder_height, 0)
        let shoulder_offset = Vec3::new(0.3, 0.75 + 0.45, 0.0);  // body_radius=0.3, shoulder at 0.45 above center
        let weapon_mesh = Mesh::new(
            &device,
            &config,
            &weapon_vertices,
            &weapon_indices,
            &camera_bind_group_layout,
            Transform::new(shoulder_offset),
        );

        // Enemy meshes (порожній вектор, заповниться через spawn_enemies)
        let enemy_meshes = Vec::new();

        // 15. Створити Skeleton Renderer для фізичного ragdoll
        let skeleton_renderer = SkeletonRenderer::new(&device, &config, &camera_bind_group_layout);

        // 16. Створити render texture для screenshot support
        let (render_texture, render_texture_view) = Self::create_render_texture(&device, &config);

        log::info!("wgpu renderer готовий до роботи!");
        log::info!("Camera: position={:?}, target={:?}", camera.position, camera.target);

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
            player_mesh,
            weapon_mesh,
            enemy_meshes,
            camera_bind_group_layout,
            skeleton_renderer,
            show_skeleton: false,
            render_texture,
            render_texture_view,
            first_frame_capture: FirstFrameCapture::new(),
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

    /// Створює offscreen render texture для screenshot support
    fn create_render_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
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

            // Пересоздаємо render texture з новим розміром
            let (render_texture, render_texture_view) = Self::create_render_texture(&self.device, &self.config);
            self.render_texture = render_texture;
            self.render_texture_view = render_texture_view;

            // Пересоздаємо depth texture з новим розміром
            let (depth_texture, depth_view) = Self::create_depth_texture(&self.device, &self.config);
            self.depth_texture = depth_texture;
            self.depth_view = depth_view;
        }
    }

    /// Рендерить один кадр
    ///
    /// Рендеринг відбувається напряму на swapchain texture.
    /// Screenshot (якщо потрібен) рендериться окремо в offscreen texture.
    ///
    /// # Повертає
    /// `Ok(())` якщо рендерінг успішний
    /// `Err(wgpu::SurfaceError)` при помилці
    ///
    /// # Помилки
    /// - `SurfaceError::Lost` - surface втрачено, треба пересоздать
    /// - `SurfaceError::OutOfMemory` - не вистачає пам'яті
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Check if we need screenshot this frame
        let need_screenshot = self.first_frame_capture.should_capture();
        if need_screenshot {
            self.first_frame_capture.init(&self.device, self.config.width, self.config.height);
        }

        // 1. Оновити camera uniform buffer
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // 2. Отримати поточний frame з surface
        let output = self.surface.get_current_texture()?;
        let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 3. Створити command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // 4. Рендеримо напряму на swapchain
        self.render_scene(&mut encoder, &output_view);

        // 5. Якщо потрібен screenshot - рендеримо ще раз в offscreen texture
        if need_screenshot {
            self.render_scene(&mut encoder, &self.render_texture_view);
            self.first_frame_capture.copy_if_needed(&mut encoder, &self.render_texture);
        }

        // 6. Відправити команди в queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // 7. Save screenshot after submit
        if need_screenshot {
            self.first_frame_capture.save_if_needed(&self.device);
        }

        // 8. Презентувати frame
        output.present();

        Ok(())
    }

    /// Внутрішній метод для рендерингу сцени в конкретний view
    fn render_scene(&self, encoder: &mut wgpu::CommandEncoder, target_view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target_view,
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

        // Малюємо старий player mesh ТІЛЬКИ якщо скелет вимкнено
        if !self.show_skeleton {
            // Малюємо player body
            self.player_mesh.render(&mut render_pass, &self.camera_bind_group);

            // Малюємо player weapon/arm
            self.weapon_mesh.render(&mut render_pass, &self.camera_bind_group);
        }

        // Малюємо enemies
        for enemy_mesh in &self.enemy_meshes {
            enemy_mesh.render(&mut render_pass, &self.camera_bind_group);
        }

        // Малюємо skeleton (якщо увімкнено)
        if self.show_skeleton {
            self.skeleton_renderer.render(&mut render_pass, &self.camera_bind_group);
        }

        // Малюємо grid (після mesh щоб правильно відображався поверх через alpha)
        self.grid.render(&mut render_pass, &self.camera_bind_group);
        // render_pass автоматично завершується при drop
    }

    /// Повертає поточний розмір вікна
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Оновлює позицію player mesh на основі Player та Combat state
    ///
    /// # Аргументи
    /// * `player` - Player struct з поточною позицією та yaw
    /// * `combat` - Combat struct зі станом атаки
    pub fn update_player(&mut self, player: &Player, combat: &Combat) {
        let yaw = player.yaw;

        // === PLAYER BODY ===
        self.player_mesh.transform.position = player.position + Vec3::new(0.0, 0.75, 0.0);
        let new_rotation = Quat::from_rotation_y(yaw);

        // DEBUG: порівняємо стару і нову ротацію
        static mut LAST_YAW: f32 = 999.0;
        unsafe {
            if (LAST_YAW - yaw).abs() > 0.01 {
                log_debug(&format!("UPDATE_PLAYER: yaw={:.1}° rotation=({:.3}, {:.3}, {:.3}, {:.3})",
                    yaw.to_degrees(),
                    new_rotation.x, new_rotation.y, new_rotation.z, new_rotation.w));
                LAST_YAW = yaw;
            }
        }

        self.player_mesh.transform.rotation = new_rotation;
        self.player_mesh.transform.scale = Vec3::ONE;

        // Debug: логуємо напрямок тіла (forward = -Z rotated by yaw)
        let forward_x = -yaw.sin();
        let forward_z = -yaw.cos();
        static mut LAST_FWD: (f32, f32) = (0.0, 0.0);
        unsafe {
            if (LAST_FWD.0 - forward_x).abs() > 0.05 || (LAST_FWD.1 - forward_z).abs() > 0.05 {
                log_debug(&format!("BODY FORWARD: ({:.2}, {:.2}) | yaw: {:.1}°",
                    forward_x, forward_z, yaw.to_degrees()));
                LAST_FWD = (forward_x, forward_z);
            }
        }

        self.player_mesh.update_transform(&self.queue);

        // === WEAPON/ARM ===
        // Позиція плеча в world space (праворуч від гравця)
        let body_radius = 0.3_f32;
        let shoulder_height = 0.45_f32;

        let right_dir = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
        let shoulder_world = player.position
            + Vec3::new(0.0, 0.75 + shoulder_height, 0.0)
            + right_dir * body_radius;

        self.weapon_mesh.transform.position = shoulder_world;

        // Rotation: base yaw + swing angle
        let base_rotation = Quat::from_rotation_y(yaw);
        let swing_rotation = Quat::from_rotation_y(combat.weapon_swing_angle);
        self.weapon_mesh.transform.rotation = base_rotation * swing_rotation;
        self.weapon_mesh.update_transform(&self.queue);
    }

    /// Оновлює анімації об'єктів
    ///
    /// # Аргументи
    /// * `delta` - Delta time в секундах
    pub fn update_animations(&mut self, delta: f32) {
        // Швидкості обертання для кожного куба (радіани/секунда)
        let rotation_speeds = [
            1.0_f32,   // Червоний куб - 1 рад/с (~57°/с)
            -0.7,      // Зелений куб - -0.7 рад/с (протилежний напрямок)
            1.5,       // Синій куб - 1.5 рад/с (швидше)
            0.3,       // Жовтий куб - 0.3 рад/с (повільно)
        ];

        // Обертаємо кожен куб
        for (i, cube) in self.cubes.iter_mut().enumerate() {
            if i < rotation_speeds.len() {
                let rotation_speed = rotation_speeds[i];
                let rotation_delta = rotation_speed * delta;

                // Обертаємо навколо осі Y
                cube.transform.rotate(0.0, rotation_delta.to_degrees(), 0.0);

                // Оновлюємо GPU buffer
                cube.update_transform(&self.queue);
            }
        }
    }

    /// Створює meshes для ворогів
    ///
    /// # Аргументи
    /// * `enemies` - Список ворогів для spawning
    pub fn spawn_enemies(&mut self, enemies: &[Enemy]) {
        self.enemy_meshes.clear();

        // Enemy колір - червоний (тіло) з темно-червоною головою
        let enemy_body_color = [0.8, 0.2, 0.2];  // Червоний
        let enemy_head_color = [0.6, 0.1, 0.1];  // Темно-червоний

        let (enemy_vertices, enemy_indices) = generate_player_mannequin(
            0.3,              // body_radius
            1.2,              // body_height
            0.25,             // head_radius
            enemy_body_color,
            enemy_head_color,
        );

        for enemy in enemies {
            let mut transform = Transform::new(enemy.position + Vec3::new(0.0, 0.75, 0.0));
            transform.rotation = Quat::from_rotation_y(enemy.yaw);

            let mesh = Mesh::new(
                &self.device,
                &self.config,
                &enemy_vertices,
                &enemy_indices,
                &self.camera_bind_group_layout,
                transform,
            );

            self.enemy_meshes.push(mesh);
        }

        log::info!("Spawned {} enemy meshes", self.enemy_meshes.len());
    }

    /// Оновлює bone transforms для skeleton renderer
    ///
    /// # Аргументи
    /// * `bone_transforms` - Список кісток з позиціями та ротаціями
    pub fn update_skeleton(&mut self, bone_transforms: &[(BoneId, Vec3, Quat)]) {
        self.skeleton_renderer.update_bones(&self.queue, bone_transforms);
    }

    /// Оновлює позиції enemy meshes
    ///
    /// # Аргументи
    /// * `enemies` - Список ворогів з оновленими позиціями
    pub fn update_enemies(&mut self, enemies: &[Enemy]) {
        for (i, enemy) in enemies.iter().enumerate() {
            if i < self.enemy_meshes.len() {
                // Оновлюємо позицію
                self.enemy_meshes[i].transform.position = enemy.position + Vec3::new(0.0, 0.75, 0.0);

                // Оновлюємо rotation
                self.enemy_meshes[i].transform.rotation = Quat::from_rotation_y(enemy.yaw);

                // Якщо ворог мертвий - зменшуємо scale (або можна приховати)
                if !enemy.is_alive() {
                    self.enemy_meshes[i].transform.scale = Vec3::new(1.0, 0.1, 1.0); // Сплющений
                } else {
                    self.enemy_meshes[i].transform.scale = Vec3::ONE;
                }

                // Оновлюємо GPU buffer
                self.enemy_meshes[i].update_transform(&self.queue);
            }
        }
    }
}
