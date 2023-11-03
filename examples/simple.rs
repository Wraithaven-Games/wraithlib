use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use wraithlib::client::loading_screen::{LoadingScreenPlugin, LoadingScreenProperties};
use wraithlib::client::main_menu::{
    ButtonProperties,
    MainMenuProperties,
    MenuScreenProperties,
    ServerListScreen,
    SettingsScreen,
    SinglePlayerScreen,
    TitleScreen,
};
use wraithlib::client::splash::{SplashImageProperties, SplashPlugin};
use wraithlib::client::ClientPlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MainMenuProperties {
            root_screen: MenuScreenProperties {
                bg_img_path: Some("images/menu/titlescreen.png".into()),
                buttons: vec![
                    ButtonProperties {
                        img_path: Some("images/menu/buttons/singleplayer.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    },
                    ButtonProperties {
                        img_path: Some("images/menu/buttons/multiplayer.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    },
                    ButtonProperties {
                        img_path: Some("images/menu/buttons/settings.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    },
                    ButtonProperties {
                        img_path: Some("images/menu/buttons/quit.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    },
                ],
                screen_comp: Some(|cmd| {
                    cmd.insert(TitleScreen);
                }),
            },
            child_screens: vec![
                MenuScreenProperties {
                    bg_img_path: Some("images/menu/singleplayer.png".into()),
                    buttons: vec![ButtonProperties {
                        img_path: Some("images/menu/buttons/back.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    }],
                    screen_comp: Some(|cmd| {
                        cmd.insert(SinglePlayerScreen);
                    }),
                },
                MenuScreenProperties {
                    bg_img_path: Some("images/menu/serverlist.png".into()),
                    buttons: vec![ButtonProperties {
                        img_path: Some("images/menu/buttons/back.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    }],
                    screen_comp: Some(|cmd| {
                        cmd.insert(ServerListScreen);
                    }),
                },
                MenuScreenProperties {
                    bg_img_path: Some("images/menu/settings.png".into()),
                    buttons: vec![ButtonProperties {
                        img_path: Some("images/menu/buttons/back.png".into()),
                        img_size: Vec2::new(200.0, 50.0),
                        ..default()
                    }],
                    screen_comp: Some(|cmd| {
                        cmd.insert(SettingsScreen);
                    }),
                },
            ],
        })
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WraithLibrary [ Simple Preview ]".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(
            ClientPlugins
                .set(SplashPlugin {
                    images: vec![SplashImageProperties {
                        path: "images/wraithaven.png".into(),
                        ..default()
                    }],
                    ..default()
                })
                .set(LoadingScreenPlugin {
                    properties: LoadingScreenProperties {
                        path: Some("images/loading.png".into()),
                        ..default()
                    },
                }),
        )
        .run();
}
