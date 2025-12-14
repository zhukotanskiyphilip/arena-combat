# Arena Combat - Progress Log
## Журнал розробки проекту

**Останнє оновлення:** 2025-12-14

---

## 📋 Поточний статус проекту

### Фаза: Планування і технічні рішення ✅
### Наступна фаза: Початок розробки

---

## 🎯 Прийняті рішення

### 1. Мова програмування: **Rust**
**Дата:** 2025-12-11
**Обґрунтування:**
- Максимальна продуктивність (98-100% C++)
- Memory safety (критично для стабільності)
- Детермінізм (готуємось до майбутнього netcode)
- GGRS бібліотека для rollback netcode

**Альтернативи що розглядались:** C++, Zig
**Документ:** [tech_stack_decision.md](tech_stack_decision.md)

---

### 2. Підхід до розробки: **БЕЗ готових game engines**
**Дата:** 2025-12-11
**Причина:**
- Легше для AI-assisted розробки (Claude генерує код)
- Повний контроль над кожним компонентом
- Простіша інтеграція мережевого коду

**Використовувані бібліотеки:**
- `wgpu` - rendering
- `winit` - вікна + input
- `glam` - математика
- `rodio` - аудіо
- `parry3d` - collision detection

---

### 3. Платформа: **PC (Windows/Linux)**
**Дата:** 2025-12-11
**Контроли:** Mouse + Keyboard
- Mouse: напрямок атаки + камера
- WASD: рух
- Інші клавіші: атака, блок, dodge

---

### 4. План розробки: **Синглплеєр → LAN мультиплеєр**

#### Phase 1: Синглплеєр з AI (3-4 місяці)
**Статус:** 🔄 В процесі (Week 1)

**Цілі:**
- [x] Технічні рішення прийняті
- [x] Rust проект створено
- [x] Базове вікно + event loop
- [ ] Базовий rendering (wgpu)
- [ ] Fluid movement система
- [ ] Directional combat (8 напрямків)
- [ ] Block/Parry/Dodge
- [ ] AI opponent (3 рівні складності)
- [ ] Training mode

**Deliverable:** Граюча демо Гравець VS AI

---

#### Phase 2: LAN Multiplayer (1-2 місяці)
**Статус:** 🔲 Не розпочато (після Phase 1)

**Цілі:**
- [ ] UDP networking (quinn або laminar)
- [ ] Host/Join система
- [ ] Local network discovery
- [ ] Input synchronization
- [ ] Basic latency compensation
- [ ] (Опційно) GGRS rollback netcode

**Deliverable:** 1v1 по локальній мережі

---

## 📁 Структура документації

### Основні документи:
1. **[arena_combat_gdd.md](arena_combat_gdd.md)** - Game Design Document (філософія гри, механіки)
2. **[tech_stack_decision.md](tech_stack_decision.md)** - Технічні рішення (мова, архітектура, roadmap)
3. **[PROGRESS.md](PROGRESS.md)** - Цей файл (журнал прогресу)

### Майбутня документація:
- `ARCHITECTURE.md` - Архітектура коду (коли почнемо писати)
- `AI_DESIGN.md` - Дизайн AI opponent
- `NETWORKING.md` - Netcode implementation details
- `BUILD.md` - Інструкції для компіляції

---

## 🗓️ Timeline

### 2025-12-11 (Сесія 1): Планування та технічні рішення
**Тривалість:** ~2 години
**Фаза:** Планування

#### Виконано:
- ✅ Обрано Rust як основну мову
- ✅ Вирішено не використовувати готові game engines
- ✅ Визначено стек: wgpu + winit + власна логіка
- ✅ План: Синглплеєр спочатку, потім LAN
- ✅ Створено tech_stack_decision.md (детальний аналіз)
- ✅ Створено PROGRESS.md (цей файл)
- ✅ Створено систему документації для AI:
  - `.claude` - основні інструкції
  - `.claude_docs/SESSION_PROTOCOL.md` - протокол роботи між сесіями
  - `.claude_docs/CODE_TEMPLATE.md` - шаблон для файлів з кодом
  - `README.md` - швидкий старт
- ✅ Визначено підхід: Синглплеєр з AI → LAN мультиплеєр
- ✅ Розроблено AI opponent систему (3 рівні складності)

#### Прийняті рішення:
1. **Платформа:** PC тільки (не мобільні) - mouse + keyboard controls
2. **Документація коду:** Кожен .rs файл має містити повний header з інструкціями
3. **Детермінізм:** З першого дня використовуємо fixed-point math (готовність до netcode)

#### Проблеми/Питання:
- [ ] Яку 3D модель манекена використаємо? (Треба знайти або створити в Blender)
- [ ] Чи є Rust вже встановлений на системі?

### Наступні кроки (Сесія 2):
- [ ] Перевірити чи встановлено Rust (`rustc --version`)
- [ ] Якщо ні - встановити через rustup
- [ ] Створити Cargo проект `cargo new arena_combat`
- [ ] Додати залежності до Cargo.toml (wgpu, winit, glam)
- [ ] Базове wgpu вікно (hello triangle)
- [ ] Імпортувати 3D модель манекена

---

### 2025-12-11 (Сесія 2): Початок розробки + Git setup
**Тривалість:** ~1.5 години
**Фаза:** Phase 1 - Setup

#### Виконано:
- ✅ Встановлено Rust 1.92.0 через rustup
- ✅ Створено Cargo проект (arena_combat)
- ✅ Налаштовано Cargo.toml з залежностями:
  - winit 0.30 - window management
  - wgpu 22.0 - graphics API
  - glam 0.29 - math library
  - env_logger, log - logging
- ✅ Створено src/main.rs з повною документацією:
  - Базовий event loop
  - Window 800x600
  - ESC для закриття
- ✅ Налаштовано .gitignore
- ✅ Ініціалізовано Git repository
- ✅ Зроблено перший коміт (commit: 8691df1)
- ✅ Створено документацію:
  - BUILD_SETUP.md - інструкція встановлення Build Tools
  - GITHUB_SETUP.md - підключення до GitHub
  - README.md для GitHub

#### Проблеми/Блокери:
- ⚠️ **Збірка не працює:** Потрібні Microsoft C++ Build Tools для Windows
  ```
  error: linking with `link.exe` failed
  note: you may need to install Visual Studio build tools
  ```
- **Рішення:** Встановити Build Tools for Visual Studio 2022
- **Інструкція:** BUILD_SETUP.md

#### Прийняті рішення:
1. **Git user:** Налаштовано локально для репозиторію
2. **Логування:** Детальні логи ТІЛЬКИ при помилках
3. **Build Tools:** Тільки Build Tools (~5 ГБ), НЕ повна Visual Studio

#### Наступні кроки (Сесія 3):
- [x] Встановити Build Tools ✅
- [x] Перевірити збірку: `cargo build` ✅
- [x] Запустити проект: `cargo run` - побачити вікно! ✅
- [x] Створити GitHub репозиторій ✅
- [x] Запушити код на GitHub ✅

---

### 2025-12-11 (Сесія 2 продовження): GitHub інтеграція
**Тривалість:** +30 хвилин

#### Виконано:
- ✅ Підключено GitHub: https://github.com/zhukotanskiyphilip/arena-combat
- ✅ Створено GitHub інтеграцію:
  - Issue templates (bug report, feature request)
  - Pull request template
  - CONTRIBUTING.md
  - LICENSE (MIT)
- ✅ Оновлено README.md
- ✅ Запушено 3 коміти на GitHub

**Репозиторій тепер містить повну документацію та setup інструкції!**

---

### 2025-12-12 (Сесія 3): Перший успішний запуск! 🎉
**Тривалість:** ~30 хвилин
**Фаза:** Phase 1 - Week 1 - Базове вікно

