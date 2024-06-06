use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy::render::texture::{ImageFilterMode, ImageSamplerDescriptor};
use robotics_lib::runner::Runner;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Tile;
use rstykrab_cache::Cache;

use crate::camera_plugin::CameraPluginCustom;
use crate::devy_debug_plugin::DebugPlugin;
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

#[derive(Resource)]
pub(crate) struct CacheForRobot { pub(crate) cache: Arc<Mutex<Cache>> }

#[derive(Resource, Default)]
pub(crate) struct TileSize { pub(crate) tile_size: f32 }

#[derive(Resource)]
pub(crate) struct CacheSize { pub(crate) cache_size: usize }

#[derive(Resource)]
struct Ticks {
    tick_amount: isize,
    current_ticks: isize
}

struct RobotRunnable { runner: Result<Runner, LibError>}

pub fn start(map: Vec<Vec<Tile>>, cache: Arc<Mutex<Cache>>, cache_size: usize, runner: Result<Runner, LibError>, tick_amount: isize) {

    let map_resource = Map { map: map };

    let cache_resource = CacheForRobot { cache };

    let tile_size_resource = TileSize { tile_size: TILE_SIZE };

    let runner_resource = RobotRunnable{ runner };

    let cache_size_resource = CacheSize{cache_size};

    let tick_resource = Ticks{ tick_amount, current_ticks: 0 };

    App::new()
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
        .insert_resource(tile_size_resource)
        .insert_resource(map_resource)
        .insert_resource(cache_resource)
        .insert_resource(tick_resource)
        .insert_resource(cache_size_resource)
        .insert_resource(ClearColor(CLEAR))
        .insert_non_send_resource(runner_resource)
        .add_systems(PreStartup, assets)
        .add_systems(PreUpdate, run_game)
        .add_plugins(CameraPluginCustom)
        .add_plugins(TileMapPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(DebugPlugin)
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

fn run_game(mut runner: NonSendMut<RobotRunnable>, mut tick_controller: ResMut<Ticks>){

    if(tick_controller.tick_amount == -1 || tick_controller.tick_amount > tick_controller.current_ticks) {
        //sleep(Duration::from_secs(5));
        let mut res = Err(LibError::NoContent);
        res = runner.runner.as_mut().unwrap().game_tick();
    }
    tick_controller.current_ticks += 1;
}



