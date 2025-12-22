# Система Скелетної Анімації - Дослідження та План Реалізації

## Зміст
1. [Основи скелетної анімації](#1-основи-скелетної-анімації)
2. [Блендінг анімацій](#2-блендінг-анімацій)
3. [Процедурна анімація](#3-процедурна-анімація)
4. [Системи руху](#4-системи-руху)
5. [Hellish Quart - аналіз](#5-hellish-quart---аналіз)
6. [Motion Matching](#6-motion-matching)
7. [Імплементація в Rust/WGPU](#7-імплементація-в-rustwgpu)
8. [План реалізації](#8-план-реалізації)

---

## 1. Основи скелетної анімації

### 1.1 Ієрархія кісток (Bone Hierarchy)

Скелет - це деревоподібна структура, де кожна кістка є вузлом з можливими дочірніми вузлами.

```
                    Root (Hips)
                        │
            ┌───────────┼───────────┐
            │           │           │
         Spine      Left Leg    Right Leg
            │           │           │
         Chest      Thigh        Thigh
            │           │           │
      ┌─────┼─────┐   Knee        Knee
      │     │     │     │           │
   L_Arm  Neck  R_Arm  Foot        Foot
      │     │     │
    L_Hand Head  R_Hand
```

**Ключові концепції:**
- **Root bone** - коренева кістка (зазвичай Hips/Pelvis)
- **Parent-Child** - дочірня кістка наслідує трансформації батьківської
- **Local transform** - трансформація відносно батька
- **World transform** - фінальна трансформація в world space

### 1.2 Bind Pose (T-Pose / A-Pose)

**Bind Pose** - нейтральна позиція, в якій скелет прив'язаний до меша.

```
T-Pose:                    A-Pose:
    O                          O
   /|\                        /|\
  /_|_\                      / | \
    |                        \ | /
   / \                        / \
  /   \                      /   \
```

- **T-Pose**: Руки горизонтально. Простіше для rigging, але менш природня деформація плечей.
- **A-Pose**: Руки під кутом ~45°. Краща деформація плечей, популярніша в сучасних іграх.

### 1.3 Skinning (Vertex Weighting)

**Skinning** - процес деформації меша на основі кісток.

```
Vertex weights приклад (лікоть):
- Upper Arm: 0.3 (30% впливу)
- Forearm: 0.7 (70% впливу)
Сума = 1.0
```

**Linear Blend Skinning (LBS):**
```rust
final_position = Σ(weight[i] * bone_matrix[i] * bind_position)
```

**Проблема LBS - "Candy Wrapper Effect":**
При сильному скручуванні об'єм "схлопується".

**Рішення - Dual Quaternion Skinning:**
Використовує dual quaternions замість матриць, зберігає об'єм.

### 1.4 Forward vs Inverse Kinematics

**Forward Kinematics (FK):**
```
Плече → Лікоть → Зап'ястя → Пальці
   ↓        ↓         ↓         ↓
rotate  rotate   rotate   final position
```
- Анімуємо від кореня до кінця
- Прямий контроль кожної кістки
- Передбачуваний результат

**Inverse Kinematics (IK):**
```
Ціль (де має бути рука)
         ↓
    Обчислення
         ↓
Пальці ← Зап'ястя ← Лікоть ← Плече
```
- Задаємо цільову позицію кінцевої точки
- Система обчислює кути всіх кісток
- Адаптивний до оточення

**Популярні IK алгоритми:**
- **CCD (Cyclic Coordinate Descent)** - простий, швидкий
- **FABRIK (Forward And Backward Reaching IK)** - природні результати
- **Jacobian** - математично точний, дорожчий

---

## 2. Блендінг анімацій

### 2.1 Blend Trees

**Blend Tree** - змішування кількох анімацій на основі параметрів.

**1D Blend Tree (швидкість):**
```
Parameter: Speed (0.0 - 1.0)

     Idle          Walk          Run
       │             │             │
       0.0          0.5          1.0
       └─────────────┴─────────────┘
              interpolation
```

**2D Blend Tree (напрямок + швидкість):**
```
                Forward
                   │
                   │ (+Y)
         ┌─────────┼─────────┐
         │    Run  │  Run    │
  Left   │   Walk  │  Walk   │  Right
  (-X)───┼────Idle─┼─────────┼───(+X)
         │   Walk  │  Walk   │
         │    Run  │  Run    │
         └─────────┼─────────┘
                   │ (-Y)
                Backward
```

**Критичні вимоги для smooth blending:**
1. Анімації однакової довжини
2. Контакт ніг в однакових точках normalized time
3. Подібна природа руху

### 2.2 Animation Layers

```
┌─────────────────────────────────────┐
│          Additive Layer             │  ← Breathing, recoil
├─────────────────────────────────────┤
│         Upper Body Layer            │  ← Shooting, aiming
├─────────────────────────────────────┤
│          Base Layer                 │  ← Locomotion
└─────────────────────────────────────┘
```

**Режими блендінгу:**
- **Override** - верхній шар повністю замінює нижній
- **Additive** - верхній шар додається до нижнього

**Avatar Mask:**
Визначає які кістки контролює кожен шар.
```rust
struct AvatarMask {
    // 1.0 = повний контроль, 0.0 = ігнорувати
    spine: f32,      // 1.0 для upper body
    arms: f32,       // 1.0 для upper body
    legs: f32,       // 0.0 для upper body (controlled by base)
    head: f32,       // 1.0 для look-at
}
```

### 2.3 Cross-Fading

**Transition** - плавний перехід між станами:
```
State A ──────╲
               ╲
                ╲────── Blend ──────╱
                                   ╱
State B ──────────────────────────╱

      ├────────────────────────────┤
              Transition Duration
```

---

## 3. Процедурна анімація

### 3.1 Root Motion vs In-Place

**In-Place Animation:**
```rust
// Анімація не рухає персонажа
// Код контролює позицію
fn update(&mut self, delta: f32) {
    // Відтворюємо анімацію бігу на місці
    self.animator.play("run_in_place");

    // Код рухає персонажа
    self.position += self.direction * self.speed * delta;
}
```
- ✅ Гнучкий контроль швидкості
- ✅ Легко змінювати напрямок
- ❌ Може бути "ковзання"

**Root Motion:**
```rust
// Анімація містить переміщення
fn update(&mut self, delta: f32) {
    // Анімація сама рухає персонажа
    let root_delta = self.animator.play_with_root_motion("run");
    self.position += root_delta;
}
```
- ✅ Природній рух
- ✅ Немає ковзання
- ❌ Менший контроль

### 3.2 Foot IK

**Проблема:** Ноги проходять крізь землю або "плавають" над нею.

**Рішення:**
```rust
fn update_foot_ik(&mut self) {
    for foot in [&mut self.left_foot, &mut self.right_foot] {
        // 1. Raycast вниз від стегна
        let ray_origin = foot.thigh_position;
        let ray_dir = Vec3::NEG_Y;

        if let Some(hit) = raycast(ray_origin, ray_dir, MAX_LEG_LENGTH) {
            // 2. Обчислюємо цільову позицію
            let target = hit.point + Vec3::Y * FOOT_HEIGHT;

            // 3. Застосовуємо IK
            self.solve_two_bone_ik(
                foot.thigh,
                foot.knee,
                foot.ankle,
                target
            );

            // 4. Нахил ступні по нормалі поверхні
            foot.ankle.rotation = align_to_normal(hit.normal);
        }
    }
}
```

### 3.3 Look-At Constraint

```rust
fn update_look_at(&mut self, target: Vec3) {
    // Напрямок до цілі
    let to_target = (target - self.head.position).normalize();

    // Обчислюємо потрібну ротацію
    let target_rotation = Quat::look_rotation(to_target, Vec3::Y);

    // Плавна інтерполяція
    self.head.rotation = self.head.rotation.slerp(
        target_rotation,
        self.look_speed * delta
    );

    // Обмеження кутів (щоб голова не крутилась на 180°)
    self.head.rotation = clamp_rotation(
        self.head.rotation,
        self.head_limits
    );
}
```

### 3.4 Procedural Leaning

```rust
fn update_lean(&mut self, angular_velocity: f32, delta: f32) {
    // Нахил при повороті
    let turn_lean = angular_velocity * TURN_LEAN_FACTOR;

    // Обмежуємо максимальний нахил
    let clamped_lean = turn_lean.clamp(-MAX_LEAN, MAX_LEAN);

    // Плавно застосовуємо
    self.spine.rotation *= Quat::from_rotation_z(clamped_lean * delta);
}
```

---

## 4. Системи руху

### 4.1 8-Directional Locomotion

**Набір анімацій:**
```
         Forward
            │
    FL      F      FR
      ╲     │     ╱
       ╲    │    ╱
   L ───── IDLE ───── R
       ╱    │    ╲
      ╱     │     ╲
    BL      B      BR
            │
         Backward

9 анімацій для Walk + 9 для Run = 18 анімацій
```

**Реалізація з 2D Blend Tree:**
```rust
struct LocomotionBlendTree {
    // Анімації по напрямках
    animations: HashMap<Direction, Animation>,
}

impl LocomotionBlendTree {
    fn sample(&self, input_x: f32, input_y: f32) -> Pose {
        // Знаходимо 3 найближчі напрямки
        let (dir1, dir2, dir3) = find_nearest_directions(input_x, input_y);

        // Обчислюємо ваги (barycentric coordinates)
        let (w1, w2, w3) = calculate_weights(input_x, input_y, dir1, dir2, dir3);

        // Блендимо пози
        blend_poses(
            &self.animations[&dir1].sample(),
            &self.animations[&dir2].sample(),
            &self.animations[&dir3].sample(),
            w1, w2, w3
        )
    }
}
```

### 4.2 Turn-In-Place

```rust
fn update_turn_in_place(&mut self) {
    // Різниця між напрямком персонажа і цільовим
    let angle_diff = self.target_yaw - self.current_yaw;

    if !self.is_moving && angle_diff.abs() > TURN_THRESHOLD {
        // Вибираємо анімацію повороту
        let turn_anim = if angle_diff > 0.0 {
            if angle_diff > 135.0 { "turn_180_right" }
            else if angle_diff > 45.0 { "turn_90_right" }
            else { "turn_45_right" }
        } else {
            // Аналогічно для лівого повороту
        };

        self.animator.play(turn_anim);
    }
}
```

### 4.3 Upper/Lower Body Split

```rust
struct LayeredAnimator {
    base_layer: AnimationLayer,      // Locomotion
    upper_body_layer: AnimationLayer, // Combat/Aiming

    upper_body_mask: AvatarMask,
}

impl LayeredAnimator {
    fn evaluate(&self, time: f32) -> Pose {
        // Базова поза (locomotion)
        let base_pose = self.base_layer.sample(time);

        // Upper body поза
        let upper_pose = self.upper_body_layer.sample(time);

        // Комбінуємо з маскою
        let mut final_pose = base_pose.clone();

        for (bone_id, mask_weight) in &self.upper_body_mask.weights {
            if *mask_weight > 0.0 {
                final_pose.bones[*bone_id] = Pose::lerp(
                    &base_pose.bones[*bone_id],
                    &upper_pose.bones[*bone_id],
                    *mask_weight
                );
            }
        }

        final_pose
    }
}
```

---

## 5. Hellish Quart - Аналіз

### 5.1 Ключові особливості

**Hellish Quart** - реалістичний симулятор фехтування від Kubold (колишній lead animator The Witcher 3).

**Фізика мечів:**
- Мечі фізично взаємодіють один з одним
- Немає "невразливості" при блоці - тільки фізичний меч блокує
- Урон розраховується в реальному часі на основі швидкості, кута, гострої кромки

**Motion Capture від HEMA:**
- Всі анімації записані від практиків Historical European Martial Arts
- Тисячі motion-captured рухів
- Автентичні історичні техніки фехтування

### 5.2 Технічна реалізація

```
┌─────────────────────────────────────────┐
│           Motion Matching               │
│  (вибір найкращого кадру з бази mocap)  │
├─────────────────────────────────────────┤
│          Physics Simulation             │
│     (Active Ragdoll + Sword Physics)    │
├─────────────────────────────────────────┤
│           Animation Blend               │
│    (плавні переходи між рухами)         │
├─────────────────────────────────────────┤
│            IK Adjustments               │
│   (корекція під фізичні взаємодії)      │
└─────────────────────────────────────────┘
```

### 5.3 Active Ragdoll

```rust
struct ActiveRagdoll {
    // Цільова поза з анімації
    target_pose: Pose,

    // Фізичні тіла для кожної кістки
    rigid_bodies: Vec<RigidBody>,

    // PD контролери для кожного joint
    joint_controllers: Vec<PDController>,
}

impl ActiveRagdoll {
    fn update(&mut self, delta: f32) {
        for (i, joint) in self.joints.iter_mut().enumerate() {
            // Цільова ротація з анімації
            let target_rotation = self.target_pose.bones[i].rotation;

            // Поточна ротація з фізики
            let current_rotation = self.rigid_bodies[i].rotation();

            // PD контролер застосовує torque
            let torque = self.joint_controllers[i].calculate(
                current_rotation,
                target_rotation,
                delta
            );

            self.rigid_bodies[i].apply_torque(torque);
        }
    }

    fn handle_impact(&mut self, bone_id: usize, impact: Impact) {
        // Послаблюємо контроль в точці удару
        self.joint_controllers[bone_id].strength *= 0.5;

        // Додаємо імпульс від удару
        self.rigid_bodies[bone_id].apply_impulse(impact.force);
    }
}
```

---

## 6. Motion Matching

### 6.1 Концепція

**Традиційний підхід:**
```
State Machine:
Idle ─→ Walk ─→ Run ─→ Turn ─→ Stop ─→ Idle
     ←─      ←─     ←─      ←─
```
Потребує сотні transitions і blend nodes.

**Motion Matching:**
```
Database: 5-10 хвилин motion capture
          ↓
      Query кожен кадр:
      - Поточна поза
      - Бажана траєкторія
          ↓
      Найкращий match з бази
          ↓
      Плавний перехід
```

### 6.2 Алгоритм

```rust
struct MotionMatchingSystem {
    // База всіх кадрів mocap
    pose_database: Vec<PoseFrame>,

    // Metadata для швидкого пошуку
    pose_features: Vec<PoseFeatures>,
}

struct PoseFeatures {
    // Позиції/швидкості ключових кісток
    left_foot_pos: Vec3,
    left_foot_vel: Vec3,
    right_foot_pos: Vec3,
    right_foot_vel: Vec3,
    hip_vel: Vec3,

    // Майбутня траєкторія (20, 40, 60 кадрів вперед)
    future_trajectory: [Vec3; 3],

    // Стан контакту з землею
    left_foot_grounded: bool,
    right_foot_grounded: bool,
}

impl MotionMatchingSystem {
    fn find_best_match(&self, current: &PoseFeatures, desired_trajectory: &[Vec3]) -> usize {
        let mut best_cost = f32::MAX;
        let mut best_index = 0;

        for (i, candidate) in self.pose_features.iter().enumerate() {
            // Вартість по позі
            let pose_cost =
                (candidate.left_foot_pos - current.left_foot_pos).length_squared() * FOOT_POS_WEIGHT +
                (candidate.right_foot_pos - current.right_foot_pos).length_squared() * FOOT_POS_WEIGHT +
                (candidate.hip_vel - current.hip_vel).length_squared() * HIP_VEL_WEIGHT;

            // Вартість по траєкторії
            let trajectory_cost = candidate.future_trajectory.iter()
                .zip(desired_trajectory.iter())
                .map(|(a, b)| (a - b).length_squared())
                .sum::<f32>() * TRAJECTORY_WEIGHT;

            // Штраф за зміну grounded state
            let contact_cost = if candidate.left_foot_grounded != current.left_foot_grounded ||
                                  candidate.right_foot_grounded != current.right_foot_grounded {
                CONTACT_CHANGE_PENALTY
            } else {
                0.0
            };

            let total_cost = pose_cost + trajectory_cost + contact_cost;

            if total_cost < best_cost {
                best_cost = total_cost;
                best_index = i;
            }
        }

        best_index
    }
}
```

### 6.3 Ваги для різних випадків

| Use Case | Pose Weight | Trajectory Weight | Contact Weight |
|----------|-------------|-------------------|----------------|
| Combat   | 0.4         | 0.3               | 0.3            |
| Walking  | 0.2         | 0.5               | 0.3            |
| Turning  | 0.3         | 0.4               | 0.3            |

---

## 7. Імплементація в Rust/WGPU

### 7.1 Структури даних

```rust
/// Кістка скелета
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Bone {
    /// Локальна трансформація (відносно батька)
    pub local_transform: Mat4,
    /// Inverse bind matrix (для skinning)
    pub inverse_bind: Mat4,
    /// Індекс батьківської кістки (-1 для root)
    pub parent_index: i32,
    pub _padding: [f32; 3],
}

/// Скелет
pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub bone_names: HashMap<String, usize>,
}

/// Vertex з bone weights
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkinnedVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub bone_ids: [u32; 4],      // До 4 кісток на vertex
    pub bone_weights: [f32; 4],  // Ваги (сума = 1.0)
}

/// Keyframe анімації
pub struct Keyframe {
    pub time: f32,
    pub bone_transforms: Vec<BoneTransform>,
}

pub struct BoneTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

/// Анімаційний кліп
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub keyframes: Vec<Keyframe>,
}
```

### 7.2 GPU Skinning Shader (WGSL)

```wgsl
// Uniforms
struct CameraUniforms {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Bone matrices storage buffer
struct BoneMatrices {
    matrices: array<mat4x4<f32>, 128>, // MAX_BONES
}

@group(1) @binding(0)
var<storage, read> bone_matrices: BoneMatrices;

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) bone_ids: vec4<u32>,
    @location(4) bone_weights: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    // Skinning: blend bone influences
    var skinned_position = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var skinned_normal = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var i: u32 = 0u; i < 4u; i = i + 1u) {
        let bone_id = input.bone_ids[i];
        let weight = input.bone_weights[i];

        if (weight > 0.0) {
            let bone_matrix = bone_matrices.matrices[bone_id];
            skinned_position += bone_matrix * vec4<f32>(input.position, 1.0) * weight;
            skinned_normal += bone_matrix * vec4<f32>(input.normal, 0.0) * weight;
        }
    }

    var output: VertexOutput;
    output.clip_position = camera.view_proj * skinned_position;
    output.world_normal = normalize(skinned_normal.xyz);
    output.tex_coords = input.tex_coords;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ndotl = max(dot(input.world_normal, light_dir), 0.0);
    let ambient = 0.3;
    let diffuse = ndotl * 0.7;

    let color = vec3<f32>(0.8, 0.6, 0.5); // Skin tone
    return vec4<f32>(color * (ambient + diffuse), 1.0);
}
```

### 7.3 Animation System

```rust
pub struct AnimationSystem {
    /// Поточний кліп
    current_clip: Option<AnimationClip>,
    /// Час відтворення
    playback_time: f32,
    /// Швидкість відтворення
    playback_speed: f32,
    /// Чи зациклено
    looping: bool,

    /// Blend target (для crossfade)
    blend_clip: Option<AnimationClip>,
    blend_time: f32,
    blend_duration: f32,
    blend_weight: f32,
}

impl AnimationSystem {
    pub fn update(&mut self, delta: f32) {
        self.playback_time += delta * self.playback_speed;

        if let Some(clip) = &self.current_clip {
            if self.playback_time >= clip.duration {
                if self.looping {
                    self.playback_time %= clip.duration;
                } else {
                    self.playback_time = clip.duration;
                }
            }
        }

        // Update blend
        if self.blend_clip.is_some() {
            self.blend_time += delta;
            self.blend_weight = (self.blend_time / self.blend_duration).min(1.0);

            if self.blend_weight >= 1.0 {
                self.current_clip = self.blend_clip.take();
                self.blend_weight = 0.0;
            }
        }
    }

    pub fn sample(&self, skeleton: &Skeleton) -> Vec<Mat4> {
        let mut bone_matrices = vec![Mat4::IDENTITY; skeleton.bones.len()];

        if let Some(clip) = &self.current_clip {
            let pose = self.sample_clip(clip, self.playback_time);

            // Apply blend if active
            let final_pose = if let Some(blend_clip) = &self.blend_clip {
                let blend_pose = self.sample_clip(blend_clip, self.playback_time);
                self.blend_poses(&pose, &blend_pose, self.blend_weight)
            } else {
                pose
            };

            // Calculate world matrices
            self.calculate_bone_matrices(skeleton, &final_pose, &mut bone_matrices);
        }

        bone_matrices
    }

    fn sample_clip(&self, clip: &AnimationClip, time: f32) -> Vec<BoneTransform> {
        // Find surrounding keyframes
        let (prev_key, next_key, factor) = self.find_keyframes(clip, time);

        // Interpolate
        prev_key.bone_transforms.iter()
            .zip(next_key.bone_transforms.iter())
            .map(|(a, b)| BoneTransform {
                translation: a.translation.lerp(b.translation, factor),
                rotation: a.rotation.slerp(b.rotation, factor),
                scale: a.scale.lerp(b.scale, factor),
            })
            .collect()
    }

    fn calculate_bone_matrices(
        &self,
        skeleton: &Skeleton,
        pose: &[BoneTransform],
        output: &mut [Mat4]
    ) {
        for (i, bone) in skeleton.bones.iter().enumerate() {
            // Local transform from pose
            let local = Mat4::from_scale_rotation_translation(
                pose[i].scale,
                pose[i].rotation,
                pose[i].translation,
            );

            // World transform (apply parent)
            let world = if bone.parent_index >= 0 {
                output[bone.parent_index as usize] * local
            } else {
                local
            };

            // Final skinning matrix = world * inverse_bind
            output[i] = world * bone.inverse_bind;
        }
    }

    /// Crossfade до нової анімації
    pub fn crossfade_to(&mut self, clip: AnimationClip, duration: f32) {
        self.blend_clip = Some(clip);
        self.blend_time = 0.0;
        self.blend_duration = duration;
        self.blend_weight = 0.0;
    }
}
```

### 7.4 WGPU Buffer Setup

```rust
pub struct SkeletalMeshRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bone_buffer: wgpu::Buffer,
    bone_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl SkeletalMeshRenderer {
    pub fn new(device: &wgpu::Device, mesh: &SkinnedMesh, skeleton: &Skeleton) -> Self {
        // Create bone storage buffer
        let bone_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Bone Matrices Buffer"),
            size: (std::mem::size_of::<Mat4>() * skeleton.bones.len()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create vertex buffer with skinning data
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skinned Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // ... pipeline setup ...

        Self {
            vertex_buffer,
            index_buffer,
            bone_buffer,
            bone_bind_group,
            pipeline,
        }
    }

    pub fn update_bones(&self, queue: &wgpu::Queue, bone_matrices: &[Mat4]) {
        queue.write_buffer(
            &self.bone_buffer,
            0,
            bytemuck::cast_slice(bone_matrices),
        );
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(1, &self.bone_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
```

---

## 8. План реалізації

### Phase 1: Базова скелетна система
1. [ ] Створити `Skeleton` структуру з ієрархією кісток
2. [ ] Імплементувати `SkinnedVertex` з bone weights
3. [ ] Написати GPU skinning shader
4. [ ] Завантажити тестову модель (glTF з анімацією)

### Phase 2: Система анімацій
1. [ ] Парсер glTF анімацій
2. [ ] Keyframe interpolation (position, rotation, scale)
3. [ ] Animation playback з loop/once
4. [ ] Crossfade між анімаціями

### Phase 3: Locomotion
1. [ ] 8-directional blend tree
2. [ ] Camera-relative movement з обертанням тіла
3. [ ] Turn-in-place анімації
4. [ ] Start/Stop transitions

### Phase 4: Layered Animation
1. [ ] Upper body / Lower body split
2. [ ] Avatar masks
3. [ ] Additive animations

### Phase 5: IK System
1. [ ] Two-bone IK (руки, ноги)
2. [ ] Foot IK для ground contact
3. [ ] Look-at constraint для голови
4. [ ] Aim constraint для зброї

### Phase 6: Advanced (Optional)
1. [ ] Motion matching prototype
2. [ ] Active ragdoll
3. [ ] Procedural leaning
4. [ ] Physics-based sword interaction

---

## Джерела

- [Skeletal Animation - Wikipedia](https://en.wikipedia.org/wiki/Skeletal_animation)
- [GDC - Motion Matching](https://www.gdcvault.com/play/1023280/Motion-Matching-and-The-Road)
- [Hellish Quart - Steam](https://store.steampowered.com/app/1000360/Hellish_Quart/)
- [LearnOpenGL - Skeletal Animation](https://learnopengl.com/Guest-Articles/2020/Skeletal-Animation)
- [glTF Skinning Tutorial](https://github.com/KhronosGroup/glTF-Tutorials/blob/main/gltfTutorial/gltfTutorial_020_Skins.md)
- [Unity Animation Layers](https://docs.unity3d.com/Manual/AnimationLayers.html)
- [Procedural Animation - Alan Zucconi](https://www.alanzucconi.com/2017/04/17/procedural-animations/)