#### Виконано:
- ✅ Встановлено Microsoft C++ Build Tools for Visual Studio 2022
- ✅ Перша успішна збірка: `cargo build` - 2 хвилини 19 секунд
- ✅ Перший успішний запуск: `cargo run`
- ✅ **Вікно працює!** 800x600, заголовок "Arena Combat Prototype"
- ✅ Закриття через хрестик працює
- ✅ ESC також закриває вікно (як і планувалось)

#### Технічні деталі збірки:
```
Compiling 95 dependencies
Time: 2m 19s
Profile: dev (unoptimized + debuginfo)
Result: SUCCESS ✅
```

#### Проблеми під час сесії:
- ⚠️ Claude Code extension викинув помилку `Error: Claude Code process exited with code 1`
  - **НЕ баг гри** - внутрішня проблема Claude Code extension
  - **Гра працює нормально** - вікно відкривається і закривається коректно

#### Підтвердження функціональності:
- [x] Вікно створюється з правильними розмірами (800x600)
- [x] Заголовок відображається: "Arena Combat Prototype"
- [x] Event loop працює
- [x] Закриття через [X] працює
- [x] ESC закриває програму

#### Статус Phase 1, Week 1:
**Завершено:** ✅ Базове вікно + event loop

#### Наступні кроки (Сесія 4):
- [x] Додати wgpu renderer - очистити екран кольором (напр. темно-синій) ✅
- [x] Додати FPS counter (відображення в заголовку вікна) ✅
- [ ] Перевірити delta time для event loop
- [ ] Додати базовий 3D camera setup

---

### 2025-12-14 (Сесія 4): wgpu Renderer + FPS Counter + Методологія 🎨
**Тривалість:** ~2 години
**Фаза:** Phase 1 - Week 1-2 - Basic Rendering

#### Виконано:
- ✅ **Створено METHODOLOGY.md** - повний документ з правилами AI-assisted розробки:
  - Протокол початку/кінця сесії
  - Стандарти документування коду
  - Правила детермінізму для майбутнього netcode
  - Workflow для роботи над проектом
  - Обов'язковий принцип: "Документуй для майбутнього себе, який нічого не пам'ятає"

- ✅ **Додано wgpu renderer** (`src/rendering/`):
  - Створено модульну структуру: `rendering/mod.rs` + `rendering/renderer.rs`
  - `WgpuRenderer` struct з повною ініціалізацією wgpu:
    - Instance (автовибір Vulkan/DX12/Metal backend)
    - Adapter (Intel Iris Xe Graphics виявлено)
    - Device + Queue
    - Surface configuration (800x600, Fifo VSync, sRGB format)
  - Метод `render()` - очищення екрану темно-синім кольором (RGB: 0.1, 0.2, 0.3)
  - Метод `resize()` - обробка зміни розміру вікна
  - Використано `Arc<Window>` для 'static lifetime surface

- ✅ **Інтегровано renderer в main.rs**:
  - Створення renderer при `resumed()` через `pollster::block_on()`
  - Виклик `render()` в `RedrawRequested` event
  - Обробка помилок: `SurfaceError::Lost`, `OutOfMemory`
  - Обробка `Resized` event

- ✅ **Додано FPS counter** (`src/fps_counter.rs`):
  - Struct `FpsCounter` з circular buffer для усереднення
  - Метод `tick()` - оновлення FPS на основі frame time
  - Метод `fps()` - повертає усереднене значення FPS
  - Метод `frame_time_ms()` - час кадру в мілісекундах
  - Усереднення по 60 кадрам для згладжування

- ✅ **Інтегровано FPS у заголовок вікна**:
  - Оновлення кожні 30 кадрів (зменшення overhead)
  - Формат: "Arena Combat Prototype - 60.0 FPS (16.67ms)"
  - Використано `static mut FRAME_COUNT` для лічильника

- ✅ **Перевірено компіляцію та запуск**:
  - `cargo check` - успішно
  - `cargo build` - успішно (1 warning про unused `size()` метод)
  - `cargo run` - вікно відкривається з темно-синім екраном
  - FPS відображається в заголовку
  - GPU виявлено: Intel(R) Iris(R) Xe Graphics (Vulkan)

#### Технічні деталі:

**Створені файли:**
- `METHODOLOGY.md` - методологія розробки (докладний протокол)
- `src/rendering/mod.rs` - модуль rendering
- `src/rendering/renderer.rs` - WgpuRenderer (270+ рядків з документацією)
- `src/fps_counter.rs` - FPS counter (130+ рядків)

**Змінені файли:**
- `src/main.rs`:
  - Додано `mod rendering;` + `mod fps_counter;`
  - `App` struct: додано `renderer: Option<WgpuRenderer>` + `fps_counter: FpsCounter`
  - `resumed()`: ініціалізація renderer
  - `RedrawRequested`: tick FPS + оновлення title + render()
  - `Resized`: виклик renderer.resize()

**Залежності (без змін):**
- wgpu = "22.1" - вже було в Cargo.toml
- winit = "0.30"
- pollster = "0.4"
- glam = "0.29"

**Структура коду:**
```
arena_combat/
├── src/
│   ├── main.rs               # ✅ Оновлено (renderer + FPS)
│   ├── fps_counter.rs        # ✅ НОВИЙ
│   └── rendering/            # ✅ НОВИЙ
│       ├── mod.rs
│       └── renderer.rs
├── METHODOLOGY.md            # ✅ НОВИЙ
└── PROGRESS.md               # ✅ Оновлено
```

#### Проблеми та рішення:

**Проблема 1:** Lifetime error з `Surface<'static>`
```
error: lifetime may not live long enough
surface: wgpu::Surface<'static> requires '1 must outlive 'static
```
**Рішення:** Використано `Arc<Window>` замість `&Window`:
- `pub async fn new(window: Arc<Window>)`
- `let surface = instance.create_surface(window.clone())`
- Зберігаємо `window: Arc<Window>` в `WgpuRenderer` struct

**Проблема 2:** Unsafe static mut для FRAME_COUNT
**Прийнято:** Допустимо для простого лічильника в single-threaded event loop
**Альтернатива (майбутнє):** Atomic або зберігання в App struct

#### Виміри продуктивності:
- **Compilation time:** ~20 секунд (incremental)
- **GPU backend:** Vulkan (автовибір)
- **Graphics card:** Intel(R) Iris(R) Xe Graphics
- **Clear color performance:** Очікуються тисячі FPS (обмежені VSync до 60)

#### Що працює:
- [x] Вікно 800x600 відкривається
- [x] Екран очищається темно-синім кольором
- [x] FPS відображається в заголовку
- [x] Resize вікна працює коректно
- [x] ESC закриває програму
- [x] Закриття через [X] працює

#### Статус Phase 1, Week 1-2:
**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)

**В процесі:**
- ⏳ 3D camera setup (Наступна сесія)

#### Наступні кроки (Сесія 5):
- [x] Додати базовий 3D camera (perspective projection) ✅
- [x] Створити coordinate system (Y-up, right-handed) ✅
- [ ] Додати camera controls (mouse look - поворот камери)
- [x] Додати grid на підлозі для візуалізації (debug) ✅
- [x] Простий shader для grid ✅

---

### 2025-12-14 (Сесія 5): 3D Camera + Grid Shader + Coordinate System 🎯
**Тривалість:** ~2 години
**Фаза:** Phase 1 - Week 2 - 3D Fundamentals

