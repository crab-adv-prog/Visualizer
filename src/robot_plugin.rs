use bevy::prelude::*;
use crate::visualizer;
use crate::visualizer::TileSize;

pub(crate) struct RobotPlugin;

impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_robot);
    }
}

fn spawn_robot(mut commands: Commands, sprite: Res<visualizer::SpriteSheetRust>, tile_size: Res<TileSize>, map: Res<visualizer::Map>) {
    let mut robot = TextureAtlasSprite::new(11);
    let world = map.as_ref().map.clone();
    let tile_size = tile_size.as_ref().tile_size;

    let robot_position = ((world.len() as f32 / 2.0).floor() * tile_size, (world[0].len() as f32/ 2.0).floor() * tile_size);

    robot.custom_size = Some(Vec2::splat(50.0));

    commands.spawn(SpriteSheetBundle {
        texture_atlas: sprite.0.clone(),
        sprite: robot,
        transform: Transform::from_xyz(robot_position.0, robot_position.1, 1.0),
        ..Default::default()
    }).insert(Name::new("Robot"));
}