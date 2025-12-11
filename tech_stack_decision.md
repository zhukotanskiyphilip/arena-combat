# Технічний стек Arena Combat
## Рішення щодо мови програмування та архітектури

---

## Підсумок рекомендацій

### ✅ **Обрана мова: Rust**
### ✅ **Архітектура: P2P Rollback Netcode**
### ✅ **Engine: Bevy (ECS)**

---

## 1. Обґрунтування вибору Rust

### Критичні переваги для Arena Combat:

#### 1.1. Rollback Netcode - готове рішення
Ваша гра **ВИМАГАЄ** rollback через fluid combat філософію. У Rust є **найкраща open-source реалізація** rollback netcode.

**Бібліотека GGRS:**
- Використовується в production fighting games
- Повна інтеграція з Bevy
- Підтримка до 8 гравців (для майбутнього)
- Spectator mode вбудований
- Battle-tested

**Альтернативи в інших мовах:**
- C++: GGPO (старіша, менш підтримувана)
- Інші мови: нічого production-ready

#### 1.2. Детермінізм за дизайном
Rust змушує вас писати детермінований код:
```rust
// Компілятор НЕ дозволить:
- Data races (multiple threads writing to same data)
- Undefined behavior
- Use-after-free

// Це критично для rollback, де малі відмінності = desync
```

#### 1.3. Memory safety = стабільний мультиплеєр
Статистика: 70% crashes в C++ multiplayer games = memory bugs.
Rust практично виключає це **на етапі компіляції**.

#### 1.4. Продуктивність
- Zero-cost abstractions
- Немає Garbage Collection паuz
- SIMD оптимізації (для batch updates)
- Benchmark: 98-100% швидкості C++

### Недоліки Rust (та як їх пом'якшити):

**1. Крива навчання**
- *Рішення:* Почати з Bevy туторіалів (Bevy простіший за чистий Rust)
- *Час:* 2-3 тижні для basics, 1-2 місяці для впевненості

**2. Compilation time**
- *Рішення:* `sccache`, incremental compilation, hot reloading в Bevy
- *Реальність:* Full rebuild 30-60 сек (vs 10-20 сек C++), incremental 2-5 сек

**3. Менше туторіалів для melee combat**
- *Рішення:* Fighting game туторіали (механіки схожі)
- *Приклад:* Kataster (open-source fighting game на Bevy)

---

## 2. Порівняння з C++

| Аспект | Rust | C++ | Переможець |
|--------|------|-----|-----------|
| **Rollback netcode** | ggrs (excellent) | GGPO (dated) | **Rust** |
| **Детермінізм** | Гарантований компілятором | Manual контроль | **Rust** |
| **Memory safety** | Compile-time checks | Runtime crashes | **Rust** |
| **Raw performance** | 98-100% | 100% | Нічия |
| **Compile time** | Повільна | Середня | C++ |
| **Dev speed** | Швидка (після навчання) | Повільна (debugging) | **Rust** |
| **Екосистема (загальна)** | Зростаюча | Велика | C++ |
| **Екосистема (netcode)** | Modern | Legacy | **Rust** |
| **Cross-platform** | Відмінна | Хороша | **Rust** |
| **Learning curve** | Steep | Medium | C++ |

**Коли обирати C++:**
- Команда вже експертна в C++
- Використовується Unreal Engine
- Потрібні останні 2% performance (нереалістично для прототипу)

**Наш випадок:** Rust перемагає в 7/10 критичних аспектах.

---

## 3. Архітектура мультиплеєра

### 3.1. Чому Rollback, а не Delay-based?

**Ваш GDD каже:**
> "Меч веде руку, не анімація веде гравця"
> "Low animation commitment"

**Delay-based netcode:**
```
Input → Wait 150ms → Server confirms → Action
```
❌ Це **вбиває** responsive feel!

**Rollback netcode:**
```
Input → Instant local action → Rollback if needed
```
✅ Зберігає fluid combat!

**Приклади:**
- Fighting games (Street Fighter V, Guilty Gear Strive) = rollback
- Souls-like (Dark Souls, Elden Ring) = delay-based (OK для slow combat)
- For Honor = hybrid (працює, але laggy)

### 3.2. Трифазний план розробки

#### **Phase 1: Offline (2-3 місяці)**
**Мета:** Довести core combat feel

```
┌─────────────────────┐
│  Single Instance    │
│                     │
│  P1 Input → ┐      │
│              ↓      │
│        Game Logic   │
│              ↓      │
│  P2 Input → ┘      │
└─────────────────────┘
```

