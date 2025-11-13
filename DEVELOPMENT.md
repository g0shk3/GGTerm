# GGTerm Development Guide

## 🚀 Quick Start (macOS)

### 1. Подготовка на средата

```bash
# Pull последните промени
git pull

# Инсталирай dependencies
npm install
```

### 2. Генериране на иконки (важно!)

Създай иконка 1024x1024 px и я преобразувай:

```bash
# Инсталирай Tauri CLI ако го няма
cargo install tauri-cli

# Генерирай всички нужни иконки
npm run tauri icon path/to/your-icon.png
```

Или използвай онлайн генератор:
- https://icon.kitchen/

### 3. Стартиране в development mode

```bash
npm run tauri:dev
```

Това ще:
- Стартира Vite dev server (React frontend)
- Компилира Rust backend
- Отвори приложението

### 4. Build за production

```bash
npm run tauri:build
```

## 📝 Keyboard Shortcuts

- `Ctrl+T` (Cmd+T на Mac) - Нов таб
- `Ctrl+S` (Cmd+S на Mac) - Session Manager

## 🔧 Troubleshooting

### Компилационни грешки на macOS

Ако получиш грешки за липсващи системни библиотеки:

```bash
# Инсталирай Xcode Command Line Tools
xcode-select --install
```

### SSH връзка не работи

Провери:
1. SSH сървърът е достъпен
2. Порт 22 е отворен
3.Credentialите са правилни

### Terminal не показва изход

Провери в Developer Console (Cmd+Option+I):
- Има ли WebSocket грешки?
- Работят ли Tauri events?

## 🎯 Следващи стъпки

1. **Тествай SSH връзката** с твой сървър
2. **Генерирай иконки** за красив вид
3. **Добави SQLite** за персистентно съхранение на сесии
4. **Криптирай паролите** с keyring

## 📂 Структура

```
GGTerm/
├── src/                    React frontend
│   ├── components/
│   │   ├── Terminal/      xterm.js terminal
│   │   ├── TabBar/        Табове
│   │   └── SessionManager/ CRUD сесии
│   └── stores/            Zustand state
└── src-tauri/             Rust backend
    └── src/
        ├── ssh/           SSH логика
        ├── db/            Database
        └── main.rs        Tauri commands
```

## 🐛 Known Issues

- Иконките трябва да се генерират преди build
- SQLite storage е temporary (in-memory)
- Паролите НЕ са криптирани (TODO)

## 📞 Support

Ако нещо не работи, провери:
- README.md
- Tauri docs: https://v2.tauri.app/
- xterm.js docs: https://xtermjs.org/