#### Виконано:
- ✅ **Створено повний 3D camera модуль** (`src/camera/`):
  - `camera/mod.rs` - експорт Camera та CameraUniform
  - `camera/camera.rs` - повна реалізація 3D camera:
    - `Camera` struct з position, target, up, fovy, aspect, znear, zfar
    - Perspective projection з правильним aspect ratio
    - View matrix (look-at transformation)
    - **OpenGL to wgpu coordinate conversion** - критична трансформація для Vulkan/DX12:
      ```rust
      // Конвертуємо з OpenGL координат (Z: -1 to 1) в wgpu (Z: 0 to 1)
      let opengl_to_wgpu = Mat4::from_cols_array(&[
          1.0, 0.0, 0.0, 0.0,
          0.0, 1.0, 0.0, 0.0,
          0.0, 0.0, 0.5, 0.0,
          0.0, 0.0, 0.5, 1.0,
      ]);
      ```
    - View-projection matrix combination
    - `update_aspect()` метод для resize events
  - `CameraUniform` - GPU buffer structure з bytemuck::Pod + Zeroable
  - Використано **glam** для всієї математики (Vec3, Mat4)

- ✅ **Створено Grid shader** (`assets/shaders/grid.wgsl`):
  - WGSL shader для рендерінгу координатної сітки
  - **Vertex shader:**
    - Приймає позицію та колір вершини
    - Трансформує через camera view-projection matrix
    - Передає world position для fade-out ефекту
  - **Fragment shader:**
    - **Distance-based fade-out** - сітка затухає на відстані (alpha зменшується з 0.6 до 0.0)
    - **Center line highlighting** - осі X та Z (0.0) яскравіші (alpha = 0.9)
    - Final color = lerp між base color та white для центральних ліній
  - Bind group @group(0) для camera uniform buffer

- ✅ **Створено Grid рендер систему** (`src/rendering/grid.rs`):
  - `GridVertex` struct - position [f32; 3] + color [f32; 3]
  - Імплементація bytemuck::Pod + Zeroable для GPU
  - `vertex_buffer_layout` descriptor для wgpu
  - `Grid` struct з vertex/index buffers та render pipeline
  - **Mesh generation:**
    - Генерація ліній паралельних X та Z осям
    - Площина Y=0 (XZ plane)
    - Розмір: -size..+size (за замовчуванням 20 units)
    - Колір: світло-сірий [0.5, 0.5, 0.5]
  - **Render pipeline:**
    - Topology: LineList (малюємо лінії, не трикутники)
    - Alpha blending включено для fade-out ефекту
    - Без culling (лінії видимі з обох сторін)
    - Без depth buffer (поки що)

- ✅ **Інтегровано camera та grid в renderer** (`src/rendering/renderer.rs`):
  - **Додано поля в WgpuRenderer:**
    ```rust
    pub camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    grid: Grid,
    ```
  - **Ініціалізація camera:**
    - Position: Vec3::new(0.0, 3.0, 8.0) - трохи вище та назад
    - Target: Vec3::ZERO - дивимось на центр
    - Aspect ratio: width / height
    - FOV: 45 градусів
  - **Створено uniform buffer:**
    - `create_buffer_init()` з CameraUniform
    - Usage: UNIFORM | COPY_DST
  - **Створено bind group layout та bind group:**
    - Binding 0: Camera uniform buffer
    - Visibility: VERTEX shader
  - **Створено grid:**
    - `Grid::new()` з розміром 20 units
    - Передано camera_bind_group_layout
  - **Оновлено render() метод:**
    - Оновлення camera uniform кожен кадр
    - `queue.write_buffer()` для camera_buffer
    - Виклик `grid.render()` в render pass
  - **Оновлено resize() метод:**
    - Виклик `camera.update_aspect()` при зміні розміру

- ✅ **Додано залежність:** bytemuck до Cargo.toml (було пропущено раніше)
  ```toml
  bytemuck = { version = "1.14", features = ["derive"] }
  ```

- ✅ **Оновлено модульну структуру:**
  - `src/main.rs` - додано `mod camera;`
  - `src/rendering/mod.rs` - додано `pub mod grid;` та `pub use grid::Grid;`

- ✅ **Перевірено компіляцію та запуск:**
  - `cargo check` - успішно
  - `cargo build` - успішно (3 warnings про unused методи - це нормально для майбутнього використання)
  - `cargo run` - **3D сітка видима!** ✨

#### Технічні деталі:

**Створені файли:**
- `src/camera/mod.rs` - camera модуль entry point (30 рядків)
- `src/camera/camera.rs` - Camera implementation (200+ рядків з документацією)
- `assets/shaders/grid.wgsl` - Grid WGSL shader (100+ рядків)
- `src/rendering/grid.rs` - Grid mesh generation та rendering (260+ рядків)

**Змінені файли:**
- `src/main.rs` - додано `mod camera;`
- `src/rendering/mod.rs` - експорт Grid
- `src/rendering/renderer.rs` - інтеграція camera та grid (100+ рядків змін)
- `Cargo.toml` - додано bytemuck dependency

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs                  # ✅ Оновлено (camera mod)
│   ├── fps_counter.rs
│   ├── camera/                  # ✅ НОВИЙ
│   │   ├── mod.rs
│   │   └── camera.rs
│   └── rendering/
│       ├── mod.rs               # ✅ Оновлено (Grid export)
│       ├── renderer.rs          # ✅ Оновлено (camera + grid)
│       └── grid.rs              # ✅ НОВИЙ
├── assets/
│   └── shaders/                 # ✅ НОВИЙ
│       └── grid.wgsl            # ✅ НОВИЙ
├── Cargo.toml                   # ✅ Оновлено (bytemuck)
└── PROGRESS.md                  # ✅ Оновлено
```

#### Проблеми та рішення:

**Проблема 1:** Lifetime error з `Surface<'static>`
```
error: lifetime may not live long enough
  --> src\rendering\renderer.rs:138:21
   |
surface: wgpu::Surface<'static> requires '1 must outlive 'static
```
**Рішення:** Використано `Arc<Window>` замість `&Window`:
- Змінено сигнатуру: `pub async fn new(window: Arc<Window>)`
- `instance.create_surface(window.clone())`
- Зберігаємо `window: Arc<Window>` в struct

**Проблема 2:** Unresolved import CameraUniform
```
error[E0432]: unresolved import `crate::camera::CameraUniform`
  --> src\rendering\renderer.rs:68:27
```
**Рішення:** Оновлено `src/camera/mod.rs`:
```rust
pub use camera::{Camera, CameraUniform};  // Було тільки Camera
```

**Проблема 3:** Missing bytemuck dependency
```
error[E0433]: failed to resolve: use of undeclared crate or module `bytemuck`
  --> src\camera\camera.rs:61:10
```
**Рішення:** Додано до Cargo.toml:
```toml
bytemuck = { version = "1.14", features = ["derive"] }
```

#### Математика та координати:

**Coordinate System:**
- **Y-up, right-handed** (OpenGL convention)
- X: вправо
- Y: вгору
- Z: на глядача

**Camera параметри:**
- Position: (0, 3, 8) - 3 units вище підлоги, 8 units назад
- Target: (0, 0, 0) - центр сцени
- FOV: 45° vertical
- Near plane: 0.1
- Far plane: 100.0

**Grid параметри:**
- Розмір: 20x20 units (-10 до +10 по X та Z)
- Інтервал: 1.0 unit між лініями
- Кількість ліній: 41 вертикальних + 41 горизонтальних = 82 лінії
- Кількість вершин: 82 * 2 = 164 vertices

**Projection conversion:**
- OpenGL NDC: X[-1,1], Y[-1,1], Z[-1,1]
- wgpu NDC: X[-1,1], Y[-1,1], Z[0,1] (Vulkan/DirectX style)
- Конверсія через `opengl_to_wgpu` matrix:
  - Z_wgpu = Z_opengl * 0.5 + 0.5

#### Warnings (очікувані):
```
warning: method `position` is never used
  --> src\camera\camera.rs:89:12
   |
warning: method `target` is never used
  --> src\camera\camera.rs:94:12
   |
warning: method `size` is never used
  --> src\rendering\renderer.rs:354:12
```
**Пояснення:** Ці методи будуть використовуватись пізніше (camera controls, UI).
**Дія:** Ігноруємо (це getter методи для майбутнього використання).

