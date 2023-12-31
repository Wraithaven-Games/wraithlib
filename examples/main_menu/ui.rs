use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_wh_elements::prelude::*;
use wraithlib::client::main_menu::*;
use wraithlib::common::uuid::Uuid;

const TITLE_SCREEN: ScreenID = ScreenID(1);
const SINGLE_PLAYER_SCREEN: ScreenID = ScreenID(2);
const MULTIPLAYER_SCREEN: ScreenID = ScreenID(3);
const SETTINGS_SCREEN: ScreenID = ScreenID(4);
const CREDITS_SCREEN: ScreenID = ScreenID(5);
const EDIT_SERVER_SCREEN: ScreenID = ScreenID(6);
const MAIN_MENU_GROUP: ScreenGroup = ScreenGroup(1);
const SERVER_ENTRY_RADIO: RadioButtonGroup = RadioButtonGroup(1);

pub fn build_canvas() -> BoxedElement {
    WhCanvas::with_flags(MainMenuCanvas) //
        .add_children(vec![
            WhScreen::new(TITLE_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .shown()
                .bg_img("images/menu/titlescreen.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .size(Val::Px(250.0), Val::Auto)
                        .add_children(vec![
                            button("Single Player") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    SINGLE_PLAYER_SCREEN,
                                )),
                            button("Multiplayer") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    MULTIPLAYER_SCREEN,
                                )),
                            button("Settings") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    SETTINGS_SCREEN,
                                )),
                            button("Credits") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    CREDITS_SCREEN,
                                )),
                            button("Quit") //
                                .on_click(AppExit),
                        ]),
                ]),
            WhScreen::new(SINGLE_PLAYER_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .bg_img("images/menu/singleplayer.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .size(Val::Px(250.0), Val::Auto)
                        .add_children(vec![
                            button("New Game"),
                            button("Load Game"),
                            button("Back") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    TITLE_SCREEN,
                                )),
                        ]),
                ]),
            WhScreen::new(MULTIPLAYER_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .bg_img("images/menu/serverlist.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                        .border(Color::WHITE, Val::Px(2.0))
                        .size(Val::Percent(80.0), Val::Percent(80.0))
                        .padding(UiRect::all(Val::Px(10.0)))
                        .add_children(vec![
                            WhScrollPane::with_flags((), ServerListPane)
                                .direction(ElementDirection::Column, Val::Px(0.0))
                                .border(Color::WHITE, Val::Px(1.0))
                                .scroll_direction(ScrollDirection::Vertical)
                                .size(Val::Percent(100.0), Val::Px(1.0))
                                .growing(),
                            WhDiv::new()
                                .direction(ElementDirection::Row, Val::Px(10.0))
                                .justify(ElementAlignment::Right)
                                .size(Val::Percent(100.0), Val::Px(80.0))
                                .add_children(vec![
                                    button("New") //
                                        .on_click(SetScreenInGroup::new(
                                            MAIN_MENU_GROUP,
                                            EDIT_SERVER_SCREEN,
                                        )),
                                    button("Delete"),
                                    button_flags(JoinServerButton, "Join"),
                                    button("Back") //
                                        .on_click(SetScreenInGroup::new(
                                            MAIN_MENU_GROUP,
                                            TITLE_SCREEN,
                                        )),
                                ]),
                        ]),
                ]),
            WhScreen::new(SETTINGS_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .bg_img("images/menu/settings.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .size(Val::Px(250.0), Val::Auto)
                        .add_children(vec![
                            button("Back") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    TITLE_SCREEN,
                                )),
                        ]),
                ]),
            WhScreen::new(CREDITS_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .bg_img("images/menu/settings.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .size(Val::Px(250.0), Val::Auto)
                        .add_children(vec![
                            button("Back") //
                                .on_click(SetScreenInGroup::new(
                                    MAIN_MENU_GROUP,
                                    TITLE_SCREEN,
                                )),
                        ]),
                ]),
            WhScreen::new(EDIT_SERVER_SCREEN)
                .in_group(MAIN_MENU_GROUP)
                .bg_img("images/menu/serverlist.png")
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .add_children(vec![
                            WhTextInput::with_flags((), ServerNameTextInput)
                                .bg_color(Color::WHITE)
                                .size(Val::Percent(100.0), Val::Px(40.0))
                                .font_size(26.0),
                            WhTextInput::with_flags((), ServerAddressTextInput)
                                .bg_color(Color::WHITE)
                                .size(Val::Percent(100.0), Val::Px(40.0))
                                .font_size(26.0),
                            WhDiv::new()
                                .direction(ElementDirection::Row, Val::Px(5.0))
                                .padding(UiRect::all(Val::Px(5.0)))
                                .add_children(vec![
                                    button_flags(ConfirmEditServerButton, "Confirm")
                                        .bg_img("images/menu/button.png")
                                        .size(Val::Px(200.0), Val::Px(50.0))
                                        .on_click(SetScreenInGroup::new(
                                            MAIN_MENU_GROUP,
                                            MULTIPLAYER_SCREEN,
                                        )),
                                    button("Back").on_click(SetScreenInGroup::new(
                                        MAIN_MENU_GROUP,
                                        MULTIPLAYER_SCREEN,
                                    )),
                                ]),
                        ]),
                ]),
        ])
}

fn button_flags<B: Bundle>(flags: B, text: &str) -> Box<WhButton<B>> {
    WhButton::with_flags(flags)
        .bg_img("images/menu/button.png")
        .size(Val::Px(200.0), Val::Px(50.0))
        .add_child(
            WhText::new()
                .text(text)
                .font_size(26.0)
                .text_color(Color::BLACK),
        )
}

fn button(text: &str) -> Box<WhButton<()>> {
    button_flags((), text)
}

pub fn server_entry_builder(uuid: Uuid, entry: ServerListEntry) -> BoxedElement {
    WhDiv::with_flags((uuid, entry.clone()))
        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
        .size(Val::Percent(100.0), Val::Px(102.0))
        .padding(UiRect::all(Val::Px(5.0)))
        .direction(ElementDirection::Row, Val::Px(5.0))
        .interaction(NodeInteraction::Radio(SERVER_ENTRY_RADIO))
        .border(Color::DARK_GRAY, Val::Px(1.0))
        .change_border_on_active(Color::WHITE, Val::Px(1.0))
        .add_children(vec![
            WhDiv::new()
                .bg_img("images/wraithaven.png")
                .size(Val::Auto, Val::Percent(100.0))
                .aspect_ratio(1.0),
            WhDiv::new()
                .direction(ElementDirection::Column, Val::Px(0.0))
                .size(Val::Px(1.0), Val::Auto)
                .growing()
                .add_children(vec![
                    WhDiv::new()
                        .direction(ElementDirection::Row, Val::Px(0.0))
                        .justify(ElementAlignment::Left)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .add_children(vec![
                            WhText::new()
                                .text(&entry.name)
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .justify(ElementAlignment::Left)
                                .growing(),
                            WhText::new()
                                .text("0ms")
                                .font_size(26.0)
                                .text_color(Color::WHITE)
                                .justify(ElementAlignment::Right),
                        ]),
                    WhText::new()
                        .text(&entry.ip)
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .justify(ElementAlignment::Left),
                    WhText::new()
                        .text("Server description.")
                        .font_size(26.0)
                        .text_color(Color::WHITE)
                        .size(Val::Percent(100.0), Val::Px(30.0))
                        .justify(ElementAlignment::Left),
                ]),
        ])
}
