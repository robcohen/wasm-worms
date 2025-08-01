use bevy::prelude::*;
use crate::game::weapons::{WeaponInventory, WindSystem, fire_weapon};
use crate::game::worm::{Worm, PlayerControlled};
use crate::game::game_state::{GameState, GamePhase};

pub struct AimingPlugin;

impl Plugin for AimingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AimingState::default())
            .add_systems(Update, (
                handle_aiming_input,
                update_trajectory_preview,
                handle_weapon_switching,
                handle_firing,
                cleanup_trajectory_preview,
            ));
    }
}

#[derive(Resource)]
pub struct AimingState {
    pub is_aiming: bool,
    pub aim_angle: f32,
    pub power: f32,
    pub max_power: f32,
    pub power_charging: bool,
    pub trajectory_points: Vec<Vec2>,
    pub last_calculated_angle: f32,
    pub last_calculated_power: f32,
}

impl Default for AimingState {
    fn default() -> Self {
        Self {
            is_aiming: false,
            aim_angle: 45.0,
            power: 0.5,
            max_power: 1.0,
            power_charging: false,
            trajectory_points: Vec::new(),
            last_calculated_angle: 0.0,
            last_calculated_power: 0.0,
        }
        }
    }
}

fn cleanup_trajectory_preview(
    mut commands: Commands,
    aiming_state: Res<AimingState>,
    preview_query: Query<Entity, With<TrajectoryPreview>>,
    crosshair_query: Query<Entity, With<AimingCrosshair>>,
    power_bar_query: Query<Entity, With<PowerBar>>,
) {
    if !aiming_state.is_aiming {
        // Clean up all aiming-related entities when not aiming
        for entity in preview_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in crosshair_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in power_bar_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}}

#[derive(Component)]
pub struct TrajectoryPreview;

#[derive(Component)]
pub struct AimingCrosshair;

#[derive(Component)]
pub struct PowerBar;

fn handle_aiming_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut aiming_state: ResMut<AimingState>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    // Only allow aiming during player's turn
    if !game_state.can_player_act() {
        return;
    }
    
    // Toggle aiming mode
    if keyboard_input.just_pressed(KeyCode::Space) {
        if !aiming_state.is_aiming {
            aiming_state.is_aiming = true;
            game_state.start_aiming();
            println!("Entered aiming mode"); // Debug output
        } else {
            aiming_state.is_aiming = false;
            aiming_state.power = 0.0;
            aiming_state.power_charging = false;
            game_state.start_new_turn(); // Return to turn mode
            println!("Exited aiming mode"); // Debug output
        }
    }
    
    if !aiming_state.is_aiming {
        return;
    }
    
    // Adjust aim angle
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        aiming_state.aim_angle += 90.0 * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        aiming_state.aim_angle -= 90.0 * time.delta_secs();
    }
    
    // Clamp angle to reasonable range
    aiming_state.aim_angle = aiming_state.aim_angle.clamp(-90.0, 90.0);
    
    // Power charging
    if keyboard_input.just_pressed(KeyCode::Enter) {
        aiming_state.power_charging = true;
        aiming_state.power = 0.0;
    }
    
    if aiming_state.power_charging {
        aiming_state.power += time.delta_secs() * 0.8; // Charge speed
        if aiming_state.power >= aiming_state.max_power {
            aiming_state.power = aiming_state.max_power;
        }
    }
}

fn handle_weapon_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon_inventory: ResMut<WeaponInventory>,
    game_state: Res<GameState>,
) {
    if game_state.game_phase != GamePhase::PlayerTurn {
        return;
    }
    
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        weapon_inventory.current_weapon = 0;
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        weapon_inventory.current_weapon = 1;
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        weapon_inventory.current_weapon = 2;
    }
    
    // Cycle weapons with Q/E
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        weapon_inventory.current_weapon = 
            (weapon_inventory.current_weapon + weapon_inventory.weapons.len() - 1) % weapon_inventory.weapons.len();
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        weapon_inventory.current_weapon = 
            (weapon_inventory.current_weapon + 1) % weapon_inventory.weapons.len();
    }
}

