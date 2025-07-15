# WASM Worms

A WebAssembly-based Worms-style artillery game built with Bevy engine.

## 🎮 Current Features (Phase 3 Complete)

### ✅ Core Game Systems
- **2D Physics System**: Gravity, velocity, collision detection with bounce and friction
- **Destructible Terrain**: Procedurally generated hills with circle-based destruction
- **Worm Entities**: Player-controlled worms with movement and jumping
- **Turn-Based Management**: Player switching, turn timers, win condition checking
- **Camera Controls**: Follow active worm, manual pan/zoom, smooth transitions

### 🚀 Weapon Systems
- **3 Weapon Types**: Bazooka (direct hit), Grenade (timed fuse), Shotgun (spread shot)
- **Trajectory Preview**: Real-time trajectory calculation with wind effects
- **Aiming System**: Angle adjustment and power charging with visual feedback
- **Explosion System**: Terrain destruction and damage calculation
- **Wind Effects**: Dynamic wind that affects projectile flight paths

### 🎯 Enhanced Controls
- **Movement**: A/D keys (W to jump)
- **Aiming**: Space to enter aiming mode, Arrow keys to adjust angle
- **Firing**: Hold Enter to charge power, release to fire
- **Weapons**: 1/2/3 keys or Q/E to cycle weapons
- **Camera**: WASD for manual control, F to return to auto-follow
- **Zoom**: +/- keys
- **Turn**: Enter or Tab to end turn

### 💥 Combat Features
- **Health System**: Visual health bars, fall damage, worm elimination
- **Damage Calculation**: Distance-based explosion damage
- **Visual Effects**: Trajectory dots, crosshair, power bar, explosion effects
- **Win Conditions**: Last team standing wins

## 🏗️ Architecture

### Components
- `RigidBody` - Physics properties (velocity, mass, bounce, friction)
- `Collider` - Collision detection (radius, ground state)
- `Worm` - Game entity (health, team, movement stats)
- `TerrainMap` - Destructible landscape data

### Systems
- `PhysicsPlugin` - Gravity, movement, ground collision
- `TerrainPlugin` - Terrain generation, destruction, mesh updates
- `WormPlugin` - Worm spawning, movement, terrain collision
- `GameStatePlugin` - Turn management, win conditions, timers
- `CameraPlugin` - Following, manual control, zoom

## 🚀 Build Commands

```bash
# Native development (requires display)
cargo run

# WASM build
wasm-pack build --target web --out-dir pkg

# Serve locally
basic-http-server .
```

## 📋 Next Phase: Advanced Features
- Multiple weapon types (Cluster Bomb, Holy Grenade, etc.)
- Power-ups and collectibles
- Animated sprites and particle effects
- Sound effects and background music
- UI system (HUD, weapon selection, health bars)