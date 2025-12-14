/*
===============================================================================
 ФАЙЛ: src/combat/hitbox.rs
===============================================================================

📋 ПРИЗНАЧЕННЯ:
  Hitbox система - зони ураження для атак.

🎯 ВІДПОВІДАЛЬНІСТЬ:
  - Hitbox struct (position, size, lifetime)
  - Collision detection (sphere vs sphere)
  - Damage application

⚠️  ВАЖЛИВІ ДЕТАЛІ:
  - Hitbox існує короткий час (~100ms)
  - Використовуємо sphere collision для простоти
  - Один hitbox може вразити кожного ворога лише раз

🕐 ІСТОРІЯ:
  2025-12-14: Створено - базова hitbox система

===============================================================================
*/

use glam::Vec3;

/// Hitbox - зона ураження
pub struct Hitbox {
    /// Центр hitbox в world space
    pub position: Vec3,

    /// Радіус hitbox (sphere collision)
    pub radius: f32,

    /// Час життя що залишився (секунди)
    pub lifetime: f32,

    /// Шкода при влучанні
    pub damage: f32,

    /// ID ворогів яких вже вразили (щоб не бити двічі)
    pub hit_enemies: Vec<usize>,
}

impl Hitbox {
    /// Створює новий hitbox
    pub fn new(position: Vec3, radius: f32, lifetime: f32, damage: f32) -> Self {
        Self {
            position,
            radius,
            lifetime,
            damage,
            hit_enemies: Vec::new(),
        }
    }

    /// Перевіряє чи hitbox ще активний
    pub fn is_active(&self) -> bool {
        self.lifetime > 0.0
    }

    /// Оновлює hitbox (зменшує lifetime)
    pub fn update(&mut self, delta: f32) {
        self.lifetime -= delta;
    }

    /// Перевіряє колізію з точкою (sphere vs point)
    pub fn collides_with_point(&self, point: Vec3) -> bool {
        let distance = (self.position - point).length();
        distance < self.radius
    }

    /// Перевіряє колізію зі сферою (sphere vs sphere)
    pub fn collides_with_sphere(&self, center: Vec3, radius: f32) -> bool {
        let distance = (self.position - center).length();
        distance < (self.radius + radius)
    }

    /// Позначає ворога як враженого
    pub fn mark_hit(&mut self, enemy_index: usize) {
        self.hit_enemies.push(enemy_index);
    }

    /// Перевіряє чи ворог вже був вражений цим hitbox
    pub fn has_hit(&self, enemy_index: usize) -> bool {
        self.hit_enemies.contains(&enemy_index)
    }
}

/// Менеджер hitbox'ів
pub struct HitboxManager {
    /// Активні hitbox'и
    pub hitboxes: Vec<Hitbox>,
}

impl HitboxManager {
    pub fn new() -> Self {
        Self {
            hitboxes: Vec::new(),
        }
    }

    /// Додає новий hitbox
    pub fn spawn(&mut self, hitbox: Hitbox) {
        self.hitboxes.push(hitbox);
    }

    /// Створює hitbox атаки на кінці зброї
    ///
    /// Зброя знаходиться на правій руці гравця, меч направлений вперед.
    /// Hitbox з'являється на кінці меча.
    pub fn spawn_attack_hitbox(&mut self, player_pos: Vec3, player_yaw: f32, damage: f32) {
        // Weapon parameters (мають співпадати з generate_armed_mannequin)
        let body_radius = 0.3;
        let arm_length = 0.6;
        let weapon_length = 1.0;
        let shoulder_height = 1.2 / 2.0 - 0.15; // body_height/2 - offset

        // Right direction (перпендикулярно до forward)
        let right = Vec3::new(player_yaw.cos(), 0.0, -player_yaw.sin());

        // Forward direction
        let forward = Vec3::new(-player_yaw.sin(), 0.0, -player_yaw.cos());

        // Позиція кінця зброї:
        // - праворуч на відстані (body_radius + arm_length)
        // - вперед на довжину меча
        // - на висоті плеча
        let weapon_tip_offset = right * (body_radius + arm_length)
            + forward * (weapon_length * 0.8)  // 80% довжини меча вперед
            + Vec3::new(0.0, shoulder_height, 0.0);

        let hitbox_pos = player_pos + weapon_tip_offset;

        let hitbox = Hitbox::new(
            hitbox_pos,
            0.5,    // radius (менший, точніший)
            0.15,   // lifetime (150ms)
            damage,
        );

        self.spawn(hitbox);
    }

    /// Оновлює всі hitbox'и та видаляє неактивні
    pub fn update(&mut self, delta: f32) {
        // Оновлюємо lifetime
        for hitbox in &mut self.hitboxes {
            hitbox.update(delta);
        }

        // Видаляємо неактивні
        self.hitboxes.retain(|h| h.is_active());
    }

    /// Повертає кількість активних hitbox'ів
    pub fn active_count(&self) -> usize {
        self.hitboxes.len()
    }
}

impl Default for HitboxManager {
    fn default() -> Self {
        Self::new()
    }
}
