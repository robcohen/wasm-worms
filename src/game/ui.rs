use bevy::prelude::*;
use crate::game::game_state::{GameState, GamePhase, TurnTimer};
use crate::game::worm::Worm;
use crate::game::weapons::WeaponInventory;
use crate::game::aiming::AimingState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_turn_timer_ui,
                update_health_bars_ui,
                update_weapon_selection_ui,
                update_game_phase_ui,
                update_power_meter_ui,
                update_wind_indicator_ui,
            ));
    }
}

// UI Components
#[derive(Component)]
pub struct TurnTimerText;

#[derive(Component)]
pub struct HealthBarUI;

#[derive(Component)]
pub struct WeaponSelectionUI;

#[derive(Component)]
pub struct GamePhaseText;

#[derive(Component)]
pub struct PowerMeterUI;

#[derive(Component)]
pub struct WindIndicatorUI;

#[derive(Component)]
pub struct GameOverUI;

fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        ..default()
    }).with_children(|parent| {
        // Top bar - Turn timer and game phase
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(60.0),
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        }).with_children(|top_bar| {
            // Turn timer
            top_bar.spawn((
                Text::new("Turn: 30s"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TurnTimerText,
            ));
            
            // Game phase indicator
            top_bar.spawn((
                Text::new("Player Turn"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::YELLOW),
                GamePhaseText,
            ));
        });
        
        // Bottom bar - Weapon selection and power meter
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        }).with_children(|bottom_bar| {
            // Weapon selection
            bottom_bar.spawn((
                Text::new("Weapon: Grenade (1/2/3 to switch)"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                WeaponSelectionUI,
            ));
            
            // Power meter (initially hidden)
            bottom_bar.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(20.0),
                    background_color: Color::srgba(0.3, 0.3, 0.3, 0.8).into(),
                    border: UiRect::all(Val::Px(2.0)),
                    display: Display::None,
                    ..default()
                },
                BorderColor(Color::WHITE),
                PowerMeterUI,
            )).with_children(|power_bar| {
                // Power fill
                power_bar.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        background_color: Color::srgb(0.2, 0.8, 0.2).into(),
                        ..default()
                    },
                ));
            });
        });
        
        // Wind indicator (top right)
        parent.spawn((
            Node {
                width: Val::Px(150.0),
                height: Val::Px(40.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.6).into(),
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                right: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            WindIndicatorUI,
        )).with_children(|wind| {
            wind.spawn((
                Text::new("Wind: →→ 15"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::CYAN),
            ));
        });
    });
}

fn update_turn_timer_ui(
    timer: Res<TurnTimer>,
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<TurnTimerText>>,
) {
    for mut text in query.iter_mut() {
        let time_left = timer.current_time.max(0.0) as i32;
        let current_team = game_state.current_player + 1;
        **text = format!("Player {} - Time: {}s", current_team, time_left);
    }
}

fn update_health_bars_ui(
    worm_query: Query<&Worm>,
    game_state: Res<GameState>,
    // We'll implement visual health bars in the world, not UI
) {
    // Health bars are already implemented as world-space UI in worm.rs
    // This could be enhanced to show a summary in the UI panel
}

fn update_weapon_selection_ui(
    weapon_inventory: Res<WeaponInventory>,
    mut query: Query<&mut Text, With<WeaponSelectionUI>>,
) {
    for mut text in query.iter_mut() {
        if let Some(current_weapon) = weapon_inventory.weapons.get(weapon_inventory.current_weapon) {
            **text = format!(
                "Weapon: {} (1/2/3 to switch, Q/E to cycle)",
                match current_weapon {
                    crate::game::weapons::WeaponType::Grenade => "Grenade",
                    crate::game::weapons::WeaponType::Bazooka => "Bazooka",
                    crate::game::weapons::WeaponType::ClusterBomb => "Cluster Bomb",
                }
            );
        }
    }
}

fn update_game_phase_ui(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<GamePhaseText>>,
) {
    for mut text in query.iter_mut() {
        **text = match game_state.game_phase {
            GamePhase::PlayerTurn => "Your Turn - Move & Aim".to_string(),
            GamePhase::Aiming => "Aiming - Use arrows, Enter to charge".to_string(),
            GamePhase::Firing => "Firing!".to_string(),
            GamePhase::ProjectileFlying => "Projectile Flying...".to_string(),
            GamePhase::Explosion => "BOOM!".to_string(),
            GamePhase::TurnTransition => "Turn Ending...".to_string(),
            GamePhase::GameOver => "Game Over!".to_string(),
        };
    }
}

fn update_power_meter_ui(
    aiming_state: Res<AimingState>,
    mut power_meter_query: Query<&mut Node, With<PowerMeterUI>>,
    mut power_fill_query: Query<&mut Node, (Without<PowerMeterUI>, With<Node>)>,
) {
    for mut power_meter in power_meter_query.iter_mut() {
        if aiming_state.is_aiming && aiming_state.power_charging {
            power_meter.display = Display::Flex;
            
            // Update power fill
            for mut fill in power_fill_query.iter_mut() {
                fill.width = Val::Percent(aiming_state.power * 100.0);
                
                // Change color based on power level
                if aiming_state.power < 0.7 {
                    fill.background_color = Color::srgb(0.2, 0.8, 0.2).into();
                } else if aiming_state.power < 0.9 {
                    fill.background_color = Color::srgb(0.8, 0.8, 0.2).into();
                } else {
                    fill.background_color = Color::srgb(0.8, 0.2, 0.2).into();
                }
            }
        } else {
            power_meter.display = Display::None;
        }
    }
}

fn update_wind_indicator_ui(
    wind_system: Res<crate::game::weapons::WindSystem>,
    mut query: Query<&mut Text, (With<WindIndicatorUI>, With<Text>)>,
) {
    for mut text in query.iter_mut() {
        let wind_strength = wind_system.force.length();
        let wind_direction = if wind_system.force.x > 0.0 { "→" } else { "←" };
        let arrow_count = (wind_strength / 20.0).clamp(0.0, 3.0) as i32;
        let arrows = wind_direction.repeat(arrow_count.max(1) as usize);
        
        **text = format!("Wind: {} {:.0}", arrows, wind_strength);
    }
}