# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

3D interactive portfolio website built with Next.js 14 (App Router), featuring an interactive Spline keyboard where keycaps represent skills, real-time collaboration with Socket.io, and MDX-powered blog system.

## Commands

```bash
npm run dev      # Start development server at localhost:3000
npm run build    # Production build
npm run lint     # Run ESLint
npm start        # Start production server
```

## Environment Variables

- `RESEND_API_KEY` - Email API (Resend)
- `NEXT_PUBLIC_WS_URL` - WebSocket server URL for real-time features

## Architecture

### Directory Structure

- `src/app/` - Next.js App Router pages and API routes
- `src/components/sections/` - Page sections (hero, skills, experience, projects, contact)
- `src/components/realtime/` - Socket.io chat, cursors, user list
- `src/components/ui/` - Shadcn UI + custom animation components
- `src/data/` - Static data (config.ts, constants.ts for skills, projects.tsx)
- `src/content/blogs/` - MDX blog posts
- `src/contexts/` - React Context (Socket.io context)
- `src/hooks/` - Custom hooks (media query, mouse position, viewport)
- `src/lib/` - Utilities (Lenis smooth scroll, MDX parsing, avatar generation)

### Key Patterns

**3D Keyboard Animation System**
- `animated-background.tsx` - Main Spline component with scroll-triggered transforms
- `animated-background-config.ts` - Section-based 3D state definitions (position, rotation, scale) for mobile/desktop
- Skill keycaps reveal titles and descriptions on hover with sound feedback

**Animation Stack**
- GSAP with ScrollTrigger for scroll-based animations
- Framer Motion for component-level animations
- Lenis for smooth scrolling (`src/lib/lenis/`)
- Reusable reveal components: `BlurIn`, `BoxReveal`, `RevealAnimation` in `reveal-animations.tsx`

**Real-time Features**
- Socket.io context in `src/contexts/socketio.tsx` manages connection, users, messages
- Session persistence via localStorage for reconnection
- `useSounds` hook for keyboard press/release and chat notification sounds

**Content System**
- MDX blogs parsed via `src/lib/mdx.ts` with gray-matter for frontmatter
- Skill definitions with descriptions and icons in `src/data/constants.ts`
- Project data with tech stacks in `src/data/projects.tsx`

### Path Aliases

`@/*` maps to `./src/*` (configured in tsconfig.json)

### Styling

Tailwind CSS with CSS variables for theming. Dark mode via `next-themes`. Component styling follows Shadcn patterns with `cn()` utility for class merging.
