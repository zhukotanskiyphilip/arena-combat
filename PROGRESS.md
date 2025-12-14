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
