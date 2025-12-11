# Підключення до GitHub
## Інструкція для Arena Combat

---

## 📋 Підготовка

У вас вже є:
- ✅ Git встановлено
- ✅ Локальний репозиторій ініціалізовано
- ✅ Перший коміт зроблено

Треба:
- 🔲 Створити репозиторій на GitHub
- 🔲 Підключити локальний репозиторій до GitHub
- 🔲 Запушити код

---

## 🚀 Крок 1: Створити репозиторій на GitHub

### Через веб-інтерфейс:

1. Перейти на https://github.com
2. Натиснути **"+"** (правий верхній кут) → **"New repository"**
3. Заповнити:
   - **Repository name:** `arena-combat` (або `arena-combat-prototype`)
   - **Description:** "Third-person melee combat game with directional fluid combat"
   - **Public** або **Private** (на ваш вибір)
   - ⚠️ **НЕ створювати** README, .gitignore, license (у нас вже є!)
4. Натиснути **"Create repository"**

### Або через GitHub CLI (якщо встановлено):

```bash
gh repo create arena-combat --public --source=. --remote=origin --push
```

---

## 🔗 Крок 2: Підключити remote

Після створення репозиторію GitHub покаже команди. Використайте:

```bash
cd c:\Claude\arena_combat

# Додати remote (замініть YOUR_USERNAME на ваш username)
git remote add origin https://github.com/YOUR_USERNAME/arena-combat.git

# Або якщо використовуєте SSH:
# git remote add origin git@github.com:YOUR_USERNAME/arena-combat.git

# Перевірити
git remote -v
```

**Очікуваний вивід:**
```
origin  https://github.com/YOUR_USERNAME/arena-combat.git (fetch)
origin  https://github.com/YOUR_USERNAME/arena-combat.git (push)
```

---

## 📤 Крок 3: Запушити код

```bash
# Перший push (встановити upstream)
git push -u origin master

# Або якщо основна гілка - main:
# git branch -M main
# git push -u origin main
```

### Якщо виникне помилка аутентифікації:

**GitHub більше не підтримує password authentication.** Потрібен **Personal Access Token (PAT)**.

#### Створити PAT:

1. GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token (classic)
3. Обрати scopes:
   - ✅ `repo` (Full control of private repositories)
4. Згенерувати токен
5. **СКОПІЮВАТИ ТОКЕН** (показується тільки раз!)

#### Використати PAT:

При `git push` замість password вставити токен.

**Або налаштувати Git credential helper:**

```bash
# Windows
git config --global credential.helper manager

# Або зберегти credentials
git config --global credential.helper store
```

---

## 🔐 Крок 4 (Опційно): Налаштувати SSH

Щоб не вводити токен кожного разу:

### Згенерувати SSH ключ:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# Натискати Enter (default location)
# Можна встановити passphrase або залишити порожнім
```

### Додати ключ до ssh-agent:

```bash
# PowerShell (Windows)
# Запустити ssh-agent
Start-Service ssh-agent

# Додати ключ
ssh-add ~\.ssh\id_ed25519
```

### Додати публічний ключ на GitHub:

1. Скопіювати вміст `~/.ssh/id_ed25519.pub`:
   ```bash
   cat ~/.ssh/id_ed25519.pub
   ```

2. GitHub → Settings → SSH and GPG keys → New SSH key
3. Вставити ключ, додати title
4. Save

### Змінити remote на SSH:

```bash
git remote set-url origin git@github.com:YOUR_USERNAME/arena-combat.git
```

### Перевірити:

```bash
ssh -T git@github.com
# Має показати: "Hi YOUR_USERNAME! You've successfully authenticated..."
```

---

## ✅ Перевірка

```bash
# Подивитися статус
git status

# Подивитися історію
git log --oneline

# Перевірити remote
git remote -v

# Перевірити що код на GitHub
# Відкрити: https://github.com/YOUR_USERNAME/arena-combat
```

---

## 📦 Що тепер на GitHub

Після успішного push на GitHub буде:

```
arena-combat/
├── .gitignore
├── Cargo.toml
├── README.md                    ← Опис проекту
├── BUILD_SETUP.md               ← Інструкція встановлення
├── GITHUB_SETUP.md              ← Цей файл
├── PROGRESS.md                  ← Журнал розробки
├── arena_combat_gdd.md          ← Game Design Document
├── tech_stack_decision.md       ← Технічні рішення
└── src/
    └── main.rs                  ← Код
```

---

## 🔄 Workflow для майбутніх змін

```bash
# 1. Зробити зміни в коді
# 2. Додати до staging
git add -A

# 3. Зробити коміт
git commit -m "Опис змін"

# 4. Запушити на GitHub
git push

# Якщо працюєте на іншому комп'ютері:
# Спочатку pull
git pull
```

---

## 🎯 Рекомендації

### .gitignore вже налаштовано для:
- ✅ `/target/` - бінарні файли
- ✅ `Cargo.lock` - lock файл
- ✅ IDE конфіги
- ✅ Логи

### Що НЕ треба коммітити:
- ❌ Великі бінарні файли (3D моделі > 100MB)
- ❌ Особисті налаштування
- ❌ Паролі, токени, ключі

### Для великих assets (майбутнє):
Використати **Git LFS** (Large File Storage):
```bash
git lfs install
git lfs track "*.glb"
git lfs track "*.ogg"
```

---

## 🐛 Troubleshooting

### "Permission denied (publickey)"
- Налаштувати SSH ключ (див. Крок 4)

### "Authentication failed"
- Використати Personal Access Token замість password

### "fatal: remote origin already exists"
```bash
git remote remove origin
# Потім додати знову
```

### Conflict при push
```bash
# Спочатку pull
git pull origin master --rebase
# Вирішити конфлікти
# Потім push
git push
```

---

## 📝 Наступні кроки

Після успішного push на GitHub:

1. ✅ Код зберігається в хмарі
2. Налаштувати GitHub Actions (CI/CD) - пізніше
3. Створити Issues для задач
4. Запросити співрозробників (якщо потрібно)

---

**Створено:** 2025-12-11
