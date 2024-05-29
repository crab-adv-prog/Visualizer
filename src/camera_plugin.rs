use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::utils::petgraph::data::Element;
use robotics_lib::world::tile::TileType;
use crate::visualizer;
use bevy::input::*;
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

fn create_camera(mut commands: Commands, map: Res<Map>, tile_size: Res<TileSize> ) {
    let world = map.as_ref().map.clone();
    let tile_size = tile_size.as_ref().tile_size;

    let camera_position = ((world.len() as f32 / 2.0).floor() * tile_size, (world[0].len() as f32/ 2.0).floor() * tile_size);

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(camera_position.0, camera_position.1, 0.0),
            ..Default::default()
        }, crate::visualizer::MyCameraMarker
    ))
        .insert(PanCam::default());
}

