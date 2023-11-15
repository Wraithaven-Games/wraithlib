use bevy::prelude::*;
use bevy_simple_text_input::TextInput;

use super::*;
use crate::client::ui::ScrollPane;

pub(super) fn build_ui(
    properties: Res<MainMenuProperties>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let center = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let bg_style = Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::None,
        ..center.clone()
    };

    let btn_col_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(400.0),
        ..center.clone()
    };

    let btn_style = Style {
        margin: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..center.clone()
    };

    if let Some(screen) = &properties.title_screen {
        commands
            .spawn((
                MainMenuScreen,
                TitleScreen,
                ImageBundle {
                    style: Style {
                        display: Display::Flex,
                        ..bg_style.clone()
                    },
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    if let Some(button) = &screen.single_player_button {
                        p.spawn((
                            SinglePlayerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.multiplayer_button {
                        p.spawn((
                            MultiplayerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.settings_button {
                        p.spawn((
                            SettingsButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.credits_button {
                        p.spawn((
                            CreditsButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }

                    if let Some(button) = &screen.quit_button {
                        p.spawn((
                            QuitButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    }
                });
            });
    }

    if let Some(screen) = &properties.single_player_screen {
        commands
            .spawn((
                MainMenuScreen,
                SinglePlayerScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.new_game_button;
                    p.spawn((
                        NewGameButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));

                    let button = &screen.load_game_button;
                    p.spawn((
                        LoadGameButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));

                    let button = &screen.back_button;
                    p.spawn((
                        BackToTitleScreenButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }

    if let Some(screen) = &properties.multiplayer_screen {
        commands
            .spawn((
                MainMenuScreen,
                MultiplayerScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..center.clone()
                    },
                    background_color: Color::NONE.into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            flex_grow: 1.0,
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            border: UiRect::all(Val::Px(2.0)),
                            overflow: Overflow::clip_y(),
                            ..center.clone()
                        },
                        background_color: Color::NONE.into(),
                        border_color: Color::RED.into(),
                        ..default()
                    })
                    .with_children(|p| {
                        p.spawn((
                            ServerListPane,
                            ScrollPane::default(),
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::Stretch,
                                    ..default()
                                },
                                background_color: Color::NONE.into(),
                                border_color: Color::PINK.into(),
                                ..default()
                            },
                        ));
                    });

                    p.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            width: Val::Percent(100.0),
                            height: Val::Px(80.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..center.clone()
                        },
                        background_color: Color::NONE.into(),
                        border_color: Color::BLUE.into(),
                        ..default()
                    })
                    .with_children(|p| {
                        let button = &screen.add_server_button;
                        p.spawn((
                            AddServerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));

                        let button = &screen.back_button;
                        p.spawn((
                            BackToTitleScreenButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    });
                });
            });

        let screen = &screen.edit_server_screen;
        commands
            .spawn((
                MainMenuScreen,
                EditServerScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::all(Val::Px(2.0)),
                        ..btn_col_style.clone()
                    },
                    background_color: Color::NONE.into(),
                    border_color: Color::YELLOW.into(),
                    ..default()
                })
                .with_children(|p| {
                    // Server name
                    p.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(40.0),
                                padding: UiRect::all(Val::Px(5.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..center.clone()
                            },
                            background_color: Color::WHITE.into(),
                            border_color: BorderColor(Color::BLACK),
                            ..default()
                        },
                        TextInput {
                            text_style: TextStyle {
                                font_size: 26.0,
                                color: Color::BLACK,
                                ..default()
                            },
                            inactive: true,
                        },
                    ));

                    // Server ip
                    p.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(40.0),
                                padding: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::top(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..center.clone()
                            },
                            background_color: Color::WHITE.into(),
                            border_color: BorderColor(Color::BLACK),
                            ..default()
                        },
                        TextInput {
                            text_style: TextStyle {
                                font_size: 26.0,
                                color: Color::BLACK,
                                ..default()
                            },
                            inactive: true,
                        },
                    ));

                    p.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            width: Val::Px(400.0),
                            height: Val::Px(60.0),
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..center.clone()
                        },
                        ..default()
                    })
                    .with_children(|p| {
                        let button = &screen.confirm_button;
                        p.spawn((
                            ConfirmEditServerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));

                        let button = &screen.back_button;
                        p.spawn((
                            BackToMultiplayerButton,
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(button.img_size.x),
                                    height: Val::Px(button.img_size.y),
                                    ..btn_style.clone()
                                },
                                image: asset_server.load(&button.img_path).into(),
                                ..default()
                            },
                        ));
                    });
                });
            });
    }

    if let Some(screen) = &properties.settings_screen {
        commands
            .spawn((
                MainMenuScreen,
                SettingsScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.back_button;
                    p.spawn((
                        BackToTitleScreenButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }

    if let Some(screen) = &properties.credits_screen {
        commands
            .spawn((
                MainMenuScreen,
                CreditsScreen,
                ImageBundle {
                    style: bg_style.clone(),
                    image: asset_server.load(&screen.bg_img_path).into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: btn_col_style.clone(),
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|p| {
                    let button = &screen.back_button;
                    p.spawn((
                        BackToTitleScreenButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(button.img_size.x),
                                height: Val::Px(button.img_size.y),
                                ..btn_style.clone()
                            },
                            image: asset_server.load(&button.img_path).into(),
                            ..default()
                        },
                    ));
                });
            });
    }
}

pub(super) fn add_server_entry(
    server_list: Query<Entity, With<ServerListPane>>,
    mut add_server_evs: EventReader<AddServerEntry>,
    mut commands: Commands,
) {
    let Ok(server_list) = server_list.get_single() else {
        return;
    };

    for _ in add_server_evs.read() {
        commands
            .spawn((
                ServerListEntry,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    border_color: Color::YELLOW.into(),
                    ..default()
                },
            ))
            .set_parent(server_list);
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuScreen>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(super) fn button_hover(
    mut ui: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in ui.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = Color::rgba(0.75, 0.75, 0.75, 1.0).into();
            }
            Interaction::None => {
                *color = Color::rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            _ => {}
        }
    }
}

pub(super) fn show_title_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<TitleScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<TitleScreen>)>,
    mut open_screen_evs: EventReader<OpenTitleScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_single_player_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<SinglePlayerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<SinglePlayerScreen>)>,
    mut open_screen_evs: EventReader<OpenSinglePlayerScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_multiplayer_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<MultiplayerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<MultiplayerScreen>)>,
    mut open_screen_evs: EventReader<OpenMultiplayerScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_settings_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<SettingsScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<SettingsScreen>)>,
    mut open_screen_evs: EventReader<OpenSettingsScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_credits_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<CreditsScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<CreditsScreen>)>,
    mut open_screen_evs: EventReader<OpenCreditsScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}

pub(super) fn show_edit_server_screen(
    mut ui_to_close: Query<&mut Style, (With<MainMenuScreen>, Without<EditServerScreen>)>,
    mut ui_to_open: Query<&mut Style, (With<MainMenuScreen>, With<EditServerScreen>)>,
    mut open_screen_evs: EventReader<OpenEditServerScreenEvent>,
) {
    for _ in open_screen_evs.read() {
        for mut style in ui_to_close.iter_mut() {
            style.display = Display::None;
        }

        for mut style in ui_to_open.iter_mut() {
            style.display = Display::Flex;
        }
    }
}
pub(super) fn text_focus_handler(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInput)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut text_input) in &mut text_input_query {
                text_input.inactive = entity != interaction_entity;
            }
        }
    }
}
