use bevy::app::AppExit;
use bevy::prelude::*;

use super::components::{QuitButton, Start2DButton, Start3DButton};
use super::styles::{HOVERED_BUTTON_COLOUR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOUR};

use crate::AppState;

pub fn interact_with_start_2d_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Start2DButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                app_state_next_state.set(AppState::Simulation2D);
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

pub fn interact_with_start_3d_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Start3DButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                app_state_next_state.set(AppState::Simulation3D);
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
