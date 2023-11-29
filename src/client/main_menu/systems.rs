use bevy::prelude::*;
use bevy_wh_elements::assets::AssetReference;
use bevy_wh_elements::components::{FocusableElement, RadioButtonElement, TextInput};

use super::*;
use crate::client::assets::AssetsWaitForLoad;
use crate::client::networking::TryConnectToServerEvent;
use crate::common::uuid::Uuid;

pub(super) fn init_main_menu(mut next_state: ResMut<NextState<MainMenuState>>) {
    next_state.set(MainMenuState::TitleScreen);
}

pub(super) fn build_ui(
    asset_server: Res<AssetServer>,
    mut asset_queue: ResMut<AssetsWaitForLoad>,
    mut properties: ResMut<MainMenuProperties>,
    mut commands: Commands,
) {
    let mut canvas = None;
    std::mem::swap(&mut canvas, &mut properties.canvas);

    if let Some(canvas) = canvas {
        let mut loader = AssetReference::new(&asset_server);
        canvas.build(&mut commands, &mut loader);
        asset_queue.add_many_to_queue(loader.get_handles());
    }
}

pub(super) fn add_server_entry(
    properties: Res<MainMenuProperties>,
    server_list: Query<Entity, With<ServerListPane>>,
    asset_server: Res<AssetServer>,
    mut asset_queue: ResMut<AssetsWaitForLoad>,
    mut add_server_evs: EventReader<AddServerEntry>,
    mut commands: Commands,
) {
    let Ok(server_list) = server_list.get_single() else {
        return;
    };

    for ev in add_server_evs.read() {
        let Some(builder) = &properties.server_entry else {
            continue;
        };

        let entry = ServerListEntry {
            ip: ev.address.clone(),
            name: ev.name.clone(),
            ..default()
        };

        let mut loader = AssetReference::new(&asset_server);
        let elem = builder(ev.uuid.clone(), entry);
        elem.build_child(&mut commands, &mut loader, Some(server_list));
        asset_queue.add_many_to_queue(loader.get_handles());
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuCanvas>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(super) fn confirm_edit_server(
    button: Query<&Interaction, (Changed<Interaction>, With<ConfirmEditServerButton>)>,
    server_name_text: Query<&TextInput, With<ServerNameTextInput>>,
    server_address_text: Query<&TextInput, With<ServerAddressTextInput>>,
    mut add_server_evs: EventWriter<AddServerEntry>,
) {
    for interaction in button.iter() {
        if let Interaction::Pressed = *interaction {
            let name = server_name_text.single().current_text();
            let address = server_address_text.single().current_text();
            add_server_evs.send(AddServerEntry {
                uuid: Uuid::from_random(),
                name: name.into(),
                address: address.into(),
            });
        }
    }
}

pub(super) fn reset_edit_server_text_inputs(
    button: Query<&Interaction, (Changed<Interaction>, With<ConfirmEditServerButton>)>,
    mut server_name_text: Query<
        &mut TextInput,
        (With<ServerNameTextInput>, Without<ServerAddressTextInput>),
    >,
    mut server_address_text: Query<
        &mut TextInput,
        (With<ServerAddressTextInput>, Without<ServerNameTextInput>),
    >,
    mut focusable: Query<&mut FocusableElement>,
) {
    for interaction in button.iter() {
        if let Interaction::Pressed = *interaction {
            for mut text_input in server_name_text.iter_mut() {
                text_input.clear();
            }
            for mut text_input in server_address_text.iter_mut() {
                text_input.clear();
            }
            for mut focus in focusable.iter_mut() {
                focus.focused = false;
            }
        }
    }
}

pub(super) fn join_server_button(
    query_button: Query<&Interaction, (Changed<Interaction>, With<JoinServerButton>)>,
    query_servers: Query<(&ServerListEntry, &RadioButtonElement)>,
    mut try_join_server_evs: EventWriter<TryConnectToServerEvent>,
) {
    for interaction in query_button.iter() {
        if let Interaction::Pressed = *interaction {
            for (server, radio) in query_servers.iter() {
                if radio.selected {
                    try_join_server_evs.send(TryConnectToServerEvent {
                        ip: server.ip.clone(),
                    });
                }
            }
        }
    }
}
