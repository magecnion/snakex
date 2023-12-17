use bevy::prelude::*;

// ECS:
//  Entities (snake) are unique “things” that are assigned groups of; collection of components with a unique id
//    Bundle: "templates" or "blueprints" for creating entities
//  Components (velocity), which are then processed using; just a normal Rust data type. generally scoped to a single piece of functionality
//  Systems, e.g.: movement system that runs on all entities with a Position and Velocity component
//      runs logic on entities, components, and resources; your game logic
//      Systems are contained in schedules
//
// Resources:
//  most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using Resources.
//  a shared global piece of data; configuration / settings
//
// Plugins:
//  All Bevy engine features are implemented as plugins
//
// The ECS's data structure is called the World
// The typical and obvious pattern is to use entities to represent "objects in the game/scene", such as the camera, the player, enemies, lights, props, UI elements, and other things.
//
// Queries
// Commands
// Transform
// Camera
// Asset + bundles
//  handle is a pointer to the asset we wanna use
// TODO: duda porque en la query puedo tener un &mut si se supone que es inmutable

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player {}

// spawn player entity and add player compoenen to them
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>, // in order to load png files
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    // get ref to player transform
    let (_, mut transform) = player_query.single_mut(); // TODO remove entity and see if it fails
    let mut direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Left) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
        // not sure if I need this normalization
        transform.translation += direction.normalize() * 500.0 * time.delta_seconds();
    }
}
