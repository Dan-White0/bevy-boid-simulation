use bevy::prelude::*;

mod camera;
mod wait_until_ready;

use camera::spawn_camera;
use wait_until_ready::wait_until_ready;

use super::AppState;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, wait_until_ready.run_if(in_state(AppState::Startup)));
    }
}
