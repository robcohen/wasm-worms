use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "WASM Worms - Simple".into(),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(move_worms)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "WASM Worms - Simple".into(),
                width: 1024.0,
                height: 768.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(move_worms)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct Worm;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    
    // Spawn a simple worm
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(bevy::render::mesh::shape::Circle::new(16.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.8, 0.2))),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    })
    .insert(Worm);
}

fn move_worms(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Worm>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let speed = 200.0;
        
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= speed * time.delta_seconds();
        }
    }
}