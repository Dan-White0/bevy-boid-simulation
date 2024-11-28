use bevy::app::AppExit;
use bevy::prelude::*;

use super::components::{QuitButton, StartButton};
use super::styles::{HOVERED_BUTTON_COLOUR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOUR};

use crate::AppState;

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                app_state_next_state.set(AppState::Simulation);
            }
            Interaction::Hovered => {
                *background_colour = HOVERED_BUTTON_COLOUR.into();
            }
            Interaction::None => {
                *background_colour = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_write: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                app_exit_event_write.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_colour = HOVERED_BUTTON_COLOUR.into();
            }
            Interaction::None => {
                *background_colour = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}
