/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/main.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Entry point програми. Ініціалізує вікно через winit та запускає game loop.

   На даному етапі (Phase 1, Week 1): Просто створює вікно та обробляє події.
   Майбутнє: Додасться wgpu renderer, game state, input handling.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Створити вікно (через winit)
   - Запустити event loop
   - Обробляти події (закриття вікна, resize, input)
   - (Майбутнє) Ініціалізувати wgpu renderer
   - (Майбутнє) Викликати game loop update

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - winit - для створення вікна та event loop
   - (Майбутнє) src/rendering/renderer.rs - wgpu renderer
   - (Майбутнє) src/core/state.rs - game state

   Експортує для:
   - Немає (це entry point)

📦 ЗАЛЕЖНОСТІ:
   - winit = "0.30" - window і event loop
   - env_logger = "0.11" - логування
   - log = "0.4" - logging macros

⚠️  ВАЖЛИВІ ОБМЕЖЕННЯ:
   1. НЕ блокувати event loop - всі операції мають бути швидкими
   2. НЕ використовувати sleep() в main loop
   3. Event loop МАЄ контролювати frame rate (наступний крок)

🧪 ТЕСТУВАННЯ:
   Запуск:
   ```bash
   cargo run
   ```

   Очікуваний результат:
   - Відкривається вікно 800x600
   - Заголовок "Arena Combat Prototype"
   - Вікно можна закрити через ESC або [X]

📝 ПРИКЛАД ВИКОРИСТАННЯ:
   ```bash
   # Запустити гру
   cargo run

   # З логуванням
   RUST_LOG=info cargo run
   ```

🕐 ІСТОРІЯ:
   2025-12-11: Створено базову структуру - вікно + event loop
   2025-12-14: Додано wgpu renderer з очищенням екрану темно-синім кольором

═══════════════════════════════════════════════════════════════════════════════
*/

mod rendering;
mod fps_counter;
mod camera;
mod input;
mod transform;
mod time;
mod player;
mod combat;
mod enemy;
mod physics;
pub mod debug_log;

use rendering::WgpuRenderer;
use fps_counter::FpsCounter;
use input::InputState;
use time::GameTime;
use player::Player;
use combat::{Combat, HitboxManager};
use enemy::Enemy;
use physics::{PhysicsWorld, ActiveRagdoll};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{PhysicalKey, KeyCode},
    window::{Window, WindowId, CursorGrabMode},
};

// ============================================================================
// APPLICATION STATE
// ============================================================================

