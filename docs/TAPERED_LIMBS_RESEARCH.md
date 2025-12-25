# Tapered Limbs Research
## Anatomically Correct 3D Human Figure Representation

**Date:** 2025-12-25
**Status:** Complete
**Applies to:** `src/rendering/skeleton_renderer.rs`, `assets/shaders/skeleton.wgsl`

---

## Table of Contents

1. [Problem Statement](#1-problem-statement)
2. [Research Findings](#2-research-findings)
3. [Solution: Tapered Capsules](#3-solution-tapered-capsules)
4. [Implementation Details](#4-implementation-details)
5. [Bone Dimensions Table](#5-bone-dimensions-table)
6. [Sources](#6-sources)

---

## 1. Problem Statement

The original skeleton renderer used **uniform capsules** (same radius at both ends) for all bones. This created an unrealistic appearance where:
- Limbs looked like "sausages" or "oval shapes"
- No anatomical tapering was visible
- The character didn't match human reference images

Human limbs naturally taper:
- **Thigh**: Thick at hip, thinner at knee
- **Calf**: Thick at knee, thinner at ankle
- **Upper arm**: Thick at shoulder, thinner at elbow
- **Forearm**: Thick at elbow, thinner at wrist

---

## 2. Research Findings

### UCL "Interactive Sketching of Mannequin Poses" Paper

The key finding came from the UCL Visual Computing research:

> "3D Primitive Human Body Model (3DPHB) uses **tapered cylinders with varying radii at each base** for limbs, spheres for joints, and two tapered cylinders for upper and lower torso."

This approach creates a more realistic human figure using simple geometric primitives.

### Ragdoll Physics Best Practices

From jMonkeyEngine and Unity ragdoll documentation:
- Capsules are preferred for limbs due to smooth collision response
- Each limb should be approximated with appropriate radius at each end
- Proportions should be based on anatomical reference (70-80kg adult)

### Mannequin.js Library

The mannequin.js JavaScript library demonstrates articulated human figures with properly proportioned body parts, though the specific implementation details for tapering weren't documented in the user guide.

---

## 3. Solution: Tapered Capsules

### Concept

Instead of a uniform cylinder, use a **frustum** (truncated cone) with hemispherical caps at each end:

```
    ___
   /   \      <- Hemisphere with radius_top
  |     |
  |     |     <- Frustum (tapered cylinder)
   \   /
    \_/       <- Hemisphere with radius_bottom
```

### Implementation Approach

Two options were considered:

1. **Pre-generated tapered meshes** - Generate unique mesh for each bone type
2. **Shader-based tapering** - Apply taper transform in vertex shader (CHOSEN)

The shader approach was chosen because:
- Maintains instanced rendering efficiency
- Only requires passing one extra float (taper_ratio) per instance
- No additional vertex buffers needed

---

## 4. Implementation Details

### Vertex Shader Modification

The taper is applied in the vertex shader based on Y position:

```wgsl
// t = 0 at top (+Y), t = 1 at bottom (-Y)
let t = clamp(0.5 - vertex.position.y, 0.0, 1.0);

// Radius scale: from 1.0 at top to taper_ratio at bottom
let radius_scale = mix(1.0, taper_ratio, t);

// Apply taper to XZ coordinates
var tapered_position = vertex.position;
tapered_position.x *= radius_scale;
tapered_position.z *= radius_scale;
```

### Instance Data Structure

```rust
pub struct BoneInstance {
    pub model_matrix: [[f32; 4]; 4],
    pub color: [f32; 3],
    pub taper_ratio: f32,  // radius_bottom / radius_top
}
```

### Taper Ratio Calculation

```rust
let taper_ratio = radius_bottom / radius_top;
```

- `taper_ratio = 1.0` - uniform cylinder (no taper)
- `taper_ratio < 1.0` - tapers toward bottom
- `taper_ratio > 1.0` - expands toward bottom (unusual)

---

## 5. Bone Dimensions Table

All measurements in meters, based on 1.8m total height reference.

| Bone | Length | Radius Top | Radius Bottom | Taper Ratio | Notes |
|------|--------|------------|---------------|-------------|-------|
| **Pelvis** | 0.15 | 0.14 | 0.14 | 1.0 | Symmetric |
| **Spine** | 0.46 | 0.12 | 0.16 | 1.33 | Expands toward chest |
| **Head** | 0.29 | 0.09 | 0.06 | 0.67 | Head wider than neck |
| **Upper Arm** | 0.32 | 0.055 | 0.038 | 0.69 | Bicep to elbow |
| **Forearm** | 0.29 | 0.042 | 0.028 | 0.67 | Elbow to wrist |
| **Thigh** | 0.45 | 0.10 | 0.065 | 0.65 | Hip to knee |
| **Calf** | 0.40 | 0.058 | 0.038 | 0.66 | Knee to ankle |

### Visual Representation

```
THIGH (taper_ratio = 0.65)
        ┌─────────┐  radius_top = 0.10m (hip)
       /           \
      /             \
     /               \
    /                 \
   /                   \
  └───────────────────┘  radius_bottom = 0.065m (knee)

CALF (taper_ratio = 0.66)
      ┌───────┐  radius_top = 0.058m (knee)
     /         \
    /           \
   /             \
  └─────────────┘  radius_bottom = 0.038m (ankle)
```

---

## 6. Sources

### Academic Papers
- **"Interactive Sketching of Mannequin Poses"** - UCL Visual Computing
  - URL: http://visual.cs.ucl.ac.uk/pubs/sketch2mannequin/
  - Key insight: 3D Primitive Human Body Model uses tapered cylinders

### Game Development Resources
- **jMonkeyEngine Ragdoll Physics**
  - URL: https://wiki.jmonkeyengine.org/docs/3.8/physics/control/ragdoll.html
  - Capsule shapes for smooth collision

- **Unity Ragdoll Physics**
  - URL: https://docs.unity3d.com/6000.2/Documentation/Manual/ragdoll-physics-section.html
  - Best practices for humanoid ragdolls

### Libraries
- **mannequin.js**
  - URL: https://boytchev.github.io/mannequin.js/
  - JavaScript articulated figure library

- **procedural_modelling (Rust)**
  - URL: https://docs.rs/procedural_modelling/
  - Procedural mesh generation including frustums

### Human Anatomy
- **SKELETON_PROPORTIONS.md** - Mathematical measurements from reference image
- **HUMAN_ANATOMY_REFERENCE.md** - Vitruvian proportions, 8-head canon

---

## Related Documents

- [SKELETON_PROPORTIONS.md](SKELETON_PROPORTIONS.md) - Body measurements
- [HUMAN_ANATOMY_REFERENCE.md](HUMAN_ANATOMY_REFERENCE.md) - Vitruvian proportions
- [ACTIVE_RAGDOLL_RESEARCH.md](ACTIVE_RAGDOLL_RESEARCH.md) - Physics system

---

**Last Updated:** 2025-12-25
