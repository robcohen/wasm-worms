use bevy::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_particles,
            cleanup_expired_particles,
        ));
    }
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub fade_rate: f32,
    pub gravity_scale: f32,
}

#[derive(Component)]
pub struct ParticleSystem;

impl Particle {
    pub fn new(velocity: Vec2, lifetime: f32, gravity_scale: f32) -> Self {
        Self {
            velocity,
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            fade_rate: 1.0 / lifetime,
            gravity_scale,
        }
    }
}

pub fn spawn_explosion_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    count: usize,
) {
    for _ in 0..count {
        let angle = fastrand::f32() * std::f32::consts::TAU;
        let speed = fastrand::f32() * 300.0 + 100.0;
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        
        let lifetime = fastrand::f32() * 1.5 + 0.5;
        let size = fastrand::f32() * 4.0 + 2.0;
        
        // Random explosion colors
        let color = match fastrand::u32(0..4) {
            0 => Color::srgb(1.0, 0.5, 0.0), // Orange
            1 => Color::srgb(1.0, 0.8, 0.0), // Yellow
            2 => Color::srgb(0.8, 0.2, 0.0), // Red
            _ => Color::srgb(0.6, 0.6, 0.6), // Gray smoke
        };
        
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Circle::new(size))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_translation(position + Vec3::new(
                (fastrand::f32() - 0.5) * 20.0,
                (fastrand::f32() - 0.5) * 20.0,
                0.1,
            )),
            Particle::new(velocity, lifetime, 0.8),
            ParticleSystem,
        ));
    }
}

pub fn spawn_dirt_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    count: usize,
) {
    for _ in 0..count {
        let angle = fastrand::f32() * std::f32::consts::PI; // Only upward
        let speed = fastrand::f32() * 200.0 + 50.0;
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        
        let lifetime = fastrand::f32() * 2.0 + 1.0;
        let size = fastrand::f32() * 3.0 + 1.0;
        
        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Circle::new(size))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.4, 0.3, 0.2)))),
            Transform::from_translation(position + Vec3::new(
                (fastrand::f32() - 0.5) * 30.0,
                (fastrand::f32() - 0.5) * 10.0,
                0.05,
            )),
            Particle::new(velocity, lifetime, 1.2),
            ParticleSystem,
        ));
    }
}

fn update_particles(
    time: Res<Time>,
    mut particle_query: Query<(&mut Transform, &mut Particle, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut transform, mut particle, mut material_handle) in particle_query.iter_mut() {
        // Update lifetime
        particle.lifetime.tick(time.delta());
        
        // Apply physics
        particle.velocity.y -= 980.0 * particle.gravity_scale * time.delta_secs();
        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();
        
        // Apply friction
        particle.velocity *= 0.98;
        
        // Fade out over time
        let alpha = 1.0 - particle.lifetime.fraction();
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let mut color = material.color;
            color.set_alpha(alpha);
            material.color = color;
        }
    }
}

fn cleanup_expired_particles(
    mut commands: Commands,
    particle_query: Query<(Entity, &Particle), With<ParticleSystem>>,
) {
    for (entity, particle) in particle_query.iter() {
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}