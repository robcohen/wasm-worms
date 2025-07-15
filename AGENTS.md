# AGENTS.md - Development Guidelines

## Project Overview
WebAssembly Worms game built with Bevy engine. Full-screen turn-based artillery game where worms fight with various weapons on destructible terrain.

## Build/Test Commands
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized release
- `cargo run` - Run native version for development
- `cargo test` - Run all tests
- `wasm-pack build --target web --out-dir pkg` - Build WASM version
- `basic-http-server .` - Serve WASM build locally

## Code Style Guidelines
- Use snake_case for variables, functions, modules (Rust convention)
- Use PascalCase for types, structs, enums
- Organize imports: std, external crates, local modules
- Use Bevy's ECS patterns: Systems, Components, Resources
- Prefer composition over inheritance
- Handle Results explicitly, avoid unwrap() in production
- Use descriptive names for game entities and components

## Development Notes
- Target: WebAssembly with native fallback for development
- Engine: Bevy 0.12+ with wasm-bindgen
- Physics: Custom 2D physics or bevy_rapier2d
- Audio: bevy_kira_audio for cross-platform sound
- Assets: Embed in binary for WASM compatibility