# Active Ragdoll Experiments Log

## Experiment 1: Initial Implementation
**Date:** 2024-12-22
**Commit:** Initial physics ragdoll

### Setup:
- All bodies dynamic
- Spherical joints between bones
- Joint motors: stiffness=500, damping=50
- Angular damping: 2.0
- Linear damping: 0.5

### Result:
- **FAIL**: Skeleton unstable, spins and shakes
- Bodies move even without input
- Looks like Brownian motion

### Analysis:
- Motor stiffness too high → causes oscillation
- Damping too low → can't counteract motor force
- No upright control → body falls over

---

## Experiment 2: Kinematic Pelvis
**Date:** 2024-12-22

### Changes:
- Pelvis changed to `kinematic_position_based`
- Position/rotation controlled via code
- Limbs still dynamic with joints

### Result:
- **FAIL**: Skeleton parts disconnected
- Brownian motion of individual bones
- Joints don't properly connect to kinematic body

### Analysis:
- Rapier spherical joints may not work well with kinematic→dynamic connection
- Joint motors only affect angular, not positional constraint
- Need stronger positional constraints or different approach

---

## Experiment 3: All Dynamic + Force Control
**Date:** 2024-12-22

### Changes:
- All bodies dynamic (including pelvis)
- Higher damping: pelvis=20/10, spine=15/5, feet=8/3, limbs=5/1
- Lower motor stiffness: 30-100 depending on body part
- Force-based pelvis control (not kinematic)
- Upright torque applied to pelvis, spine, chest

### Setup Details:
```rust
// Damping by body part
pelvis: angular=20.0, linear=10.0
spine:  angular=15.0, linear=5.0
feet:   angular=8.0,  linear=3.0
limbs:  angular=5.0,  linear=1.0

// Joint motor stiffness/damping
spine/chest: 100/20
thighs: 80/18
shins: 60/15
feet: 30/12
arms: 40/12

// Control forces
upright_force: 500.0
movement_force: 200.0
rotation_force: 100.0
```

### Result:
- **FAIL**: Limbs floating in air, disconnected
- Parts move chaotically (Brownian motion)
- No response to WASD input
- Joints don't hold bones together

### Analysis:
- **SphericalJoint has NO positional constraints!**
- Only angular motors, no translation locking
- Bones can drift apart freely in 3D space
- This is why they float - no rigid connection

---

## Experiment 4: GenericJoint with Locked Translation
**Date:** 2024-12-22

### Changes from Experiment 3:
- **CRITICAL FIX**: Changed from `SphericalJoint` to `GenericJoint`
- Used `JointAxesMask::LOCKED_SPHERICAL_AXES` to lock translation
- This makes positional connection RIGID (like real skeleton)
- Increased motor stiffness: 120-200 (was 30-100)
- Increased motor damping: 25-40 (was 12-20)

### Technical Details:
```rust
// OLD (broken):
SphericalJointBuilder::new()
    .local_anchor1(offset)
    .local_anchor2(ZERO)
    .motor_position(AngX, 0.0, stiffness, damping)
    // Only angular control - NO position lock!

// NEW (fixed):
GenericJointBuilder::new(JointAxesMask::LOCKED_SPHERICAL_AXES)
    .local_anchor1(offset)
    .local_anchor2(ZERO)
    .build()

joint.set_motor(AngX, 0.0, stiffness, damping, max_force)
// LOCKED_SPHERICAL_AXES = rigid positional connection
```

### What LOCKED_SPHERICAL_AXES Does:
- Locks all 3 translation axes (X, Y, Z)
- Allows rotation around anchor point
- Creates ball-socket joint like human shoulder/hip
- Bones CANNOT drift apart

### Result:
This fix was superseded by Experiment 5 - full skeleton optimization

---

## Experiment 5: Full Skeleton Optimization
**Date:** 2024-12-22

### Changes Based on Biomechanics Research:

