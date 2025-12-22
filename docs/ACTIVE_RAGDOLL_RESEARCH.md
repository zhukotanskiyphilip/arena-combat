# Active Ragdoll Research: GTA 4, RDR 2, Hellish Quart

## Overview

This document summarizes research on how professional games achieve stable yet natural-looking character movement using active ragdoll systems.

---

## 1. Euphoria Engine (GTA 4, RDR 2)

### Core Architecture - Three Layers

**1. Simulation Layer (Low-Level Biomechanics)**
- Complex biomechanical model with joint constraints
- Non-linear muscle actuators that simulate real muscle behavior
- Virtual skeleton with realistic physical properties

**2. Controllers Layer (Neural Networks)**
- Uses neural networks (NN) as controllers to determine force application
- Controllers are evolved using genetic algorithms (GA) over multiple generations
- Picks the "fittest" controllers for various movement tasks
- Based on Karl Sims' work on evolving virtual creature controllers

**3. Behavioral Layer (State Machine)**
- Traditional state machine (implemented in Lua)
- Manages high-level behaviors like "protect head," "grab ledge," or "maintain balance"
- Context-aware reactions to environmental stimuli

### Balance and Stability Techniques

**Virtual Skeleton Approach:**
- Full simulation of 3D character including body, muscles, and motor nervous system
- Characters possess a simulated nervous system where virtual muscles tense and relax like real muscles
- CPU combines 3D body model with biomechanics: virtual muscles, skeleton, joints, nervous system, mass, center of gravity, speed, and impact

**Animation-Physics Blending:**
- When force is applied, switches skeleton into physical simulation mode from current posture
- Maintains ragdoll simulation for a few seconds until reaching stable/recognizable posture
- Animation system then blends back into motion capture
- Finds most appropriate animation to transition to
- **NEVER uses pure ragdoll - always "active ragdoll" with self-preservation instincts**

**Stabilization Behaviors (GTA 4 Specific):**
- **Stumble Mode**: When off-balance, movement applied via pushes, character actively tries to regain balance
- Once balance regained, switches back to motion capture animation
- Characters try to balance much longer if alive after being shot
- Grab and hold injured areas while simultaneously trying to balance
- Balancing velocity push applied to gunshots - characters stumble backwards maintaining fluidity
- Different responses for body zones: legs, torso, arms, head
- Lower chance of falling when shot in shoulders/abdominal area

---

## 2. Hellish Quart

### Physics-Based Sword Fighting System

**Active Ragdoll Implementation:**
- Uses thousands of motion-captured moves driving active ragdolls
- Sharp, responsive controls combined with realistic fencing experience
- Swords physically clash and block using game engine physics (Unity PhysX)
- All damage calculated in real-time with active blade collision

**Stability Through Motion Constraints:**
- Moves are contextual to blade position and fighter stance
- Some moves flow naturally together, others require recovery time due to physical constraints of human body
- Example: Cross cuts flow together, but lunges require stepping back before next effective move
- Weapon collisions alter combatant position AND next attack choice
- Creates heavy combat system requiring concentration and planning

---

## 3. PD Controller Tuning

### Fundamental Relationship

A PD controller creates a virtual spring-damper system:
```
force = spring * (targetPosition - position) + damping * (targetVelocity - velocity)
```

### Key Parameters

**Damping Ratio (ζ):**
- **ζ < 1.0**: Underdamped - faster response but oscillates
- **ζ = 1.0**: Critically damped - fastest response without overshoot (RECOMMENDED)
- **ζ > 1.0**: Overdamped - slow response, no oscillation

**Natural Frequency (ωn):**
- ωn = √(k/m) where k = spring stiffness, m = mass
- Determines how quickly system responds

**Critical Damping Formula:**
- ζ = c / (2√(km))
- For critical damping (ζ = 1.0): c = 2√(km)

