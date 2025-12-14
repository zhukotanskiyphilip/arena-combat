# Logging та Debug

## Правила логування

1. **НЕ використовувати println!** - логи мають йти в файл
2. **Файл логів:** `debug/game_debug.log`
3. **Функція:** `log_debug(&format!("..."))`

## Як логувати в main.rs

```rust
// Функція визначена в main.rs
log_debug(&format!("MOUSE: yaw={:.1}°", yaw.to_degrees()));
```

## Як логувати в інших файлах (renderer.rs тощо)

Потрібно:
1. Додати параметр логера в функцію, або
2. Зробити log_debug публічним і експортувати

## Де дивитися логи

```
c:\Claude\arena_combat\debug\game_debug.log
```

## Запуск з логуванням

```batch
cd /d c:\Claude\arena_combat
cargo run
:: логи будуть у debug/game_debug.log
```

## Debug print тільки при зміні значення

```rust
static mut LAST_VALUE: f32 = 999.0;
unsafe {
    if (LAST_VALUE - new_value).abs() > 0.01 {
        log_debug(&format!("VALUE: {:.1}", new_value));
        LAST_VALUE = new_value;
    }
}
```

## Важливі лог-повідомлення

- `SYNC:` - одноразова синхронізація player.yaw з камерою
- `MOUSE:` - зміна yaw від мишки
- `Q_KEY:` / `E_KEY:` - зміна yaw від клавіш
- `UPDATE_PLAYER:` - виклик update_player() в renderer
- `BODY FORWARD:` - напрямок куди дивиться тіло гравця
