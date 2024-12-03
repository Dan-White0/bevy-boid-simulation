use bevy::prelude::*;

use super::AppState;

pub fn wait_until_ready(
    camera_query: Query<&Camera>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if camera_query.get_single().is_ok() {
        app_state_next_state.set(AppState::MainMenu);
    }
}
