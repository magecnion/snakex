mod camera;
mod debug;
mod movement;
mod snake;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(MovementPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
