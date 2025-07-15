use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                apply_gravity,
                apply_velocity,
                check_ground_collision,
            ).chain());
    }
}

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub mass: f32,
    pub bounce: f32,
    pub friction: f32,
    pub gravity_scale: f32,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            mass: 1.0,
            bounce: 0.3,
            friction: 0.8,
            gravity_scale: 1.0,
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub is_grounded: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: 16.0,
            is_grounded: false,
        }
    }
}

const GRAVITY: f32 = -980.0; // pixels per second squared
const GROUND_Y: f32 = -300.0; // temporary ground level

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<&mut RigidBody, With<Collider>>,
) {
    for mut body in query.iter_mut() {
        body.velocity.y += GRAVITY * body.gravity_scale * time.delta_seconds();
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RigidBody)>,
) {
    for (mut transform, mut body) in query.iter_mut() {
        transform.translation.x += body.velocity.x * time.delta_seconds();
        transform.translation.y += body.velocity.y * time.delta_seconds();
        
        // Apply friction to horizontal movement
        body.velocity.x *= body.friction.powf(time.delta_seconds());
    }
}

fn check_ground_collision(
    mut query: Query<(&mut Transform, &mut RigidBody, &mut Collider)>,
) {
    for (mut transform, mut body, mut collider) in query.iter_mut() {
        let bottom_y = transform.translation.y - collider.radius;
        
        if bottom_y <= GROUND_Y {
            // Collision with ground
            transform.translation.y = GROUND_Y + collider.radius;
            
            if body.velocity.y < 0.0 {
                body.velocity.y = -body.velocity.y * body.bounce;
                
                // Stop bouncing if velocity is too small
                if body.velocity.y.abs() < 50.0 {
                    body.velocity.y = 0.0;
                    collider.is_grounded = true;
                }
            }
        } else {
            collider.is_grounded = false;
        }
    }
}