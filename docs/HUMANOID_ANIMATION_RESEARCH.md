# Humanoid Character Animation Research
## Comprehensive Guide for Arena Combat

**Date:** 2025-12-24
**Status:** Complete
**Applies to:** Active Ragdoll, Combat Animations, Procedural Animation

---

## Table of Contents

1. [Animation Systems Overview](#1-animation-systems-overview)
2. [Skeleton/Rig Structure](#2-skeletonrig-structure)
3. [Animation Techniques for Combat](#3-animation-techniques-for-combat)
4. [Active Ragdoll (GTA IV/RDR2 Style)](#4-active-ragdoll-gta-ivrdr2-style)
5. [Procedural Animation](#5-procedural-animation)
6. [Implementation for Rust/wgpu](#6-implementation-for-rustwgpu)
7. [Best Practices for Melee Combat](#7-best-practices-for-melee-combat)
8. [Formulas Reference](#8-formulas-reference)
9. [Implementation Roadmap](#9-implementation-roadmap)
10. [Sources](#10-sources)

---

## 1. Animation Systems Overview

### Skeletal vs Procedural Animation

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **Skeletal** | Pre-authored keyframes, artist-driven | Small data files, reusable, smooth | Static, less adaptive |
| **Procedural** | Algorithm-generated in real-time | Adaptive, minimal storage, unique | Complex, harder to control |
| **Hybrid** | Combine both (our approach) | Best of both worlds | More implementation work |

**Our Current System:** Hybrid - procedural `WalkCycle` + physics-based `ActiveRagdoll`.

### Keyframe Animation Basics

**Data Structure:**
```rust
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub bone_tracks: HashMap<BoneId, BoneTrack>,
}

pub struct BoneTrack {
    pub timestamps: Vec<f32>,
    pub positions: Vec<Vec3>,
    pub rotations: Vec<Quat>,
    pub scales: Vec<Vec3>,
}
```

**Interpolation Methods:**
- **LERP** for positions: `pos = start + (end - start) * t`
- **SLERP** for rotations: Shortest path between quaternions
- **Cubic** for smooth acceleration/deceleration

**Critical:** Quaternion alignment - Q and -Q represent same rotation, but wrong alignment = 270° path instead of 90°.

### Animation State Machine

```rust
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Attacking { attack_type: AttackType },
    Hit { direction: Vec3 },
    Ragdoll,
}

pub struct AnimationStateMachine {
    pub current_state: AnimationState,
    pub transition_progress: f32,
    pub next_state: Option<AnimationState>,
    pub parameters: AnimationParameters,
}
```

**Layering System:**
- **Base Layer**: Full-body locomotion
- **Upper Body Layer**: Blocking, attacking (override)
- **Additive Layer**: Breathing, looking (adds on top)

---

## 2. Skeleton/Rig Structure

### Our 11-Bone Hierarchy

```
Pelvis (root)
├── Spine
│   ├── Head
│   ├── LeftUpperArm → LeftLowerArm
│   └── RightUpperArm → RightLowerArm
├── LeftUpperLeg → LeftLowerLeg
└── RightUpperLeg → RightLowerLeg
```

**Standard Requirements:**
- Hips/Pelvis = root element
- Minimum 11-15 bones for humanoid
- Parent-child relationships respected

### Joint Types & Performance

| Joint Type | DOF | Speed | Use Case |
|------------|-----|-------|----------|
| Hinge (Revolute) | 1 | 3x faster | Knees, elbows |
| Ball (Spherical) | 3 | 1.5x faster | Hips, shoulders, spine |
| Generic | 6 | Baseline | Only when necessary |

### Real-World Joint Limits

| Joint | Range | Notes |
|-------|-------|-------|
| Knee | 0° to 140° | Flexion only |
| Elbow | 0° to 135° | Similar to knee |
| Shoulder | ~180° swing, ~90° twist | Ball socket |
| Hip | 120° fwd, 60° back, 45° lateral | Complex |
| Spine | ±20-30° per vertebra | Limited |

### IK vs FK

**Forward Kinematics (FK):**
- Root → End direction
- You set joint angles, system calculates end position
- Good for: arm swinging, stylized motion

**Inverse Kinematics (IK):**
- End → Root direction
- You set end position, system solves joint angles
- Good for: foot placement, grabbing objects

**Two-Bone IK Formula (Law of Cosines):**
```rust
let a = upper_leg_length;
let b = lower_leg_length;
let c = (hip_to_target).length();
let knee_angle = acos((a*a + b*b - c*c) / (2.0*a*b));
```

---

## 3. Animation Techniques for Combat

### Root Motion vs In-Place

| Approach | Pros | Cons | Use For |
|----------|------|------|---------|
| **Root Motion** | Perfect sync, cinematic | Less responsive | Heavy attacks, dodges |
| **In-Place** | Maximum responsiveness | Risk of foot sliding | Basic movement, quick attacks |

**Our System:** In-place with physics - ideal for responsive combat.

### Animation Canceling

```rust
pub struct AttackAnimation {
    pub can_cancel_from_frame: u32,
    pub can_cancel_to: Vec<AnimationState>,
    pub cancel_window_frames: (u32, u32),
}

impl AnimationStateMachine {
    pub fn try_cancel(&mut self, new_state: AnimationState, frame: u32) -> bool {
        if let AnimationState::Attacking { attack_type } = &self.current_state {
            let data = get_attack_data(attack_type);
            if frame >= data.can_cancel_from_frame
                && data.can_cancel_to.contains(&new_state) {
                self.transition_to(new_state);
                return true;
            }
        }
        false
    }
}
```

### Hit Reactions

```rust
pub enum HitReaction {
    Flinch { direction: Vec3, strength: f32 },
    Stagger { duration: f32 },
    Knockback { impulse: Vec3 },
    PartialRagdoll { affected_bones: Vec<BoneId>, duration: f32 },
    FullRagdoll,
}
```

### Directional Attacks (8-Way)

```rust
pub enum AttackDirection {
    Forward, ForwardLeft, Left, BackLeft,
    Back, BackRight, Right, ForwardRight,
}

impl AttackDirection {
    pub fn from_input(input_dir: Vec3, char_forward: Vec3) -> Self {
        let angle = input_dir.angle_between(char_forward);
        let cross = char_forward.cross(input_dir).y;
        // Map angle + sign to 8 directions
    }
}
```

---

## 4. Active Ragdoll (GTA IV/RDR2 Style)

### Euphoria Engine Comparison

| Aspect | Passive Ragdoll | Active Ragdoll |
|--------|----------------|----------------|
| Muscle Forces | None | Real-time PD controllers |
| Behavior | Instantly limp | Tries to stay balanced |
| Environmental | Purely reactive | Self-preservation |
| Player Control | None | Limited directional input |

### PD Controller (Muscles)

**Formula:**
```
torque = Kp * (target_angle - current_angle) - Kd * angular_velocity
```

**Tuning Guide:**

| Body Part | Kp (Stiffness) | Kd (Damping) | Max Torque |
|-----------|----------------|--------------|------------|
| Legs | 1000-1200 | 100-120 | 800-1000 |
| Spine | 800-1000 | 80-100 | 500-600 |
| Arms | 400-500 | 40-50 | 200-250 |
| Head/Neck | 250-300 | 25-30 | 120-150 |

### Physics Blend Weight

```rust
pub struct PhysicsBlendSettings {
    pub blend_weight: f32, // 0.0 = animation, 1.0 = physics
    pub blend_duration: f32,
}
```

**Use Cases:**
- Hit reactions: 0.0 → 1.0 over 0.2s
- Recovery: 1.0 → 0.0 over 1.0s
- Normal movement: 0.3-0.5 (slight physics influence)

### Partial Ragdoll

```rust
pub fn set_partial_ragdoll(&mut self, bones: &[BoneId], weight: f32) {
    for bone_id in bones {
        if let Some(muscle) = self.muscles.muscles.get_mut(bone_id) {
            muscle.strength = 1.0 - weight;
        }
    }
}

// Example: Upper body ragdoll on chest hit
ragdoll.set_partial_ragdoll(&[
    BoneId::Spine, BoneId::Head,
    BoneId::LeftUpperArm, BoneId::LeftLowerArm,
    BoneId::RightUpperArm, BoneId::RightLowerArm,
], 0.8);
```

---

## 5. Procedural Animation

### Enhanced Walk Cycle

```rust
pub struct WalkCycle {
    pub phase: f32,
    pub speed: f32,
    pub stride_length: f32,
    pub step_height: f32,
    pub hip_sway: f32,
    pub spine_lean_forward: f32,
    pub arm_swing_amount: f32,
}

impl WalkCycle {
    pub fn get_pose(&self) -> TargetPose {
        let phase_rad = self.phase * TAU;

        // Smoother step function
        let leg_phase = smooth_step(self.phase);
        let leg_swing = (leg_phase * TAU).sin() * self.stride_length;

        // Knee bends more when leg is behind
        let knee_bend = if leg_swing < 0.0 { 0.1 }
                        else { (leg_swing * 2.0).min(1.2) };

        // Spine lean proportional to speed
        let forward_lean = self.spine_lean_forward * (self.speed / 3.0);

        // Arms swing opposite to legs
        let arm_swing = phase_rad.sin() * self.arm_swing_amount;

        // ... build TargetPose
    }
}

fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}
```

### Look-At Constraint

```rust
pub struct LookAtConstraint {
    pub bone_id: BoneId,
    pub target: Vec3,
    pub weight: f32,
    pub max_angle: f32, // e.g., 80°
}

impl LookAtConstraint {
    pub fn apply(&self, physics: &PhysicsWorld, skeleton: &Skeleton) -> Quat {
        let bone_pos = skeleton.get_bone_position(physics, self.bone_id)?;
        let bone_rot = skeleton.get_bone_rotation(physics, self.bone_id)?;

        let to_target = (self.target - bone_pos).normalize();
        let forward = bone_rot * Vec3::NEG_Z;

        let angle = forward.angle_between(to_target);
        if angle > self.max_angle { return bone_rot; }

        let axis = forward.cross(to_target).normalize();
        let look_rotation = Quat::from_axis_angle(axis, angle);

        bone_rot.slerp(look_rotation * bone_rot, self.weight)
    }
}
```

### Foot IK Solver

```rust
pub fn solve_two_bone_ik(
    hip: Vec3, knee: Vec3, foot: Vec3,
    target: Vec3, pole_target: Vec3,
) -> (Quat, Quat) {
    let upper_len = (knee - hip).length();
    let lower_len = (foot - knee).length();
    let target_dist = (target - hip).length().min((upper_len + lower_len) * 0.99);

    // Law of cosines
    let a = upper_len;
    let b = lower_len;
    let c = target_dist;
    let knee_angle = ((a*a + b*b - c*c) / (2.0*a*b)).clamp(-1.0, 1.0).acos();

    // Calculate rotations
    let to_target = (target - hip).normalize();
    let current_dir = (knee - hip).normalize();
    let hip_rotation = Quat::from_rotation_arc(current_dir, to_target);
    let knee_rotation = Quat::from_rotation_x(PI - knee_angle);

    (hip_rotation, knee_rotation)
}
```

---

## 6. Implementation for Rust/wgpu

### Animation Data Storage (glTF)

```rust
use gltf;

pub fn load_from_gltf(path: &str) -> Result<Vec<AnimationClip>, Error> {
    let (document, buffers, _) = gltf::import(path)?;

    for animation in document.animations() {
        for channel in animation.channels() {
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));

            // Read timestamps
            let timestamps: Vec<f32> = reader.read_inputs().unwrap().collect();

            // Read rotations
            match reader.read_outputs().unwrap() {
                gltf::animation::util::ReadOutputs::Rotations(rotations) => {
                    let rots: Vec<Quat> = rotations.into_f32()
                        .map(|[x, y, z, w]| Quat::from_xyzw(x, y, z, w))
                        .collect();
                }
                // ... positions, scales
            }
        }
    }
}
```

### Keyframe Interpolation

```rust
pub fn sample_rotation(track: &BoneTrack, time: f32) -> Quat {
    let (idx0, idx1, t) = find_keyframe_indices(&track.timestamps, time);

    let mut rot0 = track.rotations[idx0];
    let rot1 = track.rotations[idx1];

    // CRITICAL: Ensure shortest path
    if rot0.dot(rot1) < 0.0 {
        rot0 = -rot0;
    }

    rot0.slerp(rot1, t)
}

fn find_keyframe_indices(timestamps: &[f32], time: f32) -> (usize, usize, f32) {
    // Binary search for efficiency
    let idx1 = timestamps.partition_point(|&t| t < time);
    if idx1 == 0 { return (0, 0, 0.0); }
    if idx1 >= timestamps.len() {
        let last = timestamps.len() - 1;
        return (last, last, 0.0);
    }

    let idx0 = idx1 - 1;
    let t = (time - timestamps[idx0]) / (timestamps[idx1] - timestamps[idx0]);
    (idx0, idx1, t)
}
```

### Animation Blending

```rust
pub fn blend_animations(
    clip_a: &AnimationClip, clip_b: &AnimationClip,
    time_a: f32, time_b: f32,
    blend_weight: f32, // 0.0 = A, 1.0 = B
) -> TargetPose {
    let mut blended = TargetPose::default();

    for bone_id in BoneId::all_bones() {
        let rot_a = clip_a.sample_bone_rotation(bone_id, time_a);
        let mut rot_b = clip_b.sample_bone_rotation(bone_id, time_b);

        // Shortest path
        if rot_a.dot(rot_b) < 0.0 { rot_b = -rot_b; }

        blended.bone_rotations.insert(bone_id, rot_a.slerp(rot_b, blend_weight));
    }

    blended
}
```

### GPU Skinning (WGSL)

```wgsl
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) bone_indices: vec4<u32>,
    @location(3) bone_weights: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> bones: array<mat4x4<f32>, 64>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var skinned_pos = vec4<f32>(0.0);

    for (var i: u32 = 0u; i < 4u; i++) {
        let idx = in.bone_indices[i];
        let weight = in.bone_weights[i];
        if (weight > 0.0) {
            skinned_pos += bones[idx] * vec4(in.position, 1.0) * weight;
        }
    }

    // ... transform to clip space
}
```

---

## 7. Best Practices for Melee Combat

### Animation Priority System

```rust
#[derive(PartialOrd, Ord)]
pub enum AnimationPriority {
    Idle = 0,
    Locomotion = 1,
    Dodge = 2,
    Attack = 3,
    Hit = 4,
    Ragdoll = 5,
    Cinematic = 6,
}
```

### Attack Phases (Anticipation, Action, Recovery)

```rust
pub struct AttackAnimation {
    pub anticipation_frames: (u32, u32),  // Wind-up
    pub action_frames: (u32, u32),        // Hitboxes active
    pub recovery_frames: (u32, u32),      // Vulnerable

    pub total_duration: f32,
    pub damage: f32,
    pub can_cancel_after_frame: u32,
}
```

**Timing Guidelines (@ 30 FPS):**

| Attack Type | Anticipation | Action | Recovery | Total |
|-------------|--------------|--------|----------|-------|
| Quick jab | 3-5 frames | 2-3 frames | 5-8 frames | ~0.3s |
| Medium slash | 8-12 frames | 3-5 frames | 10-15 frames | ~0.8s |
| Heavy smash | 15-25 frames | 5-10 frames | 20-30 frames | ~1.5s |

**Human reaction time:** ~250ms. Telegraph dangerous attacks for at least this long.

### Making Attacks Feel Impactful

**The Juiciness Formula:**
1. **Anticipation** - Exaggerated wind-up
2. **Impact Frame** - Single frame freeze/squash
3. **Hit Pause** - Slow time briefly (0.03-0.1s)
4. **Camera Shake** - Proportional to strength
5. **VFX** - Sparks, dust, decals
6. **SFX** - Meaty thud/sharp clang
7. **Hitstun** - Victim animation
8. **Follow-Through** - Exaggerated recovery

```rust
pub struct ImpactEffects {
    pub freeze_duration: f32,     // 0.05s for heavy
    pub camera_shake: f32,        // 0.0-1.0
    pub time_scale: f32,          // 0.1 for slow-mo
    pub slow_mo_duration: f32,
}
```

**Attack Weight Scaling:**

| Strength | Freeze | Shake | Time Scale |
|----------|--------|-------|------------|
| Light | 0.01s | 0.1 | 0.9 |
| Medium | 0.03s | 0.3 | 0.5 |
| Heavy | 0.06s | 0.6 | 0.2 |
| Critical | 0.10s | 1.0 | 0.1 |

---

## 8. Formulas Reference

### PD Controller (Muscles)
```
torque = Kp × (θ_target - θ_current) - Kd × ω_angular
```

### Two-Bone IK (Law of Cosines)
```
knee_angle = arccos((a² + b² - c²) / (2ab))
where: a = upper_length, b = lower_length, c = target_distance
```

### Quaternion SLERP
```
q(t) = (sin((1-t)θ) / sin(θ)) × q₀ + (sin(tθ) / sin(θ)) × q₁
where: θ = arccos(q₀ · q₁)
```

### Spring Damper
```
F = -k × (x - x_target) - c × v
where: k = spring constant, c = damping coefficient
```

### Smooth Step
```rust
fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}
```

---

## 9. Implementation Roadmap

### Phase 1: Basic Animation System (Week 1)
- [x] Procedural walk cycle (exists)
- [x] Active ragdoll with muscles (exists)
- [ ] Animation clip data structure
- [ ] glTF loader
- [ ] Keyframe sampling
- [ ] Basic state machine

### Phase 2: Combat Animations (Week 2-3)
- [ ] Attack animations with phases
- [ ] Animation canceling
- [ ] Directional attacks (8-way)
- [ ] Integrate with hitbox system

### Phase 3: Hit Reactions (Week 3-4)
- [ ] Hit reaction types
- [ ] Partial ragdoll system
- [ ] Recovery animations
- [ ] Impact effects (freeze, shake, slow-mo)

### Phase 4: Blending & Layering (Week 4-5)
- [ ] Multi-layer animation
- [ ] Additive animations
- [ ] Upper body override
- [ ] Smooth transitions

### Phase 5: IK Systems (Week 5-6)
- [ ] Foot IK for terrain
- [ ] Look-at constraint
- [ ] Two-bone IK solver

### Phase 6: GPU Skinning (Week 6-7)
- [ ] Skinned mesh rendering
- [ ] Vertex shader bones
- [ ] Bind pose calculation

---

## 10. Sources

### Animation Systems
- [Skeletal Animation Introduction](https://www.nightquestgames.com/quick-introduction-to-skeletal-animation-for-video-game-development/)
- [Procedural Animation Techniques](https://www.wayline.io/blog/procedural-animation-techniques)

### State Machines & Blending
- [Unity Animation State Machines](https://docs.unity3d.com/Manual/AnimationStateMachines.html)
- [Animation Blending Techniques](https://animcoding.com/post/animation-tech-intro-part-3-blending/)

### Skeleton & IK
- [FK vs IK in Rigging](https://www.whizzystudios.com/post/forward-kinematics-vs-inverse-kinematics-in-3d-character-rigging)
- [Mecanim Humanoids](https://blog.unity.com/technology/mecanim-humanoids)

### Active Ragdoll
- [Euphoria Engine (GTA Wiki)](https://gta.fandom.com/wiki/Euphoria)
- [Active Ragdolls in Unity](https://sergioabreu-g.medium.com/how-to-make-active-ragdolls-in-unity-35347dcb952d)

### Combat Animation
- [Anatomy of an Attack](https://gdkeys.com/keys-to-combat-design-1-anatomy-of-an-attack/)
- [Impactful Melee Combat](https://www.wayline.io/blog/level-up-indie-game-melee-combat)
- [The 12 Principles in Games](https://www.linkedin.com/pulse/12-principles-animation-video-games-jonathan-cooper)

### Rust Implementation
- [glTF Skeletal Animation](https://lisyarus.github.io/blog/posts/gltf-animation.html)
- [LearnOpenGL Skeletal Animation](https://learnopengl.com/Guest-Articles/2020/Skeletal-Animation)
- [skeletal_animation crate](https://lib.rs/crates/skeletal_animation)

### GPU Skinning
- [Implementing Skeletal Animation](https://veeenu.github.io/blog/implementing-skeletal-animation/)
- [Vulkan glTF Skinning](https://github.com/SaschaWillems/Vulkan/blob/master/examples/gltfskinning/)

---

**Related Research:**
- [ACTIVE_RAGDOLL_RESEARCH.md](ACTIVE_RAGDOLL_RESEARCH.md) - Detailed ragdoll physics
- [SKELETAL_ANIMATION_SYSTEM.md](SKELETAL_ANIMATION_SYSTEM.md) - Skeleton implementation

**Next Research Topics:**
- Enemy AI behavior patterns
- Network synchronization for animations
- Animation compression techniques
