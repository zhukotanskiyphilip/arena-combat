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

use rendering::WgpuRenderer;
use fps_counter::FpsCounter;
use input::InputState;
use time::GameTime;
use player::Player;
use combat::{Combat, HitboxManager};
use enemy::{Enemy, spawn_enemies_circle};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{PhysicalKey, KeyCode},
    window::{Window, WindowId},
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
        let renderer = pollster::block_on(WgpuRenderer::new(window.clone()));

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

                // === ANIMATION UPDATE ===
                if let Some(renderer) = &mut self.renderer {
                    // Обертаємо куби з використанням delta time
                    renderer.update_animations(self.game_time.delta());
                }

                // === ENEMY UPDATE ===
                if let Some(renderer) = &mut self.renderer {
                    renderer.update_enemies(&self.enemies);
                }

                // === PLAYER UPDATE (Camera-relative movement) ===
                if let Some(renderer) = &self.renderer {
                    let delta = self.game_time.delta();

                    // Отримуємо camera directions для camera-relative руху
                    let cam_forward = renderer.camera.forward_xz();
                    let cam_right = renderer.camera.right_xz();

                    // Обчислюємо input
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

                    // Нормалізуємо якщо є рух (щоб діагональний рух не був швидшим)
                    if move_dir.length_squared() > 0.01 {
                        move_dir = move_dir.normalize();

                        // Рухаємо гравця
                        self.player.position += move_dir * self.player.move_speed * delta;
                    }

                    // Гравець ЗАВЖДИ дивиться в напрямку камери
                    // camera.yaw - це кут камери НАВКОЛО гравця
                    // Гравець має дивитися В ПРОТИЛЕЖНОМУ напрямку (від камери до таргету)
                    self.player.yaw = renderer.camera.yaw + std::f32::consts::PI;
                }

                // === PLAYER MESH UPDATE ===
                if let Some(renderer) = &mut self.renderer {
                    renderer.update_player(&self.player, &self.combat);
                }

                // === CAMERA UPDATE (Third Person) ===
                if let Some(renderer) = &mut self.renderer {
                    // Mouse look (права кнопка миші)
                    if self.input_state.mouse_right {
                        let (delta_x, delta_y) = self.input_state.mouse_delta();

                        // Конвертуємо pixel delta в радіани
                        let sensitivity = 0.003;
                        let delta_yaw = (delta_x as f32) * sensitivity;
                        let delta_pitch = (delta_y as f32) * sensitivity;

                        if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
                            renderer.camera.rotate_third_person(delta_yaw, delta_pitch);
                        }
                    }

                    // Zoom (mouse wheel)
                    // Вже обробляється в MouseWheel event, але оновимо для third person

                    // Оновлюємо позицію камери за гравцем
                    renderer.camera.update_third_person(self.player.position, 1.2);

                    // Скидаємо mouse delta після обробки
                    self.input_state.reset_mouse_delta();
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
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() {
    // Ініціалізація логування
    env_logger::init();

    log::info!("=== Arena Combat Prototype ===");
    log::info!("Версія: 0.1.0");
    log::info!("Phase 1: Week 1-2 - Basic Rendering");

    // Створити event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Spawn enemies по колу навколо центру арени
    let enemies = spawn_enemies_circle(
        glam::Vec3::ZERO, // center
        8.0,              // radius
        6,                // count - 6 ворогів по колу
    );
    log::info!("Created {} enemies", enemies.len());

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
    };

    // Запустити event loop
    log::info!("Запуск event loop...");
    event_loop.run_app(&mut app).unwrap();

    log::info!("Програма завершена");
}