**Deliverables:**
- [ ] Fluid movement (WASD, sprint, dodge)
- [ ] Directional attacks (8 напрямків)
- [ ] Block/parry system
- [ ] Hit detection
- [ ] Stamina system
- [ ] Local 1v1 на одному ПК

**Критично:** Вся логіка ДЕТЕРМІНОВАНА (готуємось до rollback)

---

#### **Phase 2: P2P Rollback (1-2 місяці)**
**Мета:** Online 1v1 duels

```
┌─ Player 1 PC ─┐         ┌─ Player 2 PC ─┐
│ Game Instance │◄───UDP──►│ Game Instance │
│ Frame: 1000   │         │ Frame: 1000   │
│               │         │               │
│ Rollback      │         │ Rollback      │
│ Manager       │         │ Manager       │
└───────────────┘         └───────────────┘
```

**Deliverables:**
- [ ] GGRS integration
- [ ] Input serialization
- [ ] State save/load (snapshots)
- [ ] Lobby system (matchmaking basic)
- [ ] Latency display
- [ ] Rollback до 8 frames

**Виклики:**
- Desync debugging (logs + checksums)
- Prediction quality
- Visual artifacts при rollback

---

#### **Phase 3: Dedicated Server (опційно, 2-3 місяці)**
**Мета:** Ranked, anti-cheat, tournaments

```
┌─ Client 1 ─┐    ┌─── Server ───┐    ┌─ Client 2 ─┐
│ Rendering  │───►│ Authoritative│◄───│ Rendering  │
│ Prediction │◄───│ Game State   │───►│ Prediction │
└────────────┘    │ Tick: 60Hz   │    └────────────┘
                  └──────────────┘
```

**Deliverables:**
- [ ] Authoritative server
- [ ] Client prediction
- [ ] Lag compensation
- [ ] Cheat detection
- [ ] Replay system
- [ ] Ranked ladder

**Примітка:** Це для майбутнього. Rollback P2P достатньо для успішної гри.

---

### 3.3. Технічні деталі Rollback

#### Детермінізм: Вимоги

**1. Fixed-point math:**
```rust
use fixed::types::I32F32; // 32.32 fixed point

struct Transform {
    x: I32F32,  // НЕ f32! Float non-deterministic на різних CPU
    y: I32F32,
    rotation: I32F32,
}
```

**2. No random() в simulation:**
```rust
// ❌ НЕ можна:
let damage = base_damage + random(0..10);

// ✅ Можна:
let damage = base_damage + hash(attacker_id + frame) % 10;
// (Детерміновано якщо seed синхронізований)
```

**3. No system time:**
```rust
// ❌ НЕ можна:
let elapsed = SystemTime::now();

// ✅ Можна:
let elapsed = game_state.frame * FRAME_TIME; // 1/60 sec
```

**4. Deterministic iteration:**
```rust
// ❌ НЕ можна:
for entity in query.iter() { }  // Порядок не гарантований

// ✅ Можна:
let mut entities: Vec<_> = query.iter().collect();
entities.sort_by_key(|e| e.id);  // Детермінований порядок
for entity in entities { }
```

#### Snapshot System

```rust
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct GameState {
    frame: u32,
    player1: PlayerState,
    player2: PlayerState,
    projectiles: Vec<Projectile>,
    // Все що може змінитись
}

struct SnapshotManager {
    // Circular buffer останніх 60 frames
    snapshots: [Option<GameState>; 60],
    current_frame: u32,
}

impl SnapshotManager {
    fn save(&mut self, state: &GameState) {
        let idx = (state.frame % 60) as usize;
        self.snapshots[idx] = Some(state.clone());
    }

    fn load(&self, frame: u32) -> Option<GameState> {
        let idx = (frame % 60) as usize;
        self.snapshots[idx].clone()
    }

    fn rollback(&self, current: u32, target: u32) -> Option<GameState> {
        assert!(current - target <= 8, "Max rollback 8 frames!");
        self.load(target)
    }
}
```

#### Input Delay Buffer

```rust
struct InputManager {
    // Локальні інпути вперед
    local_inputs: VecDeque<Input>,
    // Отримані інпути від опонента
    remote_inputs: HashMap<u32, Input>, // frame -> input

    delay_frames: u32, // 2-3 frames
}

impl InputManager {
    fn get_input_for_frame(&self, frame: u32, is_local: bool) -> Input {
        if is_local {
            // Локальний гравець грає на frame - delay
            self.local_inputs.get((frame - self.delay_frames) as usize)
                .copied()
                .unwrap_or(Input::default())
        } else {
            // Віддалений інпут (або prediction)
            self.remote_inputs.get(&frame)
                .copied()
                .unwrap_or_else(|| self.predict_input(frame))
        }
    }

    fn predict_input(&self, frame: u32) -> Input {
        // Проста prediction: копіюємо останній відомий input
        self.remote_inputs.get(&(frame - 1))
            .copied()
            .unwrap_or(Input::default())
    }
}
```

