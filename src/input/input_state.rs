/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/input/input_state.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   InputState - структура для tracking стану клавіатури та миші.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Зберігання поточної позиції миші
   - Зберігання попередньої позиції миші (для delta)
   - Tracking стану кнопок миші (ліва/права/середня)
   - Tracking натиснутих клавіш (WASD, Shift, Ctrl, тощо)
   - Надання методів для перевірки стану

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - winit::event::{MouseButton, ElementState}
   - winit::keyboard::{PhysicalKey, KeyCode}

   Експортує для:
   - main.rs - обробка input events

⚠️  ВАЖЛИВІ ОБМЕЖЕННЯ:
   1. Стан миші оновлюється ТІЛЬКИ в event handler
   2. Delta обчислюється як різниця між поточною і попередньою позицією
   3. После обчислення delta, треба викликати reset_mouse_delta()

📝 ПРИКЛАД ВИКОРИСТАННЯ:
   ```rust
   let mut input_state = InputState::new();

   // В event handler
   match event {
       WindowEvent::CursorMoved { position, .. } => {
           input_state.update_mouse_position(position.x, position.y);
       }
       WindowEvent::MouseInput { button, state, .. } => {
           input_state.update_mouse_button(button, state);
       }
       WindowEvent::KeyboardInput { event, .. } => {
           if let PhysicalKey::Code(key_code) = event.physical_key {
               input_state.update_key(key_code, event.state);
           }
       }
   }

   // В update loop
   let mouse_delta = input_state.mouse_delta();
   if mouse_delta != (0.0, 0.0) {
       // Оновити камеру
   }
   input_state.reset_mouse_delta();
   ```

🕐 ІСТОРІЯ:
   2025-12-14: Створено - tracking миші та клавіатури для camera controls

═══════════════════════════════════════════════════════════════════════════════
*/

use winit::event::{MouseButton, ElementState};
use winit::keyboard::{PhysicalKey, KeyCode};
use std::collections::HashSet;

/// Стан введення (клавіатура + миша)
///
/// Зберігає поточний стан всіх input пристроїв для використання в game loop.
#[derive(Debug)]
pub struct InputState {
    // === Mouse state ===
    /// Поточна позиція миші (screen coordinates)
    mouse_position: (f64, f64),

    /// Попередня позиція миші (для обчислення delta)
    previous_mouse_position: (f64, f64),

    /// Ліва кнопка миші натиснута
    pub mouse_left: bool,

    /// Права кнопка миші натиснута
    pub mouse_right: bool,

    /// Середня кнопка миші натиснута
    pub mouse_middle: bool,

    // === Keyboard state ===
    /// Set натиснутих клавіш (використовуємо HashSet для швидкого lookup)
    pressed_keys: HashSet<KeyCode>,
}

impl InputState {
    /// Створює новий InputState з дефолтним станом
    pub fn new() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            previous_mouse_position: (0.0, 0.0),
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
            pressed_keys: HashSet::new(),
        }
    }

    // ========================================================================
    // MOUSE METHODS
    // ========================================================================

    /// Оновлює позицію миші
    ///
    /// Викликається в CursorMoved event.
    ///
    /// # Аргументи
    /// * `x` - X координата в screen space
    /// * `y` - Y координата в screen space
    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.previous_mouse_position = self.mouse_position;
        self.mouse_position = (x, y);
    }

    /// Повертає поточну позицію миші
    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    /// Повертає mouse delta (різниця між поточною і попередньою позицією)
    ///
    /// Використовується для camera rotation.
    ///
    /// # Повертає
    /// (delta_x, delta_y) в screen space
    pub fn mouse_delta(&self) -> (f64, f64) {
        (
            self.mouse_position.0 - self.previous_mouse_position.0,
            self.mouse_position.1 - self.previous_mouse_position.1,
        )
    }

    /// Скидає mouse delta (встановлює previous = current)
    ///
    /// Викликається після обробки mouse delta в update loop,
    /// щоб не обробляти той самий delta двічі.
    pub fn reset_mouse_delta(&mut self) {
        self.previous_mouse_position = self.mouse_position;
    }

    /// Оновлює стан кнопки миші
    ///
    /// Викликається в MouseInput event.
    ///
    /// # Аргументи
    /// * `button` - MouseButton (Left/Right/Middle)
    /// * `state` - ElementState (Pressed/Released)
    pub fn update_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        let pressed = state == ElementState::Pressed;

        match button {
            MouseButton::Left => self.mouse_left = pressed,
            MouseButton::Right => self.mouse_right = pressed,
            MouseButton::Middle => self.mouse_middle = pressed,
            _ => {} // Ігноруємо інші кнопки (Back, Forward, тощо)
        }
    }

    // ========================================================================
    // KEYBOARD METHODS
    // ========================================================================

    /// Оновлює стан клавіші
    ///
    /// Викликається в KeyboardInput event.
    ///
    /// # Аргументи
    /// * `key_code` - KeyCode клавіші
    /// * `state` - ElementState (Pressed/Released)
    pub fn update_key(&mut self, key_code: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed_keys.insert(key_code);
            }
            ElementState::Released => {
                self.pressed_keys.remove(&key_code);
            }
        }
    }

    /// Перевіряє чи натиснута клавіша
    ///
    /// # Аргументи
    /// * `key_code` - KeyCode клавіші для перевірки
    ///
    /// # Повертає
    /// `true` якщо клавіша натиснута
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    // ========================================================================
    // CONVENIENCE METHODS (для WASD та інших популярних клавіш)
    // ========================================================================

    /// Перевіряє чи натиснута W (вперед)
    pub fn is_w_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyW)
    }

    /// Перевіряє чи натиснута A (вліво)
    pub fn is_a_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyA)
    }

    /// Перевіряє чи натиснута S (назад)
    pub fn is_s_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyS)
    }

    /// Перевіряє чи натиснута D (вправо)
    pub fn is_d_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyD)
    }

    /// Перевіряє чи натиснута Space (вгору / jump)
    pub fn is_space_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::Space)
    }

    /// Перевіряє чи натиснута Shift (вниз / sprint)
    pub fn is_shift_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::ShiftLeft) || self.is_key_pressed(KeyCode::ShiftRight)
    }

    /// Перевіряє чи натиснута Ctrl (special action)
    pub fn is_ctrl_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::ControlLeft) || self.is_key_pressed(KeyCode::ControlRight)
    }

    /// Перевіряє чи натиснута Q (поворот вліво)
    pub fn is_q_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyQ)
    }

    /// Перевіряє чи натиснута E (поворот вправо)
    pub fn is_e_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyE)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