fn handle_firing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut aiming_state: ResMut<AimingState>,
    mut game_state: ResMut<GameState>,
    weapon_inventory: Res<WeaponInventory>,
    worm_query: Query<&Transform, (With<Worm>, With<PlayerControlled>)>,
) {
    if !matches!(game_state.game_phase, GamePhase::Aiming) || !aiming_state.is_aiming {
        return;
    }
    
    // Fire when releasing Enter or pressing mouse
    if keyboard_input.just_released(KeyCode::Enter) && aiming_state.power_charging {
        if let Some(current_weapon) = weapon_inventory.weapons.get(weapon_inventory.current_weapon) {
            // Find active worm position
            for worm_transform in worm_query.iter() {
                let angle_rad = aiming_state.aim_angle.to_radians();
                let direction = Vec2::new(angle_rad.cos(), angle_rad.sin());
                
                // Offset firing position slightly from worm center
                let firing_position = worm_transform.translation + Vec3::new(
                    direction.x * 20.0,
                    direction.y * 20.0,
                    0.1,
                );
                
                fire_weapon(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    current_weapon.clone(),
                    firing_position,
                    direction,
                    aiming_state.power,
                );
                
                // Update game state
                game_state.start_firing();
                game_state.projectile_launched();
                
                // Reset aiming state
                aiming_state.is_aiming = false;
                aiming_state.power_charging = false;
                aiming_state.power = 0.5;
                break;
            }
        }
    }
}

fn update_trajectory_preview(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    aiming_state: Res<AimingState>,
    weapon_inventory: Res<WeaponInventory>,
    wind: Res<WindSystem>,
    worm_query: Query<&Transform, (With<Worm>, With<PlayerControlled>)>,
    preview_query: Query<Entity, With<TrajectoryPreview>>,
    crosshair_query: Query<Entity, With<AimingCrosshair>>,
    power_bar_query: Query<Entity, With<PowerBar>>,
) {
    // Clean up existing preview elements
    for entity in preview_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in crosshair_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in power_bar_query.iter() {
        commands.entity(entity).despawn();
    }
    
    if !aiming_state.is_aiming {
        return;
    }
    
    // Find active worm
    let worm_transform = if let Some(transform) = worm_query.iter().next() {
        transform
    } else {
        return;
    };
    
    let current_weapon = if let Some(weapon) = weapon_inventory.weapons.get(weapon_inventory.current_weapon) {
        weapon
    } else {
        return;
    };
    
    let weapon_stats = current_weapon.get_stats();
    let angle_rad = aiming_state.aim_angle.to_radians();
    let direction = Vec2::new(angle_rad.cos(), angle_rad.sin());
    
    // Only recalculate trajectory if angle or power changed significantly
    let current_weapon = weapon_inventory.weapons.get(weapon_inventory.current_weapon).unwrap();
    let angle_rad = aiming_state.aim_angle.to_radians();
    let direction = Vec2::new(angle_rad.cos(), angle_rad.sin());
    let weapon_stats = current_weapon.get_stats();
    let initial_velocity = direction * aiming_state.power * weapon_stats.projectile_speed;
    
    let mut trajectory_points = Vec::new();
    let mut pos = worm_transform.translation.truncate();
    let mut velocity = initial_velocity;
    
    // Simulate trajectory with proper physics
    let dt = 0.02; // 50 FPS simulation
    for _ in 0..200 {
        trajectory_points.push(pos);
        
        // Apply physics (matching the actual projectile physics)
        velocity.y += -980.0 * weapon_stats.gravity_scale * dt; // Gravity
        velocity += wind.force * weapon_stats.wind_resistance * dt; // Wind effect
        pos += velocity * dt;
        
        // Stop if we hit the ground or go too far
        if pos.y < -400.0 || pos.x.abs() > 2000.0 {
            break;
        }
    }
    }
    
    // Spawn trajectory preview dots
    for (i, point) in trajectory_points.iter().enumerate() {
        if i % 3 == 0 { // Show every 3rd point to avoid clutter
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Circle::new(2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.6)))),
            Transform::from_translation(Vec3::new(point.x, point.y, 0.5)),
            TrajectoryPreview,
            ));
        }
    }
    
    // Spawn aiming crosshair
    let crosshair_pos = worm_transform.translation + Vec3::new(
        direction.x * 50.0,
        direction.y * 50.0,
        0.5,
    );
    
    commands.spawn((
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(8.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(current_weapon.get_color()))),
        Transform::from_translation(crosshair_pos),
        AimingCrosshair,
    ));
    
    // Spawn power bar
    let power_width = 100.0 * aiming_state.power;
    let power_pos = worm_transform.translation + Vec3::new(-50.0, 40.0, 0.5);
    
    // Power bar background
    commands.spawn((
        Mesh2d(meshes.add(bevy::math::primitives::Rectangle::new(100.0, 10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.3, 0.3, 0.3, 0.8)))),
        Transform::from_translation(power_pos),
        PowerBar,
    ));
    
    // Power bar fill
    if power_width > 0.0 {
        let fill_color = if aiming_state.power < 0.7 {
            Color::srgb(0.2, 0.8, 0.2)
        } else if aiming_state.power < 0.9 {
            Color::srgb(0.8, 0.8, 0.2)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };
        
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Rectangle::new(power_width, 8.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(fill_color))),
            Transform::from_translation(power_pos + Vec3::new((power_width - 100.0) / 2.0, 0.0, 0.1)),
            PowerBar,
        ));
    }
}