use bevy::prelude::*;

use super::RunSimulation;

pub fn start_stop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut run_simulation: ResMut<RunSimulation>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        run_simulation.0 = !run_simulation.0;
    }
}
