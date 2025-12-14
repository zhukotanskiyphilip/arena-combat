/*
===============================================================================
 ФАЙЛ: src/combat/mod.rs
===============================================================================

📋 ПРИЗНАЧЕННЯ:
  Combat система - атаки, cooldowns, стани бою.

🎯 ВІДПОВІДАЛЬНІСТЬ:
  - Attack state machine (Ready → Attacking → Cooldown → Ready)
  - Attack timing (duration, cooldown)
  - Attack direction tracking
  - Hitbox generation
  - Damage calculation

⚠️  ВАЖЛИВІ ДЕТАЛІ:
  - Attack duration: час виконання атаки (анімація)
  - Cooldown: час між атаками
  - Attack можна виконати тільки в Ready стані

🕐 ІСТОРІЯ:
  2025-12-14: Створено - базова attack state machine
  2025-12-14: Додано hitbox система

===============================================================================
*/

pub mod hitbox;

pub use hitbox::{Hitbox, HitboxManager};

use glam::Vec3;

/// Стан атаки гравця
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackState {
    /// Готовий атакувати
    Ready,
    /// Виконує атаку (час в секундах що залишився)
    Attacking(f32),
    /// Cooldown після атаки (час в секундах що залишився)
    Cooldown(f32),
}

impl Default for AttackState {
    fn default() -> Self {
        Self::Ready
    }
}

/// Combat компонент для entity
///
/// Відстежує attack state, timing та напрямок атаки.
pub struct Combat {
    /// Поточний стан атаки
    pub state: AttackState,

    /// Тривалість атаки (секунди)
    pub attack_duration: f32,

    /// Час cooldown між атаками (секунди)
    pub attack_cooldown: f32,

    /// Напрямок останньої атаки (normalized)
    pub attack_direction: Vec3,

    /// Прогрес атаки (0.0 = початок, 1.0 = кінець)
    /// Корисно для анімації
    pub attack_progress: f32,

    /// Кут замаху зброї (радіани)
    /// Swing: від -45° (замах назад) до +90° (удар вперед)
    pub weapon_swing_angle: f32,
}

impl Combat {
    /// Створює новий Combat компонент
    pub fn new() -> Self {
        Self {
            state: AttackState::Ready,
            attack_duration: 0.35,  // 350ms атака
            attack_cooldown: 0.15,  // 150ms cooldown
            attack_direction: Vec3::NEG_Z,
            attack_progress: 0.0,
            weapon_swing_angle: 0.0,
        }
    }

    /// Перевіряє чи можна атакувати
    pub fn can_attack(&self) -> bool {
        matches!(self.state, AttackState::Ready)
    }

    /// Починає атаку в заданому напрямку
    ///
    /// # Returns
    /// `true` якщо атака почалася, `false` якщо не можна атакувати
    pub fn start_attack(&mut self, direction: Vec3) -> bool {
        if !self.can_attack() {
            return false;
        }

        self.state = AttackState::Attacking(self.attack_duration);
        self.attack_direction = direction.normalize_or_zero();
        self.attack_progress = 0.0;

        true
    }

    /// Оновлює combat state
    ///
    /// # Аргументи
    /// * `delta` - Delta time в секундах
    pub fn update(&mut self, delta: f32) {
        // Swing animation constants
        let swing_start = -0.8_f32;  // -45° замах назад
        let swing_end = 1.6_f32;     // +90° удар вперед
        let swing_range = swing_end - swing_start;

        match self.state {
            AttackState::Ready => {
                // Повертаємо меч в нейтральну позицію
                self.weapon_swing_angle = 0.0;
            }
            AttackState::Attacking(remaining) => {
                let new_remaining = remaining - delta;

                // Оновлюємо прогрес (0→1)
                self.attack_progress = 1.0 - (new_remaining / self.attack_duration).max(0.0);

                // Swing angle: ease-out для швидкого удару
                // Використовуємо квадратичну функцію для прискорення
                let eased = self.attack_progress * (2.0 - self.attack_progress);
                self.weapon_swing_angle = swing_start + eased * swing_range;

                if new_remaining <= 0.0 {
                    // Атака завершена → cooldown
                    self.state = AttackState::Cooldown(self.attack_cooldown);
                    self.attack_progress = 1.0;
                    self.weapon_swing_angle = swing_end;
                } else {
                    self.state = AttackState::Attacking(new_remaining);
                }
            }
            AttackState::Cooldown(remaining) => {
                let new_remaining = remaining - delta;

                // Повертаємо меч назад (easing)
                let cooldown_progress = 1.0 - (new_remaining / self.attack_cooldown).max(0.0);
                self.weapon_swing_angle = swing_end * (1.0 - cooldown_progress);

                if new_remaining <= 0.0 {
                    // Cooldown завершено → ready
                    self.state = AttackState::Ready;
                    self.attack_progress = 0.0;
                    self.weapon_swing_angle = 0.0;
                } else {
                    self.state = AttackState::Cooldown(new_remaining);
                }
            }
        }
    }

    /// Перевіряє чи зараз атакує
    pub fn is_attacking(&self) -> bool {
        matches!(self.state, AttackState::Attacking(_))
    }

    /// Перевіряє чи в cooldown
    pub fn is_cooldown(&self) -> bool {
        matches!(self.state, AttackState::Cooldown(_))
    }
}

impl Default for Combat {
    fn default() -> Self {
        Self::new()
    }
}
