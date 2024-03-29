use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use self::assets::WhAssetPlugin;
use crate::client::camera::CameraPlugin;
use crate::client::gamestates::ClientGameStatePlugin;
use crate::client::loading_screen::LoadingScreenPlugin;
use crate::client::main_menu::MainMenuPlugin;
use crate::client::splash::SplashPlugin;
use crate::client::ui::WhUiPlugin;

pub mod assets;
pub mod camera;
pub mod gamestates;
pub mod loading_screen;
pub mod main_menu;
pub mod splash;
pub mod ui;

#[derive(Debug, Default)]
pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(WhAssetPlugin)
            .add(CameraPlugin)
            .add(ClientGameStatePlugin)
            .add(LoadingScreenPlugin::default())
            .add(MainMenuPlugin)
            .add(SplashPlugin::default())
            .add(WhUiPlugin);

        #[cfg(feature = "networking")]
        {
            use bevy_wh_net::client::ClientNetworkingPlugin;
            group = group.add(ClientNetworkingPlugin);
        }

        group
    }
}