---

## 4. Технічний стек (Rust)

### 4.1. Core Dependencies

```toml
[dependencies]
# Game Engine
bevy = { version = "0.12", features = ["dynamic_linking"] }

# Networking & Rollback
ggrs = "0.10"              # Rollback netcode library
bevy_ggrs = "0.15"         # Bevy integration for GGRS
renet = "0.0.15"           # Reliable UDP transport
bevy_renet = "0.0.12"      # Bevy integration for renet

# Math & Physics
glam = "0.24"              # Math library (векторна математика)
fixed = "1.24"             # Fixed-point arithmetic
rapier3d = "0.17"          # 3D physics (опційно, можливо custom)

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"            # Бінарна серіалізація (швидка)

# Input
leafwing-input-manager = "0.11"  # Зручна система інпутів

# Audio
bevy_kira_audio = "0.18"   # 3D spatial audio

# Dev tools
bevy-inspector-egui = "0.21"  # Runtime inspector
bevy_mod_debugdump = "0.9"    # System graph visualization
```

### 4.2. Структура проекту

```
arena_combat/
├── Cargo.toml
├── .gitignore
├── README.md
│
├── assets/                    # Game resources
│   ├── models/
│   │   ├── mannequin.glb
│   │   └── weapons/
│   ├── sounds/
│   │   ├── sword_swing.ogg
│   │   └── hit_impact.ogg
│   ├── shaders/
│   └── materials/
│
├── src/
│   ├── main.rs               # Entry point, Bevy app setup
│   │
│   ├── lib.rs                # Library root (для tests)
│   │
│   ├── core/                 # ✅ ДЕТЕРМІНОВАНА логіка (NO IO)
│   │   ├── mod.rs
│   │   │
│   │   ├── state.rs          # GameState struct (Clone + Serialize)
│   │   ├── input.rs          # Input enum & buffering
│   │   │
│   │   ├── combat/
│   │   │   ├── mod.rs
│   │   │   ├── attack.rs     # Attack detection & damage
│   │   │   ├── defense.rs    # Block, parry, dodge
│   │   │   └── stagger.rs    # Hitstun calculation
│   │   │
│   │   ├── physics/
│   │   │   ├── mod.rs
│   │   │   ├── collision.rs  # Hitbox vs hurtbox
│   │   │   └── movement.rs   # Kinematic movement
│   │   │
│   │   └── math/
│   │       ├── fixed_point.rs  # Fixed-point wrappers
│   │       └── deterministic.rs # Детерміновані утиліти
│   │
│   ├── systems/              # Bevy ECS systems
│   │   ├── mod.rs
│   │   │
│   │   ├── player/
│   │   │   ├── input_system.rs
│   │   │   ├── movement_system.rs
│   │   │   └── animation_system.rs
│   │   │
│   │   ├── combat/
│   │   │   ├── attack_system.rs
│   │   │   ├── block_system.rs
│   │   │   └── damage_system.rs
│   │   │
│   │   └── stamina_system.rs
│   │
│   ├── network/              # Multiplayer (Phase 2)
│   │   ├── mod.rs
│   │   │
│   │   ├── rollback.rs       # GGRS setup & callbacks
│   │   ├── matchmaking.rs    # Lobby, P2P connection
│   │   ├── sync.rs           # State synchronization
│   │   └── packets.rs        # Network message types
│   │
│   ├── rendering/            # ❌ НЕ впливає на simulation
│   │   ├── mod.rs
│   │   │
│   │   ├── mannequin.rs      # 3D model rendering
│   │   ├── effects/
│   │   │   ├── trails.rs     # Weapon trails
│   │   │   ├── particles.rs  # Blood, dust
│   │   │   └── hitstop.rs    # Screen freeze effect
│   │   │
│   │   ├── camera.rs         # Camera follow & shake
│   │   └── ui.rs             # Health bars, stamina
│   │
│   ├── audio/
│   │   ├── mod.rs
│   │   └── spatial_sound.rs
│   │
│   ├── resources/            # Bevy resources
│   │   ├── config.rs         # Game settings
│   │   └── assets.rs         # Asset handles
│   │
│   └── plugins/              # Bevy plugins (модулі)
│       ├── game_plugin.rs    # Main game logic
│       ├── network_plugin.rs # Networking
│       └── debug_plugin.rs   # Dev tools
│
└── tests/
    ├── determinism_test.rs   # Тест детермінізму
    ├── rollback_test.rs      # Тест rollback
    └── combat_test.rs        # Unit tests бою
```

