# Human Biomechanics for Games & Physics Optimization

## Executive Summary

Based on research of Unity/Unreal ragdoll implementations, human biomechanics, and physics engine optimization:

### Minimum Viable Skeleton: 10-12 Bones
- Pelvis (root)
- Spine (single physics body, not 3 separate)
- Head
- 2× Upper Leg, 2× Lower Leg
- 2× Upper Arm, 2× Lower Arm

**Optional additions**: Shoulders, hands/feet (diminishing returns)

### Joint Types Matter
- **Hinge joints** (knees/elbows): 1x baseline cost
- **Ball joints** (hips/shoulders): 1.5x baseline cost
- **Generic joints**: 2-3x baseline cost
- **Use simplest joint that achieves behavior**

### Performance Recommendations
1. **Solver iterations**: 4-6 (NOT 10+)
2. **Physics Hz**: 50-60 Hz fixed timestep
3. **Collision filtering**: Disable self-collision via groups
4. **Sleeping**: Enable automatic deactivation
5. **Rapier SIMD**: Enable in Cargo.toml

---

## Part 1: Human Skeletal Structure for Games

### Key Joints and Movement Ranges

| Joint | Type | Axes | Limits (degrees) | Notes |
|-------|------|------|------------------|-------|
| Knee | Hinge | 1 DOF | 0° to 150° | Only flexion, no twist |
| Elbow | Hinge | 1 DOF | 0° to 145° | Only flexion, no twist |
| Hip | Ball | 3 DOF | Forward 120°, Back 45°, Side 45° | Cone limits |
| Shoulder | Ball | 3 DOF | Forward 180°, Back 45°, Side 180° | Wide range |
| Spine | Universal/Ball | 2-3 DOF | Limited twist/bend | Often simplified to 1 body |
| Neck | Ball | 3 DOF | ~80° each direction | Head rotation |
| Ankle | Universal | 2 DOF | Dorsi/plantar 45°, inversion 30° | Optional for games |

### Critical Stability Rules

1. **Avoid small angle limits** (< 5-15°) - causes jitter
2. **Mass ratios** between connected bodies should be < 10:1
3. **Realistic limb masses**:
   - Pelvis: 10kg (heaviest)
   - Chest: 8kg
   - Head: 4-5kg
   - Upper leg: 5kg
   - Lower leg: 3kg
   - Upper arm: 2kg
   - Lower arm: 1kg

---

## Part 2: Muscle Simulation (PD Controllers)

### What Are Muscles in Physics Terms?

**Muscles = Torque Generators** that:
- Apply rotational forces at joints
- Move bones toward target poses
- Resist external forces (gravity, impacts)

### PD Controller Formula

```
torque = Kp * (target_angle - current_angle) + Kd * (0 - current_velocity)
```

Where:
- **Kp** (position spring): How strongly to reach target (1000-10000)
- **Kd** (damping): Smoothing/stability (50-500)
- **Max torque**: Muscle strength limit (50-200 Nm per joint)

### Tuning for Stability

**Critical Damping Ratio** (ζ = 1.0 for fastest response without overshoot):
```
ζ = damping / (2 * sqrt(stiffness * mass))
```

For mass=5kg, stiffness=1000:
```
damping = 2 * sqrt(1000 * 5) = 141
```

Current implementation likely **underdamped** (oscillates).

---

## Part 3: Performance Bottlenecks

### Physics Time Breakdown
- **Collision Detection: 70-85%** (broad + narrow phase)
- **Constraint Solving: 10-20%** (joints)
- **Integration: 5-10%** (update positions/velocities)

### Ragdoll-Specific Issues
1. **Self-collision most expensive** - 10-15 colliders checking each other
2. **Too many joints** (14 joints for 19 bones)
3. **Fingers/toes add minimal visual quality** but cost performance
4. **GenericJoint** slower than specialized joints

### Solver Iterations: Diminishing Returns

| Iterations | Stability | CPU Cost | Use Case |
|------------|-----------|----------|----------|
| 2-3 | Poor | Fastest | Background objects |
| 4-6 | Good | **Recommended** | Most ragdolls |
| 8-10 | Very good | 2x slower | Main characters |
| 16+ | Marginal gain | 4x slower | Rarely justified |

**Recommendation**: Start at 4, increase to 8 only if unstable.

---

## Part 4: Rapier3D Optimization

### 1. Enable SIMD and Parallelization

```toml
[dependencies]
rapier3d = { version = "0.22", features = ["simd-stable", "parallel"] }
```

### 2. Build in Release Mode

