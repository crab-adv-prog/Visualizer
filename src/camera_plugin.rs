use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::visualizer::{Map, TileSize};

pub(crate) struct CameraPluginCustom;

#[derive(Component)]
struct CameraDrag { start_drag: Option<Vec2> }

impl Plugin for CameraPluginCustom{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanCamPlugin::default())
            .add_systems(Startup, create_camera);

    }
}

fn create_camera(mut commands: Commands) {

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }, crate::visualizer::MyCameraMarker
    ))
        .insert(PanCam::default());
}

