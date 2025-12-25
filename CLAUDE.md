# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Last Updated:** 2025-12-24

---

## IMPORTANT: This File is the Source of Truth

> **CLAUDE.md = Entry Point for Every Chat Session**

**Rules:**
1. **This file defines what exists in the project**
2. If you add something to project → add it here
3. If you remove something from here → remove it from project
4. All other docs are referenced FROM here

---

## Project Summary

**Arena Combat** - third-person melee combat game prototype in **Rust** (no game engine).
Inspired by: Jedi Academy, Mount & Blade, Blade of Darkness.

**Tech Stack:** wgpu (graphics) + winit (window) + rapier3d (physics) + glam (math)

**Current Phase:** Phase 1 - Singleplayer (Sessions 1-15+)

---

## Build & Run Commands

**Primary method:**
```batch
run.bat                  # Build + run (use this!)
```

**Manual commands (Windows terminal):**
```bash
cargo build              # Debug build
cargo build --release    # Optimized build
cargo run                # Run game
cargo check              # Fast compilation check
cargo clippy             # Linter
cargo fmt                # Format code
```

**Controls:**
- WASD - Move (camera-relative)
- Right Mouse + Drag - Rotate camera
- Mouse Wheel - Zoom
- Left Click - Attack
- Q/E - Manual rotation
- ESC - Exit

---

## Project Structure

```
arena-combat/
├── src/
│   ├── main.rs              # Entry point, game loop
│   ├── camera/              # Third-person camera (orbit/zoom/pan)
│   ├── input/               # Mouse + keyboard state tracking
│   ├── player/              # Player position, yaw, movement
│   ├── combat/              # Attack state machine, hitbox
│   │   ├── mod.rs           # Ready → Attacking → Cooldown
│   │   └── hitbox.rs        # Sphere-based collision
│   ├── enemy/               # Enemy spawning, state
│   ├── physics/             # Active ragdoll system (GTA IV style)
│   │   ├── ragdoll.rs       # RagdollMode: Active/Ragdoll/Recovery
│   │   ├── skeleton.rs      # 11-bone humanoid, joints
│   │   └── muscle.rs        # PD controllers, WalkCycle, TargetPose
│   ├── transform/           # Position, rotation, scale + matrices
│   ├── time/                # Delta time tracking (GameTime)
│   └── rendering/           # wgpu renderer
│       ├── renderer.rs      # Main renderer, camera uniforms
│       ├── mesh.rs          # Primitives (cube, cylinder, sphere)
│       ├── grid.rs          # Floor grid with fade-out shader
│       └── skeleton_renderer.rs
├── assets/shaders/          # WGSL shaders
├── docs/                    # Research documents
├── debug/                   # Debug logs, dev rules
├── PROGRESS.md              # Development timeline
├── METHODOLOGY.md           # AI-assisted dev protocol
└── Cargo.toml               # Dependencies
```

---

## Key Files to Read

**Priority order for context:**

1. **PROGRESS.md** - What's done, what's next, recent sessions
2. **docs/RESEARCH_INDEX.md** - Index of all research (check before investigating!)
3. **METHODOLOGY.md** - Development rules, commit conventions
4. **docs/arena_combat_gdd.md** - Game design philosophy

---

## Current State (Session 16)

**Implemented:**
- Window + wgpu renderer (Vulkan/DX12/Metal)
- 3D camera with orbit/zoom/pan
- Player mannequin with movement
- Combat system (Ready → Attacking → Cooldown)
- **Attack phases: Anticipation → Action → Recovery**
- Weapon swing animation with phase-aware easing
- Hitbox collision detection (active only in Action phase)
- 6 enemy mannequins
- Active ragdoll physics (GTA 4/RDR 2 style)
- **Enhanced WalkCycle: smooth_step, spine lean, configurable params**
- Delta time, FPS counter

**Not Yet Implemented:**
- Enemy AI (enemies are static)
- Block/parry mechanics
- Player health/damage
- Sound effects
- Network multiplayer (Phase 2)

---

## Architecture Overview

### Physics-Based Animation (Active Ragdoll)

