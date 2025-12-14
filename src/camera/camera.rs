/*
═══════════════════════════════════════════════════════════════════════════════
 ФАЙЛ: src/camera/camera.rs
═══════════════════════════════════════════════════════════════════════════════

📋 ПРИЗНАЧЕННЯ:
   Camera - 3D камера з perspective projection для Arena Combat.

   Система координат: Y-up, right-handed (як в OpenGL)
   - +X = право
   - +Y = вгору
   - +Z = назад (до камери)
   - -Z = вперед (від камери, напрямок погляду)

🎯 ВІДПОВІДАЛЬНІСТЬ:
   - Зберігання позиції та орієнтації камери
   - Обчислення view matrix (перетворення world → camera space)
   - Обчислення projection matrix (perspective)
   - Надання uniform buffer даних для shader

🔗 ЗВ'ЯЗКИ З ІНШИМИ ФАЙЛАМИ:
   Імпортує:
   - glam - математика (Vec3, Mat4)

   Експортує для:
   - rendering/renderer.rs - створення та оновлення камери

📦 ЗАЛЕЖНОСТІ:
   - glam = "0.29" - векторна математика з SIMD оптимізаціями

⚠️  ВАЖЛИВІ ОБМЕЖЕННЯ:
   1. Координатна система: Y-up, right-handed (OpenGL convention)
   2. Projection: perspective з FOV в радіанах
   3. Для wgpu потрібна коригуюча матриця (OpenGL → Vulkan/DX)

🧪 ТЕСТУВАННЯ:
   ```rust
   let camera = Camera::new(
       Vec3::new(0.0, 2.0, 5.0),  // позиція
       Vec3::new(0.0, 0.0, 0.0),  // дивимось на
       800.0 / 600.0              // aspect ratio
   );

   let view_proj = camera.build_view_projection_matrix();
   ```

🕐 ІСТОРІЯ:
   2025-12-14: Створено - базова 3D camera з perspective projection

═══════════════════════════════════════════════════════════════════════════════
*/

use glam::{Mat4, Vec3};

/// 3D Camera з perspective projection
///
/// Координатна система: Y-up, right-handed
/// - +X вправо, +Y вгору, +Z назад (до камери)
/// - -Z = forward (напрямок погляду)
pub struct Camera {
    /// Позиція камери в world space
    pub position: Vec3,

    /// Точка на яку дивиться камера (target)
    pub target: Vec3,

    /// Вектор "вгору" для камери (зазвичай Vec3::Y)
    pub up: Vec3,

    /// Field of View (вертикальний кут огляду) в радіанах
    pub fovy: f32,

    /// Aspect ratio (width / height)
    pub aspect: f32,

    /// Ближня площина відсікання
    pub znear: f32,

    /// Дальня площина відсікання
    pub zfar: f32,
}

impl Camera {
    /// Створює нову камеру з заданими параметрами
    ///
    /// # Аргументи
    /// * `position` - Позиція камери в world space
    /// * `target` - Точка на яку дивиться камера
    /// * `aspect` - Aspect ratio (width / height)
    ///
    /// # Повертає
    /// Нову Camera з дефолтними FOV=45° та z-planes
    ///
    /// # Приклад
    /// ```
    /// let camera = Camera::new(
    ///     Vec3::new(0.0, 2.0, 5.0),
    ///     Vec3::new(0.0, 0.0, 0.0),
    ///     800.0 / 600.0
    /// );
    /// ```
    pub fn new(position: Vec3, target: Vec3, aspect: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y, // Стандартний "вгору" = (0, 1, 0)
            fovy: 45.0_f32.to_radians(), // 45 градусів у радіанах
            aspect,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    /// Будує view matrix (world space → camera space)
    ///
    /// Використовує "look-at" матрицю для перетворення координат
    /// з world space в camera space.
    ///
    /// # Повертає
    /// Mat4 - view матриця
    pub fn build_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Будує projection matrix (camera space → clip space)
    ///
    /// Використовує perspective projection з FOV.
    /// ВАЖЛИВО: Для wgpu потрібна коригуюча матриця OpenGL → Vulkan/DX.
    ///
    /// # Повертає
    /// Mat4 - projection матриця
    pub fn build_projection_matrix(&self) -> Mat4 {
        // Базова perspective projection (OpenGL style)
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        // Коригуюча матриця для wgpu (OpenGL → Vulkan/DirectX)
        // Vulkan/DX мають NDC Z в діапазоні [0, 1], а OpenGL [-1, 1]
        // Також Y інвертований
        #[rustfmt::skip]
        let opengl_to_wgpu = Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,  // Інвертуємо Y
            0.0, 0.0, 0.5, 0.0,   // Масштабуємо Z
            0.0, 0.0, 0.5, 1.0,   // Зміщуємо Z
        ]);

        opengl_to_wgpu * proj
    }

    /// Будує комбіновану view-projection матрицю
    ///
    /// Це комбінація view та projection матриць, яка трансформує
    /// координати з world space безпосередньо в clip space.
    ///
    /// # Повертає
    /// Mat4 - view-projection матриця
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        self.build_projection_matrix() * self.build_view_matrix()
    }

    /// Оновлює aspect ratio (при зміні розміру вікна)
    ///
    /// # Аргументи
    /// * `width` - Ширина вікна
    /// * `height` - Висота вікна
    pub fn update_aspect(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect = width as f32 / height as f32;
        }
    }

    /// Переміщує камеру на вказану позицію
    ///
    /// # Аргументи
    /// * `position` - Нова позиція камери
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Змінює точку на яку дивиться камера
    ///
    /// # Аргументи
    /// * `target` - Нова target точка
    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    /// Повертає напрямок forward (напрямок погляду камери)
    ///
    /// # Повертає
    /// Нормалізований Vec3 від камери до target
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// Повертає напрямок right (праворуч від камери)
    ///
    /// # Повертає
    /// Нормалізований Vec3 праворуч
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }
}

/// Uniform buffer для передачі в shader
///
/// Це структура яка буде передаватись в GPU через uniform buffer.
/// ВАЖЛИВО: Повинна мати правильне вирівнювання для GPU (16 bytes).
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// View-Projection матриця (4x4 = 16 floats = 64 bytes)
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    /// Створює новий CameraUniform
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    /// Оновлює uniform з камери
    ///
    /// # Аргументи
    /// * `camera` - Камера з якої взяти view-projection матрицю
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}
