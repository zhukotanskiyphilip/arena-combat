# Skeleton Proportions - Mathematical Reference

**Date:** 2025-12-25
**Source:** `references/body_figure_ref_1.jpg`
**Applies to:** `src/physics/skeleton.rs`

---

## Overview

This document contains mathematically measured proportions from the reference image, used to create an anatomically correct ragdoll skeleton.

---

## 1. Vertical Positions (from ground)

All values as proportion of total height (0.0 = ground, 1.0 = crown).

| Landmark | Proportion | For 1.8m body |
|----------|------------|---------------|
| Ground | 0.00 | 0.00m |
| Ankles | 0.03 | 0.05m |
| **Knees** | **0.25** | **0.45m** |
| **Crotch** | **0.50** | **0.90m** (EXACT MIDPOINT!) |
| Navel | 0.58 | 1.04m |
| Elbows (relaxed) | 0.62 | 1.12m |
| Lower chest | 0.68 | 1.22m |
| Nipples | 0.78 | 1.40m |
| **Shoulders** | **0.84** | **1.51m** |
| Chin | 0.88 | 1.58m |
| Crown | 1.00 | 1.80m |

**Key insight:** Crotch is at EXACTLY 50% of body height. This is critical for correct proportions.

---

## 2. Horizontal Widths

Measured as proportion of total height, from center of body.

| Measurement | Proportion | For 1.8m | Notes |
|-------------|------------|----------|-------|
| Head (diameter) | 0.10 | 0.18m | Full width |
| Neck (diameter) | 0.06 | 0.11m | |
| **Shoulders (half)** | **0.24** | **0.43m** | From center to edge |
| Shoulders (full) | 0.48 | 0.86m | Total width |
| Chest (radius) | 0.09 | 0.16m | |
| Waist (radius) | 0.08 | 0.14m | Narrowest point |
| Pelvis (radius) | 0.08 | 0.14m | |
| **Hip joints (half)** | **0.08** | **0.14m** | From center to hip joint |

**Key insight:** Shoulders are significantly wider than hips (ratio ~1.7:1).

---

## 3. Segment Lengths

| Segment | Proportion | For 1.8m | Notes |
|---------|------------|----------|-------|
| Head | 0.12 | 0.22m | Chin to crown |
| Neck | 0.04 | 0.07m | |
| **Torso** | **0.34** | **0.61m** | Shoulders to crotch |
| - Chest | 0.16 | 0.29m | |
| - Abdomen | 0.10 | 0.18m | |
| - Pelvis | 0.08 | 0.15m | |
| **Thigh** | **0.25** | **0.45m** | Crotch to knee |
| **Calf** | **0.22** | **0.40m** | Knee to ankle |
| Foot | 0.03 | 0.05m | |
| **Upper Arm** | **0.18** | **0.32m** | Shoulder to elbow |
| **Forearm** | **0.16** | **0.29m** | Elbow to wrist |
| Hand | 0.10 | 0.18m | |

---

## 4. Limb Radii (Thickness)

| Body Part | Proportion | For 1.8m | Notes |
|-----------|------------|----------|-------|
| Head | 0.05 | 0.09m | Radius |
| Chest | 0.09 | 0.16m | Radius |
| Pelvis | 0.08 | 0.14m | Radius |
| **Thigh (upper)** | **0.045** | **0.08m** | Radius - THICK |
| **Calf** | **0.025** | **0.045m** | Radius |
| **Bicep** | **0.028** | **0.05m** | Radius |
| **Forearm** | **0.02** | **0.036m** | Radius |

---

## 5. A-Pose Arm Angle

In the reference image, arms are in relaxed A-pose:
- Angle from vertical: ~15-25 degrees
- Used in code: **25 degrees (0.44 radians)**

---

## 6. Implementation Constants (for skeleton.rs)

