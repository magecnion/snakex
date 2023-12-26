use crate::movement::{Velocity, SNAKE_SPEED};

use bevy::prelude::*;
#[derive(Bundle)]
struct SnakeBundle {
    velocity: Velocity,
    head: SpriteBundle,
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((SnakeBundle {
        velocity: Velocity {
            value: Vec3::new(SNAKE_SPEED, 0.0, 0.0),
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