#### Що працює:

- [x] 3D camera з perspective projection
- [x] Coordinate system (Y-up, right-handed)
- [x] Grid на підлозі (XZ plane, Y=0)
- [x] Grid shader з fade-out ефектом
- [x] Center lines highlighting (X=0, Z=0)
- [x] View-projection matrix оновлюється кожен кадр
- [x] Resize коректно оновлює aspect ratio
- [x] FPS counter працює (60 FPS з VSync)
- [x] Темно-синій background (arena atmosphere)

#### Візуальний результат:

Тепер при запуску `cargo run` бачимо:
- Темно-синій фон (RGB: 0.1, 0.2, 0.3)
- Координатна сітка 20x20 на підлозі
- Сітка з перспективою (ближче = більша, далі = менша)
- Fade-out ефект на відстані
- Яскравіші центральні лінії (осі X та Z)
- FPS counter в заголовку (~60 FPS)

#### Статус Phase 1, Week 2:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Coordinate system setup (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Grid shader з WGSL (Сесія 5)

**В процесі:**
- ⏳ Camera controls (mouse look) - залишилось на майбутнє

#### Наступні кроки (Сесія 6):

**Option A - Camera Controls:**
- [ ] Додати mouse input handling (MouseMotion event)
- [ ] Реалізувати orbit camera controls (drag to rotate)
- [ ] Додати keyboard controls (WASD для переміщення камери)
- [ ] Опційно: zoom (mouse wheel)

**Option B - 3D Models:**
- [ ] Завантажити простий GLTF model (куб або конус для тестування)
- [ ] Створити vertex/index buffers для mesh
- [ ] Базовий shader для 3D моделі
- [ ] Відрендерити модель на сцені

**Option C - Delta Time:**
- [ ] Додати delta time tracking
- [ ] Підготувати fixed timestep loop (60 FPS)
- [ ] Розділити render FPS від game logic FPS

**Рекомендація:** Почати з Option A (Camera Controls) - це дасть можливість оглядати майбутні 3D моделі з різних кутів.

---

### 2025-12-14 (Сесія 6): Camera Controls - Orbit, Zoom, Pan 🎮
**Тривалість:** ~1 година
**Фаза:** Phase 1 - Week 2 - Interactive Camera

#### Виконано:
- ✅ **Створено InputState систему** (`src/input/`):
  - `input/mod.rs` - модуль entry point
  - `input/input_state.rs` - повна реалізація input tracking:
    - Mouse position tracking (current + previous для delta)
    - Mouse button state (left/right/middle)
    - Keyboard state (HashSet<KeyCode> для швидкого lookup)
    - Convenience methods: `is_w_pressed()`, `is_a_pressed()`, etc.
    - `mouse_delta()` - різниця позицій для camera rotation
    - `reset_mouse_delta()` - скидання після обробки

- ✅ **Додано Orbit Camera** (`src/camera/camera.rs`):
  - `orbit(delta_yaw, delta_pitch)` метод:
    - Spherical coordinates математика
    - Конверсія Cartesian → Spherical → Cartesian
    - **Pitch clamping [-89°, +89°]** - не дає камері перевернутись
    - Yaw необмежений (360° обертання)
  - Формули:
    ```rust
    // Spherical to Cartesian:
    x = r * cos(pitch) * cos(yaw)
    y = r * sin(pitch)
    z = r * cos(pitch) * sin(yaw)
    ```

- ✅ **Додано Zoom** (`src/camera/camera.rs`):
  - `zoom(delta)` метод
  - Mouse wheel handling в main.rs
  - **Обмеження відстані [1.0, 50.0] units**
  - LineDelta: 0.5 units per scroll line
  - PixelDelta: ~50 pixels = 1 unit

- ✅ **Додано Pan (WASD)** (`src/camera/camera.rs`):
  - `pan(offset)` метод - переміщує і camera і target
  - W/S - forward/backward (проекція на XZ plane)
  - A/D - left/right (camera right vector)
  - **Pan speed: 0.1 units per frame**

- ✅ **Інтегровано в main.rs**:
  - Input events handling:
    - `CursorMoved` → update mouse position
    - `MouseInput` → update button state
    - `MouseWheel` → zoom camera
    - `KeyboardInput` → update key state + ESC handling
  - Camera update loop в `RedrawRequested`:
    - Orbit при mouse_left + drag
    - Pan при WASD pressed
    - Reset mouse delta після обробки

#### Технічні деталі:

**Створені файли:**
- `src/input/mod.rs` - input модуль (25 рядків)
- `src/input/input_state.rs` - InputState struct (240+ рядків)

**Змінені файли:**
- `src/camera/camera.rs` - додано orbit(), zoom(), pan() методи (+100 рядків)
- `src/main.rs` - input handling та camera update loop (+80 рядків)

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs                  # ✅ Оновлено (input + camera update)
│   ├── fps_counter.rs
│   ├── input/                   # ✅ НОВИЙ
│   │   ├── mod.rs
│   │   └── input_state.rs
│   ├── camera/
│   │   ├── mod.rs
│   │   └── camera.rs            # ✅ Оновлено (orbit/zoom/pan)
│   └── rendering/
│       ├── mod.rs
│       ├── renderer.rs
│       └── grid.rs
└── PROGRESS.md                  # ✅ Оновлено
```

#### Математика Orbit Camera:

**Spherical Coordinates:**
- `radius` = відстань від target до camera
- `yaw` = кут в XZ plane (горизонтальне обертання)
- `pitch` = кут від XZ plane (вертикальне обертання)

**Конверсія:**
```
Cartesian → Spherical:
  yaw = atan2(z, x)
  pitch = asin(y / radius)

Spherical → Cartesian:
  x = r * cos(pitch) * cos(yaw)
  y = r * sin(pitch)
  z = r * cos(pitch) * sin(yaw)