### Typical Values (Unity ConfigurableJoint reference)
- Position Spring: 500-2000 (higher = stiffer)
- Position Damper: 50-100 (tuned for critical damping around 0.7-1.0)
- Maximum Force: 10,000-60,000+ (often set to infinity for strong control)

### Per-Joint Values for Ragdoll
| Body Part | Stiffness | Damping | Notes |
|-----------|-----------|---------|-------|
| Spine/Chest | 100 | 20 | High - maintain vertical posture |
| Neck/Head | 50 | 15 | Medium - some freedom |
| Thighs | 80 | 18 | High - weight bearing |
| Shins | 60 | 15 | Medium |
| Feet | 30 | 12 | Low - ground contact |
| Arms | 40 | 12 | Low - free movement |

---

## 4. Joint Damping and Limits

### Angular Limits Best Practices
- **AVOID** small joint angles (< 5-15 degrees) - causes instability
- Instead of small angles, use zero if restricting movement
- Set realistic human joint limits to prevent unnatural poses

### Damping Settings
- Damping range: 0 (no damping) to 1.0 (prevents oscillation)
- Values 0-1: Allow oscillation with some damping
- Values > 1: Increasingly damped motion with no oscillation
- **Recommended: 0.7-1.0 for smooth, natural movement**

### Solver Iterations (CRITICAL for Stability)
- Default is often too low for stable ragdolls
- **Position Iterations**: 8+ recommended
- **Velocity Iterations**: 8+ recommended
- Higher values = more accurate but more expensive

### Projection (For Extreme Cases)
- Enable projection mode
- Prevents joint "stretching" when extreme forces applied
- Safety mechanism when solver can't maintain joint constraints

---

## 5. Balance Controllers (Inverted Pendulum Model)

### Theoretical Foundation
- Character balance modeled as inverted pendulum
- Unstable by default - requires active control to prevent falling
- Control system monitors angle and moves support point under center of mass

### Center of Mass (COM) Tracking
- Traditional bipedal control uses single or double inverted pendulum models
- Focus on COM motion relative to support polygon
- Support polygon = convex hull of ground contact points (footprint)

### Balance Condition
- **Necessary (not sufficient)**: COM must lie over support polygon
- For single foot: COM must be within footprint of that foot
- For two feet: COM can be anywhere within combined footprint

### Zero Moment Point (ZMP)
- Point on ground where horizontal moment of ground reaction forces = zero
- ZMP must remain inside support polygon for stable locomotion
- If ZMP falls outside support area, character tips over
- Used extensively in humanoid robotics, applicable to game physics

---

## 6. Human Balance Mechanisms (To Simulate)

### Ankle Strategy (Small Perturbations)
- Rotate about ankle joints to shift COM
- Fast, low-energy response
- **Implementation**: Apply torque to ankle joints toward balance

### Hip Strategy (Medium Perturbations)
- Bend at hips to move upper body
- Creates counter-rotation
- **Implementation**: Apply opposing torques to hip and ankle

### Stepping Strategy (Large Perturbations)
- Move support base under COM
- Most effective but requires foot placement
- **Implementation**: IK-based foot repositioning when COM exceeds threshold

### Arm Reactions
- Counterbalance with arm movements
- Adds authenticity to recovery
- **Implementation**: Apply torques to shoulders opposite to fall direction

---

## 7. Muscle Co-Activation for Stability

### Biomechanical Principle
- Antagonistic muscles (opposing pairs) co-activate
- Increases joint stiffness
- Prevents unwanted movement, especially with uncertainty

### Game Implementation

**1. Increased Joint Damping:**
- When balance threatened, increase joint damping
- Simulates muscle co-activation stiffening
- Makes joints more resistant to external forces

**2. Antagonistic Torques:**
- Apply small opposing torques to same joint
- Creates stiffness without net movement
- Stabilizes against perturbations

