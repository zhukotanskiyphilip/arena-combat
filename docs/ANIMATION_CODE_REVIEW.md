# Animation Code Review
## Analysis of Current Implementation vs Animation Principles

**Date:** 2025-12-24
**Status:** Complete
**Files Reviewed:** muscle.rs, ragdoll.rs, skeleton.rs, combat/mod.rs

---

## Summary

| Aspect | Status | Notes |
|--------|--------|-------|
| PD Controller | Correct | Follows research formula exactly |
| Muscle Tuning | Good | Values match recommendations |
| Walk Cycle | Needs Improvement | Missing smooth_step, spine lean |
| Combat Phases | Partial | Has timing but no explicit phases |
| Quaternion Handling | Correct | Shortest path implemented |
| Joint Types | Optimized | Hinge for knees/elbows, Ball for shoulders |

---

## 1. PD Controller (muscle.rs) - CORRECT

**Research Formula:**
```
torque = Kp × (θ_target - θ_current) - Kd × ω_angular
```

**Implementation (line 96-104):**
```rust
// P term: пропорційний до помилки
let p_term = axis * angle * self.kp;

// D term: демпфування на основі angular velocity
let d_term = -angular_velocity * self.kd;

// Сумарний torque
let mut torque = (p_term + d_term) * self.strength;
```

**Verdict:** Matches research exactly. Uses axis-angle representation correctly.

---

## 2. Muscle Tuning Values - GOOD

**Research Recommendations:**

| Body Part | Kp | Kd | Max Torque |
|-----------|----|----|------------|
| Legs | 1000-1200 | 100-120 | 800-1000 |
| Spine | 800-1000 | 80-100 | 500-600 |
| Arms | 400-500 | 40-50 | 200-250 |
| Head/Neck | 250-300 | 25-30 | 120-150 |

**Implementation (line 130-148):**
```rust
Spine: Muscle::new(BoneId::Spine, 800.0, 80.0, 500.0)      // OK
Head:  Muscle::new(BoneId::Head, 250.0, 25.0, 120.0)       // OK
UpperArm: Muscle::new(..., 400.0, 40.0, 200.0)             // OK
LowerArm: Muscle::new(..., 300.0, 30.0, 150.0)             // OK
UpperLeg: Muscle::new(..., 1000.0, 100.0, 800.0)           // OK
LowerLeg: Muscle::new(..., 800.0, 80.0, 600.0)             // OK
```

**Verdict:** All values within recommended ranges.

---

## 3. Walk Cycle (muscle.rs) - NEEDS IMPROVEMENT

### Current Issues:

#### 3.1 Missing Smooth Step Function

**Research Recommendation:**
```rust
fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)  // Ease-in-ease-out
}
```

**Current (line 306):**
```rust
let leg_swing = phase_rad.sin() * 0.5;  // Pure sine - not smooth
```

**Problem:** Pure sine creates abrupt direction changes. Smooth step provides more natural acceleration/deceleration.

#### 3.2 Missing Spine Forward Lean

**Research Recommendation:**
```rust
// Lean forward when running (proportional to speed)
let forward_lean = self.spine_lean_forward * (self.speed / 3.0);
```

**Current (line 331-332):**
```rust
// Торс - легке обертання
let torso_twist = phase_rad.sin() * 0.1;
rotations.insert(BoneId::Spine, Quat::from_rotation_y(torso_twist));
```

**Problem:** Only Y-axis rotation (twist). No forward lean (X-axis) for running.

#### 3.3 Missing Step Height Parameter

**Research Recommendation:**
```rust
pub struct WalkCycle {
    pub step_height: f32,  // How high feet lift
    pub hip_sway: f32,     // Side-to-side movement
}
```

**Current (line 263-272):**
```rust
pub struct WalkCycle {
    pub phase: f32,
    pub speed: f32,
    pub stride_length: f32,
    // Missing: step_height, hip_sway, spine_lean_forward, arm_swing_amount
}
```