```

**Sensitivity:**
- 0.005 радіан/піксель (~0.3°/піксель)
- Інвертовані delta для інтуїтивного руху

#### Controls Summary:

| Input | Action | Details |
|-------|--------|---------|
| Left Mouse + Drag | Orbit | Обертання навколо target |
| Mouse Wheel | Zoom | Відстань 1.0 - 50.0 units |
| W | Pan Forward | В напрямку погляду (XZ plane) |
| S | Pan Backward | Назад |
| A | Pan Left | Вліво |
| D | Pan Right | Вправо |
| ESC | Exit | Закрити програму |

#### Warnings (очікувані):
```
warning: unused import: `grid::Grid`
warning: unused import: `PhysicalKey` (в input_state.rs)
warning: unused import: `MouseButton` (в main.rs)
warning: methods `mouse_position`, `is_space_pressed`, `is_shift_pressed`, `is_ctrl_pressed` are never used
```
**Пояснення:** Методи для майбутнього використання (Space = jump, Shift = sprint).

#### Що працює:

- [x] Orbit camera (mouse drag)
- [x] Zoom (mouse wheel)
- [x] Pan (WASD)
- [x] Pitch clamping (не перевертається)
- [x] Distance limits (1.0 - 50.0)
- [x] Smooth movement
- [x] FPS залишається стабільним (~60)

#### Статус Phase 1, Week 2:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ **Camera controls - orbit, zoom, pan (Сесія 6)** ✨

#### Наступні кроки (Сесія 7):

**Option A - 3D Models:**
- [ ] Завантажити простий GLTF model (куб для тестування)
- [ ] Створити mesh rendering pipeline
- [ ] Базовий shader для 3D моделі
- [ ] Відрендерити модель на сцені

**Option B - Delta Time + Fixed Timestep:**
- [ ] Додати delta time tracking
- [ ] Підготувати fixed timestep loop (60 FPS physics)
- [ ] Розділити render FPS від game logic FPS

**Option C - Basic Lighting:**
- [ ] Додати directional light
- [ ] Простий diffuse shading
- [ ] Normal vectors для mesh

**Рекомендація:** Option A (3D Models) - потрібен об'єкт на сцені для подальшої роботи над combat системою.

---

### 2025-12-14 (Сесія 7): 3D Mesh Rendering + Cube + Depth Buffer 📦
**Тривалість:** ~45 хвилин
**Фаза:** Phase 1 - Week 2 - 3D Objects

#### Виконано:
- ✅ **Створено mesh rendering систему** (`src/rendering/mesh.rs`):
  - `MeshVertex` struct (position + normal + color)
  - `generate_cube()` функція:
    - 24 вершини (4 на грань для різних нормалей)
    - 36 індексів (6 граней × 2 трикутники)
    - CCW winding order
    - Нормалі направлені назовні
  - `Mesh` struct з vertex/index buffers та render pipeline
  - Indexed drawing з depth stencil

- ✅ **Створено mesh shader** (`assets/shaders/mesh.wgsl`):
  - Vertex shader: transform position через view-projection
  - Fragment shader з базовим diffuse освітленням:
    - Directional light (0.5, 1.0, 0.3) - зверху-спереду-справа
    - Ambient: 0.3 (щоб тіні не були повністю чорними)
    - Diffuse: dot(N, L) Lambert lighting
  - `lighting = ambient + diffuse`

- ✅ **Додано Depth Buffer** (`src/rendering/renderer.rs`):
  - `create_depth_texture()` функція
  - Format: Depth32Float
  - Оновлюється при resize
  - Використовується в render pass

- ✅ **Оновлено Grid pipeline**:
  - Додано depth_stencil state (раніше було None)
  - Тепер grid правильно взаємодіє з 3D об'єктами

- ✅ **Інтегровано куб в renderer**:
  - Червонуватий куб 1x1x1 в центрі сцени
  - Колір: [0.8, 0.3, 0.3]
  - Позиція: центр (0, 0, 0), нижня грань на Y=−0.5

#### Технічні деталі:

**Створені файли:**
- `src/rendering/mesh.rs` - Mesh система (260+ рядків)
- `assets/shaders/mesh.wgsl` - Mesh shader з освітленням (90+ рядків)

**Змінені файли:**
- `src/rendering/mod.rs` - експорт mesh компонентів
- `src/rendering/renderer.rs` - depth buffer + cube integration
- `src/rendering/grid.rs` - додано depth_stencil state

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs
│   ├── fps_counter.rs
│   ├── input/
│   │   ├── mod.rs
│   │   └── input_state.rs
│   ├── camera/
│   │   ├── mod.rs
│   │   └── camera.rs
│   └── rendering/
│       ├── mod.rs              # ✅ Оновлено (mesh exports)
│       ├── renderer.rs         # ✅ Оновлено (depth + cube)
│       ├── grid.rs             # ✅ Оновлено (depth_stencil)
│       └── mesh.rs             # ✅ НОВИЙ
├── assets/
│   └── shaders/
│       ├── grid.wgsl
│       └── mesh.wgsl           # ✅ НОВИЙ
└── PROGRESS.md                 # ✅ Оновлено
```

#### Cube Geometry:

**Vertices (24 total, 4 per face):**
```
Front (Z+):  4 vertices, normal [0, 0, 1]
Back (Z-):   4 vertices, normal [0, 0, -1]
Top (Y+):    4 vertices, normal [0, 1, 0]
Bottom (Y-): 4 vertices, normal [0, -1, 0]
Right (X+):  4 vertices, normal [1, 0, 0]
Left (X-):   4 vertices, normal [-1, 0, 0]
```

**Indices (36 total, 6 per face):**
- 2 трикутники на грань
- CCW winding для front face

#### Lighting Model:

```wgsl
// Directional light
let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));

// Ambient + Diffuse
let ambient = 0.3;
let diffuse = max(dot(normal, light_dir), 0.0);
let lighting = min(ambient + diffuse, 1.0);

// Final color
final_color = input.color * lighting;
```

#### Проблема та рішення:

**Проблема:** Grid pipeline не мав depth_stencil, але render pass використовував depth buffer
```
Render pipeline targets are incompatible with render pass
Incompatible depth-stencil attachment format:
  RenderPass uses Depth32Float, RenderPipeline uses None
```

**Рішення:** Додали depth_stencil state до Grid pipeline:
```rust
depth_stencil: Some(wgpu::DepthStencilState {
    format: wgpu::TextureFormat::Depth32Float,
    depth_write_enabled: true,
    depth_compare: wgpu::CompareFunction::Less,
    ...
})
```

#### Що працює:

- [x] Червоний куб рендериться на сцені
- [x] Базове diffuse освітлення (видно грані)
- [x] Depth buffer (правильний z-ordering)
- [x] Grid залишається видимим
- [x] Camera controls працюють з кубом
- [x] Resize працює (depth texture оновлюється)
- [x] FPS стабільний (~60)

#### Візуальний результат:

Тепер при запуску `cargo run` бачимо:
- Темно-синій фон
- Координатна сітка 20x20 на підлозі
- **Червонуватий куб 1x1x1 в центрі** ✨
- Куб освітлений зверху-спереду
- Різні грані мають різну яскравість (lighting)
- Можна обертати камеру і бачити куб з різних сторін

#### Статус Phase 1, Week 2-3:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Camera controls - orbit, zoom, pan (Сесія 6)
- ✅ **3D Mesh rendering + Cube + Depth Buffer (Сесія 7)** ✨

#### Наступні кроки (Сесія 8):

**Option A - Transform System:**
- [ ] Додати Model matrix (position, rotation, scale)
- [ ] Model uniform buffer
- [ ] Можливість переміщати/обертати об'єкти

**Option B - Multiple Objects:**
- [ ] Рендеринг декількох кубів
- [ ] Різні позиції та кольори
- [ ] Instance rendering (опційно)

**Option C - Delta Time + Game Loop:**
- [ ] Delta time tracking
- [ ] Fixed timestep для physics
- [ ] Розділення render/update

**Рекомендація:** Option A (Transform System) - потрібна можливість позиціонувати об'єкти для подальшої роботи над гравцем та AI.

---

### 2025-12-14 (Сесія 8): Transform System + Multiple Objects 🎯
**Тривалість:** ~30 хвилин
**Фаза:** Phase 1 - Week 3 - Transform & Positioning

#### Виконано:
- ✅ **Створено Transform модуль** (`src/transform/`):
  - `transform/mod.rs` - модуль entry point
  - `transform/transform.rs` - повна реалізація:
    - `Transform` struct (position, rotation, scale)
    - `model_matrix()` - обчислення Model matrix (S*R*T order)
    - Quaternion rotation (уникає gimbal lock)
    - Helper методи: `rotate()`, `translate()`, `forward()`, `right()`, `up()`
    - `set_rotation_euler()` - встановлення обертання через кути
  - `TransformUniform` - GPU buffer structure:
    - Model matrix (4x4)
    - Normal matrix (3x3 для коректної трансформації нормалей)
    - Proper padding для GPU alignment

- ✅ **Оновлено mesh shader** (`assets/shaders/mesh.wgsl`):
  - Додано `TransformUniform` struct в shader
  - group(1) binding(0) для transform uniform
  - Vertex shader тепер трансформує через Model matrix:
    ```wgsl
    let world_position = transform.model * vec4<f32>(input.position, 1.0);
    output.clip_position = camera.view_proj * world_position;
    ```
  - Normal matrix для коректного освітлення при scale/rotation

- ✅ **Оновлено Mesh** (`src/rendering/mesh.rs`):
  - Mesh тепер містить Transform та TransformUniform
  - `new()` приймає Transform параметр
  - Transform bind group (group 1)
  - `update_transform()` метод для оновлення GPU buffer
  - Pipeline layout з двома bind group layouts

