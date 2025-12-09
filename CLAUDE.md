# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Important**: When making significant changes to the project (new features, architectural changes, new file types, or updated constraints), automatically update this file to keep it accurate and helpful for future sessions.

## Project Overview

This is a macOS replica portfolio website built with pure HTML, JavaScript, and CSS. **No external libraries or frameworks are used** (except pdf.js for PDF rendering).

## Architecture

The project simulates a macOS desktop environment with:

- **Dock**: Bottom app bar with icons (Finder/Portfolio, Safari/Articles, Photos/Gallery, Contact, Terminal/Skills, Trash/Archive)
- **Windows**: Draggable window system with z-index management for focus states
- **Finder-style navigation**: Hierarchical folder/file structure for projects, about, resume, and trash sections
- **File types**: Supports `.txt` (text viewer), `.img` (image viewer), `.url` (external links), `.pdf`, and `.fig` file simulations

### Visual Effects

The site features extensive liquid glass and 3D animation effects:

- **Liquid Glass**: Frosted glass (backdrop-filter blur/saturate) on windows, dock, menus, tooltips, notifications
- **3D Animations**: Window open/close with perspective transforms, dock bounce with rotation, gallery/social card tilt on hover
- **Interactive Effects**: Mouse glow following cursor, floating background orbs with parallax, click ripples, particle trails when dragging windows
- **CSS Variables**: Glass effects controlled via `--glass-blur`, `--glass-saturation`, `--liquid-transition`, `--perspective`, `--tilt-amount`

### Key Files

- `index.html` - Main HTML structure (includes floating orbs container, mouse glow element)
- `styles.css` - All styling including liquid glass effects and 3D animations
- `app.js` - Application logic including:
  - Configuration data (dock apps, tech stack, socials, locations/file system)
  - Window state management (`WINDOW_CONFIG`)
  - 3D interactive effects (`initialize3DEffects()` and related functions)

### Data Structure

The `locations` export defines the virtual file system:
- `work`: Project folders with nested files (descriptions, URLs, images, Figma links)
- `about`: Personal info and images
- `resume`: PDF resume
- `trash`: Archived items

## Development

Run the development server:

```bash
python3 main.py
```

This starts a local server at `http://localhost:3000` with caching disabled for development. The server is required because the site uses ES modules and fetch requests that need HTTP (not file://).

## Constraints

- Must remain pure HTML/CSS/JS - no frameworks, libraries, or build tools
- Maintain macOS-like UX patterns (window management, dock behavior, finder navigation)
- **Performance is critical**: Always optimize for smooth 60fps animations. Avoid expensive CSS properties like multiple `box-shadow` layers, heavy `backdrop-filter` blur values (keep under 20px), `filter: brightness()`, and constant JavaScript animation loops. Use `transform` and `opacity` for animations, add `will-change` hints sparingly, and use `{ passive: true }` on scroll/mouse event listeners.