**3. Dynamic Stiffness Adjustment:**
- Increase joint spring strength during impacts
- Reduce during normal movement for fluidity
- Mimics natural muscle tension control

---

## 8. Root Motion vs Physics-Driven Locomotion

### Root Motion (Animation-Driven)

**Advantages:**
- Simple authoring - movement baked into animation
- Non-cyclical locomotion possible (pause, step, pause)
- Precise control over motion trajectory
- Natural-looking complex moves (lunges, dodges)
- Prevents character stepping outside collision capsule

**Disadvantages:**
- Can cause abrupt position changes if not blended carefully
- Less responsive to external forces
- Network replication challenges
- Limited physical interaction

### Physics-Driven Locomotion

**Advantages:**
- Realistic momentum and inertia
- Natural response to external forces
- Better physics interactions
- More emergent behaviors

**Disadvantages:**
- Can suffer from tunneling at high speeds
- Foot sliding without careful IK correction
- Less precise control
- More complex to tune

### Hybrid Approach (RECOMMENDED for Active Ragdolls)
1. Use physics for locomotion, sync animation to movement speed
2. Choose animation (stand/walk/jog/run) based on physical velocity
3. Advance animation based on: (physical speed) / (animation speed)
4. Apply IK for foot placement on uneven terrain
5. Use root motion for specific actions (attacks, dodges) then blend back

---

## 9. Kinematic vs Dynamic Bodies

### When to Use Kinematic
- Player-controlled movement base (hip/root)
- Precise, scripted movements
- Character shouldn't respond to physics forces during animation
- Prevents physics engine from adding unwanted movement

### When to Use Dynamic
- Limbs in active ragdoll
- Any part that should respond to collisions
- When realistic physics interactions needed
- Balance recovery requires dynamic response

### Hybrid Technique (Hip Stabilization) - IMPLEMENTED
1. Create kinematic rigidbody for pelvis/root
2. Connect via joints to ragdoll limbs
3. Limbs are dynamic but follow kinematic root
4. Ragdoll automatically stabilizes toward target orientation
5. Still allows some physics response while maintaining stability

---

## 10. Implementation Summary for Arena Combat

### Current Architecture

```
Pelvis (Root) - KINEMATIC
    │
    ├── Spine - Dynamic (joint motor stiffness: 100)
    │     └── Chest - Dynamic (joint motor stiffness: 100)
    │           ├── Neck - Dynamic (joint motor stiffness: 50)
    │           │     └── Head - Dynamic
    │           ├── Left Arm chain - Dynamic (stiffness: 40)
    │           └── Right Arm chain - Dynamic (stiffness: 40)
    │
    ├── Left Leg chain - Dynamic (stiffness: 80/60/30)
    └── Right Leg chain - Dynamic (stiffness: 80/60/30)
```

### Key Parameters Used
- Body angular damping: 5.0 (limbs), 8.0 (feet)
- Body linear damping: 1.0 (limbs), 3.0 (feet)
- CCD enabled for dynamic bodies
- Joint motor stiffness: 30-100 depending on body part
- Joint motor damping: 12-20 depending on body part

### Movement Control
1. `set_move_direction()` - sets walking direction and target yaw
2. `update_kinematic_root()` - smoothly interpolates pelvis position/rotation
3. Joint motors pull limbs toward target poses
4. Physics handles natural limb swinging and collision response

---

## Sources

- Euphoria Ragdoll Overhaul - GTA5-Mods.com
- NaturalMotion's euphoria Technology - AiGameDev.com
- The Wonders of Euphoria; Physics in Games
- Hellish Quart - Steam Store
- Unity Manual: Joint and Ragdoll stability
- Zero moment point - Wikipedia
- Support polygon - Wikipedia
- PD-controllers Tutorial - Matthew P. Kelly
- Unity ConfigurableJoint Documentation
- Muscle coactivation: definitions, mechanisms, and functions (PMC)
- Balancing of Active Ragdolls in Games - Medium