```rust
// === БАЗОВІ КОНСТАНТИ ===
const TOTAL_HEIGHT: f32 = 1.80;

// === ВЕРТИКАЛЬНІ ПОЗИЦІЇ ===
const KNEE: f32 = 0.45;          // 0.25 × 1.8
const CROTCH: f32 = 0.90;        // 0.50 × 1.8 - MIDPOINT!
const SHOULDER: f32 = 1.51;      // 0.84 × 1.8

// === ШИРИНИ ===
const SHOULDER_HALF_WIDTH: f32 = 0.43;  // 0.24 × 1.8
const HIP_HALF_WIDTH: f32 = 0.14;       // 0.08 × 1.8
const CHEST_RADIUS: f32 = 0.16;         // 0.09 × 1.8
const PELVIS_RADIUS: f32 = 0.14;        // 0.08 × 1.8

// === ДІАМЕТРИ КІНЦІВОК ===
const THIGH_RADIUS: f32 = 0.08;         // 0.045 × 1.8
const CALF_RADIUS: f32 = 0.045;         // 0.025 × 1.8
const BICEP_RADIUS: f32 = 0.05;         // 0.028 × 1.8
const FOREARM_RADIUS: f32 = 0.036;      // 0.02 × 1.8
const HEAD_RADIUS: f32 = 0.09;          // 0.05 × 1.8

// === ДОВЖИНИ СЕГМЕНТІВ ===
const HEAD_LENGTH: f32 = 0.22;          // 0.12 × 1.8
const NECK_LENGTH: f32 = 0.07;          // 0.04 × 1.8
const TORSO_LENGTH: f32 = 0.61;         // 0.34 × 1.8
const THIGH_LENGTH: f32 = 0.45;         // 0.25 × 1.8
const CALF_LENGTH: f32 = 0.40;          // 0.22 × 1.8
const UPPER_ARM_LENGTH: f32 = 0.32;     // 0.18 × 1.8
const FOREARM_LENGTH: f32 = 0.29;       // 0.16 × 1.8

// === SHOULDER OFFSET ===
// Shoulder joint protrudes beyond torso
const SHOULDER_OFFSET: f32 = SHOULDER_HALF_WIDTH - CHEST_RADIUS + 0.02;  // ~0.29m
```

---

## 7. Visual Diagram

```
                    ┌───┐ 1.00 (1.80m) Crown
                    │ O │ HEAD (r=0.09)
                    └─┬─┘ 0.88 (1.58m) Chin
                      │   NECK
    ┌─────────────────┼─────────────────┐ 0.84 (1.51m) Shoulders
    │                 │                 │
    │    UPPER ARM    │    SPINE        │    UPPER ARM
    │    (r=0.05)     │    (r=0.16)     │    (r=0.05)
    │                 │                 │
    ├─────────────────┼─────────────────┤ 0.62 (1.12m) Elbows
    │                 │                 │
    │    FOREARM      │                 │    FOREARM
    │    (r=0.036)    │    PELVIS       │    (r=0.036)
    │                 │    (r=0.14)     │
    └─────────────────┼─────────────────┘ 0.50 (0.90m) Crotch/Wrists
                    ┌─┴─┐
                    │   │
          THIGH     │   │     THIGH
          (r=0.08)  │   │     (r=0.08)
                    │   │
                    ├───┤ 0.25 (0.45m) Knees
                    │   │
          CALF      │   │     CALF
          (r=0.045) │   │     (r=0.045)
                    │   │
                    └───┘ 0.03 (0.05m) Ankles
                    ═════ 0.00 (0.00m) Ground
```

---

## 8. Common Mistakes to Avoid

1. **Legs too short** - Crotch MUST be at 50% height
2. **Shoulders too narrow** - Shoulders should be ~1.7x hip width
3. **Thighs too thin** - Thighs are the thickest limbs
4. **Arms too long** - Fingertips at ~mid-thigh when relaxed
5. **Head too big** - Head is only 1/8 of body height

---

## Related Documents

- [HUMAN_ANATOMY_REFERENCE.md](HUMAN_ANATOMY_REFERENCE.md) - Vitruvian proportions theory
- [ACTIVE_RAGDOLL_RESEARCH.md](ACTIVE_RAGDOLL_RESEARCH.md) - Physics implementation
- Reference image: `references/body_figure_ref_1.jpg`

---

**Last Updated:** 2025-12-25
