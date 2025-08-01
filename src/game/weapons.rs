use bevy::prelude::*;
use crate::game::physics::{RigidBody, Collider};

use crate::game::worm::Worm;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WeaponInventory::default())
            .insert_resource(WindSystem::new())
            .add_systems(Update, (
                update_wind_system,
                apply_wind_to_projectiles,
                projectile_collision,
                explosion_system,
                cleanup_expired_projectiles,
                change_wind_on_turn_end,
            ));
    }
}

#[derive(Resource)]
pub struct WeaponInventory {
    pub weapons: Vec<WeaponType>,
    pub current_weapon: usize,
}

impl Default for WeaponInventory {
    fn default() -> Self {
        Self {
            weapons: vec![
                WeaponType::Bazooka,
                WeaponType::Grenade,
                WeaponType::Shotgun,
            ],
            current_weapon: 0,
        }
    }
}

#[derive(Resource)]
pub struct WindSystem {
    pub force: Vec2,
    pub change_timer: Timer,
}

impl WindSystem {
    pub fn new() -> Self {
        Self {
            force: Vec2::new(0.0, 0.0),
            change_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
    
    pub fn generate_new_wind(&mut self) {
        let strength = (fastrand::f32() - 0.5) * 120.0; // -60 to +60 wind force
        self.force = Vec2::new(strength, 0.0); // Only horizontal wind
        println!("New wind: {:.1}", strength); // Debug output
    }
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    Grenade,
    Bazooka,
    ClusterBomb,
}

impl WeaponType {
    pub fn get_stats(&self) -> WeaponStats {
        match self {
            WeaponType::Bazooka => WeaponStats {
                damage: 50.0,
                explosion_radius: 80.0,
                projectile_speed: 600.0,
                gravity_scale: 1.0,
                wind_resistance: 0.5,
                fuse_time: None,
                projectile_count: 1,
            },
            WeaponType::Grenade => WeaponStats {
                damage: 60.0,
                explosion_radius: 100.0,
                projectile_speed: 400.0,
                gravity_scale: 1.2,
                wind_resistance: 0.8,
                fuse_time: Some(3.0),
                projectile_count: 1,
            },
            WeaponType::ClusterBomb => WeaponStats {
                damage: 35.0,
                explosion_radius: 60.0,
                projectile_speed: 350.0,
                gravity_scale: 1.0,
                wind_resistance: 0.7,
                fuse_time: Some(2.5),
                projectile_count: 3, // Splits into 3 smaller bombs
            },
        }
    }
    
    pub fn get_color(&self) -> Color {
        match self {
            WeaponType::Grenade => Color::srgb(0.2, 0.8, 0.2),
            WeaponType::Bazooka => Color::srgb(1.0, 0.5, 0.0),
            WeaponType::ClusterBomb => Color::srgb(0.8, 0.2, 0.8),
        }
    }
}

#[derive(Clone)]
pub struct WeaponStats {
    pub damage: f32,
    pub explosion_radius: f32,
    pub projectile_speed: f32,
    pub gravity_scale: f32,
    pub wind_resistance: f32,
    pub fuse_time: Option<f32>,
    pub projectile_count: u32,
}

#[derive(Component)]
pub struct Projectile {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub explosion_radius: f32,
    pub fuse_timer: Option<Timer>,
    pub wind_resistance: f32,
    pub has_exploded: bool,
}

#[derive(Component)]
pub struct Explosion {
    pub radius: f32,
    pub damage: f32,
    pub lifetime: Timer,
}

pub fn fire_weapon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    weapon_type: WeaponType,
    position: Vec3,
    direction: Vec2,
    power: f32,
) {
    let stats = weapon_type.get_stats();
    let velocity = direction.normalize() * stats.projectile_speed * power;
    
    // Calculate spread for shotgun
    let spread_angle = if matches!(weapon_type, WeaponType::Shotgun) { 0.3 } else { 0.0 };
    
    for i in 0..stats.projectile_count {
        let mut projectile_velocity = velocity;
        
        // Apply spread for multiple projectiles
        if stats.projectile_count > 1 {
            let angle_offset = (i as f32 - (stats.projectile_count as f32 - 1.0) / 2.0) * spread_angle;
            let cos_a = angle_offset.cos();
            let sin_a = angle_offset.sin();
            projectile_velocity = Vec2::new(
                velocity.x * cos_a - velocity.y * sin_a,
                velocity.x * sin_a + velocity.y * cos_a,
            );
        }
        
        let fuse_timer = stats.fuse_time.map(|time| Timer::from_seconds(time, TimerMode::Once));
        
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Circle::new(4.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(weapon_type.get_color()))),
            Transform::from_translation(position),
            Projectile {
                weapon_type: weapon_type.clone(),
                damage: stats.damage,
                explosion_radius: stats.explosion_radius,
                fuse_timer,
                wind_resistance: stats.wind_resistance,
                has_exploded: false,
            },
            RigidBody {
                velocity: projectile_velocity,
                gravity_scale: stats.gravity_scale,
                mass: 0.1,
                bounce: 0.3,
                friction: 0.9,
            },
            Collider {
                radius: 4.0,
                is_grounded: false,
            },
        ));
    }
}

