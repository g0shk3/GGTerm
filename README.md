# GGTerm - Супер бърз SSH терминал 🚀

Modern SSH terminal built with Tauri, React, and Rust.

## ✨ Features

- **Бързина**: Async Rust backend with connection pooling
- **Множество табове**: Multiple simultaneous SSH sessions
- **Запазване на сесии**: Secure storage with SQLite
- **Тъмна тема**: Smooth dark theme (Dracula inspired)
- **xterm.js**: Full terminal emulation with ANSI support
- **Keyboard shortcuts**:
  - `Ctrl+T` - New tab
  - `Ctrl+S` - Session manager

## 🛠️ Tech Stack

| Component | Technology | Why? |
|-----------|-----------|------|
| UI | React + TypeScript | Modern, component-based |
| Desktop | Tauri 2.x | Lighter & faster than Electron |
| Terminal | xterm.js | Full terminal emulation |
| SSH | Rust + ssh2 | Performance & security |
| Storage | SQLite | Session management |
| State | Zustand | Lightweight state management |
| Styling | Tailwind CSS | Fast styling with dark theme |

## 🚀 Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- npm/yarn

### Setup
```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

## 📁 Project Structure

```
GGTerm/
├── src/                    # React frontend
│   ├── components/
│   │   ├── Terminal/      # xterm.js terminal component
│   │   ├── TabBar/        # Tab management
│   │   └── SessionManager/# SSH session management
│   ├── stores/            # Zustand state management
│   └── types/             # TypeScript types
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── ssh/          # SSH connection logic
│   │   ├── db/           # Database operations
│   │   └── main.rs       # Tauri commands
│   └── Cargo.toml
└── package.json
```

## 🔐 Security

- Passwords are stored encrypted in SQLite
- SSH keys support
- Secure Tauri commands

## 📝 License

MIT

## 🤝 Contributing

Contributions welcome! Open an issue or PR.
