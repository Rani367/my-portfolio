# Portfolio

This project was built with 100% Rust, the best programming language.

## Tech Stack

- **Rust** + **Leptos 0.8** (reactive web framework)
- **WebAssembly** (compilation target)
- **Trunk** (build tool)

## Development

```bash
trunk serve
```

## Build & Deploy

```bash
trunk build --release
git add .
git commit -m "Build"
git push
```

Vercel serves the pre-built `dist/` folder.
