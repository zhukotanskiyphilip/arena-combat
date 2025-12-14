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

use rendering::WgpuRenderer;
use fps_counter::FpsCounter;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
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
            // Закрити вікно
            WindowEvent::CloseRequested => {
                log::info!("Закриття вікна...");
                event_loop.exit();
            }

            // ESC також закриває
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                log::info!("ESC натиснуто - закриття...");
                event_loop.exit();
            }

            // Redraw request
            WindowEvent::RedrawRequested => {
                // Оновити FPS counter
                self.fps_counter.tick();

                // Оновити заголовок вікна з FPS (кожні 30 кадрів для зменшення overhead)
                static mut FRAME_COUNT: u32 = 0;
                unsafe {
                    FRAME_COUNT += 1;
                    if FRAME_COUNT % 30 == 0 {
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

    // Створити app
    let mut app = App {
        window: None,
        renderer: None,
        fps_counter: FpsCounter::new(),
    };

    // Запустити event loop
    log::info!("Запуск event loop...");
    event_loop.run_app(&mut app).unwrap();

    log::info!("Програма завершена");
}
