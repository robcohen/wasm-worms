# WASM Worms Implementation Plan

## Phase 1: Project Setup & Foundation
- [ ] Initialize Cargo project with Bevy dependencies
- [ ] Configure WASM build pipeline (wasm-pack, wasm-bindgen)
- [ ] Set up basic HTML/JS wrapper for WASM
- [ ] Create basic Bevy app with window and camera
- [ ] Test WASM build and deployment

## Phase 2: Core Game Systems
- [ ] Implement 2D physics system (gravity, collisions)
- [ ] Create destructible terrain system
- [ ] Add basic worm entity with movement
- [ ] Implement turn-based game state management
- [ ] Add camera controls (zoom, pan, follow)

## Phase 3: Gameplay Mechanics
- [ ] Weapon system architecture (projectiles, explosions)
- [ ] Basic weapons: Bazooka, Grenade, Shotgun
- [ ] Trajectory calculation and aiming system
- [ ] Health system and worm elimination
- [ ] Wind effects on projectiles

## Phase 4: Advanced Features
- [ ] Multiple weapon types (Cluster Bomb, Holy Grenade, etc.)
- [ ] Power-ups and collectibles
- [ ] Animated sprites and particle effects
- [ ] Sound effects and background music
- [ ] UI system (HUD, weapon selection, health bars)

## Phase 5: Polish & Optimization
- [ ] WASM performance optimization
- [ ] Mobile touch controls
- [ ] Game balance and difficulty tuning
- [ ] Visual polish and animations
- [ ] Multiplayer foundation (local hot-seat)

## Technical Architecture

### Core Components
- `Worm` - Player character with health, position, team
- `Weapon` - Weapon stats, ammo, damage
- `Projectile` - Physics body for bullets/grenades
- `Terrain` - Destructible landscape chunks
- `GameState` - Turn management, win conditions

### Key Systems
- `MovementSystem` - Handle worm movement and physics
- `WeaponSystem` - Firing, trajectory, damage calculation
- `TerrainSystem` - Destruction and collision detection
- `TurnSystem` - Player switching, time limits
- `CameraSystem` - Follow action, smooth transitions

### WASM Considerations
- Embed all assets in binary
- Optimize for 60fps in browser
- Minimize WASM binary size
- Handle browser compatibility issues