fn projectile_movement(
    time: Res<Time>,
    wind: Res<WindSystem>,
    mut query: Query<(&mut RigidBody, &Projectile)>,
) {
    for (mut body, projectile) in query.iter_mut() {
        // Apply wind force
        body.velocity += wind.force * projectile.wind_resistance * time.delta_secs();
    }
}

fn projectile_collision(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut terrain: ResMut<crate::game::terrain::TerrainMap>,
    mut game_state: ResMut<crate::game::game_state::GameState>,
    mut projectile_query: Query<(Entity, &Transform, &mut Projectile, &Collider)>,
    time: Res<Time>,
) {
    for (entity, transform, mut projectile, collider) in projectile_query.iter_mut() {
        if projectile.has_exploded {
            continue;
        }
        
        // Update fuse timer
        if let Some(ref mut timer) = projectile.fuse_timer {
            timer.tick(time.delta());
            if timer.finished() {
                explode_projectile(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut terrain,
                    &mut game_state,
                    entity,
                    transform.translation,
                    &projectile,
                );
                continue;
            }
        }
        
        // Check terrain collision
        let world_x = transform.translation.x + (terrain.width as f32 / 2.0);
        let world_y = transform.translation.y + (terrain.height as f32 / 2.0);
        
        if terrain.check_collision(world_x, world_y, collider.radius) {
            explode_projectile(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut terrain,
                &mut game_state,
                entity,
                transform.translation,
                &projectile,
            );
        }
    }
}

fn explode_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    terrain: &mut ResMut<crate::game::terrain::TerrainMap>,
    game_state: &mut ResMut<crate::game::game_state::GameState>,
    projectile_entity: Entity,
    position: Vec3,
    projectile: &Projectile,
) {
    // Remove projectile
    commands.entity(projectile_entity).despawn();
    
    // Destroy terrain in explosion radius
    let world_x = position.x + (terrain.width as f32 / 2.0);
    let world_y = position.y + (terrain.height as f32 / 2.0);
    terrain.destroy_circle(world_x, world_y, projectile.explosion_radius);
    
    // Update game state
    game_state.explosion_started();
    
    // Spawn explosion particles
    crate::game::particles::spawn_explosion_particles(
        commands,
        meshes,
        materials,
        position,
        (projectile.explosion_radius / 5.0) as usize, // Scale particle count with explosion size
    );
    
    // Spawn dirt particles
    crate::game::particles::spawn_dirt_particles(
        commands,
        meshes,
        materials,
        position,
        (projectile.explosion_radius / 8.0) as usize,
    );
    
    // Create explosion effect
    commands.spawn((
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(projectile.explosion_radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 0.5, 0.0, 0.6)))),
        Transform::from_translation(position),
        Explosion {
            radius: projectile.explosion_radius,
            damage: projectile.damage,
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
        },
    ));
}

fn explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<crate::game::game_state::GameState>,
    mut explosion_query: Query<(Entity, &Transform, &mut Explosion)>,
    mut worm_query: Query<(&Transform, &mut Worm), Without<Explosion>>,
) {
    for (entity, transform, mut explosion) in explosion_query.iter_mut() {
        explosion.lifetime.tick(time.delta());
        
        if explosion.lifetime.just_finished() {
            // Damage worms in explosion radius
            for (worm_transform, mut worm) in worm_query.iter_mut() {
                let distance = transform.translation.distance(worm_transform.translation);
                if distance <= explosion.radius {
                    let damage_ratio = 1.0 - (distance / explosion.radius);
                    let damage = explosion.damage * damage_ratio;
                    worm.health -= damage;
                    worm.health = worm.health.max(0.0);
                }
            }
            
            // End turn after explosion
            game_state.end_turn();
            
            // Remove explosion effect
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_expired_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in query.iter() {
        // Remove projectiles that have gone too far off screen
        if transform.translation.y < -1000.0 || transform.translation.x.abs() > 2000.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn change_wind_on_turn_end(
    mut wind_system: ResMut<WindSystem>,
    game_state: Res<crate::game::game_state::GameState>,
) {
    // Change wind when turn transitions happen
    if game_state.game_phase == crate::game::game_state::GamePhase::TurnTransition {
        // Only change wind occasionally (30% chance)
        if fastrand::f32() < 0.3 {
            wind_system.generate_new_wind();
        }
    }
}
    }
}

fn update_wind(
    time: Res<Time>,
    mut wind: ResMut<WindSystem>,
) {
    wind.change_timer.tick(time.delta());
    if wind.change_timer.just_finished() {
        wind.generate_new_wind();
    }
}