The game uses **Active Ragdoll** like GTA IV/RDR2:
- All bones are dynamic rigid bodies with capsule colliders
- Muscles (PD controllers) apply torque to reach target poses
- `MuscleSystem` drives `Skeleton` through `TargetPose`
- `WalkCycle` generates procedural walking animation

**Key formula (muscle.rs):**
```
torque = Kp * (target_angle - current_angle) - Kd * angular_velocity
```

**WalkCycle parameters:**
```rust
stride_length: 0.5,       // leg swing amplitude (radians)
step_height: 0.15,        // foot lift height
hip_sway: 0.05,           // side-to-side hip movement
spine_lean_forward: 0.1,  // torso lean when moving
arm_swing_amount: 0.3,    // arm swing amplitude
```

**smooth_step function:** `t * t * (3.0 - 2.0 * t)` - ease-in-ease-out for natural animation

**Modes:**
```rust
Active   - Player controlled, muscles apply forces
Ragdoll  - Pure physics, muscles relaxed
Recovery - Gradual return to control
```

### Combat System

```
AttackState::Ready → Attacking → Cooldown → Ready

Attack has 3 phases inside Attacking state:
  Anticipation (100ms) → Action (150ms) → Recovery (100ms)
  - Anticipation: can cancel, no damage
  - Action: hitbox active, damage
  - Recovery: vulnerable, cannot cancel
```

**Key methods:**
- `combat.get_phase()` - returns current AttackPhase
- `combat.is_hitbox_active()` - true only during Action phase
- `combat.can_cancel()` - true only during Anticipation

**Key files:** `src/combat/mod.rs`, `src/combat/hitbox.rs`

### Coordinate System

- **Y-up, right-handed** (OpenGL convention)
- Player spawn: (0, 0.5, 0)
- Grid: 20x20 units
- Camera: spherical coordinates (yaw, pitch, distance)

---

## Development Rules

1. **No Unauthorized Changes** - Only implement what's explicitly requested
2. **Use file-based logging** - `log_debug()` writes to `debug/game_debug.log`, NOT println!
3. **Check research first** - Before investigating, check docs/RESEARCH_INDEX.md
4. **Update this file** - When adding/removing features, update CLAUDE.md
5. **Analyze failures immediately** - When operation fails, immediately:
   - Analyze the cause
   - Document the solution in CLAUDE.md (Common Issues section)
   - Do this WITHOUT user reminder
6. **Log errors for debugging** - When build/run fails:
   - Errors auto-logged to `debug/build_output.log`
   - Read this file to see compiler errors
   - Fix errors based on log content
7. **Safe code removal** - Before removing imports/functions/code:
   - Use Grep to find ALL usages in codebase
   - "Disabled at runtime" ≠ "safe to remove" (code still compiles!)
   - Remove import only if function has ZERO references
   - Example: `enemies = Vec::new()` disables enemies but `spawn_enemies()` still uses `generate_player_mannequin`

---

## Git Workflow

**Commit format:**
```
Sessions N-M: [System] - [Feature list]
```

**After each session:**
```bash
git add -A
git commit -m "Sessions N-M: [description]"
git push
```

**Always:**
1. Update PROGRESS.md after each session
2. Record next steps for following session

---

## Session Checklists

### Start:
- [ ] Read PROGRESS.md (latest session)
- [ ] Check docs/RESEARCH_INDEX.md for relevant prior research
- [ ] Run `cargo build` to verify state
- [ ] Plan 1-3 tasks for this session

### End:
- [ ] Update PROGRESS.md with what was done
- [ ] If research was done, add to docs/ and update RESEARCH_INDEX.md
- [ ] Verify `cargo build && cargo run` works
- [ ] Commit changes

---

## Research Documents

**Before starting new research, check existing docs:**

