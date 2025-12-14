# BUG 001: Детальне розслідування

## Проблема
Персонаж НЕ обертається при повороті камери мишкою.
Q/E кнопки ПРАЦЮЮТЬ - персонаж обертається.

## Що ми знаємо точно

### 1. Q/E працює
- Натискаємо Q/E - персонаж візуально обертається
- Код: `self.player.yaw -= 2.0 * delta` (main.rs:302-306)

### 2. Логи показують що код виконується
```
MOUSE: cam_yaw=36.1° player_yaw=576.1° delta=0.015
UPDATE_PLAYER: yaw=575.1° rotation=(0.000, -0.953, 0.000, 0.302)
GPU upload model[0]: [-0.808, 0.000, 0.590, 0.000]
```
- `player.yaw` змінюється
- `UPDATE_PLAYER` викликається
- Quaternion змінюється
- GPU upload відбувається

### 3. Але візуально персонаж НЕ обертається при мишці

## Гіпотези

### Гіпотеза A: Камера і персонаж обертаються разом
Якщо камера і персонаж обертаються на однаковий кут, то ракурс не змінюється.
**АЛЕ:** Користувач каже що на старті бачив БІК персонажа, не спину.

### Гіпотеза B: Синхронізація при старті неправильна
На старті: `player.yaw = camera.yaw + PI`
Це має дати спину до камери. Але користувач бачить бік.

### Гіпотеза C: Щось перезаписує player.yaw
Може десь є інший код що скидає yaw?

### Гіпотеза D: Transform не застосовується до правильного mesh
Може `player_mesh` це не той mesh що рендериться?

## Тести для перевірки

### Тест 1: Перевірити початковий стан
- Який yaw камери на старті?
- Який yaw персонажа на старті?
- Що бачить користувач? (спина/бік/обличчя)

### Тест 2: Перевірити що Q/E і мишка змінюють той самий player.yaw
- Додати лог в Q/E блок
- Порівняти з MOUSE логом

### Тест 3: Перевірити порядок виконання
1. player.yaw змінюється (main.rs)
2. update_player викликається (main.rs)
3. transform.rotation встановлюється (renderer.rs)
4. update_transform викликається (renderer.rs)
5. render викликається

## Код що змінює player.yaw

### main.rs (mouse):
```rust
if self.input_state.mouse_right {
    let (delta_x, delta_y) = self.input_state.mouse_delta();
    if delta_x.abs() > 0.1 || delta_y.abs() > 0.1 {
        renderer.camera.rotate_third_person(delta_yaw, delta_pitch);
        self.player.yaw += delta_yaw;  // <-- ТУТ
    }
}
```

### main.rs (Q/E):
```rust
if self.input_state.is_q_pressed() {
    self.player.yaw -= 2.0 * delta;  // <-- ТУТ
}
if self.input_state.is_e_pressed() {
    self.player.yaw += 2.0 * delta;  // <-- ТУТ
}
```

## Порядок викликів в RedrawRequested

1. game_time.update()
2. fps_counter.tick()
3. enemies_spawned check
4. **player_synced check** (one-time sync)
5. combat.update()
6. enemies.update()
7. renderer.update_enemies()
8. **CAMERA + PLAYER UPDATE block:**
   - mouse_right check -> player.yaw += delta_yaw
   - Q/E check -> player.yaw += delta
   - movement
9. **renderer.update_player()** -> transform.rotation = Quat::from_rotation_y(yaw)
10. camera.update_third_person()
11. renderer.render()

## Файли

- main.rs:280-306 - mouse + Q/E rotation
- main.rs:341-343 - update_player call
- renderer.rs:539-577 - update_player implementation
- mesh.rs:679-700 - update_transform