- ✅ **Створено множинні куби** (`src/rendering/renderer.rs`):
  - `cubes: Vec<Mesh>` замість одного cube
  - 4 куби з різними позиціями та кольорами:
    - Червоний куб (0, 0.5, 0) - центр
    - Зелений куб (-3, 0.5, 0) - зліва
    - Синій куб (3, 0.5, 0) - справа
    - Жовтий куб (0, 0.75, -4) - позаду, більший (1.5x)
  - Кожен куб має свій Transform uniform

#### Технічні деталі:

**Створені файли:**
- `src/transform/mod.rs` - transform модуль (30 рядків)
- `src/transform/transform.rs` - Transform struct (180+ рядків)

**Змінені файли:**
- `src/main.rs` - додано `mod transform;`
- `src/rendering/mesh.rs` - Transform integration
- `src/rendering/renderer.rs` - multiple cubes
- `assets/shaders/mesh.wgsl` - Model matrix support

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs                  # ✅ Оновлено (transform mod)
│   ├── fps_counter.rs
│   ├── input/
│   │   ├── mod.rs
│   │   └── input_state.rs
│   ├── camera/
│   │   ├── mod.rs
│   │   └── camera.rs
│   ├── transform/               # ✅ НОВИЙ
│   │   ├── mod.rs
│   │   └── transform.rs
│   └── rendering/
│       ├── mod.rs
│       ├── renderer.rs          # ✅ Оновлено (multiple cubes)
│       ├── grid.rs
│       └── mesh.rs              # ✅ Оновлено (Transform)
├── assets/
│   └── shaders/
│       ├── grid.wgsl
│       └── mesh.wgsl            # ✅ Оновлено (Model matrix)
└── PROGRESS.md                  # ✅ Оновлено
```

#### Transform Math:

**Model Matrix = T * R * S:**
```rust
Mat4::from_scale_rotation_translation(scale, rotation, position)
```

**Normal Matrix:**
- `transpose(inverse(model))` для коректної трансформації нормалей
- Критично для non-uniform scale

**Quaternion Rotation:**
- Використовуємо `glam::Quat`
- Уникаємо gimbal lock
- `from_euler(YXZ, yaw, pitch, roll)` для зручності

#### Cubes Configuration:

| Cube | Position | Size | Color (RGB) |
|------|----------|------|-------------|
| Center | (0, 0.5, 0) | 1.0 | (0.8, 0.3, 0.3) Red |
| Left | (-3, 0.5, 0) | 1.0 | (0.3, 0.8, 0.3) Green |
| Right | (3, 0.5, 0) | 1.0 | (0.3, 0.3, 0.8) Blue |
| Back | (0, 0.75, -4) | 1.5 | (0.9, 0.8, 0.2) Yellow |

#### Що працює:

- [x] Transform system (position, rotation, scale)
- [x] Model matrix обчислення
- [x] Normal matrix для освітлення
- [x] Множинні об'єкти на сцені
- [x] Кожен об'єкт має свій Transform
- [x] Camera controls працюють з усіма об'єктами
- [x] Освітлення правильне на всіх кубах
- [x] FPS стабільний (~60)

#### Візуальний результат:

Тепер при запуску `cargo run` бачимо:
- Темно-синій фон
- Координатна сітка 20x20
- **4 кольорових куби на різних позиціях** ✨
- Освітлення працює на кожному кубі
- Можна обертати камеру навколо всієї сцени

#### Статус Phase 1, Week 3:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Camera controls - orbit, zoom, pan (Сесія 6)
- ✅ 3D Mesh rendering + Cube + Depth Buffer (Сесія 7)
- ✅ **Transform System + Multiple Objects (Сесія 8)** ✨

#### Наступні кроки (Сесія 9):

**Option A - Delta Time + Animation:**
- [ ] Delta time tracking
- [ ] Fixed timestep loop
- [ ] Анімація обертання куба

**Option B - GLTF Loading:**
- [ ] Завантаження .glb моделей
- [ ] Парсинг vertex/index data
- [ ] Текстури (опційно)

**Option C - Player Character:**
- [ ] Базовий манекен з примітивів
- [ ] Player movement (WASD)
- [ ] Camera слідує за гравцем

**Рекомендація:** Option A (Delta Time) - потрібен для анімацій та gameloop separation.

---

### 2025-12-14 (Сесія 9): Delta Time + Cube Animation 🎬
**Тривалість:** ~20 хвилин
**Фаза:** Phase 1 - Week 3 - Animation & Time

#### Виконано:
- ✅ **Створено time модуль** (`src/time/`):
  - `time/mod.rs` - модуль entry point
  - `time/game_time.rs` - GameTime struct:
    - Delta time tracking (час між кадрами)
    - Total elapsed time
    - Frame counter
    - Delta clamping (max 100ms для уникнення physics explosions)
    - Методи: `update()`, `delta()`, `total()`, `frame_count()`

- ✅ **Інтегровано GameTime в main loop**:
  - `game_time.update()` викликається на початку кожного кадру
  - `game_time.frame_count()` замість unsafe static FRAME_COUNT
  - Delta time передається в `renderer.update_animations()`

- ✅ **Додано анімацію обертання кубів**:
  - `WgpuRenderer::update_animations(delta)` метод
  - Кожен куб обертається з різною швидкістю:
    - Червоний: 1.0 рад/с (~57°/с)
    - Зелений: -0.7 рад/с (протилежний напрямок)
    - Синій: 1.5 рад/с (швидше)
    - Жовтий: 0.3 рад/с (повільно)
  - `cube.transform.rotate()` для обертання
  - `cube.update_transform()` для оновлення GPU buffer

#### Технічні деталі:

**Створені файли:**
- `src/time/mod.rs` - time модуль (25 рядків)
- `src/time/game_time.rs` - GameTime struct (120+ рядків)

**Змінені файли:**
- `src/main.rs` - GameTime integration
- `src/rendering/renderer.rs` - update_animations() метод

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs                  # ✅ Оновлено (GameTime)
│   ├── fps_counter.rs
│   ├── input/
│   │   ├── mod.rs
│   │   └── input_state.rs
│   ├── camera/
│   │   ├── mod.rs
│   │   └── camera.rs
│   ├── transform/
│   │   ├── mod.rs
│   │   └── transform.rs
│   ├── time/                    # ✅ НОВИЙ
│   │   ├── mod.rs
│   │   └── game_time.rs
│   └── rendering/
│       ├── mod.rs
│       ├── renderer.rs          # ✅ Оновлено (animations)
│       ├── grid.rs
│       └── mesh.rs
└── PROGRESS.md                  # ✅ Оновлено
```

#### Delta Time Math:

**Frame-rate independence:**
```rust
// Рух зі швидкістю 5 units/second (незалежно від FPS)
position += velocity * speed * delta;

// При 60 FPS: delta ≈ 0.0167s
// При 30 FPS: delta ≈ 0.0333s
// Результат однаковий за секунду!
```

**Delta clamping:**
```rust
// Якщо гра лагає (наприклад, 500ms між кадрами)
// Обмежуємо до 100ms щоб уникнути physics explosions
self.delta_time = raw_delta.min(0.1);
```

#### Rotation Animation:

| Cube | Speed (rad/s) | Direction | Full rotation |
|------|---------------|-----------|---------------|
| Red | 1.0 | CW | ~6.28s |
| Green | 0.7 | CCW | ~9.0s |
| Blue | 1.5 | CW | ~4.2s |
| Yellow | 0.3 | CW | ~21s |

#### Що працює:

- [x] Delta time tracking
- [x] Frame-rate independent animation
- [x] Куби обертаються з різними швидкостями
- [x] Обертання в різних напрямках
- [x] Transform GPU buffer оновлюється кожен кадр
- [x] FPS стабільний (~60)
- [x] Camera controls працюють під час анімації

