use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(spawn_camera)
        .add_system(camera_z_scroll);
    }
}

fn spawn_camera(mut commands:Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn_bundle(camera);
}

fn camera_z_scroll(mut query: Query<&mut Transform, &Camera2d>
    , mut scroll_evr: EventReader<MouseWheel>
){
    let mut transform = query.single_mut();

    for ev in scroll_evr.iter() {
        if ev.unit == MouseScrollUnit::Line {
            transform.scale -= ev.y * 0.05;
        }
    }
}