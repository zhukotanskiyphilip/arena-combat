# Human Anatomy Reference for 3D Character Physics
## Пропорції та геометрія людського тіла для Active Ragdoll

**Date:** 2025-12-25
**Status:** Complete
**Applies to:** physics/skeleton.rs, skeleton proportions, joint placement

---

## Table of Contents

1. [Vitruvian Man Proportions](#1-vitruvian-man-proportions)
2. [8-Head Canon](#2-8-head-canon)
3. [Joint Positions in Head Units](#3-joint-positions-in-head-units)
4. [Segment Lengths](#4-segment-lengths)
5. [Width Proportions](#5-width-proportions)
6. [Implementation Constants](#6-implementation-constants)
7. [Sources](#7-sources)

---

## 1. Vitruvian Man Proportions

Leonardo da Vinci's Vitruvian Man (c. 1490) establishes classical human proportions based on Vitruvius's architectural treatise.

### Core Principles

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│    With arms outstretched, a man is as wide        │
│    as he is tall (forms a square)                   │
│                                                     │
│    The navel is the center of a circle that        │
│    touches fingertips and toes when limbs          │
│    are spread                                       │
│                                                     │
│    The genitals mark the exact midpoint of         │
│    total body height                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Key Ratios from Leonardo's Notes

| Measurement | Proportion |
|-------------|------------|
| Palm width | 4 fingers |
| Foot length | 4 palms |
| Cubit (elbow to fingertips) | 6 palms |
| Total height | 4 cubits = 24 palms |
| Face height | 1/10 of body |
| Head height (chin to crown) | 1/8 of body |

---

## 2. 8-Head Canon

The classical artistic standard where total body height equals 8 head units.

```
Head 1:  ┌───┐ Crown to chin
         │ O │
         └───┘
Head 2:  ───── Chin to nipple line
              (shoulders at ~1.3 heads)
Head 3:  ───── Nipple to navel
              (elbows at ~3 heads)
Head 4:  ───── Navel to CROTCH (midpoint!)
              (wrists at ~4 heads)
Head 5:  ───── Crotch to mid-thigh
              (fingertips at ~4.5-5 heads)
Head 6:  ───── Mid-thigh to KNEE

Head 7:  ───── Knee to mid-calf

Head 8:  ───── Mid-calf to ground
         ═════ (floor level)
```

### Variations

| Figure Type | Height in Heads | Use |
|-------------|-----------------|-----|
| Average person | 7.5 | Realistic characters |
| Ideal/Noble | 8.0 | Classical heroes |
| Heroic | 8.5 | Gods, superheroes |
| Stylized | 6-7 | Cartoon/chibi |

---

## 3. Joint Positions in Head Units

For an 8-head figure, measured from **top of head** (0) to **ground** (8):

### Vertical Positions

| Joint/Landmark | Head Units | For 1.8m Body |
|----------------|------------|---------------|
| Crown of head | 0.0 | 1.800m |
| Chin | 1.0 | 1.575m |
| Shoulders | ~1.3 | ~1.508m |
| Nipple line | 2.0 | 1.350m |
| Elbows | ~3.0 | ~1.125m |
| Navel | 3.0 | 1.125m |
| **Crotch** | **4.0** | **0.900m** |
| Wrists | ~4.0 | ~0.900m |
| Fingertips | ~4.5-5.0 | ~0.675-0.900m |
| **Knees** | **6.0** | **0.450m** |
| Ankles | ~7.5 | ~0.113m |
| Ground | 8.0 | 0.000m |

### Horizontal Positions (Width)

| Measurement | Head Units | For 1.8m Body |
|-------------|------------|---------------|
| Shoulder width | ~2.0 | ~0.450m |
| Hip/pelvis width | ~1.3 | ~0.300m |
| Head width | ~0.8 | ~0.180m |

---

## 4. Segment Lengths

### Torso Segments

| Segment | Head Units | For 1.8m | Notes |
|---------|------------|----------|-------|
| Head + neck | ~1.2 | 0.27-0.30m | Head alone is 1.0 |
| Neck to shoulders | ~0.3 | ~0.07m | Clavicle region |
| Shoulders to navel | ~1.7 | ~0.40m | Ribcage + upper abs |
| Navel to crotch | ~1.0 | ~0.23m | Pelvis |
| **Total torso** | ~3.0 | ~0.70m | Shoulders to crotch |

### Arm Segments

| Segment | Head Units | For 1.8m | Notes |
|---------|------------|----------|-------|
| Upper arm | ~1.3 | 0.30m | Shoulder to elbow |
| Forearm | ~1.0-1.2 | 0.25-0.27m | Elbow to wrist |
| Hand | ~0.7-0.8 | ~0.18m | Wrist to fingertips |
| **Total arm** | ~3.0 | ~0.75m | Armpit to fingertips |

### Leg Segments

| Segment | Head Units | For 1.8m | Notes |
|---------|------------|----------|-------|
| Upper leg (thigh) | ~2.0 | 0.45m | Hip joint to knee |
| Lower leg (calf) | ~2.0 | 0.45m | Knee to ankle |
| Foot | ~0.5 | ~0.10m | Ankle to ground |
| **Total leg** | ~4.0 | ~0.90m | Crotch to ground |

---

## 5. Width Proportions

### Cross-Section View (Front)

```
              ┌─0.18m─┐
              │  HEAD │
              └───────┘
       ┌──────0.45m──────┐
       │    SHOULDERS    │
       └─────────────────┘
            ┌─0.30m─┐
            │ CHEST │
            └───────┘
            ┌─0.30m─┐
            │ PELVIS│
            └───────┘
```

### Key Width Ratios

- Shoulders are wider than hips (~1.5x)
- Shoulders ≈ 2 head widths
- Hips ≈ 1.3 head widths
- Head width ≈ 0.8 head heights

---

## 6. Implementation Constants

### For skeleton.rs (1.8m character)

```rust
// === VITRUVIAN PROPORTIONS ===
const TOTAL_HEIGHT: f32 = 1.80;      // 8 heads
const HEAD_HEIGHT: f32 = 0.225;       // 1/8 of total

// === VERTICAL POSITIONS (from ground) ===
const GROUND: f32 = 0.0;
const ANKLE_HEIGHT: f32 = 0.10;       // ~0.4 heads
const KNEE_HEIGHT: f32 = 0.45;        // 2 heads
const CROTCH_HEIGHT: f32 = 0.90;      // 4 heads (body midpoint!)
const NAVEL_HEIGHT: f32 = 1.08;       // ~4.8 heads
const ELBOW_HEIGHT: f32 = 1.10;       // ~4.9 heads (when relaxed)
const SHOULDER_HEIGHT: f32 = 1.51;    // ~6.7 heads
const CHIN_HEIGHT: f32 = 1.58;        // 7 heads
const CROWN_HEIGHT: f32 = 1.80;       // 8 heads

// === WIDTHS ===
const SHOULDER_WIDTH: f32 = 0.45;     // 2 heads
const HIP_WIDTH: f32 = 0.30;          // 1.3 heads
const HEAD_WIDTH: f32 = 0.18;         // 0.8 heads

// === SEGMENT LENGTHS ===
// Torso
const PELVIS_LENGTH: f32 = 0.18;      // ~0.8 heads
const SPINE_LENGTH: f32 = 0.50;       // ~2.2 heads (includes chest)
const HEAD_NECK_LENGTH: f32 = 0.28;   // ~1.2 heads

// Arms
const UPPER_ARM_LENGTH: f32 = 0.30;   // ~1.3 heads
const FOREARM_LENGTH: f32 = 0.27;     // ~1.2 heads

// Legs
const THIGH_LENGTH: f32 = 0.45;       // 2 heads
const CALF_LENGTH: f32 = 0.45;        // 2 heads
```

### Joint Attachment Points

```rust
// Spine attaches to TOP of pelvis
spine_offset = Vec3::new(0.0, PELVIS_LENGTH / 2.0, 0.0);

// Head attaches to TOP of spine
head_offset = Vec3::new(0.0, SPINE_LENGTH / 2.0, 0.0);

// Arms attach to SIDES of spine (at shoulder level)
left_arm_offset = Vec3::new(-SHOULDER_WIDTH / 2.0, 0.20, 0.0);
right_arm_offset = Vec3::new(SHOULDER_WIDTH / 2.0, 0.20, 0.0);

// Legs attach to BOTTOM of pelvis
left_leg_offset = Vec3::new(-HIP_WIDTH / 2.0, -PELVIS_LENGTH / 2.0, 0.0);
right_leg_offset = Vec3::new(HIP_WIDTH / 2.0, -PELVIS_LENGTH / 2.0, 0.0);

// Lower segments attach to END of parent
forearm_offset = Vec3::new(-UPPER_ARM_LENGTH, 0.0, 0.0);  // left
calf_offset = Vec3::new(0.0, -THIGH_LENGTH, 0.0);
```

---

## 7. Sources

### Primary Sources

1. **Vitruvian Man PMC Study (2020)**
   - URL: https://pmc.ncbi.nlm.nih.gov/articles/PMC7284298/
   - Modern 3D body scanner measurements vs Leonardo's proportions
   - Confirmed proportions within 10% for most measurements

2. **Life Drawing Academy - Body Proportions**
   - URL: https://lifedrawing.academy/drawing-lessons/body-proportions
   - Classical 7.5/8 head canon with joint positions

3. **Wikipedia - Body Proportions**
   - URL: https://en.wikipedia.org/wiki/Body_proportions
   - Historical and cultural variations

4. **Making Comics - Standard Proportions**
   - URL: https://makingcomics.com/2014/01/19/standard-proportions-human-body/
   - Practical proportions for character design

### Key Findings from Research

1. **Crotch is the exact midpoint** - This is critical for leg proportion
2. **Knees at 2 heads from ground** - Common mistake is placing them higher
3. **Elbows align with navel/waist** - Important for arm hanging naturally
4. **Shoulders wider than hips** - Ratio ~1.5:1 for males
5. **Arms = 3 heads total** - Upper arm slightly longer than forearm

---

## Related Research

- [SKELETAL_ANIMATION_SYSTEM.md](SKELETAL_ANIMATION_SYSTEM.md) - Bone hierarchy, animation
- [ACTIVE_RAGDOLL_RESEARCH.md](ACTIVE_RAGDOLL_RESEARCH.md) - Physics-based character control
- [BIOMECHANICS_AND_OPTIMIZATION.md](BIOMECHANICS_AND_OPTIMIZATION.md) - Joint limits, movement patterns
- [3D_SPACE_UNDERSTANDING.md](3D_SPACE_UNDERSTANDING.md) - Coordinate systems, transforms

---

## Implementation Notes

### Why These Proportions Matter

1. **Physics stability** - Correct mass distribution prevents ragdoll from being top-heavy
2. **Visual correctness** - Character looks "right" even without detailed mesh
3. **Joint constraints** - Real anatomical limits prevent unnatural poses
4. **Animation blending** - Procedural animation looks natural with correct proportions

### Common Mistakes to Avoid

1. Making legs too short (under 4 heads)
2. Placing shoulders too low
3. Arms too long or too short
4. Pelvis too wide (makes character look feminine)
5. Head too big (makes character look childlike)

---

**Last Updated:** 2025-12-25
