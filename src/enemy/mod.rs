/*
===============================================================================
 ФАЙЛ: src/enemy/mod.rs
===============================================================================

📋 ПРИЗНАЧЕННЯ:
  Enemy система - вороги на арені.

🎯 ВІДПОВІДАЛЬНІСТЬ:
  - Enemy struct (position, health, state)
  - Enemy spawning
  - (Майбутнє) Enemy AI, pathfinding
  - (Майбутнє) Enemy attacks

⚠️  ВАЖЛИВІ ДЕТАЛІ:
  - Enemies статичні поки що (без AI)
  - Health: 0 = мертвий
  - Position в world space (Y-up)

🕐 ІСТОРІЯ:
  2025-12-14: Створено - базовий Enemy struct

===============================================================================
*/

use glam::Vec3;

/// Стан ворога
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    /// Живий, активний
    Alive,
    /// Мертвий (для cleanup або respawn)
    Dead,
}

impl Default for EnemyState {
    fn default() -> Self {
        Self::Alive
    }
}

/// Enemy - ворог на арені
pub struct Enemy {
    /// Позиція в world space
    pub position: Vec3,

    /// Кут повороту навколо Y (радіани)
    pub yaw: f32,

    /// Поточне здоров'я
    pub health: f32,

    /// Максимальне здоров'я
    pub max_health: f32,

    /// Стан ворога
    pub state: EnemyState,
}

impl Enemy {
    /// Створює нового ворога на позиції
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            health: 100.0,
            max_health: 100.0,
            state: EnemyState::Alive,
        }
    }

    /// Створює ворога з поворотом до цілі
    pub fn new_facing(position: Vec3, look_at: Vec3) -> Self {
        let dir = look_at - position;
        let yaw = dir.x.atan2(-dir.z);

        Self {
            position,
            yaw,
            health: 100.0,
            max_health: 100.0,
            state: EnemyState::Alive,
        }
    }

    /// Чи живий ворог
    pub fn is_alive(&self) -> bool {
        self.state == EnemyState::Alive && self.health > 0.0
    }

    /// Завдає шкоди ворогу
    pub fn take_damage(&mut self, damage: f32) {
        if !self.is_alive() {
            return;
        }

        self.health = (self.health - damage).max(0.0);

        if self.health <= 0.0 {
            self.state = EnemyState::Dead;
        }
    }

    /// Напрямок куди дивиться ворог
    pub fn forward(&self) -> Vec3 {
        Vec3::new(-self.yaw.sin(), 0.0, -self.yaw.cos())
    }
}

/// Спавнить ворогів по колу навколо центру
pub fn spawn_enemies_circle(center: Vec3, radius: f32, count: usize) -> Vec<Enemy> {
    let mut enemies = Vec::with_capacity(count);

    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        let x = center.x + radius * angle.cos();
        let z = center.z + radius * angle.sin();
        let position = Vec3::new(x, 0.0, z);

        // Ворог дивиться на центр
        enemies.push(Enemy::new_facing(position, center));
    }

    enemies
}
