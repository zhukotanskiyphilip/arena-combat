# Arena Combat Prototype
> Third-person melee combat game with directional fluid combat

🚧 **Status:** Early Development (Phase 1) 🚧

---

## 📖 Про проект

Arena Combat - це прототип бойової системи для 3D файтингу в стилі **Jedi Academy** та **Mount & Blade**. Основна ідея: **fluid melee combat** де напрямок атаки визначається рухом миші, а гравець зберігає мобільність під час бою.

### Ключові особливості:

- **Directional combat** - 8 напрямків атаки (контроль мишею)
- **Fluid movement** - можна рухатись під час атаки
- **Low animation commitment** - більшість дій можна скасувати
- **Readable combat** - зрозуміло що робить противник

---

## 🎯 Поточний статус

### Phase 1: Singleplayer (В розробці)
- [x] Rust проект створено
- [x] Базове вікно (winit)
- [ ] 3D rendering (wgpu)
- [ ] Camera controller
- [ ] 3D модель манекена
- [ ] Fluid movement (WASD)
- [ ] Directional attacks
- [ ] Block/Parry system
- [ ] AI opponent

### Phase 2: LAN Multiplayer (Планується)
- [ ] UDP netcode
- [ ] Local network discovery
- [ ] Input synchronization
- [ ] (Опційно) Rollback netcode (GGRS)

---

## 🛠️ Технології

- **Мова:** Rust 1.92+
- **Rendering:** wgpu (Vulkan/DirectX 12/Metal)
- **Window:** winit
- **Math:** glam
- **NO game engine** - повний контроль над кодом

**Чому Rust?**
- Продуктивність на рівні C++
- Memory safety (менше crashes)
- Детермінізм (готовність до netcode)
- Чудова екосистема для ігор

---

## 🚀 Як запустити

### Вимоги:
- Rust 1.70+ ([встановити](https://rustup.rs/))
- Git

### Збірка:
```bash
git clone https://github.com/YOUR_USERNAME/arena-combat.git
cd arena-combat

# Debug build
cargo run

# Release build (оптимізована)
cargo run --release

# З логуванням
RUST_LOG=info cargo run
```

### Контроли:
- **ESC** або **[X]** - закрити вікно
- *(Більше контролів буде додано)*

---

## 📁 Структура проекту

```
arena_combat/
├── src/
│   ├── main.rs              # Entry point
│   ├── core/                # Game logic (буде)
│   ├── rendering/           # Graphics (буде)
│   ├── ai/                  # AI opponent (буде)
│   └── input/               # Controls (буде)
│
├── docs/                    # Документація
│   ├── arena_combat_gdd.md  # Game Design Document
│   ├── tech_stack_decision.md
│   └── PROGRESS.md          # Журнал розробки
│
├── Cargo.toml               # Залежності Rust
└── README.md                # Цей файл
```

---

## 📚 Документація

- [Game Design Document](arena_combat_gdd.md) - філософія гри, механіки
- [Technical Stack](tech_stack_decision.md) - технічні рішення
- [Progress Log](PROGRESS.md) - детальний журнал розробки

---

## 🎮 Геймплейна філософія

> **"Меч веде руку, не анімація веде гравця"**

Це не Dark Souls (commitment-based) і не DMC (combo strings).
Це fluid combat де:
- Ти постійно контролюєш зброю
- Можеш змінити напрямок mid-swing
- Читаєш противника через анімації, не UI

**Референси:**
- **Jedi Academy** - fluid movement + lightsaber combat
- **Mount & Blade** - directional melee system
- **Blade of Darkness** - weight & impact

---

## 🤝 Розробка

Проект розробляється з допомогою **AI-assisted development**. Вся документація структурована так, щоб новий розробник (людина чи AI) міг швидко підхопити контекст.

### Хочете долучитись?
1. Прочитайте [PROGRESS.md](PROGRESS.md) - поточний статус
2. Ознайомтесь з [GDD](arena_combat_gdd.md) - філософія гри
3. Перевірте Issues - що треба зробити

---

## 📝 Ліцензія

TBD (To Be Determined)

---

## 🔗 Корисні посилання

- [Rust Book](https://doc.rust-lang.org/book/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/) (для референсів)

---

---

## 🤝 Як долучитись

Ознайомтесь з [CONTRIBUTING.md](CONTRIBUTING.md) для деталей.

Короткий процес:
1. Fork проекту
2. Створити feature бранч
3. Закомітити зміни
4. Відкрити Pull Request

---

## 📜 Ліцензія

Цей проект ліцензовано під MIT License - дивіться [LICENSE](LICENSE).

---

**Останнє оновлення:** 2025-12-11
**Версія:** 0.1.0
**Розробник:** zhukotanskiyphilip
**Репозиторій:** https://github.com/zhukotanskiyphilip/arena-combat
