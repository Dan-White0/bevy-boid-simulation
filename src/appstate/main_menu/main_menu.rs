use super::components::{MainMenu, QuitButton, StartButton};
use bevy::prelude::*;

use super::styles::{
    get_button_text_style, get_title_text_style, BUTTON_STYLE, MAIN_MENU_STYLE,
    NORMAL_BUTTON_COLOUR, TITLE_STYLE,
};
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // ------ Title ------
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle::from_section(
                        "Bevy Boids Simulation!",
                        get_title_text_style(&asset_server),
                    ));
                });

            // ------ Start Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    StartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Simulation",
                        get_button_text_style(asset_server),
                    ));
                });

            // ------ Quit Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        get_button_text_style(asset_server),
                    ));
                });
        })
        .id();

    main_menu_entity
}
