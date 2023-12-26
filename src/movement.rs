use bevy::{prelude::*, time};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, snake_movement));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<time::Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

pub const SNAKE_SPEED: f32 = 100.0;

fn snake_movement(
    mut query: Query<&mut Velocity>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<time::Time>,
) {
    let mut velocity = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        velocity.value = Vec3::new(-SNAKE_SPEED, 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::Right) {
        velocity.value = Vec3::new(SNAKE_SPEED, 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::Up) {
        velocity.value = Vec3::new(0.0, SNAKE_SPEED, 0.0);
    } else if keyboard_input.pressed(KeyCode::Down) {
        velocity.value = Vec3::new(0.0, -SNAKE_SPEED, 0.0);
    }
}