/// Головна структура додатку
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<WgpuRenderer>,
    fps_counter: FpsCounter,
    input_state: InputState,
    game_time: GameTime,
    player: Player,
    combat: Combat,
    hitbox_manager: HitboxManager,
    enemies: Vec<Enemy>,
    enemies_spawned: bool,

    // Physics-based ragdoll
    physics_world: Option<PhysicsWorld>,
    ragdoll: Option<ActiveRagdoll>,
    use_physics_player: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Створюємо вікно при старті
        let window_attributes = Window::default_attributes()
            .with_title("Arena Combat Prototype")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        log::info!("Вікно створено: 800x600");

        // Ініціалізуємо wgpu renderer
        log::info!("Ініціалізація renderer...");
        let mut renderer = pollster::block_on(WgpuRenderer::new(window.clone()));
        renderer.show_skeleton = true;  // Увімкнути візуалізацію скелета

        // Захоплюємо та ховаємо курсор для FPS-style керування камерою
        // Курсор буде прихований і миша завжди обертатиме камеру
        if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined) {
            log::warn!("Не вдалося захопити курсор (Confined): {:?}", e);
            // Спробуємо Locked як fallback
            if let Err(e2) = window.set_cursor_grab(CursorGrabMode::Locked) {
                log::warn!("Не вдалося захопити курсор (Locked): {:?}", e2);
            }
        }
        window.set_cursor_visible(false);
        log::info!("Курсор захоплено та приховано");

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            // Mouse position (для camera rotation)
            WindowEvent::CursorMoved { position, .. } => {
                self.input_state.update_mouse_position(position.x, position.y);
            }

            // Mouse buttons (для drag rotation та атаки)
            WindowEvent::MouseInput { button, state, .. } => {
                self.input_state.update_mouse_button(button, state);

                // Ліва кнопка миші = атака
                if button == MouseButton::Left && state == ElementState::Pressed {
                    // Напрямок атаки = куди дивиться гравець
                    let attack_dir = self.player.forward();
                    if self.combat.start_attack(attack_dir) {
                        // Spawn hitbox на кінці зброї
                        self.hitbox_manager.spawn_attack_hitbox(
                            self.player.position,
                            self.player.yaw,
                            50.0, // damage
                        );
                        log::info!("Attack! Hitbox spawned");
                    }
                }
            }

            // Mouse wheel (для zoom)
            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_amount = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => y * 0.5,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.y / 50.0) as f32,
                };

                if let Some(renderer) = &mut self.renderer {
                    renderer.camera.zoom_third_person(zoom_amount);
                }
            }

            // Keyboard input
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if let PhysicalKey::Code(key_code) = key_event.physical_key {
                    self.input_state.update_key(key_code, key_event.state);

                    // ESC - закриття
                    if key_code == KeyCode::Escape && key_event.state == ElementState::Pressed {
                        log::info!("ESC натиснуто - закриття...");
                        event_loop.exit();
                    }
                }
            }

            // Закрити вікно
            WindowEvent::CloseRequested => {
                log::info!("Закриття вікна...");
                event_loop.exit();
            }

            // Redraw request
            WindowEvent::RedrawRequested => {
                // Оновити час
                self.game_time.update();

                // Оновити FPS counter
                self.fps_counter.tick();

                // Оновити заголовок вікна з FPS (кожні 30 кадрів для зменшення overhead)
                if self.game_time.frame_count() % 30 == 0 {
                    if let Some(window) = &self.window {
                        let fps = self.fps_counter.fps();
                        let title = format!(
                            "Arena Combat Prototype - {:.1} FPS ({:.2}ms)",
                            fps,
                            self.fps_counter.frame_time_ms()
                        );
                        window.set_title(&title);
                    }
                }

                // === ENEMY SPAWNING (one-time) ===
                if !self.enemies_spawned {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.spawn_enemies(&self.enemies);
                        self.enemies_spawned = true;
                    }
                }

                // === COMBAT UPDATE ===
                self.combat.update(self.game_time.delta());

                // === HITBOX UPDATE & COLLISION ===
                {
                    let delta = self.game_time.delta();
                    self.hitbox_manager.update(delta);

                    // Перевіряємо колізії hitbox ↔ enemies
                    let enemy_radius = 0.5; // Приблизний радіус ворога
                    for hitbox in &mut self.hitbox_manager.hitboxes {
                        for (i, enemy) in self.enemies.iter_mut().enumerate() {
                            // Пропускаємо мертвих та вже вражених
                            if !enemy.is_alive() || hitbox.has_hit(i) {
                                continue;
                            }

                            // Collision check (enemy position + height offset для центру)
                            let enemy_center = enemy.position + glam::Vec3::new(0.0, 1.0, 0.0);
                            if hitbox.collides_with_sphere(enemy_center, enemy_radius) {
                                // HIT!
                                enemy.take_damage(hitbox.damage);
                                hitbox.mark_hit(i);
                                log::info!("Enemy {} hit! Health: {}", i, enemy.health);

                                if !enemy.is_alive() {
                                    log::info!("Enemy {} killed!", i);
                                }
                            }
                        }
                    }
                }

                // === PHYSICS UPDATE ===
                if let (Some(physics), Some(ragdoll)) = (&mut self.physics_world, &mut self.ragdoll) {
                    let delta = self.game_time.delta();

                    // Оновлюємо ragdoll (м'язи + цільова поза)
                    ragdoll.update(physics, delta);

                    // Крок фізики
                    physics.step(delta);

                    // Оновлюємо skeleton renderer з bone transforms
                    if let Some(renderer) = &mut self.renderer {
                        let bone_transforms = ragdoll.get_bone_transforms(physics);
                        renderer.update_skeleton(&bone_transforms);
                    }
                }

                // === ANIMATION UPDATE ===
                if let Some(renderer) = &mut self.renderer {
                    // Обертаємо куби з використанням delta time
                    renderer.update_animations(self.game_time.delta());
                }

                // === ENEMY UPDATE ===
                if let Some(renderer) = &mut self.renderer {
                    renderer.update_enemies(&self.enemies);
                }

                // === CAMERA + PLAYER UPDATE (в одному блоці!) ===
                if let Some(renderer) = &mut self.renderer {
                    let delta = self.game_time.delta();

                    // Mouse look - миша ЗАВЖДИ обертає камеру (як в екшн іграх)
                    // Курсор захоплений та прихований, тому немає потреби тримати кнопку
                    {
                        let (delta_x, delta_y) = self.input_state.mouse_delta();

                        // Базова чутливість для звичайної миші
                        // Тачпад зазвичай дає менші дельти, тому автоматично підвищуємо
                        let base_sensitivity = 0.003;

                        // Якщо delta дуже мала (тачпад) - збільшуємо чутливість
                        let magnitude = (delta_x * delta_x + delta_y * delta_y).sqrt();
                        let sensitivity = if magnitude > 0.0 && magnitude < 5.0 {
                            // Тачпад дає малі delta - підвищуємо чутливість
                            base_sensitivity * 3.0
                        } else {
                            base_sensitivity
                        };

                        let delta_yaw = (delta_x as f32) * sensitivity;
                        let delta_pitch = (delta_y as f32) * sensitivity;

                        // Знижений поріг для тачпада
                        if delta_x.abs() > 0.01 || delta_y.abs() > 0.01 {
                            renderer.camera.rotate_third_person(delta_yaw, delta_pitch);
                        }
                    }
                    self.input_state.reset_mouse_delta();

                    // Q/E - обертає камеру
                    let turn_speed = 2.0_f32; // радіан/секунда
                    if self.input_state.is_q_pressed() {
                        renderer.camera.rotate_third_person(-turn_speed * delta, 0.0);
                    }
                    if self.input_state.is_e_pressed() {
                        renderer.camera.rotate_third_person(turn_speed * delta, 0.0);
                    }

                    // Отримуємо camera directions для camera-relative руху
                    let cam_forward = renderer.camera.forward_xz();
                    let cam_right = renderer.camera.right_xz();

                    // Обчислюємо input direction
                    let mut move_dir = glam::Vec3::ZERO;

                    // W/S - рух вперед/назад (відносно камери)
                    if self.input_state.is_w_pressed() {
                        move_dir += cam_forward;
                    }
                    if self.input_state.is_s_pressed() {
                        move_dir -= cam_forward;
                    }

                    // A/D - strafe вліво/вправо (відносно камери)
                    if self.input_state.is_a_pressed() {
                        move_dir -= cam_right;
                    }
                    if self.input_state.is_d_pressed() {
                        move_dir += cam_right;
                    }

                    // === ТРЕТЯ ОСОБА: ПЕРСОНАЖ ДИВИТЬСЯ В НАПРЯМКУ РУХУ ===
                    if self.use_physics_player {
                        // Фізичний ragdoll - передаємо напрямок руху
                        if let Some(ragdoll) = &mut self.ragdoll {
                            ragdoll.set_move_direction(move_dir);
                        }
                    } else {
                        // Старий кінематичний гравець
                        if move_dir.length_squared() > 0.01 {
                            move_dir = move_dir.normalize();

                            // Встановлюємо цільовий напрямок для плавного обертання
                            self.player.set_target_direction(move_dir);

                            // Рухаємо гравця
                            self.player.position += move_dir * self.player.move_speed * delta;
                        } else {
                            // Коли не рухаємось - персонаж зберігає поточний напрямок
                            self.player.is_moving = false;
                        }

                        // Плавне обертання персонажа до target_yaw
                        self.player.smooth_rotate(delta);
                    }
                }

                // === PLAYER MESH UPDATE ===
                if !self.use_physics_player {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.update_player(&self.player, &self.combat);
                    }
                }

                // === CAMERA POSITION UPDATE (слідує за гравцем) ===
                if let Some(renderer) = &mut self.renderer {
                    let player_pos = if self.use_physics_player {
                        // Позиція з фізичного ragdoll
                        if let (Some(physics), Some(ragdoll)) = (&self.physics_world, &self.ragdoll) {
                            ragdoll.get_position(physics)
                        } else {
                            self.player.position
                        }
                    } else {
                        self.player.position
                    };
                    renderer.camera.update_third_person(player_pos, 1.2);
                }

                // Рендеринг
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            // Surface втрачено - треба пересоздать
                            log::warn!("Surface lost, recreating...");
                            if let Some(window) = &self.window {
                                let size = window.inner_size();
                                renderer.resize(size);
                            }
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("Out of memory!");
                            event_loop.exit();
                        }
                        Err(e) => {
                            log::error!("Render error: {:?}", e);
                        }
                    }
                }
            }

            // Resize вікна
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Запит на перемальовування
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        // Raw mouse motion - краще працює коли курсор захоплений
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            // Debug: раскоментуй для діагностики тачпада
            // log::debug!("RAW_DELTA: x={:.3}, y={:.3}", delta.0, delta.1);
            self.input_state.accumulate_raw_mouse_delta(delta.0, delta.1);
        }
    }
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() {
    // Налаштовуємо panic hook для логування паніки у файл
    debug_log::setup_panic_hook();

    // Ініціалізація логування з перенаправленням у файл
    // Встановлюємо RUST_LOG якщо не встановлено (для wgpu validation)
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn,wgpu_core=warn,wgpu_hal=warn");
    }

    // Створюємо кастомний logger що пише і в консоль і в файл
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            let msg = format!("[{}] {}: {}", record.level(), record.target(), record.args());

            // Логуємо у файл для wgpu помилок та попереджень
            if record.target().starts_with("wgpu") || record.level() <= log::Level::Warn {
                debug_log::log_console(&msg);
            }

            writeln!(buf, "{}", msg)
        })
        .init();

    debug_log::log_console("=== Application Started ===");
    log::info!("=== Arena Combat Prototype ===");
    log::info!("Версія: 0.1.0");
    log::info!("Phase 1: Week 1-2 - Basic Rendering");

    // Створити event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Enemies вимкнені для тестування ragdoll
    let enemies = Vec::new();

    // Створюємо фізичний світ та ragdoll
    let mut physics_world = PhysicsWorld::new();
    physics_world.create_ground(0.0);  // Земля на Y=0

    // Створюємо ragdoll на висоті 2м
    let ragdoll = ActiveRagdoll::new(&mut physics_world, glam::Vec3::new(0.0, 2.0, 0.0));
    log::info!("Physics ragdoll created");

    // Створити app
    let mut app = App {
        window: None,
        renderer: None,
        fps_counter: FpsCounter::new(),
        input_state: InputState::new(),
        game_time: GameTime::new(),
        player: Player::new(glam::Vec3::new(0.0, 0.0, 5.0)), // Старт трохи попереду
        combat: Combat::new(),
        hitbox_manager: HitboxManager::new(),
        enemies,
        enemies_spawned: false,
        physics_world: Some(physics_world),
        ragdoll: Some(ragdoll),
        use_physics_player: true,  // Увімкнено фізичного ragdoll гравця
    };

    // Запустити event loop
    log::info!("Запуск event loop...");
    event_loop.run_app(&mut app).unwrap();

    log::info!("Програма завершена");
}
