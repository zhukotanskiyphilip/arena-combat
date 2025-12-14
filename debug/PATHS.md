# Project Paths

## Windows Native (PowerShell/CMD)
```
cd c:\Claude\arena_combat
cargo run
```

## WSL / Bash
```
cd /mnt/c/Claude/arena_combat
cargo run
```

## Project Root
- Windows: `c:\Claude\arena_combat`
- WSL: `/mnt/c/Claude/arena_combat`

## Key Directories
- Source: `src/`
- Shaders: `assets/shaders/`
- Debug docs: `debug/`

## Key Source Files
- `src/main.rs` - головний файл, event loop, input handling, player/camera update
- `src/camera/camera.rs` - Camera struct, rotate_third_person(), update_third_person()
- `src/player/player.rs` - Player struct (position, yaw)
- `src/rendering/renderer.rs` - WgpuRenderer, update_player()
- `src/rendering/mesh.rs` - Mesh, MeshVertex, generate_player_body()
- `src/input/input_state.rs` - InputState, mouse_right, mouse_delta()
- `src/combat/mod.rs` - Combat, weapon swing
- `src/transform/transform.rs` - Transform struct

## Debug Files
- `debug/BUG_001_player_rotation.md` - поточний баг
- `debug/DEVELOPMENT_RULES.md` - правила розробки
- `debug/PATHS.md` - цей файл