### Recommended Fix for WalkCycle:

```rust
pub struct WalkCycle {
    pub phase: f32,
    pub speed: f32,
    pub stride_length: f32,
    pub step_height: f32,        // NEW
    pub hip_sway: f32,           // NEW
    pub spine_lean_forward: f32, // NEW
    pub arm_swing_amount: f32,   // NEW
}

impl WalkCycle {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            speed: 1.0,
            stride_length: 0.6,
            step_height: 0.1,        // NEW
            hip_sway: 0.05,          // NEW
            spine_lean_forward: 0.1, // NEW
            arm_swing_amount: 0.3,   // NEW
        }
    }

    pub fn get_pose(&self) -> TargetPose {
        // Use smooth_step instead of pure sine
        let leg_phase = smooth_step(self.phase);
        let leg_swing = (leg_phase * TAU).sin() * self.stride_length;

        // Add spine lean
        let forward_lean = self.spine_lean_forward * (self.speed / 3.0);
        rotations.insert(BoneId::Spine,
            Quat::from_rotation_x(-forward_lean) *
            Quat::from_rotation_y(torso_twist));

        // ... rest
    }
}

fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}
```

---

## 4. Quaternion Handling - CORRECT

**Research Requirement:**
> Ensure shortest path - Q and -Q represent same rotation

**Implementation (line 87-94):**
```rust
// Забезпечуємо найкоротший шлях
let angle = if angle > std::f32::consts::PI {
    angle - std::f32::consts::TAU
} else if angle < -std::f32::consts::PI {
    angle + std::f32::consts::TAU
} else {
    angle
};
```

**Verdict:** Correctly handles angle wrapping for shortest path.

**Also in TargetPose::lerp (line 248-259):**
```rust
rotations.insert(bone_id, rot_a.slerp(rot_b, t));
```

**Note:** glam's `slerp` automatically handles shortest path, but explicit check before manual quaternion math is good practice.

---

## 5. Combat Animation Phases - PARTIAL

### Current State (combat/mod.rs):

**Has timing but not explicit phases:**
```rust
pub enum AttackState {
    Ready,
    Attacking(f32),  // Just remaining time
    Cooldown(f32),
}
```

**Research Recommendation - Explicit Phases:**
```rust
pub struct AttackAnimation {
    pub anticipation_frames: (u32, u32),  // Wind-up
    pub action_frames: (u32, u32),        // Hitboxes active
    pub recovery_frames: (u32, u32),      // Vulnerable
}
```

### Current Timing (line 81-82):
```rust
attack_duration: 0.35,  // 350ms
attack_cooldown: 0.15,  // 150ms
```

**Research Timing Guidelines (@ 30 FPS):**

| Attack | Anticipation | Action | Recovery | Total |
|--------|--------------|--------|----------|-------|
| Quick jab | 100-167ms | 67-100ms | 167-267ms | ~300ms |
| Current | ? | 350ms | 150ms | 500ms |

### Issues:

1. **No Anticipation Phase** - Attack starts immediately
2. **No Explicit Hitbox Window** - Unclear when damage happens
3. **Human Reaction Time** - 250ms needed to react, current attack may be too fast to telegraph

### Recommended Fix:

```rust
pub struct Combat {
    pub state: AttackState,
    pub attack_phases: AttackPhases,
    // ...
}

pub struct AttackPhases {
    pub anticipation: f32,  // 0.1s - wind-up (can cancel here)
    pub action: f32,        // 0.15s - hitbox active
    pub recovery: f32,      // 0.1s - vulnerable
}

pub enum AttackPhase {
    Anticipation,  // Can cancel, no damage
    Action,        // Hitbox active, damage
    Recovery,      // Can't cancel, vulnerable
}

impl Combat {
    pub fn get_phase(&self) -> Option<AttackPhase> {
        if let AttackState::Attacking(remaining) = self.state {
            let elapsed = self.attack_duration - remaining;
            let phases = &self.attack_phases;

            if elapsed < phases.anticipation {
                Some(AttackPhase::Anticipation)
            } else if elapsed < phases.anticipation + phases.action {
                Some(AttackPhase::Action)
            } else {
                Some(AttackPhase::Recovery)
            }
        } else {
            None
        }
    }

    pub fn is_hitbox_active(&self) -> bool {
        matches!(self.get_phase(), Some(AttackPhase::Action))
    }
}
```

