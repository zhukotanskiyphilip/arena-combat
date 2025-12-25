# Research Index
## Knowledge Base for Arena Combat Development

**Last Updated:** 2025-12-24

---

## Purpose

This index maps all research documents in the project. When starting a new chat session, check this index to find relevant prior research before starting new investigations.

---

## Research Documents

### Animation & Character

| Topic | File | Status | Summary |
|-------|------|--------|---------|
| **Human Anatomy** | [HUMAN_ANATOMY_REFERENCE.md](HUMAN_ANATOMY_REFERENCE.md) | Complete | Vitruvian proportions, 8-head canon, segment lengths, joint positions |
| **Skeleton Proportions** | [SKELETON_PROPORTIONS.md](SKELETON_PROPORTIONS.md) | Complete | Mathematical measurements from reference image, exact constants for skeleton.rs |
| **Tapered Limbs** | [TAPERED_LIMBS_RESEARCH.md](TAPERED_LIMBS_RESEARCH.md) | Complete | Anatomically correct limb shapes with different radii at ends |
| Humanoid Animation | [HUMANOID_ANIMATION_RESEARCH.md](HUMANOID_ANIMATION_RESEARCH.md) | Complete | Skeletal animation, IK/FK, state machines, blending, GPU skinning |
| Animation Code Review | [ANIMATION_CODE_REVIEW.md](ANIMATION_CODE_REVIEW.md) | Complete | Analysis of current code vs animation principles |
| Active Ragdoll | [ACTIVE_RAGDOLL_RESEARCH.md](ACTIVE_RAGDOLL_RESEARCH.md) | Complete | GTA IV/RDR2 style physics, PD controllers, muscle systems |
| Skeletal System | [SKELETAL_ANIMATION_SYSTEM.md](SKELETAL_ANIMATION_SYSTEM.md) | Complete | Bone hierarchy, joint constraints, skeleton implementation |
| 3D Space | [3D_SPACE_UNDERSTANDING.md](3D_SPACE_UNDERSTANDING.md) | Complete | Coordinate systems, transforms, camera math |
| Biomechanics | [BIOMECHANICS_AND_OPTIMIZATION.md](BIOMECHANICS_AND_OPTIMIZATION.md) | Complete | Real-world joint limits, movement patterns |

### Combat & Gameplay

| Topic | File | Status | Summary |
|-------|------|--------|---------|
| Combat Design | [arena_combat_gdd.md](arena_combat_gdd.md) | Complete | Game design document, 5 pillars, directional combat |
| Combat Animation | [HUMANOID_ANIMATION_RESEARCH.md#combat](HUMANOID_ANIMATION_RESEARCH.md) | Complete | Attack phases, canceling, hit reactions, impact effects |

### Technical

| Topic | File | Status | Summary |
|-------|------|--------|---------|
| Tech Stack | [../tech_stack_decision.md](../tech_stack_decision.md) | Complete | Rust, wgpu, rapier3d decisions |
| Ragdoll Experiments | [RAGDOLL_EXPERIMENTS_LOG.md](RAGDOLL_EXPERIMENTS_LOG.md) | Ongoing | Parameter tuning, test results |

### Pending Research

| Topic | Priority | Notes |
|-------|----------|-------|
| Enemy AI Patterns | High | Behavior trees, difficulty scaling |
| Network Animation Sync | Medium | Rollback netcode for animations |
| Animation Compression | Low | Quaternion quantization, keyframe reduction |
| Sound Design for Combat | Medium | Impact sounds, timing |
| Particle Effects | Low | VFX for attacks, hits |

---

## How to Use This Index

### Starting a New Chat Session

1. Check RESEARCH_INDEX.md for existing research
2. Read relevant research files before asking questions
3. Reference existing research in your prompts

### Adding New Research

1. Create file in `docs/` with naming: `TOPIC_RESEARCH.md`
2. Add entry to this index with status and summary
3. Link related research at bottom of new file
4. Update QUICK_START.md if research is critical

### Research File Template

```markdown
# [Topic] Research
## [Subtitle]

**Date:** YYYY-MM-DD
**Status:** In Progress | Complete
**Applies to:** [relevant systems]

---

## Table of Contents
[sections]

---

## 1. Section Name
[content]

---

## Sources
- [Source Name](URL)

---

**Related Research:**
- [Other relevant docs]

**Next Research Topics:**
- [Future investigations]
```

---

## Quick Reference by Task

### "I need to implement..."

| Task | Read First |
|------|-----------|
| Walking animation | HUMANOID_ANIMATION_RESEARCH.md (Section 5) |
| Attack system | HUMANOID_ANIMATION_RESEARCH.md (Section 3, 7) |
| Ragdoll on hit | ACTIVE_RAGDOLL_RESEARCH.md |
| Foot placement | HUMANOID_ANIMATION_RESEARCH.md (Section 5.3) |
| Enemy AI | *pending research* |
| Multiplayer | *pending research* |

### "I need to understand..."

| Concept | Read First |
|---------|-----------|
| **Body proportions** | SKELETON_PROPORTIONS.md, HUMAN_ANATOMY_REFERENCE.md |
| How bones work | SKELETAL_ANIMATION_SYSTEM.md |
| PD controllers | ACTIVE_RAGDOLL_RESEARCH.md |
| Quaternion math | 3D_SPACE_UNDERSTANDING.md |
| Combat feel | arena_combat_gdd.md, HUMANOID_ANIMATION_RESEARCH.md (Section 7) |

---

## Cross-References

```
QUICK_START.md
    └── links to → RESEARCH_INDEX.md (this file)
                        └── links to → Individual research files
                                            └── link to → Related research

PROGRESS.md
    └── references → Research used in each session
```

---

## Maintenance

- **Weekly:** Review pending research priorities
- **Per Session:** Add new findings to existing docs or create new ones
- **Before Major Features:** Check index for relevant prior research
