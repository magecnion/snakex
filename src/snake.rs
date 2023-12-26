use crate::movement::Velocity;

use bevy::prelude::*;
#[derive(Bundle)]
struct SnakeBundle {
    velocity: Velocity,
    head: SpriteBundle,
}

const SNAKE_VELOCITY: Vec3 = Vec3::new(100.0, 0.0, 0.0);

fn spawn_snake(mut commands: Commands) {
    commands.spawn((SnakeBundle {
        velocity: Velocity {
            value: SNAKE_VELOCITY,
        },
        head: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(3.7, 0.7, 1.7),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            ..default()
        },
    },));
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake);
    }
}