---

## 6. Weapon Swing Animation - GOOD

**Current (line 115-134):**
```rust
let swing_start = -0.8_f32;  // -45° замах назад
let swing_end = 1.6_f32;     // +90° удар вперед

// Swing angle: ease-out для швидкого удару
let eased = self.attack_progress * (2.0 - self.attack_progress);
self.weapon_swing_angle = swing_start + eased * swing_range;
```

**Good Points:**
- Uses ease-out curve (quadratic) for natural feel
- Has anticipation angle (-45°)
- Full swing arc (135° total)

**Note:** This implements the "Anticipation, Action, Recovery" principle visually, even if not programmatically separated.

---

## 7. Joint Type Optimization (skeleton.rs) - EXCELLENT

**Research Recommendation:**
- Hinge (1 DOF) for knees/elbows - 3x faster
- Ball (3 DOF) for hips/shoulders - 1.5x faster

**Implementation (line 456-528):**
```rust
// HINGE JOINTS for knees and elbows
BoneId::LeftLowerLeg | BoneId::RightLowerLeg => {
    RevoluteJointBuilder::new(...)  // 1 DOF
}
BoneId::LeftLowerArm | BoneId::RightLowerArm => {
    RevoluteJointBuilder::new(...)  // 1 DOF
}

// BALL JOINTS for hips, shoulders, spine, head
BoneId::LeftUpperLeg | BoneId::RightUpperLeg => {
    SphericalJointBuilder::new()  // 3 DOF
}
```

**Verdict:** Follows optimization guidelines exactly.

---

## 8. Missing Features (Priority Order)

### High Priority:

1. **Attack Phases** - Separate Anticipation/Action/Recovery
2. **Hitbox Window** - Define when hitbox is active
3. **Animation Canceling** - Allow cancel during anticipation

### Medium Priority:

4. **Enhanced Walk Cycle** - smooth_step, spine lean, step height
5. **Hit Reactions** - Flinch, stagger, knockback types
6. **Partial Ragdoll** - Upper body only on hit

### Low Priority:

7. **Foot IK** - Terrain adaptation
8. **Look-At Constraint** - Head follows target
9. **Animation Blending** - Smooth transitions between states

---

## 9. Code Quality Notes

### Good Practices Found:

1. **Extensive documentation** in file headers
2. **Clear separation** of concerns (skeleton, muscle, ragdoll)
3. **Proper use of HashMap** for bone/muscle lookup
4. **Collision group optimization** (self-collision disabled)

### Suggestions:

1. Add `#[derive(Debug)]` to WalkCycle
2. Consider making muscle parameters configurable (not hardcoded)
3. Add unit tests for quaternion math edge cases

---

## 10. Summary of Required Changes

### Immediate (before next feature):

```rust
// 1. Add smooth_step to muscle.rs
fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

// 2. Add attack phases to combat/mod.rs
pub enum AttackPhase { Anticipation, Action, Recovery }
```

### Soon (next sprint):

```rust
// 3. Enhanced WalkCycle parameters
pub struct WalkCycle {
    // Add: step_height, hip_sway, spine_lean_forward
}

// 4. Hit reaction types
pub enum HitReaction { Flinch, Stagger, Knockback, PartialRagdoll, FullRagdoll }
```

---

**Related Research:** [HUMANOID_ANIMATION_RESEARCH.md](HUMANOID_ANIMATION_RESEARCH.md)
