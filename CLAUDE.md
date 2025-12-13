# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Important**: When making significant changes to the project (new features, architectural changes, new file types, or updated constraints), automatically update this file to keep it accurate and helpful for future sessions.

## Project Overview

This is a macOS replica portfolio website built with **Rust and Leptos**, compiled to WebAssembly. It simulates a complete macOS desktop environment in the browser.

## Architecture

### Tech Stack
- **Rust** - Primary language
- **Leptos 0.8** - Reactive web framework (CSR mode)
- **WebAssembly** - Compilation target
- **Trunk** - Build tool and dev server

### Features
- **Dock**: Bottom app bar with icons (Finder, Safari, Photos, Contact, Terminal, Trash)
- **Windows**: Draggable window system with z-index management, minimize/maximize/close
- **Finder**: Hierarchical folder/file navigation for projects, about, resume, and trash
- **Spotlight Search**: Search apps and locations via menu bar icon
- **Startup Screen**: Apple-style boot animation (toggle via `STARTUP_SCREEN_ENABLED` in `src/components/startup/startup.rs`)
- **Notifications**: Toast notifications with auto-dismiss

### Key Directories

```
src/
├── lib.rs              # Main app component, window declarations
├── data/               # Static data (dock apps, locations, tech stack, socials, gallery)
│   ├── mod.rs
│   ├── dock.rs         # DOCK_APPS array
│   ├── locations.rs    # Virtual file system (LOCATIONS, file items)
│   ├── tech_stack.rs   # Skills for terminal
│   ├── socials.rs      # Social links for contact
│   └── gallery.rs      # Photo gallery data
├── state/              # Global state management
│   ├── mod.rs
│   ├── app_state.rs    # Window open/close, focus, spotlight
│   ├── window_state.rs # WindowId enum, window configurations
│   ├── navigation_state.rs # Finder navigation
│   └── notification_state.rs # Notification system
├── hooks/              # Custom hooks (window dragging, keyboard)
└── components/         # UI components
    ├── windows/        # Window component + content components
    │   ├── window.rs   # Base Window component
    │   ├── finder.rs   # Finder file browser
    │   ├── terminal.rs # Terminal with skills
    │   ├── contact.rs  # Contact with socials
    │   ├── photos.rs   # Photo gallery
    │   ├── safari.rs   # Safari browser
    │   ├── about.rs    # About This Mac
    │   ├── resume.rs   # PDF viewer
    │   ├── txtfile.rs  # Text file viewer
    │   └── imgfile.rs  # Image file viewer
    ├── menu_bar/       # Top menu bar
    ├── dock/           # Bottom dock
    ├── spotlight/      # Spotlight search overlay
    ├── startup/        # Startup screen animation
    └── notifications/  # Notification center
```

### Styling
- `styles.css` - All CSS including liquid glass effects, 3D animations
- CSS is shared from the original JS version - class names must match

## Development

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Run Development Server
```bash
trunk serve
```
Opens at `http://localhost:8080` with hot reload.

### Build for Production
```bash
trunk build --release
```
Output in `dist/` directory.

### Configuration
- `Trunk.toml` - Trunk build configuration
- `Cargo.toml` - Rust dependencies
- `src/components/startup/startup.rs` - Set `STARTUP_SCREEN_ENABLED = false` for development

## Constraints

- Maintain macOS-like UX patterns (window management, dock behavior, finder navigation)
- CSS class names must match `styles.css` exactly for styling to work
- **Performance**: Use signals efficiently, avoid unnecessary re-renders
- Keep the same visual effects (liquid glass, 3D animations) via existing CSS
- **NEVER ignore warnings** - Address all compiler warnings and deprecation notices immediately. Do not proceed with other work until warnings are resolved.
