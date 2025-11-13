# Gemini Project Context: GGTerm

This document provides context for the GGTerm project, an SSH terminal built with Tauri, React, and Rust.

## Project Overview

GGTerm is a modern, fast SSH terminal for desktop. It uses a React frontend for the user interface and a Rust backend for handling SSH connections and session management.

### Key Technologies:

*   **Frontend:** React, TypeScript, Vite, Tailwind CSS
*   **Backend:** Rust, Tauri
*   **Terminal Emulation:** xterm.js
*   **State Management:** Zustand
*   **Database:** SQLite for session storage

### Architecture:

*   The user interface is a single-page application built with React.
*   Tauri is used to wrap the React application in a native desktop window and to expose backend functionality (written in Rust) to the frontend.
*   The Rust backend handles SSH connections using the `ssh2` crate.
*   SSH sessions are stored in a local SQLite database, accessed through the `tauri-plugin-sql` plugin.
*   Communication between the frontend and backend is done through Tauri's command and event system.

## Building and Running

### Prerequisites

*   Node.js 18+
*   Rust 1.70+
*   npm or yarn

### Development

To run the application in development mode:

```bash
npm install
npm run tauri:dev
```

This will start the Vite development server for the frontend and build and run the Tauri application.

### Production

To build the application for production:

```bash
npm run tauri:build
```

This will create a standalone executable for your platform.

## Development Conventions

*   **Styling:** Tailwind CSS is used for styling. Utility classes are preferred over custom CSS.
*   **State Management:** Zustand is used for global state management. Stores are located in the `src/stores` directory.
*   **Backend:** The Rust backend follows standard Rust conventions. The main application logic is in `src-tauri/src/main.rs`.
*   **Tauri Commands:** Backend functions exposed to the frontend are defined as Tauri commands using the `#[tauri::command]` attribute.
