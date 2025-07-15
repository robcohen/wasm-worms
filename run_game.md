L# How to Run WASM Worms

## Current Status
The game code is complete with all Phase 3 features, but there's a WASM compilation issue with the `getrandom` dependency that needs to be resolved for web deployment.

## Option 1: Fix WASM Build (Recommended)

The issue is that Bevy 0.11 pulls in `getrandom` 0.3 which doesn't support WASM by default. Here are solutions:

### Solution A: Use Bevy 0.12+ 
```toml
# In Cargo.toml, change:
bevy = { version = "0.12", default-features = false, features = [...] }
```

### Solution B: Override getrandom
```toml
# Add to Cargo.toml:
[dependencies]
getrandom = { version = "0.2", features = ["js"] }

# Or use dependency override:
[dependency-overrides]
getrandom = { version = "0.2", features = ["js"] }
```

### Solution C: Use WASM-specific Bevy features
```toml
[dependencies.bevy]
version = "0.11"
default-features = false
features = [
    "bevy_winit",
    "bevy_render", 
    "bevy_core_pipeline",
    "bevy_sprite",
    "png",
    "webgl2",
    "wasm-bindgen",
]
```

## Option 2: Native Development

For development and testing, you can run the native version (requires a display):

```bash
cargo run
```

## Option 3: Manual WASM Setup

1. Fix the getrandom issue using one of the solutions above
2. Build WASM:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```
3. Serve locally:
   ```bash
   basic-http-server .
   ```
4. Open `http://localhost:4000` in your browser

## Game Controls

### Movement
- **A/D**: Move left/right
- **W**: Jump

### Combat
- **Space**: Enter/exit aiming mode
- **Arrow Keys**: Adjust aim angle (in aiming mode)
- **Hold Enter**: Charge power, release to fire
- **1/2/3** or **Q/E**: Switch weapons

### Camera
- **WASD**: Manual camera control
- **F**: Return to auto-follow
- **+/-**: Zoom in/out

### Game Management
- **Enter/Tab**: End turn

## Weapons
1. **Bazooka** (1): Direct-hit rocket with medium explosion
2. **Grenade** (2): Timed fuse bomb with large blast radius  
3. **Shotgun** (3): Spread shot with multiple projectiles

## Features
- Realistic physics with gravity and wind effects
- Destructible terrain
- Health system with visual health bars
- Turn-based gameplay
- Trajectory preview with power charging
- Multiple weapon types with different behaviors
- Fall damage and worm elimination

The game is fully functional - just needs the WASM build issue resolved for web deployment!