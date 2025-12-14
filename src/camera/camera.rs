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
        // glam::perspective_rh вже враховує правильну систему координат
        #[rustfmt::skip]
        let opengl_to_wgpu = Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,   // Y БЕЗ інверсії (glam вже правильно рахує)
            0.0, 0.0, 0.5, 0.0,   // Масштабуємо Z: [-1,1] → [0,1]
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

    // ========================================================================
    // ORBIT CAMERA CONTROLS
    // ========================================================================

    /// Обертає камеру навколо target (orbit camera)
    ///
    /// Використовує spherical coordinates для обертання.
    /// Це дозволяє обертати камеру навколо фіксованої точки (target).
    ///
    /// # Аргументи
    /// * `delta_yaw` - Обертання по горизонталі (радіани, +/-)
    /// * `delta_pitch` - Обертання по вертикалі (радіани, +/-)
    ///
    /// # Обмеження
    /// - Pitch обмежений діапазоном [-89°, +89°] щоб не перевернути камеру
    /// - Yaw необмежений (можна обертатись на 360°)
    ///
    /// # Математика
    /// 1. Обчислюємо вектор від target до camera
    /// 2. Конвертуємо в spherical coordinates (radius, yaw, pitch)
    /// 3. Додаємо delta_yaw та delta_pitch
    /// 4. Обмежуємо pitch
    /// 5. Конвертуємо назад в Cartesian coordinates
    /// 6. Оновлюємо position = target + offset
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // 1. Вектор від target до camera
        let offset = self.position - self.target;
        let radius = offset.length();

        // Якщо камера ДУ close до target - skip
        if radius < 0.1 {
            return;
        }

        // 2. Поточні spherical координати
        // yaw = кут в XZ plane (горизонтальне обертання)
        // pitch = кут від XZ plane (вертикальне обертання)
        let current_yaw = offset.z.atan2(offset.x);
        let current_pitch = (offset.y / radius).asin();

        // 3. Додаємо delta
        let new_yaw = current_yaw + delta_yaw;
        let new_pitch = current_pitch + delta_pitch;

        // 4. Обмежуємо pitch (не даємо камері перевернутись)
        // Обмежуємо до [-89°, +89°] (залишаємо невеличкий запас від ±90°)
        let max_pitch = 89.0_f32.to_radians();
        let clamped_pitch = new_pitch.clamp(-max_pitch, max_pitch);

        // 5. Конвертуємо spherical → Cartesian
        // x = r * cos(pitch) * cos(yaw)
        // y = r * sin(pitch)
        // z = r * cos(pitch) * sin(yaw)
        let new_offset = Vec3::new(
            radius * clamped_pitch.cos() * new_yaw.cos(),
            radius * clamped_pitch.sin(),
            radius * clamped_pitch.cos() * new_yaw.sin(),
        );

        // 6. Оновлюємо position
        self.position = self.target + new_offset;
    }

    /// Zoom (наближення/віддалення від target)
    ///
    /// Переміщує камеру ближче або далі від target вздовж напрямку погляду.
    ///
    /// # Аргументи
    /// * `delta` - Зміна відстані (+ = ближче, - = далі)
    ///
    /// # Обмеження
    /// - Мінімальна відстань: 1.0 unit (не даємо камері зайти всередину target)
    /// - Максимальна відстань: 50.0 units
    pub fn zoom(&mut self, delta: f32) {
        let offset = self.position - self.target;
        let current_distance = offset.length();

        // Обчислюємо нову відстань
        let new_distance = current_distance - delta; // Мінус бо + це zoom in

        // Обмежуємо відстань
        let clamped_distance = new_distance.clamp(1.0, 50.0);

        // Оновлюємо position зі збереженням напрямку
        if offset.length() > 0.01 {
            let direction = offset.normalize();
            self.position = self.target + direction * clamped_distance;
        }
    }

    /// Переміщує target (pan camera)
    ///
    /// Переміщує і камеру і target на вказаний offset.
    /// Зберігає відносну позицію камери щодо target.
    ///
    /// # Аргументи
    /// * `offset` - Вектор переміщення в world space
    pub fn pan(&mut self, offset: Vec3) {
        self.position += offset;
        self.target += offset;
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
