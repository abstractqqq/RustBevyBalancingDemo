use bevy::prelude::*;
use crate::map::Tile;

pub struct AsciiPlugin;
pub struct AsciiSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app:&mut App){
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    location: (usize, usize)
) -> Entity {

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(32.0));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation : translation,
            ..Default::default()
        }
        , ..Default::default()

    }).insert(Tile { loc: location})
    .id()


}

fn load_ascii(mut commands: Commands
    , assets: Res<AssetServer>
    , mut texture_atlas:ResMut<Assets<TextureAtlas>>){

        let image = assets.load("ascii.png");
        let atlas = TextureAtlas::from_grid_with_padding(
            image, 
            Vec2::splat(9.0),
            16, 16,
            Vec2::splat(2.0),
            Vec2::ZERO
        );

        let atlas_handle = texture_atlas.add(atlas);

        commands.insert_resource(AsciiSheet(atlas_handle));
}