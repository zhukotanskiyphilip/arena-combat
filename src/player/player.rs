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

    /// Поточний кут повороту навколо Y (в радіанах)
    /// 0 = дивиться в -Z напрямку
    pub yaw: f32,

    /// Цільовий кут (куди персонаж повертається)
    pub target_yaw: f32,

    /// Швидкість руху (units/second)
    pub move_speed: f32,

    /// Швидкість повороту (radians/second)
    pub turn_speed: f32,

    /// Чи персонаж зараз рухається
    pub is_moving: bool,
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
            target_yaw: 0.0,
            move_speed: 5.0,   // 5 units/second
            turn_speed: 10.0,  // швидке плавне обертання
            is_moving: false,
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

    /// Встановлює цільовий кут на основі напрямку руху
    ///
    /// # Аргументи
    /// * `move_dir` - Напрямок руху в world space (нормалізований)
    pub fn set_target_direction(&mut self, move_dir: Vec3) {
        if move_dir.length_squared() > 0.01 {
            // Player forward = (-sin(yaw), 0, -cos(yaw))
            // Щоб forward == move_dir:
            //   -sin(yaw) = move_dir.x  →  sin(yaw) = -move_dir.x
            //   -cos(yaw) = move_dir.z  →  cos(yaw) = -move_dir.z
            // Тому: yaw = atan2(-move_dir.x, -move_dir.z)
            self.target_yaw = (-move_dir.x).atan2(-move_dir.z);
            self.is_moving = true;
        } else {
            self.is_moving = false;
        }
    }

    /// Плавно обертає персонажа до target_yaw
    ///
    /// # Аргументи
    /// * `delta` - Delta time в секундах
    pub fn smooth_rotate(&mut self, delta: f32) {
        // Обчислюємо найкоротшу різницю кутів
        let mut diff = self.target_yaw - self.yaw;

        // Нормалізуємо до [-PI, PI] для найкоротшого шляху
        while diff > std::f32::consts::PI {
            diff -= std::f32::consts::TAU;
        }
        while diff < -std::f32::consts::PI {
            diff += std::f32::consts::TAU;
        }

        // Плавне обертання
        let max_rotation = self.turn_speed * delta;
        if diff.abs() <= max_rotation {
            // Достатньо близько - завершуємо
            self.yaw = self.target_yaw;
        } else {
            // Обертаємось у напрямку target
            self.yaw += diff.signum() * max_rotation;
        }

        // Нормалізуємо yaw
        self.normalize_yaw();
    }

    /// Нормалізує yaw до [-PI, PI]
    fn normalize_yaw(&mut self) {
        while self.yaw > std::f32::consts::PI {
            self.yaw -= std::f32::consts::TAU;
        }
        while self.yaw < -std::f32::consts::PI {
            self.yaw += std::f32::consts::TAU;
        }
    }

    /// Встановлює yaw напряму (для синхронізації з камерою коли не рухаємось)
    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.target_yaw = yaw;
        self.normalize_yaw();
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

/// Допоміжна функція для обчислення yaw з camera.yaw
/// Синхронізує player forward з camera forward
pub fn camera_yaw_to_player_yaw(cam_yaw: f32) -> f32 {
    // camera.forward_xz() = (-cos(cam_yaw), 0, -sin(cam_yaw))
    // player.forward()    = (-sin(player_yaw), 0, -cos(player_yaw))
    // Щоб вони співпадали: player_yaw = PI/2 - cam_yaw
    std::f32::consts::FRAC_PI_2 - cam_yaw
}