### 4.3. Модульна структура (Separation of Concerns)

**Принцип:** Детермінована логіка відокремлена від I/O.

```rust
// ❌ ПОГАНО: Логіка змішана з rendering
fn attack_system(
    mut query: Query<&mut Player>,
    mut gizmos: Gizmos,  // Rendering!
) {
    for mut player in &mut query {
        player.attack();
        gizmos.line(/* draw attack arc */);  // Side effect!
    }
}

// ✅ ДОБРЕ: Логіка окремо
// core/combat/attack.rs (детермінована)
pub fn calculate_attack(state: &GameState, input: Input) -> AttackResult {
    // Чиста функція, NO side effects
}

// systems/combat/attack_system.rs (Bevy)
fn attack_system(
    mut game_state: ResMut<GameState>,
    inputs: Res<InputBuffer>,
) {
    let result = calculate_attack(&game_state, inputs.current());
    game_state.apply(result);
}

// rendering/effects/trails.rs (візуальне)
fn render_attack_trails(
    query: Query<&AttackState>,
    mut gizmos: Gizmos,
) {
    for attack in &query {
        if attack.is_active {
            gizmos.line(/* draw */);
        }
    }
}
```

---

## 5. Development Roadmap

### Milestone 1: Offline Prototype (3 місяці)
**Deliverable:** Playable local 1v1

**Week 1-2: Setup & Basic Movement**
- [ ] Rust + Bevy project setup
- [ ] Basic 3D scene (arena, mannequins)
- [ ] WASD movement
- [ ] Camera controls
- [ ] Sprint & jump

**Week 3-4: Directional Input**
- [ ] Mouse delta tracking
- [ ] 8-directional input detection
- [ ] Light attack (animation placeholder)
- [ ] Attack direction visualization (debug)

**Week 5-6: Hit Detection**
- [ ] Weapon hitbox (capsule)
- [ ] Player hurtbox (cylinder)
- [ ] Collision detection
- [ ] Basic damage system
- [ ] Health bar UI

**Week 7-8: Combat Mechanics**
- [ ] Heavy attack (charge)
- [ ] Block system (directional)
- [ ] Stamina drain/regen
- [ ] Stagger on hit

**Week 9-10: Advanced Combat**
- [ ] Parry timing window
- [ ] Dodge/roll with i-frames
- [ ] Combo chains (3-hit)
- [ ] Attack cancel (feint)

**Week 11-12: Polish & Feel**
- [ ] Hitstop implementation
- [ ] Camera shake
- [ ] Sound effects
- [ ] Weapon trails (visual)
- [ ] Playtesting & balance

---

### Milestone 2: Online Multiplayer (2 місяці)
**Deliverable:** P2P netcode працює

**Week 13-14: Determinism Prep**
- [ ] Migrate до fixed-point math
- [ ] Remove all non-deterministic code
- [ ] Implement GameState serialization
- [ ] Write determinism tests

**Week 15-16: GGRS Integration**
- [ ] Add ggrs + bevy_ggrs dependencies
- [ ] Implement ggrs::GameState trait
- [ ] Local rollback testing (simulated latency)
- [ ] Snapshot save/load

**Week 17-18: Networking**
- [ ] UDP transport (renet)
- [ ] Lobby/matchmaking UI
- [ ] P2P connection establishment
- [ ] Input serialization & send

**Week 19-20: Rollback Testing & Debug**
- [ ] Desync detection (checksums)
- [ ] Rollback visualization (debug)
- [ ] Latency testing (50ms, 100ms, 150ms)
- [ ] Prediction improvement

---

### Milestone 3: Content & Polish (1-2 місяці)
**Deliverable:** Refined gameplay

- [ ] Multiple weapon types
- [ ] Arena variations
- [ ] More attack moves
- [ ] Advanced combos
- [ ] Ranked matchmaking
- [ ] Replay system
- [ ] Tournament mode

---

## 6. Performance Targets

### Frame Rate
- **Target:** Locked 60 FPS
- **Minimum:** 60 FPS (дропи неприйнятні для fighting game)

