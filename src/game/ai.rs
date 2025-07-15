use bevy::prelude::*;
use crate::game::game_state::{GameState, GamePhase};
use crate::game::worm::{Worm, PlayerControlled};
use crate::game::aiming::AimingState;
use crate::game::weapons::{WeaponInventory, fire_weapon};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AIController::default())
            .add_systems(Update, (
                ai_decision_making,
                ai_execute_action,
            ));
    }
}

#[derive(Resource)]
pub struct AIController {
    pub thinking_time: Timer,
    pub current_action: AIAction,
    pub target_angle: f32,
    pub target_power: f32,
    pub action_timer: Timer,
}

impl Default for AIController {
    fn default() -> Self {
        Self {
            thinking_time: Timer::from_seconds(2.0, TimerMode::Once),
            current_action: AIAction::Thinking,
            target_angle: 45.0,
            target_power: 0.5,
            action_timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AIAction {
    Thinking,
    Moving,
    Aiming,
    Firing,
    Done,
}

#[derive(Component)]
pub struct AIControlled;

fn ai_decision_making(
    mut ai_controller: ResMut<AIController>,
    game_state: Res<GameState>,
    time: Res<Time>,
    ai_worm_query: Query<(&Transform, &Worm), (With<AIControlled>, Without<PlayerControlled>)>,
    target_worm_query: Query<(&Transform, &Worm), (With<PlayerControlled>, Without<AIControlled>)>,
) {
    // Only act when it's AI's turn and in player turn phase
    if game_state.game_phase != GamePhase::PlayerTurn {
        return;
    }
    
    // Check if current player is AI controlled
    let is_ai_turn = ai_worm_query.iter().any(|(_, worm)| 
        worm.team == game_state.current_player && worm.health > 0.0
    );
    
    if !is_ai_turn {
        return;
    }
    
    match ai_controller.current_action {
        AIAction::Thinking => {
            ai_controller.thinking_time.tick(time.delta());
            if ai_controller.thinking_time.finished() {
                // Make AI decision
                if let Some((ai_transform, _)) = ai_worm_query.iter()
                    .find(|(_, worm)| worm.team == game_state.current_player) {
                    
                    if let Some((target_transform, _)) = target_worm_query.iter().next() {
                        // Calculate angle and power to hit target
                        let distance = target_transform.translation - ai_transform.translation;
                        
                        // Simple AI: aim roughly at target with some randomness
                        ai_controller.target_angle = distance.y.atan2(distance.x).to_degrees() 
                            + (fastrand::f32() - 0.5) * 30.0; // Add some inaccuracy
                        
                        let distance_factor = distance.length() / 500.0;
                        ai_controller.target_power = (distance_factor * 0.8 + 0.2)
                            .clamp(0.3, 1.0);
                        
                        ai_controller.current_action = AIAction::Aiming;
                        ai_controller.action_timer.reset();
                    }
                }
            }
        }
        _ => {} // Other actions handled in ai_execute_action
    }
}

fn ai_execute_action(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ai_controller: ResMut<AIController>,
    mut aiming_state: ResMut<AimingState>,
    mut game_state: ResMut<GameState>,
    weapon_inventory: Res<WeaponInventory>,
    time: Res<Time>,
    ai_worm_query: Query<&Transform, (With<AIControlled>, With<Worm>)>,
) {
    // Only execute when it's AI's turn
    let is_ai_turn = ai_worm_query.iter().any(|_| true); // Simplified check
    if !is_ai_turn || game_state.current_player != 1 { // Assume AI is player 1
        return;
    }
    
    match ai_controller.current_action {
        AIAction::Aiming => {
            // Start aiming
            if !aiming_state.is_aiming {
                aiming_state.is_aiming = true;
                game_state.start_aiming();
            }
            
            // Gradually adjust aim to target
            let angle_diff = ai_controller.target_angle - aiming_state.aim_angle;
            if angle_diff.abs() > 2.0 {
                aiming_state.aim_angle += angle_diff.signum() * 30.0 * time.delta_secs();
            } else {
                aiming_state.aim_angle = ai_controller.target_angle;
                ai_controller.current_action = AIAction::Firing;
                ai_controller.action_timer.reset();
            }
        }
        
        AIAction::Firing => {
            ai_controller.action_timer.tick(time.delta());
            
            // Start charging power
            if !aiming_state.power_charging {
                aiming_state.power_charging = true;
                aiming_state.power = 0.0;
            }
            
            // Charge to target power
            if aiming_state.power < ai_controller.target_power {
                aiming_state.power += time.delta_secs() * 0.8;
            } else {
                // Fire!
                if let Some(current_weapon) = weapon_inventory.weapons.get(weapon_inventory.current_weapon) {
                    if let Some(ai_transform) = ai_worm_query.iter().next() {
                        let angle_rad = aiming_state.aim_angle.to_radians();
                        let direction = Vec2::new(angle_rad.cos(), angle_rad.sin());
                        
                        let firing_position = ai_transform.translation + Vec3::new(
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
                        
                        // Reset AI state
                        aiming_state.is_aiming = false;
                        aiming_state.power_charging = false;
                        aiming_state.power = 0.5;
                        
                        ai_controller.current_action = AIAction::Done;
                        ai_controller.thinking_time.reset();
                    }
                }
            }
        }
        
        AIAction::Done => {
            // Wait for turn to end naturally
            if game_state.game_phase == GamePhase::PlayerTurn {
                ai_controller.current_action = AIAction::Thinking;
            }
        }
        
        _ => {}
    }
}