use bevy::prelude::*;
use bevy::render::texture::{ImageFilterMode, ImageSamplerDescriptor};

use robot::RobotPlugin;

mod robot;
mod ui;

pub const CLEAR: Color = Color::rgb(0.1, 0.9, 0.5);
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
struct MyCameraMarker;

#[derive(Resource, Component)]
struct SpriteSheetRust (Handle<TextureAtlas>);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_systems(Startup, camera)
        .add_systems(PreStartup, assets)
        .add_plugins( (RobotPlugin, DefaultPlugins
            .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (900.0 * RESOLUTION, 900.0).into(),
                title: "Robot_Word".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
            })
            .set(ImagePlugin{
                default_sampler: ImageSamplerDescriptor {
                    min_filter: ImageFilterMode::Nearest,
                    mag_filter: ImageFilterMode::Nearest,
                    ..Default::default()
            }})))
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0), ..Default::default()
        }, MyCameraMarker
    ));
}

fn assets(mut commands: Commands, assets: Res<AssetServer>, mut atlas:  ResMut<Assets<TextureAtlas>>) {
        let image = assets.load("SpriteSheetRust.png");
        let texture_atlas = TextureAtlas::from_grid(
            image,
            Vec2::new(8.0, 8.0),
            5,
            3,
            Some(Vec2::splat(2.0)),
            Some(Vec2::splat(0.0))
            );

        let texture_atlas_handle = atlas.add(texture_atlas);
        commands.insert_resource(SpriteSheetRust(texture_atlas_handle));

    }