### Netcode
- **Input latency:** < 50ms locally
- **Rollback tolerance:** Up to 8 frames (133ms @ 60fps)
- **Ideal ping:** < 50ms
- **Playable ping:** < 150ms
- **Max ping:** 200ms (degraded experience)

### Memory
- **GameState size:** < 10 KB (для швидкого clone)
- **Total RAM:** < 500 MB
- **VRAM:** < 1 GB

---

## 7. Testing Strategy

### Unit Tests (Rust)
```rust
#[test]
fn test_determinism() {
    let state1 = GameState::new();
    let state2 = GameState::new();

    let input = Input::Attack(Direction::Top);

    let result1 = simulate(state1, input);
    let result2 = simulate(state2, input);

    assert_eq!(result1, result2, "Same input = same result!");
}

#[test]
fn test_rollback_consistency() {
    let mut state = GameState::new();

    // Simulate 10 frames
    for i in 0..10 {
        state = simulate(state, Input::Idle);
    }
    let checkpoint = state.clone();

    // Rollback to frame 5
    let mut rolled_back = snapshots[5].clone();

    // Re-simulate 5->10
    for i in 5..10 {
        rolled_back = simulate(rolled_back, Input::Idle);
    }

    assert_eq!(checkpoint, rolled_back, "Rollback consistency!");
}
```

### Integration Tests
- Local multiplayer (split-screen)
- Simulated network latency
- Packet loss scenarios

### Playtesting Metrics
- Average match duration
- Parry success rate (should be 10-20%)
- Attack diversity (all 8 directions used?)
- Player retention

---

## 8. Risks & Mitigation

### Risk 1: Rust Learning Curve
**Impact:** Medium
**Probability:** High
**Mitigation:**
- Start з Bevy туторіалів
- Use ChatGPT/Claude для код review
- Спільнота Bevy дуже активна (Discord)

### Risk 2: Desync Issues
**Impact:** Critical
**Probability:** Medium
**Mitigation:**
- Frequent determinism tests
- Comprehensive logging
- Checksum validation кожен frame
- Community playtest early

### Risk 3: Feel vs Netcode Trade-off
**Impact:** High
**Probability:** Medium
**Mitigation:**
- Playtest з різними input delay (2f, 3f, 4f)
- Adaptive delay based on connection
- Rollback до 8 frames maximum

### Risk 4: Scope Creep
**Impact:** Medium
**Probability:** High
**Mitigation:**
- Stick to GDD phases strictly
- No new features до Phase 1 complete
- Weekly milestone reviews

---

## 9. Decision Summary

### ✅ Final Tech Stack:

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| Language | **Rust** | Memory safety, determinism, ggrs |
| Engine | **Bevy 0.12** | ECS, active community, fast iteration |
| Netcode | **GGRS** | Best rollback library, Bevy integration |
| Transport | **Renet** | Reliable UDP, Bevy integration |
| Physics | **Custom + Rapier** | Simple collision, potential for complex |
| Math | **Fixed-point (fixed crate)** | Deterministic cross-platform |
| Audio | **bevy_kira_audio** | Spatial 3D sound |
| Rendering | **Bevy (wgpu)** | Modern, cross-platform |

### 📋 Next Steps:

1. **Setup Rust & Bevy** (1 day)
   - Install Rust (`rustup`)
   - Create new Bevy project
   - Verify compilation & examples

2. **Create Project Structure** (1 day)
   - Implement folder structure (above)
   - Setup Git repo
   - Configure .gitignore

3. **Basic Scene** (2-3 days)
   - Render arena
   - Spawn 2 mannequins
   - Basic camera

4. **Start Milestone 1** (see roadmap)

---

## 10. Додаткові ресурси

### Learning Rust for Games:
- **Bevy Book:** https://bevyengine.org/learn/book/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Bevy Cheatbook:** https://bevy-cheatbook.github.io/

### Rollback Netcode:
- **GGRS Examples:** https://github.com/gschup/ggrs
- **Netcode Explained:** https://ki.infil.net/w02-netcode.html
- **Kataster (example game):** https://github.com/gschup/kataster

### Fighting Game Mechanics:
- **Fantasy Strike Blog:** (design principles)
- **Core-A Gaming YouTube:** (mechanics breakdowns)

### Bevy Community:
- **Discord:** https://discord.gg/bevy
- **Reddit:** r/bevy
- **Examples Repo:** https://github.com/bevyengine/bevy/tree/main/examples

---

**Документ створено:** 2025-12-11
**Версія:** 1.0
**Автор:** Technical Decision Document для Arena Combat Prototype
