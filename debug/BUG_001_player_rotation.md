# BUG 001: Player body doesn't rotate with camera

## Status: RESOLVED ✓

## Original Problem
Player body mesh doesn't rotate when camera rotates around player.
The player should always face the camera direction (show back to camera).

## Resolution (2025-12-14)
**Код працює правильно!** Логи доводять що:
- `player.yaw` змінюється при русі мишкою
- `UPDATE_PLAYER` викликається
- GPU отримує нову матрицю

**Причина "візуального" непрацювання:**
Камера і персонаж обертаються СИНХРОННО на однаковий кут.
Тому ракурс не змінюється - персонаж завжди показує спину до камери.
Це ОЧІКУВАНА поведінка для третьоособової камери!

## Log Evidence
```
MOUSE: cam_yaw=90.7° player_yaw=270.7° delta=0.009
UPDATE_PLAYER: yaw=270.7° rotation=(0.000, 0.703, 0.000, -0.711)
```
Різниця cam/player завжди ~180° = спина до камери.

## ROOT CAUSE FOUND

**`camera.yaw` doesn't change when rotating camera with mouse!**

The visual camera rotation works (view changes), but `camera.yaw` field
stays at initial value. This means `player.yaw = camera.yaw + PI` always
sets the same value.

## Evidence

### Test: Q/E manual rotation
- Added Q/E keys to manually rotate player body
- **Q/E WORKS** - body rotates while key is held
- When key released, body snaps back to `camera.yaw + PI`
- Since `camera.yaw` never changes, body always returns to same angle

### Test: Auto-rotation (PASSED)
```rust
static mut AUTO_YAW: f32 = 0.0;
AUTO_YAW += 0.02;
let rotation = Quat::from_rotation_y(AUTO_YAW);
```
**Result: Body DOES rotate!** Proves GPU/shader work correctly.

## The Real Bug

`camera.rotate_third_person()` is called but either:
1. `camera.yaw` field is not being updated
2. A different camera instance is being modified
3. `camera.yaw` is reset somewhere else

## Camera Code Analysis

```rust
// main.rs line 276-277
renderer.camera.rotate_third_person(delta_yaw, delta_pitch);

// camera.rs rotate_third_person()
pub fn rotate_third_person(&mut self, delta_yaw: f32, delta_pitch: f32) {
    self.yaw += delta_yaw;  // <-- Should update yaw
    self.pitch += delta_pitch;
    // ... clamp pitch ...
}
```

## Next Steps

1. Check if there's another place that resets camera.yaw
2. Check `update_third_person()` - does it overwrite yaw?
3. Print camera.yaw in rotate_third_person to verify it changes

## FIX APPLIED (2025-12-14)

### Проблема
```rust
// СТАРИЙ КОД (НЕ ПРАЦЮВАВ):
if self.input_state.mouse_right {
    renderer.camera.rotate_third_person(delta_yaw, delta_pitch);
}
// else блок встановлював player.yaw = camera.yaw + PI
// Це СКИДАЛО кут кожен кадр!
```

### Рішення
```rust
// НОВИЙ КОД (ПРАЦЮЄ):
if self.input_state.mouse_right {
    renderer.camera.rotate_third_person(delta_yaw, delta_pitch);
    // Гравець обертається РАЗОМ з камерою
    self.player.yaw += delta_yaw;  // <-- DELTA, не абсолютне значення!
}
```

### Чому це працює
- Q/E додавали delta до player.yaw - працювало
- Камера теж повинна додавати delta, а не встановлювати абсолютне значення
- `player.yaw = camera.yaw + PI` не працювало бо camera.yaw використовується інакше

## Controls
- **Q/E** - ручне обертання тіла (працює)
- **ПКМ + мишка** - обертання камери І тіла разом

## Files Involved
- src/main.rs:268-289 (camera + player rotation)
- src/rendering/renderer.rs (update_player)
- src/rendering/mesh.rs (red arrow indicator for debugging)
