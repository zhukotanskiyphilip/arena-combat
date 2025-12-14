/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/transform/transform.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Transform - позиція, обертання та масштаб об'єкта в 3D просторі.

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Зберігання position (Vec3), rotation (Quat), scale (Vec3)
   - Обчислення Model matrix (local → world space)
   - TransformUniform для передачі в shader

🔗 ЗВ'ЯЗКИ:
   Використовується в: rendering/mesh.rs

⚠️  ВАЖЛИВІ ДЕТАЛІ:
   - Transform order: Scale → Rotate → Translate (S*R*T)
   - Rotation: Quaternion (уникає gimbal lock)
   - Default: position=(0,0,0), rotation=identity, scale=(1,1,1)

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базовий Transform з Model matrix

═══════════════════════════════════════════════════════════════════════════════
*/

use glam::{Mat4, Quat, Vec3};

/// Transform - позиціонування об'єкта в 3D просторі
///
/// Містить position, rotation, scale для обчислення Model matrix.
/// Model matrix трансформує координати з local space в world space.
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// Позиція в world space
    pub position: Vec3,

    /// Обертання (Quaternion для уникнення gimbal lock)
    pub rotation: Quat,

    /// Масштаб по кожній осі
    pub scale: Vec3,
}

impl Transform {
    /// Створює новий Transform з заданою позицією
    ///
    /// # Аргументи
    /// * `position` - Позиція в world space
    ///
    /// # Повертає
    /// Transform з rotation=identity, scale=(1,1,1)
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Створює Transform з позицією, обертанням та масштабом
    pub fn from_position_rotation_scale(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Створює Transform з позицією та рівномірним масштабом
    pub fn with_scale(position: Vec3, uniform_scale: f32) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(uniform_scale),
        }
    }

    /// Обчислює Model matrix
    ///
    /// Transform order: Scale → Rotate → Translate
    /// M = T * R * S
    ///
    /// # Повертає
    /// Mat4 - model matrix (local → world space)
    pub fn model_matrix(&self) -> Mat4 {
        // glam надає зручний метод для цього
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Встановлює обертання через Euler angles (degrees)
    ///
    /// # Аргументи
    /// * `pitch` - Обертання навколо X (degrees)
    /// * `yaw` - Обертання навколо Y (degrees)
    /// * `roll` - Обертання навколо Z (degrees)
    pub fn set_rotation_euler(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.rotation = Quat::from_euler(
            glam::EulerRot::YXZ, // Yaw first, then pitch, then roll
            yaw.to_radians(),
            pitch.to_radians(),
            roll.to_radians(),
        );
    }

    /// Обертає об'єкт на вказані кути (degrees)
    ///
    /// Додає обертання до існуючого.
    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        let delta = Quat::from_euler(
            glam::EulerRot::YXZ,
            yaw.to_radians(),
            pitch.to_radians(),
            roll.to_radians(),
        );
        self.rotation = delta * self.rotation;
    }

    /// Переміщує об'єкт на вказаний offset
    pub fn translate(&mut self, offset: Vec3) {
        self.position += offset;
    }

    /// Повертає forward vector (напрямок -Z в local space, трансформований в world)
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Повертає right vector (+X в local space, трансформований в world)
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Повертає up vector (+Y в local space, трансформований в world)
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

/// Uniform buffer для Transform (Model matrix)
///
/// Передається в shader для трансформації вершин.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    /// Model matrix (4x4 = 16 floats = 64 bytes)
    pub model: [[f32; 4]; 4],

    /// Normal matrix (верхня ліва 3x3 частина inverse transpose model matrix)
    /// Використовується для коректної трансформації нормалей
    /// Padding до 16 bytes alignment
    pub normal_matrix: [[f32; 4]; 3],

    /// Padding для вирівнювання (16 bytes alignment)
    pub _padding: [f32; 4],
}

impl TransformUniform {
    /// Створює новий TransformUniform з identity matrix
    pub fn new() -> Self {
        Self {
            model: Mat4::IDENTITY.to_cols_array_2d(),
            normal_matrix: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            _padding: [0.0; 4],
        }
    }

    /// Оновлює uniform з Transform
    pub fn update(&mut self, transform: &Transform) {
        let model = transform.model_matrix();
        self.model = model.to_cols_array_2d();

        // Normal matrix = transpose(inverse(model))
        // Для uniform scale можна просто взяти upper-left 3x3
        // Для non-uniform scale потрібен повний inverse transpose
        let normal_mat = model.inverse().transpose();

        // Беремо верхню ліву 3x3 частину
        self.normal_matrix = [
            [normal_mat.x_axis.x, normal_mat.x_axis.y, normal_mat.x_axis.z, 0.0],
            [normal_mat.y_axis.x, normal_mat.y_axis.y, normal_mat.y_axis.z, 0.0],
            [normal_mat.z_axis.x, normal_mat.z_axis.y, normal_mat.z_axis.z, 0.0],
        ];
    }
}

impl Default for TransformUniform {
    fn default() -> Self {
        Self::new()
    }
}
