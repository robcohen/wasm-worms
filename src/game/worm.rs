use bevy::prelude::*;
use crate::game::physics::{RigidBody, Collider};
use crate::game::terrain::TerrainMap;

pub struct WormPlugin;

impl Plugin for WormPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_worms)
            .add_systems(Update, (
                worm_movement,
                worm_terrain_collision,
                update_worm_health_display,
                handle_worm_death,
                worm_fall_damage,
            ));
    }
}

#[derive(Component)]
pub struct Worm {
    pub health: f32,
    pub max_health: f32,
    pub team: u32,
    pub move_speed: f32,
    pub jump_force: f32,
}

impl Default for Worm {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            team: 0,
            move_speed: 150.0,
            jump_force: 400.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct DeadWorm;

fn spawn_worms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    terrain: Res<crate::game::terrain::TerrainMap>,
) {
    // Calculate spawn positions based on terrain size
    let terrain_width = terrain.width as f32;
    let terrain_height = terrain.height as f32;
    
    // Spawn player worm on left side
    let player_x = -terrain_width * 0.3;
    let player_y = terrain_height * 0.3;
    
    commands.spawn((
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_translation(Vec3::new(player_x, player_y, 1.0)),
        Worm {
            team: 0,
            ..default()
        },
        RigidBody::default(),
        Collider::default(),
        PlayerControlled,
    ));
    
    // Spawn enemy worm on right side
    let enemy_x = terrain_width * 0.3;
    let enemy_y = terrain_height * 0.3;
    
    commands.spawn((
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_translation(Vec3::new(enemy_x, enemy_y, 1.0)),
        Worm {
            team: 1,
            ..default()
        },
        RigidBody::default(),
        Collider::default(),
    ));
}

fn worm_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<crate::game::game_state::GameState>,
    mut query: Query<(&mut RigidBody, &Worm, &Collider), (With<PlayerControlled>, Without<DeadWorm>)>,
) {
    // Only allow movement during player's turn
    if !game_state.can_player_act() {
        return;
    }
    
    for (mut body, worm, collider) in query.iter_mut() {
        if worm.health <= 0.0 || worm.team != game_state.current_player {
            continue;
        }
        
        let mut movement = 0.0;
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            movement -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            movement += 1.0;
        }
        
        // Apply horizontal movement
        body.velocity.x = movement * worm.move_speed;
        
        // Jump - use different key to avoid conflict with aiming
        if keyboard_input.just_pressed(KeyCode::KeyW) && collider.is_grounded {
            body.velocity.y = worm.jump_force;
        }
    }
}

fn worm_terrain_collision(
    terrain: Res<TerrainMap>,
    mut query: Query<(&mut Transform, &mut RigidBody, &mut Collider), With<Worm>>,
) {
    for (mut transform, mut body, mut collider) in query.iter_mut() {
        let world_x = transform.translation.x + (terrain.width as f32 / 2.0);
        let world_y = transform.translation.y + (terrain.height as f32 / 2.0);
        
        // Check collision with terrain
        if terrain.check_collision(world_x, world_y, collider.radius) {
            // Simple collision response - push out of terrain
            let mut found_safe_position = false;
            
            // Try moving up to find safe position
            for offset in 1..=32 {
                let test_y = world_y + offset as f32;
                if !terrain.check_collision(world_x, test_y, collider.radius) {
                    transform.translation.y = test_y - (terrain.height as f32 / 2.0);
                    body.velocity.y = body.velocity.y.max(0.0); // Stop downward movement
                    collider.is_grounded = true;
                    found_safe_position = true;
                    break;
                }
            }
            
            if !found_safe_position {
                // If we can't find a safe position above, try moving horizontally
                for x_offset in [-1, 1] {
                    for offset in 1..=32 {
                        let test_x = world_x + (x_offset * offset) as f32;
                        if !terrain.check_collision(test_x, world_y, collider.radius) {
                            transform.translation.x = test_x - (terrain.width as f32 / 2.0);
                            body.velocity.x = 0.0;
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn update_worm_health_display(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    worm_query: Query<(Entity, &Transform, &Worm), Without<DeadWorm>>,
    health_bar_query: Query<Entity, With<HealthBar>>,
) {
    // Remove existing health bars
    for entity in health_bar_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Create new health bars for living worms
    for (_worm_entity, transform, worm) in worm_query.iter() {
        if worm.health <= 0.0 {
            continue;
        }
        
        let health_ratio = worm.health / worm.max_health;
        let bar_width = 40.0;
        let bar_height = 6.0;
        
        // Health bar background
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Rectangle::new(bar_width, bar_height))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.3, 0.3, 0.3, 0.8)))),
            Transform::from_translation(
                transform.translation + Vec3::new(0.0, 25.0, 0.2)
            ),
            HealthBar,
        ));
        
        // Health bar fill
        if health_ratio > 0.0 {
            let fill_width = bar_width * health_ratio;
            let health_color = if health_ratio > 0.6 {
                Color::srgb(0.2, 0.8, 0.2)
            } else if health_ratio > 0.3 {
                Color::srgb(0.8, 0.8, 0.2)
            } else {
                Color::srgb(0.8, 0.2, 0.2)
            };
            
            commands.spawn((
                Mesh2d(meshes.add(bevy::math::primitives::Rectangle::new(fill_width, bar_height - 2.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(health_color))),
                Transform::from_translation(
                    transform.translation + Vec3::new((fill_width - bar_width) / 2.0, 25.0, 0.3)
                ),
                HealthBar,
            ));
        }
    }
}

fn handle_worm_death(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut worm_query: Query<(Entity, &mut MeshMaterial2d<ColorMaterial>, &Worm), (Without<DeadWorm>, Changed<Worm>)>,
) {
    for (entity, mut material, worm) in worm_query.iter_mut() {
        if worm.health <= 0.0 {
            // Mark worm as dead
            commands.entity(entity).insert(DeadWorm);
            
            // Change worm color to indicate death
            material.0 = materials.add(ColorMaterial::from(Color::srgba(0.5, 0.5, 0.5, 0.7)));
            
            // Remove physics components to make it static
            commands.entity(entity).remove::<RigidBody>();
            commands.entity(entity).remove::<PlayerControlled>();
        }
    }
}

fn worm_fall_damage(
    mut worm_query: Query<(&mut Worm, &RigidBody, &Collider), Without<DeadWorm>>,
) {
    for (mut worm, body, collider) in worm_query.iter_mut() {
        // Apply fall damage when hitting ground at high speed
        if collider.is_grounded && body.velocity.y.abs() > 300.0 {
            let fall_damage = (body.velocity.y.abs() - 300.0) * 0.1;
            worm.health -= fall_damage;
            worm.health = worm.health.max(0.0);
        }
    }
}