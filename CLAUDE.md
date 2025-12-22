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
- **Startup Screen**: Apple-style boot animation (auto-enabled in release builds, disabled in debug)
- **Notifications**: Toast notifications with auto-dismiss
- **iOS Mobile Mode**: Automatic transformation to iOS-style interface on mobile devices (<768px)

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
├── hooks/              # Custom hooks (keyboard, clock, battery, network, mobile)
│   └── use_mobile.rs   # Device detection and mobile mode
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
    ├── mobile/         # iOS-style mobile components
    │   ├── home_screen.rs   # iOS app grid
    │   ├── app_container.rs # Full-screen app with swipe-to-close
    │   └── status_bar.rs    # iOS status bar
    ├── menu_bar/       # Top menu bar
    ├── dock/           # Bottom dock
    ├── spotlight/      # Spotlight search overlay
    ├── startup/        # Startup screen animation
    └── notifications/  # Notification center
```

### State Architecture
- **Context-based**: State provided via `provide_context()` at app root, consumed via `use_app_state()`, `use_navigation_state()`, `use_notification_state()`
- **RwSignal-based**: All reactive state uses `RwSignal<T>` for read/write access
- **Window management**: `AppState.windows` is a `HashMap<WindowId, WindowState>` - all windows pre-registered, open/close toggles visibility
- **Z-index management**: `AppState.next_z_index()` ensures focused window is always on top
- **Mobile mode**: `AppState.is_mobile` signal tracks device type, `mobile_active_app` tracks the currently open mobile app

### Mobile Mode (iOS Style)
The site automatically transforms into an iOS-like interface on mobile devices (viewport < 768px):

**Behavior:**
- Desktop elements (menu bar, dock, windows) are hidden via CSS
- iOS-style home screen displays with app grid (4 columns, 3 on very small phones)
- Tapping an app opens it full-screen with slide-up animation
- Swipe down or tap "Home" to close and return to home screen
- Status bar shows time, network status, and battery

**Key Components:**
- `use_mobile_detection()` hook monitors viewport width and sets `AppState.is_mobile`
- `MobileHomeScreen` renders the app grid and dock bar
- `MobileAppContainer` wraps each app with header, swipe gesture handling, and animations

**CSS Breakpoints:**
- Desktop: >= 768px
- Tablet: 640px - 767px
- Phone: < 640px

### Virtual File System
- Static data in `src/data/locations.rs` defines `LOCATIONS` array with nested `FileItem` children
- `FileItem` can be folders (with children) or files (with `FileType`: Txt, Img, Url, Pdf, Fig)
- Finder navigation uses `NavigationState` with path stack for back/forward
- Opening files routes to appropriate viewer via `AppState.open_txt_file()` or `AppState.open_img_file()`

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
Opens at `http://localhost:3000` with hot reload (auto-opens browser).

### Build for Production
```bash
trunk build --release
```
Output in `dist/` directory.

**Note**: A pre-commit hook automatically runs `trunk build --release` when source files are staged, so you don't need to manually build before committing.

### Configuration
- `Trunk.toml` - Trunk build configuration
- `Cargo.toml` - Rust dependencies

## Common Patterns

### Adding a New Window
1. Add variant to `WindowId` enum in `src/state/window_state.rs`
2. Add to `WindowId::all()` and `WindowId::as_str()` methods
3. Create content component in `src/components/windows/`
4. Export from `src/components/mod.rs`
5. Add `<Window>` declaration in `src/lib.rs` with content component as child
6. Optionally add to dock in `src/data/dock.rs` and update `WindowId::from_dock_id()`

### Adding New Files to Finder
1. Add `FileItem` entry to appropriate location in `src/data/locations.rs`
2. For new file types, ensure corresponding viewer window exists

## Performance Patterns

### Signal Access - Use `.with()` Over `.get()`
When reading from signals inside memos without needing ownership, use `.with()` to avoid cloning:
```rust
// BAD: Clones the entire HashMap
let is_open = Memo::new(move |_| {
    app_state.windows.get().get(&window_id).map(|w| w.is_open).unwrap_or(false)
});

// GOOD: Borrows without cloning
let is_open = Memo::new(move |_| {
    app_state.windows.with(|windows| {
        windows.get(&window_id).map(|w| w.is_open).unwrap_or(false)
    })
});
```

### Animations - Bypass Leptos Reactivity
For 60fps animations (like dock magnification), bypass Leptos signals entirely:
- Use a single `requestAnimationFrame` loop instead of per-component Effects
- Read signals with `.get_untracked()` to avoid triggering reactivity
- Apply transforms directly via `el.style().set_property()`
- Use lerping for smooth transitions: `current += (target - current) * LERP_FACTOR`

See `src/components/dock/dock.rs` for the reference implementation.

### Keyed Rendering with `<For>`
For lists that may change, use `<For>` with proper keys instead of `.map().collect_view()`:
```rust
// GOOD: Only changed items re-render
<For
    each={move || items.get().iter().enumerate().map(|(i, item)| (i, item.id)).collect::<Vec<_>>()}
    key={|(_, id)| *id}
    children={move |(idx, _)| { ... }}
/>
```

### Async Cleanup
For notifications or timed operations, check if already dismissed before proceeding:
```rust
spawn_local(async move {
    TimeoutFuture::new(5000).await;
    if !is_hiding.get_untracked() {  // Guard against double-dismiss
        // proceed with dismissal
    }
});
```

## Constraints

- **NEVER commit on behalf of the user** - Stage changes and build, but let the user commit themselves
- Maintain macOS-like UX patterns (window management, dock behavior, finder navigation)
- CSS class names must match `styles.css` exactly for styling to work
- **Top-tier performance** - Always write performance-efficient code. Use signals efficiently, avoid unnecessary re-renders, minimize allocations, and follow the Performance Patterns documented above. Every line of code should be optimized for speed and memory usage.
- Keep the same visual effects (liquid glass, 3D animations) via existing CSS
- **NEVER ignore warnings** - Address all compiler warnings and deprecation notices immediately. Do not proceed with other work until warnings are resolved.
