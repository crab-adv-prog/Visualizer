use bevy::prelude::*;
use bevy::render::texture::{ImageFilterMode, ImageSamplerDescriptor};
use robotics_lib::world::tile::Tile;
use crate::camera_plugin::CameraPluginCustom;

use crate::robot_plugin::RobotPlugin;
use crate::tilemap_plugin::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.4, 0.4, 0.4);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 50.0;

#[derive(Component)]
pub(crate) struct MyCameraMarker;

#[derive(Resource, Component)]
pub(crate) struct SpriteSheetRust(pub(crate) Handle<TextureAtlas>);

#[derive(Resource, Default)]
pub(crate) struct Map { pub(crate) map: Vec<Vec<Tile>> }

#[derive(Resource, Default)]
pub(crate) struct TileSize { pub(crate) tile_size: f32 }

pub(crate) fn start(map: Vec<Vec<Tile>>) {

    let map_resource = Map { map };
    let tile_size_resource = TileSize { tile_size: TILE_SIZE };

    App::new()
        .insert_resource(tile_size_resource)
        .insert_resource(map_resource)
        .insert_resource(ClearColor(CLEAR))
        .add_systems(PreStartup, assets)
        .add_plugins(CameraPluginCustom)
        .add_plugins(TileMapPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (900.0 * RESOLUTION, 900.0).into(),
                    title: "Robot_Word".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    min_filter: ImageFilterMode::Nearest,
                    mag_filter: ImageFilterMode::Nearest,
                    mipmap_filter: ImageFilterMode::Nearest,
                    ..Default::default()
                }
            }))
        .run();
}

fn assets(mut commands: Commands, assets: Res<AssetServer>, mut atlas: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("SpriteSheetRust.png");
    let texture_atlas = TextureAtlas::from_grid(
        image,
        Vec2::new(32.0, 32.0),
        12,
        6,
        Some(Vec2::splat(3.0)),
        Some(Vec2::splat(0.0)),
    );

    let texture_atlas_handle = atlas.add(texture_atlas);
    commands.insert_resource(SpriteSheetRust(texture_atlas_handle));
}


