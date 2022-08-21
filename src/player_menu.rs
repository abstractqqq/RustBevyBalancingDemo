// use bevy::{prelude::*};
// pub struct PlayerMenuPlugin;

// impl Plugin for PlayerMenuPlugin {
//     fn build(&self, app: &mut App){
//         app.add_startup_system(spawn_menu);
//     }
// }



// fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>){

//     commands.spawn_bundle(menu_control())
//     .with_children(|parent|{
//         parent.spawn_bundle(button())
//         .with_children(|parent|{
//             parent.spawn_bundle(button_text(&asset_server, "X"));
//         });
//         parent.spawn_bundle(button())
//         .with_children(|parent|{
//             parent.spawn_bundle(button_text(&asset_server, "Y"));
//         });
//     });
// }

// fn menu_control() -> NodeBundle {
//     NodeBundle {
//         transform: Transform::from_xyz(0., 0., 0.001),
//         ..Default::default()
//     }

// }

// fn button() -> ButtonBundle {
//     ButtonBundle {
//         style: Style {
//             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             ..Default::default()
//         },
        
//         ..Default::default()
//     }
// }

// fn button_text(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
//     TextBundle {
//         style: Style {
//             margin: UiRect::all(Val::Px(10.0)),
//             ..Default::default()
//         },
//         text: Text::from_section(
//             label,
//             TextStyle {
//                 font: asset_server.load("fonts/ArchitectsDaughter-Regular.ttf"),
//                 font_size: 20.0,
//                 color: Color::WHITE
//             }
//         ),
//         ..Default::default()
//     }
// }