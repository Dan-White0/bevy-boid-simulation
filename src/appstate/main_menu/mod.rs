use bevy::prelude::*;

mod components;
mod interactions;
mod main_menu;
mod styles;
mod view;

pub use interactions::{
    interact_with_quit_button, interact_with_start_2d_button, interact_with_start_3d_button,
};
pub use main_menu::{despawn_main_menu, spawn_main_menu};
use view::set_main_menu_view;

use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interact_with_start_2d_button,
                interact_with_start_3d_button,
                interact_with_quit_button,
            )
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            OnEnter(AppState::MainMenu),
            (spawn_main_menu, set_main_menu_view),
        )
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
