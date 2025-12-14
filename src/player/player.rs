/*
===============================================================================
 ФАЙЛ: src/player/player.rs
===============================================================================

📋 ПРИЗНАЧЕННЯ:
  Player struct - гравець з позицією, напрямком та рухом.

🎯 ВІДПОВІДАЛЬНІСТЬ:
  - Зберігання позиції в world space
  - Facing direction (yaw angle)
  - Movement logic (WASD input → position change)
  - Movement speed

⚠️  ВАЖЛИВІ ДЕТАЛІ:
  - Position: Vec3 в world space (Y-up)
  - Yaw: кут повороту навколо Y (0 = дивиться в -Z, як камера)
  - Movement speed: units/second (використовуйте delta time!)
  - Player рухається по XZ plane (Y = const для наземного руху)

🕐 ІСТОРІЯ:
  2025-12-14: Створено - базовий Player з позицією та рухом

===============================================================================
*/

use glam::Vec3;

/// Player - гравець з позицією та рухом
///
/// Гравець має позицію в world space та facing direction (yaw).
/// Рух відбувається по XZ plane з постійною швидкістю.
pub struct Player {
    /// Позиція в world space
    pub position: Vec3,

    /// Кут повороту навколо Y (в радіанах)
    /// 0 = дивиться в -Z напрямку
    pub yaw: f32,

    /// Швидкість руху (units/second)
    pub move_speed: f32,

    /// Швидкість повороту (radians/second)
    pub turn_speed: f32,
}

impl Player {
    /// Створює нового гравця на заданій позиції
    ///
    /// # Аргументи
    /// * `position` - Початкова позиція в world space
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            move_speed: 5.0,  // 5 units/second
            turn_speed: 3.0,  // ~170 degrees/second
        }
    }

    /// Повертає forward vector (напрямок куди дивиться гравець)
    ///
    /// Forward = -Z при yaw=0, обертається навколо Y
    pub fn forward(&self) -> Vec3 {
        Vec3::new(-self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    /// Повертає right vector (вправо від гравця)
    pub fn right(&self) -> Vec3 {
        Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin())
    }

    /// Рухає гравця вперед/назад
    ///
    /// # Аргументи
    /// * `amount` - Напрямок та інтенсивність (-1.0 до 1.0)
    /// * `delta` - Delta time в секундах
    pub fn move_forward(&mut self, amount: f32, delta: f32) {
        let movement = self.forward() * amount * self.move_speed * delta;
        self.position += movement;
    }

    /// Рухає гравця вліво/вправо (strafe)
    ///
    /// # Аргументи
    /// * `amount` - Напрямок та інтенсивність (-1.0 = left, 1.0 = right)
    /// * `delta` - Delta time в секундах
    pub fn strafe(&mut self, amount: f32, delta: f32) {
        let movement = self.right() * amount * self.move_speed * delta;
        self.position += movement;
    }

    /// Повертає гравця (yaw)
    ///
    /// # Аргументи
    /// * `amount` - Напрямок та інтенсивність (-1.0 = left, 1.0 = right)
    /// * `delta` - Delta time в секундах
    pub fn turn(&mut self, amount: f32, delta: f32) {
        self.yaw += amount * self.turn_speed * delta;

        // Нормалізуємо yaw до [-PI, PI]
        while self.yaw > std::f32::consts::PI {
            self.yaw -= 2.0 * std::f32::consts::PI;
        }
        while self.yaw < -std::f32::consts::PI {
            self.yaw += 2.0 * std::f32::consts::PI;
        }
    }

    /// Оновлює гравця на основі input
    ///
    /// # Аргументи
    /// * `forward` - Forward/backward input (-1.0 до 1.0)
    /// * `strafe` - Left/right strafe input (-1.0 до 1.0)
    /// * `turn` - Turn input (-1.0 до 1.0)
    /// * `delta` - Delta time в секундах
    pub fn update(&mut self, forward: f32, strafe: f32, turn: f32, delta: f32) {
        if forward.abs() > 0.01 {
            self.move_forward(forward, delta);
        }
        if strafe.abs() > 0.01 {
            self.strafe(strafe, delta);
        }
        if turn.abs() > 0.01 {
            self.turn(turn, delta);
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}
