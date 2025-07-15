use bevy::prelude::*;
use crate::game::worm::Worm;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState::new())
            .insert_resource(TurnTimer::new(30.0))
            .add_systems(Update, (
                update_turn_timer,
                handle_turn_end,
                handle_turn_transition,
                update_active_player_indicator,
            ));
    }
}

#[derive(Resource)]
pub struct GameState {
    pub current_player: u32,
    pub game_phase: GamePhase,
    pub teams: Vec<Team>,
    pub winner: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    PlayerTurn,
    Aiming,
    Firing,
    ProjectileFlying,
    Explosion,
    TurnTransition,
    GameOver,
}

#[derive(Clone)]
pub struct Team {
    pub id: u32,
    pub color: Color,
    pub worms_alive: u32,
}

#[derive(Resource)]
pub struct TurnTimer {
    pub max_time: f32,
    pub current_time: f32,
    pub is_active: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_player: 0,
            game_phase: GamePhase::PlayerTurn,
            teams: vec![
                Team { id: 0, color: Color::srgb(0.2, 0.8, 0.2), worms_alive: 1 },
                Team { id: 1, color: Color::srgb(0.8, 0.2, 0.2), worms_alive: 1 },
            ],
            winner: None,
        }
    }
    
    pub fn can_player_act(&self) -> bool {
        matches!(self.game_phase, GamePhase::PlayerTurn | GamePhase::Aiming)
    }
    
    pub fn start_aiming(&mut self) {
        if self.game_phase == GamePhase::PlayerTurn {
            self.game_phase = GamePhase::Aiming;
        }
    }
    
    pub fn start_firing(&mut self) {
        if self.game_phase == GamePhase::Aiming {
            self.game_phase = GamePhase::Firing;
        }
    }
    
    pub fn projectile_launched(&mut self) {
        if self.game_phase == GamePhase::Firing {
            self.game_phase = GamePhase::ProjectileFlying;
        }
    }
    
    pub fn explosion_started(&mut self) {
        if self.game_phase == GamePhase::ProjectileFlying {
            self.game_phase = GamePhase::Explosion;
        }
    }
    
    pub fn end_turn(&mut self) {
        self.game_phase = GamePhase::TurnTransition;
        self.current_player = (self.current_player + 1) % self.teams.len() as u32;
        // After a brief transition, return to PlayerTurn
    }
    
    pub fn start_new_turn(&mut self) {
        self.game_phase = GamePhase::PlayerTurn;
    }
    }
    
    pub fn next_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.teams.len() as u32;
        self.game_phase = GamePhase::PlayerTurn;
    }
    
    pub fn get_current_team(&self) -> Option<&Team> {
        self.teams.iter().find(|team| team.id == self.current_player)
    }
    
    pub fn check_win_condition(&mut self, worm_query: &Query<&Worm>) {
        // Count alive worms per team
        let mut team_counts = vec![0u32; self.teams.len()];
        
        for worm in worm_query.iter() {
            if worm.health > 0.0 {
                team_counts[worm.team as usize] += 1;
            }
        }
        
        // Update team alive counts
        for (i, team) in self.teams.iter_mut().enumerate() {
            team.worms_alive = team_counts[i];
        }
        
        // Check for winner
        let alive_teams: Vec<_> = team_counts.iter().enumerate()
            .filter(|(_, &count)| count > 0)
            .collect();
            
        if alive_teams.len() == 1 {
            self.winner = Some(alive_teams[0].0 as u32);
            self.game_phase = GamePhase::GameOver;
        }
    }
}

impl TurnTimer {
    pub fn new(max_time: f32) -> Self {
        Self {
            max_time,
            current_time: max_time,
            is_active: true,
        }
    }
    
    pub fn reset(&mut self) {
        self.current_time = self.max_time;
        self.is_active = true;
    }
    
    pub fn pause(&mut self) {
        self.is_active = false;
    }
    
    pub fn resume(&mut self) {
        self.is_active = true;
    }
    
    pub fn is_expired(&self) -> bool {
        self.current_time <= 0.0
    }
    
    pub fn time_remaining_ratio(&self) -> f32 {
        (self.current_time / self.max_time).clamp(0.0, 1.0)
    }
}

fn update_turn_timer(
    time: Res<Time>,
    mut timer: ResMut<TurnTimer>,
    game_state: Res<GameState>,
) {
    if timer.is_active && game_state.game_phase == GamePhase::PlayerTurn {
        timer.current_time -= time.delta_secs();
        timer.current_time = timer.current_time.max(0.0);
    }
}

fn handle_turn_end(
    mut game_state: ResMut<GameState>,
    mut timer: ResMut<TurnTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    worm_query: Query<&Worm>,
) {
    let should_end_turn = timer.is_expired() 
        || keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::Tab);
    
    if should_end_turn && game_state.game_phase == GamePhase::PlayerTurn {
        game_state.check_win_condition(&worm_query);
        
        if game_state.winner.is_none() {
            game_state.next_turn();
            timer.reset();
        }
    }
}

fn handle_turn_transition(
    mut game_state: ResMut<GameState>,
    mut timer: ResMut<TurnTimer>,
    time: Res<Time>,
) {
    if game_state.game_phase == GamePhase::TurnTransition {
        // Brief pause between turns
        static mut TRANSITION_TIME: f32 = 0.0;
        unsafe {
            TRANSITION_TIME += time.delta_secs();
            if TRANSITION_TIME >= 1.0 {
                TRANSITION_TIME = 0.0;
                game_state.start_new_turn();
                timer.reset();
            }
        }
    }
}

#[derive(Component)]
pub struct PlayerIndicator;

fn update_active_player_indicator(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    indicator_query: Query<Entity, With<PlayerIndicator>>,
    worm_query: Query<(&Transform, &Worm)>,
) {
    // Remove existing indicators
    for entity in indicator_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Add indicator to current player's worm
    if let Some(current_team) = game_state.get_current_team() {
        for (transform, worm) in worm_query.iter() {
            if worm.team == game_state.current_player && worm.health > 0.0 {
                // Spawn indicator above worm
                commands.spawn((
                    Mesh2d(meshes.add(bevy::math::primitives::Circle::new(6.0))),
                    MeshMaterial2d(materials.add(ColorMaterial::from(current_team.color))),
                    Transform::from_translation(
                        transform.translation + Vec3::new(0.0, 30.0, 0.1)
                    ),
                    PlayerIndicator,
                ));
                break;
            }
        }
    }
}