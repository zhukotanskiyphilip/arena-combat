# Розуміння 3D простору, обертання та руху тіл

## Джерела
- [Coordinate Systems - Scratchapixel](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/coordinate-systems.html)
- [3D Math Primer for Game Programmers](https://www.3dgep.com/3d-math-primer-for-game-programmers/)
- [Rotation in Three Dimensions - gamemath.com](https://gamemath.com/book/orient.html)
- [Understanding Quaternions](https://www.3dgep.com/understanding-quaternions/)
- [OpenGL Tutorial - Matrices](http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/)
- [NASA - Aircraft Rotations](https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/aircraft-rotations/)
- [Unit Circle - Mathematics LibreTexts](https://math.libretexts.org/Bookshelves/Precalculus/Precalculus_1e_(OpenStax)/05:_Trigonometric_Functions/5.02:_Unit_Circle_-_Sine_and_Cosine_Functions)

---

## 1. Система координат

### 1.1 Базові визначення

**Простір** - це математична модель, в якій об'єкти мають позицію. 3D простір має три виміри: ширина, висота, глибина.

**Координати** - це числа, що описують позицію точки в просторі. В 3D це три числа: (x, y, z).

**Осі координат** - це три перпендикулярні лінії, що перетинаються в точці початку координат (0, 0, 0):
- **X** - горизонтальна вісь (ліво-право)
- **Y** - вертикальна вісь (верх-низ)
- **Z** - вісь глибини (вперед-назад)

### 1.2 Права vs Ліва система координат

**Як визначити?** Використай руку:
- Великий палець → напрямок +X
- Вказівний палець → напрямок +Y
- Середній палець → напрямок +Z

Якщо це працює з ПРАВОЮ рукою → права система координат.
Якщо це працює з ЛІВОЮ рукою → ліва система координат.

```
ПРАВА СИСТЕМА (OpenGL, Vulkan, Blender):     ЛІВА СИСТЕМА (DirectX, Unity):
    +Y                                            +Y
     |                                             |
     |                                             |
     +--- +X                                       +--- +X
    /                                               \
   +Z (до нас)                                      +Z (від нас)
```

**Наш проект (wgpu/Vulkan):** Права система, Y-up, -Z = forward.

### 1.3 Напрямок обертання

В правій системі: якщо великий палець правої руки вказує в напрямку осі, пальці загортаються в напрямку ПОЗИТИВНОГО обертання (проти годинникової стрілки, дивлячись "вниз" по осі).

---

## 2. Одиничне коло та тригонометрія

### 2.1 Одиничне коло (Unit Circle)

Коло з радіусом 1, центр в (0, 0).

Для будь-якого кута θ, точка на колі має координати:
```
x = cos(θ)
y = sin(θ)
```

```
                90° (π/2)
                   |  (0, 1)
                   |
                   |
 180° (π) -------- O -------- 0° (0)
 (-1, 0)           |          (1, 0)
                   |
                   |
               270° (3π/2)
                 (0, -1)
```

### 2.2 Ключові значення

| Кут (град) | Кут (рад) | cos(θ) | sin(θ) |
|------------|-----------|--------|--------|
| 0°         | 0         | 1      | 0      |
| 90°        | π/2       | 0      | 1      |
| 180°       | π         | -1     | 0      |
| 270°       | 3π/2      | 0      | -1     |

### 2.3 Важливі тотожності

```
sin²(θ) + cos²(θ) = 1

sin(θ + 90°) = cos(θ)
cos(θ + 90°) = -sin(θ)

sin(90° - θ) = cos(θ)
cos(90° - θ) = sin(θ)
```

### 2.4 Конвертація градуси ↔ радіани

```
радіани = градуси × π/180
градуси = радіани × 180/π
```

---

## 3. Обертання в 2D

### 3.1 Формула обертання точки

Щоб повернути точку (x, y) на кут θ навколо початку координат:

```
x' = x × cos(θ) - y × sin(θ)
y' = x × sin(θ) + y × cos(θ)
```

### 3.2 Матриця обертання 2D

```
R(θ) = | cos(θ)  -sin(θ) |
       | sin(θ)   cos(θ) |
```

---

## 4. Обертання в 3D

### 4.1 Кути Ейлера (Yaw, Pitch, Roll)

Три кути, що описують орієнтацію тіла в просторі. Термінологія з авіації:

**Yaw (рискання)** - обертання навколо вертикальної осі Y
- Ніс літака рухається ліво-право
- Як обертання голови "ні"

**Pitch (тангаж)** - обертання навколо горизонтальної осі X
- Ніс літака рухається вгору-вниз
- Як кивок голови "так"

**Roll (крен)** - обертання навколо осі Z (вперед)
- Нахил крил ліво-право
- Як нахил голови до плеча

```
        +Y (Yaw)
         |
         |
         +------ +X (Pitch)
        /
       /
      +Z (Roll)
```

### 4.2 Матриці обертання навколо осей

**Обертання навколо осі Y (Yaw):**
```
Ry(θ) = | cos(θ)   0   sin(θ) |
        |   0      1     0    |
        |-sin(θ)   0   cos(θ) |
```

Результат для точки:
```
x' = x × cos(θ) + z × sin(θ)
y' = y
z' = -x × sin(θ) + z × cos(θ)
```

**Обертання навколо осі X (Pitch):**
```
Rx(θ) = | 1    0       0     |
        | 0  cos(θ)  -sin(θ) |
        | 0  sin(θ)   cos(θ) |
```

**Обертання навколо осі Z (Roll):**
```
Rz(θ) = | cos(θ)  -sin(θ)  0 |
        | sin(θ)   cos(θ)  0 |
        |   0        0     1 |
```

### 4.3 Порядок обертання ВАЖЛИВИЙ!

Обертання НЕ комутативні: Rx × Ry ≠ Ry × Rx

Стандартний порядок: Yaw → Pitch → Roll (Y → X → Z)

### 4.4 Gimbal Lock (Блокування карданного підвісу)

**Проблема:** Коли pitch = ±90°, осі yaw і roll стають паралельними, і ми втрачаємо один ступінь свободи.

**Рішення:** Використовувати кватерніони замість кутів Ейлера.

---

## 5. Кватерніони

### 5.1 Що це?

Розширення комплексних чисел до 4 вимірів:
```
q = w + xi + yj + zk
```
де i² = j² = k² = ijk = -1

Для обертання: w - скалярна частина, (x, y, z) - векторна частина.

### 5.2 Одиничний кватерніон

Кватерніон з довжиною 1. Тільки одиничні кватерніони представляють обертання.
```
|q| = √(w² + x² + y² + z²) = 1
```

### 5.3 Кватерніон обертання

Обертання на кут θ навколо одиничного вектора (ax, ay, az):
```
q = (cos(θ/2), ax×sin(θ/2), ay×sin(θ/2), az×sin(θ/2))
```

**ВАЖЛИВО:** Кут ділиться на 2!

### 5.4 Обертання навколо осі Y (Yaw)

```rust
Quat::from_rotation_y(yaw)
// = (cos(yaw/2), 0, sin(yaw/2), 0)
// = (w, x, y, z) де x=0, z=0
```

### 5.5 Переваги кватерніонів

1. **Немає Gimbal Lock** - завжди 3 ступені свободи
2. **Компактність** - 4 числа замість 9 (матриця)
3. **Плавна інтерполяція** (SLERP) - для анімації
4. **Чисельна стабільність** - менше помилок округлення

### 5.6 Особливості

- **q і -q представляють ОДНЕ обертання** (подвійне покриття)
- **Множення некомутативне:** q1 × q2 ≠ q2 × q1
- **Спряжений кватерніон** q* = (w, -x, -y, -z) - обернене обертання

---

## 6. Forward Vector та напрямок руху

### 6.1 Базовий forward

В нашій системі (Y-up, права) при yaw=0:
- Forward = -Z = (0, 0, -1)

### 6.2 Forward після обертання на yaw

Застосовуємо матрицю Ry(yaw) до вектора (0, 0, -1):
```
forward.x = 0 × cos(yaw) + (-1) × sin(yaw) = -sin(yaw)
forward.y = 0
forward.z = -0 × sin(yaw) + (-1) × cos(yaw) = -cos(yaw)
```

**Результат:**
```rust
forward = Vec3::new(-sin(yaw), 0.0, -cos(yaw))
```

### 6.3 Right Vector

Right = Forward повернутий на 90° за годинниковою (в XZ площині):
```rust
right = Vec3::new(cos(yaw), 0.0, -sin(yaw))
```

---

## 7. Трансформації та матриці

### 7.1 Model-View-Projection (MVP)

Три матриці, що перетворюють вершини:

**Model (World) Matrix:**
- Переміщує об'єкт з локальних координат у світові
- Включає: позицію, обертання, масштаб

**View Matrix:**
- Переміщує світ відносно камери
- = Обернена матриця позиції камери

**Projection Matrix:**
- Проектує 3D на 2D екран
- Perspective (перспектива) або Orthographic (ортографічна)

### 7.2 Порядок застосування

```
ClipPosition = Projection × View × Model × LocalPosition
```

Множення справа наліво: спочатку Model, потім View, потім Projection.

### 7.3 Model Matrix = Scale × Rotate × Translate

```
M = T × R × S
```

Порядок множення: спочатку Scale, потім Rotate, потім Translate.

---

## 8. Практичне застосування в нашому проекті

### 8.1 Координатна система

```
    +Y (вгору)
     |
     |
     +------ +X (вправо)
    /
   /
  +Z (до камери / назад)

Forward = -Z
```

### 8.2 Камера третьої особи

```rust
// Позиція камери відносно target
camera_offset = Vec3::new(
    distance * cos(pitch) * cos(yaw),  // X
    distance * sin(pitch),              // Y (висота)
    distance * cos(pitch) * sin(yaw),  // Z
);
camera_position = target + camera_offset;

// Напрямок "вперед" камери (від камери до target)
camera_forward = Vec3::new(-cos(yaw), 0, -sin(yaw));
```

### 8.3 Синхронізація гравця з камерою

**Проблема:** Формули forward різні для камери та гравця.

```rust
// Камера:
camera_forward = (-cos(cam_yaw), 0, -sin(cam_yaw))

// Гравець:
player_forward = (-sin(player_yaw), 0, -cos(player_yaw))
```

**Щоб player_forward == camera_forward:**
```
-sin(player_yaw) = -cos(cam_yaw)  →  sin(player_yaw) = cos(cam_yaw)
-cos(player_yaw) = -sin(cam_yaw)  →  cos(player_yaw) = sin(cam_yaw)
```

Це тригонометрична тотожність: sin(90° - x) = cos(x), cos(90° - x) = sin(x)

**Рішення:**
```rust
player_yaw = PI/2 - cam_yaw
```

### 8.4 Застосування обертання до mesh

```rust
// Конвертуємо yaw в кватерніон
let rotation = Quat::from_rotation_y(yaw);

// Застосовуємо до transform
mesh.transform.rotation = rotation;

// GPU отримує model matrix
let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, position);
```

---

## 9. Типові помилки та їх уникнення

### 9.1 Плутанина з напрямком осей

**Помилка:** Припускати що +Z = forward
**Реальність:** В OpenGL/Vulkan -Z = forward

### 9.2 Неправильний порядок обертання

**Помилка:** Множити матриці в неправильному порядку
**Рішення:** Завжди перевіряти порядок: Yaw → Pitch → Roll

### 9.3 Градуси замість радіанів

**Помилка:** Передавати градуси в sin/cos
**Рішення:** Завжди конвертувати: `radians = degrees * PI / 180`

### 9.4 Gimbal Lock

**Помилка:** Pitch = ±90° при використанні Euler angles
**Рішення:** Використовувати кватерніони для інтерполяції та зберігання

### 9.5 Різні формули forward

**Помилка:** Припускати що всі forward vectors обчислюються однаково
**Рішення:** Перевіряти формулу для кожного компонента (камера vs гравець)

---

## 10. Камера третьої особи (Third Person Camera)

### Додаткові джерела
- [Orbit Camera - Catlike Coding](https://catlikecoding.com/unity/tutorials/movement/orbit-camera/)
- [Third Person Camera Design - Gamedeveloper](https://www.gamedeveloper.com/design/third-person-camera-design-with-free-move-zone)
- [Tips and Tricks for Robust Third-Person Camera - Game AI Pro](http://www.gameaipro.com/GameAIPro/GameAIPro_Chapter47_Tips_and_Tricks_for_a_Robust_Third-Person_Camera_System.pdf)
- [Third-person camera with spring arm - Godot Docs](https://docs.godotengine.org/en/stable/tutorials/3d/spring_arm.html)
- [Camera Evolution in Third-Person Games](https://www.gamedeveloper.com/design/camera-evolution-in-third-person-games)

### 10.1 Типи камер третьої особи

**1. Фіксована камера (Fixed Camera)**
- Позиція та кут камери задаються розробником
- Приклади: ранні Resident Evil, God of War
- Переваги: кінематографічний контроль
- Недоліки: може бути незручно для навігації

**2. Камера що слідує (Follow/Tracking Camera)**
- Автоматично слідує за персонажем
- Базова: просто тримає персонажа в центрі
- З лагом: "гумова стрічка" - камера плавно наздоганяє

**3. Орбітальна камера (Orbit/Arc Ball Camera)**
- Гравець контролює обертання навколо персонажа
- Камера завжди на сфері з центром у персонажі
- Параметри: відстань (radius), yaw, pitch

**4. Over-the-Shoulder (Через плече)**
- Камера зміщена вбік від персонажа
- Популярна в шутерах (Gears of War, Resident Evil 4)
- Краще для прицілювання

**5. Lock-On камера**
- Камера фіксується на ворогові
- Персонаж завжди обертається до ворога
- Приклади: Dark Souls, Zelda

### 10.2 Орбітальна камера - математика

Камера рухається по поверхні сфери навколо target (персонажа):

```
Параметри:
- target: Vec3      - центр сфери (позиція персонажа)
- distance: f32     - радіус сфери
- yaw: f32          - горизонтальний кут (навколо Y)
- pitch: f32        - вертикальний кут (вгору/вниз)
```

**Позиція камери в сферичних координатах:**
```rust
// Зсув камери від target
camera_offset = Vec3::new(
    distance * pitch.cos() * yaw.cos(),   // X
    distance * pitch.sin(),                // Y (висота)
    distance * pitch.cos() * yaw.sin(),   // Z
);

camera_position = target + camera_offset;
```

**Чому саме так?**
- `pitch.cos()` - проекція на горизонтальну площину
- `pitch.sin()` - висота над горизонтом
- `yaw.cos()` та `yaw.sin()` - розподіл по X та Z

```
Вигляд збоку (YZ):           Вигляд зверху (XZ):

    Camera                        Z
      *                           |
     /|                           |   * Camera
    / | height = d*sin(pitch)     |  /
   /  |                           | / horizontal = d*cos(pitch)
  /   |                           |/
 /____|_____ horizontal          -+-------- X
Target                          Target
```

### 10.3 Керування камерою

**Обертання (mouse look):**
```rust
pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
    self.yaw += delta_yaw;
    self.pitch += delta_pitch;

    // Обмежуємо pitch щоб не перевернутись
    self.pitch = self.pitch.clamp(-85°, 85°);
}
```

**Zoom (колесо миші):**
```rust
pub fn zoom(&mut self, delta: f32) {
    self.distance -= delta;
    self.distance = self.distance.clamp(2.0, 20.0);
}
```

### 10.4 Camera-Relative Movement (рух відносно камери)

**Концепція:** Коли гравець натискає "вперед" (W), персонаж рухається в напрямку куди дивиться камера, НЕ в напрямку куди дивиться персонаж.

```
Камера дивиться на схід:

        N
        |
    W --+-- E  (camera forward)
        |
        S

Натискання W → персонаж рухається на схід
Натискання A → персонаж рухається на північ
```

**Реалізація:**
```rust
// Отримуємо напрямки камери (без Y компоненти)
let cam_forward = camera.forward_xz().normalize();
let cam_right = camera.right_xz().normalize();

// Збираємо input
let mut move_dir = Vec3::ZERO;
if input.w_pressed { move_dir += cam_forward; }
if input.s_pressed { move_dir -= cam_forward; }
if input.d_pressed { move_dir += cam_right; }
if input.a_pressed { move_dir -= cam_right; }

// Нормалізуємо щоб діагональний рух не був швидшим
if move_dir != Vec3::ZERO {
    move_dir = move_dir.normalize();
}

// Рухаємо персонажа
player.position += move_dir * speed * delta_time;
```

### 10.5 Орієнтація персонажа

Два основні підходи:

**1. Персонаж дивиться куди камера (наш підхід):**
```rust
// Персонаж завжди повернутий спиною до камери
player.yaw = PI/2 - camera.yaw;
```
- Використовується в: combat games, shooters
- Переваги: точне прицілювання, strafe movement
- Недоліки: менш природній рух при exploration

**2. Персонаж дивиться в напрямку руху:**
```rust
// Персонаж повертається в напрямку move_dir
if move_dir != Vec3::ZERO {
    let target_yaw = move_dir.x.atan2(move_dir.z);
    player.yaw = lerp(player.yaw, target_yaw, turn_speed * delta);
}
```
- Використовується в: adventure games, exploration
- Переваги: природній рух
- Недоліки: складніше прицілювання

### 10.6 Колізія камери (Spring Arm)

**Проблема:** Камера проходить крізь стіни та об'єкти.

**Рішення - Spring Arm:**
1. Пускаємо raycast від target до бажаної позиції камери
2. Якщо промінь зіткнувся з об'єктом - ставимо камеру перед об'єктом

```rust
fn update_camera_with_collision(&mut self) {
    let desired_position = self.calculate_orbit_position();
    let direction = (desired_position - self.target).normalize();
    let max_distance = (desired_position - self.target).length();

    // Raycast від персонажа до бажаної позиції камери
    if let Some(hit) = raycast(self.target, direction, max_distance) {
        // Зіткнення! Ставимо камеру ближче
        self.position = hit.point - direction * 0.2; // Невеликий offset
    } else {
        // Немає зіткнення
        self.position = desired_position;
    }
}
```

**Whisker Raycasts (додаткові промені):**
- Кілька променів під кутом до основного
- Виявляють перешкоди до того як персонаж буде закритий
- Камера плавно обходить перешкоди

### 10.7 Camera Lag (затримка камери)

**Позиційний lag:**
```rust
// Плавне слідування за target
let desired_target = player.position;
self.target = lerp(self.target, desired_target, lag_speed * delta);
```

**Ротаційний lag:**
```rust
// Плавне обертання
self.current_yaw = lerp(self.current_yaw, self.target_yaw, rotation_lag * delta);
```

### 10.8 Free Move Zone

**Концепція:** Персонаж може рухатись в межах "вільної зони" без реакції камери.

```
+---------------------------+
|                           |
|     +---------------+     |
|     |  Free Zone    |     |
|     |   (no camera  |     |
|     |    movement)  |     |
|     +---------------+     |
|                           |
|     Camera follows only   |
|     when player exits     |
|     the free zone         |
+---------------------------+
```

```rust
fn update_camera(&mut self, player_pos: Vec3) {
    let offset = player_pos - self.target;
    let horizontal_dist = Vec2::new(offset.x, offset.z).length();

    if horizontal_dist > FREE_ZONE_RADIUS {
        // Персонаж вийшов з free zone - камера слідує
        let excess = horizontal_dist - FREE_ZONE_RADIUS;
        let dir = Vec2::new(offset.x, offset.z).normalize();
        self.target.x += dir.x * excess;
        self.target.z += dir.y * excess;
    }
    // Інакше - камера не рухається
}
```

---

## 11. Швидкий довідник формул

```rust
// Одиничне коло
x = cos(angle)
y = sin(angle)

// 2D обертання
x' = x*cos(a) - y*sin(a)
y' = x*sin(a) + y*cos(a)

// Forward vector (yaw only, Y-up, -Z forward)
forward = Vec3::new(-sin(yaw), 0, -cos(yaw))

// Right vector
right = Vec3::new(cos(yaw), 0, -sin(yaw))

// Кватерніон обертання навколо Y
Quat::from_rotation_y(angle) = (cos(angle/2), 0, sin(angle/2), 0)

// Конвертація
radians = degrees * PI / 180
degrees = radians * 180 / PI
```