| Topic | Document |
|-------|----------|
| **Human Anatomy** | [docs/HUMAN_ANATOMY_REFERENCE.md](docs/HUMAN_ANATOMY_REFERENCE.md) |
| **Skeleton Proportions** | [docs/SKELETON_PROPORTIONS.md](docs/SKELETON_PROPORTIONS.md) |
| Character Animation | [docs/HUMANOID_ANIMATION_RESEARCH.md](docs/HUMANOID_ANIMATION_RESEARCH.md) |
| Animation Code Review | [docs/ANIMATION_CODE_REVIEW.md](docs/ANIMATION_CODE_REVIEW.md) |
| Active Ragdoll | [docs/ACTIVE_RAGDOLL_RESEARCH.md](docs/ACTIVE_RAGDOLL_RESEARCH.md) |
| Skeleton System | [docs/SKELETAL_ANIMATION_SYSTEM.md](docs/SKELETAL_ANIMATION_SYSTEM.md) |
| 3D Math | [docs/3D_SPACE_UNDERSTANDING.md](docs/3D_SPACE_UNDERSTANDING.md) |
| All Research | [docs/RESEARCH_INDEX.md](docs/RESEARCH_INDEX.md) |

---

## Key Patterns

### Camera-relative movement
Player WASD movement is relative to camera direction, not player facing.

### Quaternion handling
Always ensure shortest path when interpolating rotations (check dot product sign).

### Joint optimization
- Hinge joints (1 DOF) for knees/elbows - 3x faster
- Ball joints (3 DOF) for shoulders/hips - 1.5x faster

---

## Common Issues

**Build fails:**
- Need Microsoft C++ Build Tools on Windows
- See BUILD_SETUP.md
- "cargo not recognized": run.bat now auto-loads `%USERPROFILE%\.cargo\env.bat`

**Import removal breaks build:**
- Cause: Removed import for function still used elsewhere in codebase
- Example: Removed `generate_player_mannequin` thinking "enemies disabled", but `spawn_enemies()` still compiles
- Prevention: Always grep for usages before removing ANY import
- Fix: Re-add the import, or remove all usages first

**Render issues:**
- Check GPU backend: `cargo run` shows detected GPU
- Uses `Backends::PRIMARY` (D3D12 on Windows, Metal on macOS, Vulkan on Linux)

**D3D12 INVALID_SUBRESOURCE_STATE fix (wgpu 24.0+):**
- Problem: wgpu 22.x had bugs with D3D12 resource state transitions
- Symptom: Validation errors `INVALID_SUBRESOURCE_STATE #538` in console
- Root cause: Swapchain textures require explicit PRESENT → RENDER_TARGET → PRESENT transitions
- Solution: Upgraded to wgpu 24.0 which handles transitions correctly
- Screenshot: Render to separate offscreen texture with COPY_SRC, don't copy from swapchain

**Skeleton rendering tapered capsules fix (Session 16+):**
- Problem: Shader-based taper applied in LOCAL Y space, not accounting for bone rotation
- Solution: Pre-generated geometry approach:
  1. Created `BoneType` enum to group similar bones (UpperArm for left/right)
  2. Function `generate_tapered_capsule_real(length, radius_top, radius_bottom)` generates mesh with REAL dimensions
  3. Each bone type has its own mesh with correct proportions baked in
  4. Shader just applies position/rotation (no scaling/taper logic)
  5. Fixed `BoneInstance` memory layout: `color: [f32; 4]` for proper alignment
- Key files: `src/rendering/skeleton_renderer.rs`, `assets/shaders/skeleton.wgsl`

**Claude Code Bash tool limitation:**
- Bash tool uses Linux shell, cargo not available there
- Cannot run build/test from Claude Code directly
- After code changes, ask user to run `run.bat` to build and test
- `run.bat` does: cargo build → run exe if success
- Build errors logged to: `debug/build_output.log` - read this file after failed build!
- Note: Windows doesn't have `tee`, use `2>` redirect for stderr

---

## Philosophy

> "The sword leads the hand, not animation leads the player"

**Five Pillars:**
1. Directional Input - attack direction from mouse
2. Fluid Movement - move during attacks
3. Low Animation Commitment - cancel early
4. Weight & Impact - hitstop, knockback
5. Readable Combat - clear feedback

---

## Links

- **Progress:** [PROGRESS.md](PROGRESS.md)
- **Methodology:** [METHODOLOGY.md](METHODOLOGY.md)
- **Research Index:** [docs/RESEARCH_INDEX.md](docs/RESEARCH_INDEX.md)
- **GDD:** [docs/arena_combat_gdd.md](docs/arena_combat_gdd.md)
- **GitHub:** https://github.com/zhukotanskiyphilip/arena-combat
