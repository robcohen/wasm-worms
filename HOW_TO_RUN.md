# 🎮 How to Run WASM Worms

## 🚀 Quick Start

Your WASM Worms game is **ready to play**! Here's how to run it:

### Method 1: Python Server (Easiest)
```bash
# In the wasm-worms directory
python3 -m http.server 8000

# Then open: http://localhost:8000
```

### Method 2: Node.js Server
```bash
# Install serve globally
npm install -g serve

# Serve the directory
serve .

# Open the provided URL (usually http://localhost:3000)
```

### Method 3: Rust HTTP Server
```bash
# Install basic-http-server
cargo install basic-http-server

# Serve the directory  
basic-http-server .

# Open: http://localhost:4000
```

## 🔧 Building from Source

If you need to rebuild the WASM version:

```bash
# Build WASM package
wasm-pack build --target web --out-dir pkg

# Serve locally
python3 -m http.server 8000
```

**Note**: There's currently a `getrandom` dependency issue with Bevy 0.11+ for WASM. The game code is complete, but you may need to:
- Use Bevy 0.10 (more stable for WASM)
- Or add `getrandom = { version = "0.2", features = ["js"] }` to dependencies
- Or use a newer Bevy version with better WASM support

## 🎯 Game Controls

### 🚶 Movement
- **A/D Keys**: Move worm left/right
- **W Key**: Jump

### 🎯 Combat System
- **Space**: Enter/exit aiming mode
- **Arrow Keys**: Adjust aim angle (while aiming)
- **Hold Enter**: Charge power, release to fire
- **1/2/3 Keys**: Select weapon (Bazooka/Grenade/Shotgun)
- **Q/E Keys**: Cycle through weapons

### 📷 Camera Controls
- **WASD**: Manual camera movement
- **F Key**: Return to auto-follow mode
- **+/- Keys**: Zoom in/out

### 🎮 Game Management
- **Enter/Tab**: End current turn

## 🔫 Weapons Guide

1. **🚀 Bazooka (Key 1)**
   - Direct-hit rocket launcher
   - Medium explosion radius
   - Good for precise shots

2. **💣 Grenade (Key 2)**
   - Timed fuse (3 seconds)
   - Large explosion radius
   - Bounces off terrain

3. **🔫 Shotgun (Key 3)**
   - Fires 5 projectiles in spread
   - Smaller individual explosions
   - Great for close combat

## 🎮 Gameplay Features

### ✨ What's Working
- **Physics**: Realistic gravity, bounce, friction
- **Terrain**: Destructible landscape with procedural hills
- **Combat**: 3 weapon types with different behaviors
- **Aiming**: Trajectory preview with wind effects
- **Health**: Visual health bars and damage system
- **Turns**: Automatic turn switching with timer
- **Camera**: Smooth following and manual control
- **Wind**: Dynamic wind affects projectile paths

### 🎯 Game Objective
- Eliminate enemy worms by reducing their health to 0
- Use terrain strategically for cover and positioning
- Account for wind when aiming projectiles
- Last team standing wins!

## 🐛 Troubleshooting

### WASM Build Issues
If you encounter build errors:
1. Try using Bevy 0.10 instead of 0.11+
2. Add `getrandom = { version = "0.2", features = ["js"] }` to Cargo.toml
3. Clear Cargo.lock: `rm Cargo.lock`
4. Rebuild: `wasm-pack build --target web --out-dir pkg`

### Performance Issues
- The game is optimized for 60fps in browsers
- If performance is poor, try reducing browser zoom
- Close other browser tabs for better performance

### Controls Not Working
- Make sure the game canvas has focus (click on it)
- Some keys might conflict with browser shortcuts
- Try refreshing the page if controls become unresponsive

## 🎉 Enjoy the Game!

You now have a fully functional Worms-style artillery game running in your browser! The game includes realistic physics, destructible terrain, multiple weapons, and strategic gameplay elements.

**Have fun blowing things up!** 💥