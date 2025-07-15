use bevy::prelude::*;

pub mod physics;
pub mod terrain;
pub mod worm;
pub mod game_state;
pub mod camera;
pub mod weapons;
pub mod aiming;

use physics::PhysicsPlugin;
use terrain::TerrainPlugin;
use worm::WormPlugin;
use game_state::GameStatePlugin;
use camera::CameraPlugin;
use weapons::WeaponPlugin;
use aiming::AimingPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PhysicsPlugin,
                TerrainPlugin,
                WormPlugin,
                GameStatePlugin,
                CameraPlugin,
                WeaponPlugin,
                AimingPlugin,
            ))
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5, // Start zoomed out to see more terrain
            ..default()
        }),
    ));
}