**Phase 1: Bone Reduction (19 → 11 bones)**
- Removed: Chest, Neck, Shoulders (L/R), Hands (L/R), Feet (L/R)
- Merged: Spine+Chest → single Spine, Neck+Head → single Head
- Arms connect directly to Spine (no shoulder bones)
- Result: 42% fewer rigid bodies to simulate

**Phase 2: Specialized Joint Types**
- **Hinge joints** (RevoluteJoint) for knees and elbows - 1 DOF
  - 3x faster than GenericJoint
  - Only rotation around X axis
  - Limits: 0° to ~140° (knees), 0° to ~135° (elbows)

- **Ball joints** (SphericalJoint) for hips, shoulders, spine, head - 3 DOF
  - 1.5x faster than GenericJoint
  - Full rotational freedom
  - Used for: LeftUpperLeg, RightUpperLeg, LeftUpperArm, RightUpperArm, Spine, Head

**Phase 3: Collision Filtering**
```rust
let collision_groups = InteractionGroups::new(
    Group::GROUP_1,  // This ragdoll's group
    Group::ALL & !Group::GROUP_1  // Collide with everything EXCEPT self
);
```
- Prevents expensive self-collision checks
- Most critical optimization for ragdolls

**Phase 4: Physics Parameters**
- Set physics Hz to 60 (dt = 1.0/60.0)
- Motor parameters tuned per-joint type
- Solver iterations handled by Rapier defaults

### Technical Summary:

**Before:**
- 19 bones (Pelvis, Spine, Chest, Neck, Head, 2×Shoulder, 2×UpperArm, 2×Forearm, 2×Hand, 2×Thigh, 2×Shin, 2×Foot)
- 14+ GenericJoint connections (6 DOF each)
- No collision filtering → expensive self-collision
- Unoptimized solver iterations

**After:**
- 11 bones (Pelvis, Spine, Head, 2×UpperArm, 2×LowerArm, 2×UpperLeg, 2×LowerLeg)
- 10 joints: 4× Hinge + 6× Ball (specialized types)
- Collision filtering enabled
- 60 Hz physics update rate

### Expected Performance Gains:
- **3-5x faster** overall physics simulation
- Based on research:
  - 42% fewer rigid bodies
  - 3x faster hinge joints (elbows/knees)
  - 1.5x faster ball joints vs generic
  - Self-collision eliminated (70-85% of collision cost)

### Result:
**BUILD SUCCESSFUL** - Ready for testing

---

## Key Learnings So Far

### What Doesn't Work:
1. **High motor stiffness + low damping** → oscillation
2. **Kinematic root + spherical joints** → disconnected limbs
3. **No upright control** → body falls over

### What Might Work (from research):
1. **High damping on root bodies** (pelvis, spine)
2. **Force-based control** instead of kinematic
3. **Upright torque** to maintain vertical orientation
4. **Properly tuned damping ratio** (~0.7-1.0)

### Damping Ratio Formula:
```
ζ = damping / (2 * sqrt(stiffness * mass))
```
For critical damping (ζ=1.0), with mass ~10kg and stiffness 100:
- damping = 2 * sqrt(100 * 10) = 2 * 31.6 = 63.2

Current values may still be too low!

---

## Next Steps to Try

1. **If still unstable:**
   - Increase joint motor damping significantly (to 50-100)
   - Reduce stiffness even more (to 50)
   - Try fixed joints instead of spherical for spine

2. **If falls over:**
   - Increase upright_force
   - Apply upright control to more bones
   - Add "balance spring" to ground

3. **If too stiff:**
   - Reduce damping on limbs
   - Reduce control forces

---

## Reference: GTA 4 Approach (from research)
- Never pure ragdoll - always "active ragdoll"
- Behavioral layer manages states
- Animation blending back when stable
- Different zones have different responses

## Reference: Hellish Quart Approach
- Motion-captured moves driving active ragdolls
- Contextual moves based on blade position
- Physical sword collision for damage