#### Візуальний результат:

Тепер при запуску `cargo run` бачимо:
- Темно-синій фон
- Координатна сітка 20x20
- **4 куби що ОБЕРТАЮТЬСЯ!** 🎬
- Кожен куб обертається з різною швидкістю
- Освітлення динамічно змінюється при обертанні
- Camera controls працюють одночасно

#### Статус Phase 1, Week 3:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Camera controls - orbit, zoom, pan (Сесія 6)
- ✅ 3D Mesh rendering + Cube + Depth Buffer (Сесія 7)
- ✅ Transform System + Multiple Objects (Сесія 8)
- ✅ **Delta Time + Cube Animation (Сесія 9)** 🎬

#### Наступні кроки (Сесія 10):

**Option A - Player Character:**
- [ ] Створити манекен з примітивів (капсула + куби)
- [ ] Player movement (WASD переміщує персонажа)
- [ ] Camera слідує за гравцем

**Option B - GLTF Loading:**
- [ ] Завантаження .glb моделей
- [ ] Парсинг vertex/index data
- [ ] Текстури (опційно)

**Option C - Fixed Timestep:**
- [ ] Розділення render/update loops
- [ ] 60 FPS fixed physics timestep
- [ ] Інтерполяція для render

**Рекомендація:** Option A (Player Character) - час рухатись від тестових кубів до гравця.

---

### 2025-12-14 (Сесія 10): Player Character + Movement 🎮
**Тривалість:** ~30 хвилин
**Фаза:** Phase 1 - Week 3-4 - Player Character

#### Виконано:
- ✅ **Створено Player модуль** (`src/player/`):
  - `player/mod.rs` - модуль entry point
  - `player/player.rs` - Player struct:
    - Position (Vec3 в world space)
    - Yaw (кут повороту навколо Y)
    - Movement speed (5 units/second)
    - Turn speed (3 rad/second)
    - Методи: `forward()`, `right()`, `move_forward()`, `strafe()`, `turn()`, `update()`
    - Frame-rate independent movement через delta time

- ✅ **Створено mesh примітиви** (`src/rendering/mesh.rs`):
  - `generate_cylinder()` - циліндр вздовж Y-осі
  - `generate_sphere()` - сфера з параметричним tessellation
  - `generate_player_mannequin()` - капсулоподібна фігура гравця:
    - Тіло: циліндр (radius=0.3, height=1.5)
    - Голова: сфера (radius=0.25) на верху
    - Body color: синій [0.2, 0.6, 0.9]
    - Head color: тілесний [0.9, 0.8, 0.7]

- ✅ **Інтегровано player в renderer** (`src/rendering/renderer.rs`):
  - `player_mesh: Mesh` - mesh для візуалізації гравця
  - `update_player(player)` - оновлення позиції та обертання mesh
  - Player рендериться разом з кубами та grid

- ✅ **Додано player movement в main.rs**:
  - W/S - рух вперед/назад
  - A/D - strafe вліво/вправо
  - Q/E - поворот вліво/вправо
  - Camera слідує за гравцем (offset: 0, 5, 10)

- ✅ **Оновлено InputState** (`src/input/input_state.rs`):
  - Додано `is_q_pressed()` та `is_e_pressed()` для повороту

#### Технічні деталі:

**Створені файли:**
- `src/player/mod.rs` - player модуль (25 рядків)
- `src/player/player.rs` - Player struct (120+ рядків)

**Змінені файли:**
- `src/main.rs` - player integration, movement logic
- `src/rendering/renderer.rs` - player_mesh, update_player()
- `src/rendering/mesh.rs` - cylinder, sphere, mannequin generators
- `src/input/input_state.rs` - Q/E key methods

**Структура коду після сесії:**
```
arena_combat/
├── src/
│   ├── main.rs                  # ✅ Оновлено (player)
│   ├── fps_counter.rs
│   ├── input/
│   │   ├── mod.rs
│   │   └── input_state.rs       # ✅ Оновлено (Q/E)
│   ├── camera/
│   │   ├── mod.rs
│   │   └── camera.rs
│   ├── transform/
│   │   ├── mod.rs
│   │   └── transform.rs
│   ├── time/
│   │   ├── mod.rs
│   │   └── game_time.rs
│   ├── player/                  # ✅ НОВИЙ
│   │   ├── mod.rs
│   │   └── player.rs
│   └── rendering/
│       ├── mod.rs
│       ├── renderer.rs          # ✅ Оновлено (player_mesh)
│       ├── grid.rs
│       └── mesh.rs              # ✅ Оновлено (primitives)
└── PROGRESS.md                  # ✅ Оновлено
```

#### Player Movement Math:

**Forward vector (based on yaw):**
```rust
// yaw=0 → дивиться в -Z
// Обертання навколо Y
forward = Vec3::new(-sin(yaw), 0.0, -cos(yaw))
right = Vec3::new(cos(yaw), 0.0, -sin(yaw))
```

**Frame-rate independent:**
```rust
// Position change = direction * speed * delta
position += forward * amount * move_speed * delta;
```

#### Controls Summary:

| Input | Action | Details |
|-------|--------|---------|
| W | Move Forward | Player forward direction |
| S | Move Backward | Player backward |
| A | Strafe Left | Perpendicular to forward |
| D | Strafe Right | Perpendicular to forward |
| Q | Turn Left | Rotate player CCW |
| E | Turn Right | Rotate player CW |
| Left Mouse + Drag | Orbit Camera | Обертання камери |
| Mouse Wheel | Zoom | Відстань камери |

#### Що працює:

- [x] Player mannequin рендериться
- [x] WASD рух працює (frame-rate independent)
- [x] Q/E поворот гравця
- [x] Camera слідує за гравцем
- [x] Куби продовжують обертатися
- [x] FPS стабільний (~60)

#### Візуальний результат:

Тепер при запуску `cargo run` бачимо:
- Темно-синій фон
- Координатна сітка 20x20
- 4 куби що обертаються
- **Синій манекен гравця** 🎮
- Манекен рухається по WASD
- Манекен повертається по Q/E
- Camera слідує за гравцем

#### Статус Phase 1, Week 3-4:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Camera controls - orbit, zoom, pan (Сесія 6)
- ✅ 3D Mesh rendering + Cube + Depth Buffer (Сесія 7)
- ✅ Transform System + Multiple Objects (Сесія 8)
- ✅ Delta Time + Cube Animation (Сесія 9)
- ✅ **Player Character + Movement (Сесія 10)** 🎮

#### Наступні кроки (Сесія 11):

**Option A - Combat System Basics:**
- [ ] Attack direction (mouse → напрямок удару)
- [ ] Basic attack animation (swing)
- [ ] Hitbox system

**Option B - Third Person Camera:**
- [ ] Camera за спиною гравця
- [ ] Mouse look впливає на камеру
- [ ] Player повертається разом з камерою

