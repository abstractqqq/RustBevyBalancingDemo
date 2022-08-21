use bevy::{
    prelude::*
};
use bevy_easings::EasingsPlugin;

mod main_camera;
use main_camera::CameraPlugin;
mod player;
use player::PlayerPlugin;
mod overlay_text;
use overlay_text::FPSPlugin;
mod debug;
use debug::DebugPlugin;
mod ascii;
use ascii::AsciiPlugin;
mod map;
use map::MapPlugin;


/// This example illustrates how to create UI text and update it in a system. It displays the
/// current FPS in the top left corner, as well as text that changes colour in the bottom right.
/// For text within a scene, please see the text2d example.
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "My Grid".to_string(),
            resizable: false, 
            ..Default::default()
        })
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(FPSPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(MapPlugin)
        .run();
}