```toml
[profile.dev.package.rapier3d]
opt-level = 3

[profile.release]
codegen-units = 1
lto = true
```

### 3. Collision Filtering (Critical!)

```rust
let collision_groups = InteractionGroups::new(
    Group::GROUP_1,  // This ragdoll
    Group::ALL & !Group::GROUP_1  // Collide with all except self
);

ColliderBuilder::capsule_y(...)
    .collision_groups(collision_groups)
```

**Massive performance gain** - prevents expensive self-collision checks.

### 4. Solver Tuning

```rust
IntegrationParameters {
    dt: 1.0 / 60.0,  // 60 Hz
    max_velocity_iterations: 4,  // ← KEY: Start low!
    max_position_iterations: 1,
    // ... rest default
}
```

### 5. Physics Frequency

- **Recommendation**: 50-60 Hz (dt = 0.016-0.020)
- **NOT** 120 Hz - diminishing returns, 2x CPU cost
- **Interpolate rendering** if > physics Hz

---

## Concrete Implementation Plan

### Simplified Skeleton (11 bones)

```
Pelvis (root, 10kg)
├── Spine (8kg) - single body, NOT 3 separate
│   ├── Head (4kg)
│   ├── LeftUpperArm (2kg) - Hinge to spine (simplified)
│   │   └── LeftLowerArm (1kg) - Hinge joint (elbow)
│   └── RightUpperArm (2kg)
│       └── RightLowerArm (1kg) - Hinge joint (elbow)
├── LeftUpperLeg (5kg) - Ball joint (hip)
│   └── LeftLowerLeg (3kg) - Hinge joint (knee)
└── RightUpperLeg (5kg) - Ball joint (hip)
    └── RightLowerLeg (3kg) - Hinge joint (knee)
```

**Removed**:
- Chest (merged into Spine)
- Neck (merged into Spine-Head connection)
- Shoulders (arms connect directly to spine)
- Hands/Feet (optional detail)

**Result**: 11 bones, 10 joints (was 19 bones, 14+ joints)

### Joint Type Selection

| Joint | Old | New | Speedup |
|-------|-----|-----|---------|
| Knees | Generic (6 DOF) | **Hinge (1 DOF)** | 3x faster |
| Elbows | Generic (6 DOF) | **Hinge (1 DOF)** | 3x faster |
| Hips | Generic (6 DOF) | **Ball (3 DOF)** | 1.5x faster |
| Shoulders | Generic (6 DOF) | **Ball (3 DOF)** | 1.5x faster |
| Spine-Pelvis | Generic | **Ball** | 1.5x faster |
| Spine-Head | Generic | **Ball** | 1.5x faster |

### Performance Gains Estimate

**Before**:
- 19 rigid bodies
- 14+ generic joints (6 DOF each)
- No collision filtering
- Iterations: unknown (possibly 8-10)

**After**:
- 11 rigid bodies (-42%)
- 10 joints: 4 hinge + 6 ball
- Collision filtering enabled
- Iterations: 4-6

**Expected total speedup**: 3-5x faster physics

---

## Implementation Checklist

### Phase 1: Reduce Bones
- [x] Research complete
- [ ] Redefine BoneId enum (11 bones)
- [ ] Update skeleton hierarchy
- [ ] Adjust bone dimensions/masses
- [ ] Test visual quality

### Phase 2: Optimize Joints
- [ ] Replace GenericJoint with specialized types:
  - RevoluteJoint (hinge) for knees/elbows
  - SphericalJoint (ball) for hips/shoulders/spine
- [ ] Set realistic angular limits
- [ ] Lower motor stiffness (prevent oscillation)

### Phase 3: Collision Optimization
- [ ] Add InteractionGroups to all colliders
- [ ] Disable self-collision via groups
- [ ] Profile collision detection time

### Phase 4: Solver Tuning
- [ ] Set max_velocity_iterations = 4
- [ ] Set max_position_iterations = 1
- [ ] Test stability, increase only if needed

### Phase 5: Sleeping & Damping
- [ ] Ensure sleeping(true) on all bodies
- [ ] Increase damping: angular=8-15, linear=3-8
- [ ] Test sleep/wake behavior

### Phase 6: Profile & Iterate
- [ ] Measure FPS before/after
- [ ] Profile physics time breakdown
- [ ] Adjust iterations per-character if needed

---

## Sources

See ACTIVE_RAGDOLL_RESEARCH.md for full bibliography.

Key sources:
- Unity Ragdoll Wizard documentation
- Rapier3D optimization guide
- Box2D solver iteration research
- PuppetMaster active ragdoll system
- Gaffer on Games: Fix Your Timestep
