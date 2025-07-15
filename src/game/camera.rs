use bevy::prelude::*;
use crate::game::worm::{Worm, PlayerControlled};
use crate::game::game_state::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CameraController::default())
            .add_systems(Update, (
                camera_follow_active_worm,
                camera_manual_controls,
                camera_zoom_controls,
            ));
    }
}

#[derive(Resource)]
pub struct CameraController {
    pub follow_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub manual_control: bool,
    pub target_position: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            follow_speed: 2.0,
            zoom_speed: 0.1,
            min_zoom: 0.3,
            max_zoom: 2.0,
            manual_control: false,
            target_position: Vec3::ZERO,
        }
    }
}

fn camera_follow_active_worm(
    mut camera_controller: ResMut<CameraController>,
    game_state: Res<GameState>,
    worm_query: Query<&Transform, (With<Worm>, Without<Camera>)>,
) {
    if camera_controller.manual_control {
        return;
    }
    
    // Find the active player's worm
    for transform in worm_query.iter() {
        // For now, just follow any worm - in a full implementation,
        // you'd match the worm's team to the current player
        camera_controller.target_position = transform.translation;
        break;
    }
}

fn camera_manual_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_controller: ResMut<CameraController>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut manual_movement = Vec3::ZERO;
    let move_speed = 300.0;
    
    // Check for manual camera movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        manual_movement.y += move_speed * time.delta_seconds();
        camera_controller.manual_control = true;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        manual_movement.y -= move_speed * time.delta_seconds();
        camera_controller.manual_control = true;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        manual_movement.x -= move_speed * time.delta_seconds();
        camera_controller.manual_control = true;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        manual_movement.x += move_speed * time.delta_seconds();
        camera_controller.manual_control = true;
    }
    
    // Return to auto-follow mode
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        camera_controller.manual_control = false;
    }
    
    // Apply manual movement
    if manual_movement != Vec3::ZERO {
        for mut transform in camera_query.iter_mut() {
            transform.translation += manual_movement;
            camera_controller.target_position = transform.translation;
        }
    }
    
    // Smooth camera following when not in manual mode
    if !camera_controller.manual_control {
        for mut transform in camera_query.iter_mut() {
            let target = camera_controller.target_position;
            let current = transform.translation;
            let distance = target - current;
            
            if distance.length() > 10.0 {
                let movement = distance * camera_controller.follow_speed * time.delta_seconds();
                transform.translation += movement;
            }
        }
    }
}

fn camera_zoom_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_controller: ResMut<CameraController>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    let mut zoom_delta = 0.0;
    
    if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
        zoom_delta -= camera_controller.zoom_speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract) {
        zoom_delta += camera_controller.zoom_speed * time.delta_seconds();
    }
    
    if zoom_delta != 0.0 {
        for mut projection in camera_query.iter_mut() {
            projection.scale += zoom_delta;
            projection.scale = projection.scale.clamp(
                camera_controller.min_zoom,
                camera_controller.max_zoom
            );
        }
    }
}