**Option C - Collision Detection:**
- [ ] Player-cube collision
- [ ] Basic physics (не проходити крізь об'єкти)
- [ ] Ground collision

**Рекомендація:** Option B (Third Person Camera) - для combat потрібен кращий camera control.

---

### 2025-12-14 (Сесія 11): Third Person Camera 🎥
**Тривалість:** ~30 хвилин
**Фаза:** Phase 1 - Week 4 - Camera System

#### Виконано:
- ✅ **Third Person Camera система** (`src/camera/camera.rs`):
  - Додано yaw/pitch/distance поля для spherical coordinates
  - `update_third_person(target_pos, height)` - камера слідує за гравцем
  - `rotate_third_person(delta_yaw, delta_pitch)` - mouse look
  - `zoom_third_person(delta)` - zoom 2-20 units
  - `forward_xz()` / `right_xz()` - camera directions для руху гравця
  - Pitch clamping: -30° до +85° (не перевертається)

- ✅ **Camera-relative movement** (`src/main.rs`):
  - WASD рух тепер відносний до камери
  - W = вперед куди дивиться камера
  - Player автоматично повертається в напрямку руху
  - Нормалізація діагонального руху

- ✅ **Mouse look controls**:
  - Права кнопка миші + drag = обертання камери
  - Mouse wheel = zoom
  - Sensitivity: 0.003 rad/pixel

#### Технічні деталі:

**Змінені файли:**
- `src/camera/camera.rs` - third person camera methods (+90 рядків)
- `src/main.rs` - camera-relative movement, mouse look

**Camera Math:**
```rust
// Camera offset від target
offset = Vec3::new(
    distance * pitch.cos() * yaw.cos(),
    distance * pitch.sin(),
    distance * pitch.cos() * yaw.sin(),
);
camera.position = target + offset;

// Forward direction (куди дивиться камера)
forward_xz = Vec3::new(-yaw.cos(), 0.0, -yaw.sin());
```

#### Controls Summary:

| Input | Action | Details |
|-------|--------|---------|
| W | Move Forward | Куди дивиться камера |
| S | Move Backward | Протилежно камері |
| A | Strafe Left | Відносно камери |
| D | Strafe Right | Відносно камери |
| Right Mouse + Drag | Rotate Camera | Обертання навколо гравця |
| Mouse Wheel | Zoom | 2-20 units distance |
| ESC | Exit | Закрити програму |

#### Що працює:

- [x] Third person camera слідує за гравцем
- [x] Mouse look (права кнопка)
- [x] Camera-relative WASD movement
- [x] Player auto-rotate в напрямку руху
- [x] Zoom (mouse wheel)
- [x] Pitch clamping (камера не перевертається)
- [x] FPS стабільний (~60)

#### Статус Phase 1, Week 4:

**Завершено:**
- ✅ Базове вікно + event loop (Сесія 3)
- ✅ wgpu renderer + clear color (Сесія 4)
- ✅ FPS counter (Сесія 4)
- ✅ 3D camera з perspective projection (Сесія 5)
- ✅ Grid visualization (Сесія 5)
- ✅ Camera controls - orbit, zoom, pan (Сесія 6)
- ✅ 3D Mesh rendering + Cube + Depth Buffer (Сесія 7)
- ✅ Transform System + Multiple Objects (Сесія 8)
- ✅ Delta Time + Cube Animation (Сесія 9)
- ✅ Player Character + Movement (Сесія 10)
- ✅ **Third Person Camera (Сесія 11)** 🎥

#### Наступні кроки (Сесія 12):

**Option A - Combat System Basics:**
- [ ] Attack input (mouse click → атака)
- [ ] Attack direction (куди дивиться гравець)
- [ ] Basic hitbox system
- [ ] Attack cooldown

**Option B - Collision Detection:**
- [ ] Player-cube collision
- [ ] Basic physics (не проходити крізь об'єкти)
- [ ] Ground collision

**Option C - Animation System:**
- [ ] Keyframe animation структура
- [ ] Walk/Idle animation blending
- [ ] Attack animation

**Рекомендація:** Option A (Combat System) - основна мета проекту.

---

## 💡 Ключові концепції проекту

### Філософія бою (з GDD):
> "Меч веде руку, не анімація веде гравця"

**П'ять стовпів:**
1. **Directional Input** - напрямок атаки = рух миші
2. **Fluid Movement** - рух під час атаки
3. **Low Animation Commitment** - можна скасувати дії
4. **Weight & Impact** - кожен удар відчувається
5. **Readable Combat** - зрозуміло що відбувається

### Технічні принципи:
- **Детермінізм** - готуємось до netcode з дня 1
- **Fixed timestep** - 60 FPS симуляція
- **Separation of concerns** - логіка окремо від рендеру
- **Data-driven** - налаштування в конфігах, не в коді

---

## 🔧 Технічний стек (фінальний)

```toml
[dependencies]
# Core
wgpu = "0.18"                    # Graphics API
winit = "0.29"                   # Window + Input
glam = "0.24"                    # Math (vectors, matrices)

# Physics
parry3d = "0.13"                 # Collision detection

# Assets
gltf = "1.4"                     # 3D model loading

# Audio
rodio = "0.17"                   # Sound playback

# Networking (Phase 2)
quinn = "0.10"                   # UDP/QUIC
# або laminar = "0.5"

# Utils
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"                  # Serialization
fixed = "1.24"                   # Fixed-point math
```

---

## 📂 Структура проекту (планована)

```
arena_combat/
├── Cargo.toml
├── README.md
├── docs/                        # Документація
│   ├── arena_combat_gdd.md
│   ├── tech_stack_decision.md
│   ├── PROGRESS.md
│   └── ...
│
├── assets/                      # Ресурси
│   ├── models/
│   │   ├── mannequin.glb
│   │   └── weapons/
│   ├── sounds/
│   └── textures/
│
└── src/                         # Код
    ├── main.rs                  # Entry point
    ├── core/                    # Game logic
    │   ├── state.rs
    │   ├── combat.rs
    │   └── physics.rs
    ├── ai/                      # AI opponent
    │   └── behavior.rs
    ├── rendering/               # Graphics
    │   ├── renderer.rs
    │   └── shaders/
    ├── input/                   # Controls
    │   ├── mouse.rs
    │   └── keyboard.rs
    ├── audio/
    └── network/                 # Phase 2
```

---

## 🎮 AI Opponent Design

### Рівні складності:

| Level | Reaction Time | Parry Rate | Behavior |
|-------|---------------|------------|----------|
| Easy | 500ms | 10% | Random attacks |
| Medium | 300ms | 30% | Pattern recognition (basic) |
| Hard | 150ms | 60% | Counters combos |
| Master | 100ms | 80% | Reads all moves |

### AI Decision Tree:
```
1. Аналіз ситуації (відстань, stamina, стан гравця)
2. Якщо гравець атакує → блокувати/парирувати
3. Якщо гравець відкритий → атакувати
4. Якщо далеко → наближатись
5. Якщо мало stamina → відступати
```

---

## 🚧 Поточні завдання

### TODO (найближчі):
1. ⬜ Встановити Rust toolchain
2. ⬜ Створити Cargo проект `cargo new arena_combat`
3. ⬜ Додати залежності (wgpu, winit, glam)
4. ⬜ Hello triangle (базове wgpu вікно)
5. ⬜ Імпортувати 3D модель манекена

### В процесі:
- Документація (цей файл)

### Завершено:
- ✅ Технічні рішення
- ✅ Вибір мови програмування
- ✅ План розробки

---

## 📝 Нотатки для майбутніх сесій

### Важливо пам'ятати:
1. **Детермінізм з дня 1** - використовуємо fixed-point math, не float
2. **Розділення логіки і рендеру** - core/ не знає про rendering/
3. **60 FPS фіксований timestep** - для передбачуваності
4. **AI повинен бути fun, не perfect** - навіть Hard AI має робити помилки

### Питання для вирішення:
- [ ] Яку 3D модель манекена використаємо? (Blender? Asset pack?)
- [ ] Формат аудіо файлів? (OGG? WAV?)
- [ ] Як візуалізувати напрямок атаки? (Debug arrows спочатку)

### Ресурси:
- Rust Book: https://doc.rust-lang.org/book/
- wgpu Tutorial: https://sotrh.github.io/learn-wgpu/
- Collision detection: https://parry.rs/docs/

---

## 🔄 Як оновлювати цей документ

Після кожної сесії:
1. Оновити дату "Останнє оновлення"
2. Додати до Timeline що було зроблено
3. Оновити TODO списки
4. Додати нові рішення до "Прийняті рішення"
5. Записати питання/проблеми в "Нотатки"

---

**Наступна сесія почне з читання цього файлу!**
