use bevy::asset::AssetContainer;
use bevy::prelude::*;

use crate::visualizer;
use crate::visualizer::{CacheForRobot, TileSize};

pub(crate) struct RobotPlugin;

#[derive(Resource, Component, Default)]
struct TimerCache {
    timer: Timer
}

impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, do_something)
            .insert_resource(TimerCache {timer: Timer::from_seconds(2.0, TimerMode::Repeating)})
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

fn do_something(mut timer: ResMut<TimerCache>, time: Res<Time>, cache: Res<CacheForRobot>){
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            let history = cache.as_ref().cache.lock().unwrap();
            if let Ok(recent_actions) = history.get_recent_actions(5) {
                println!("Recent Action: {:?}", recent_actions);
            } else {
                println!("Error: Invalid count specified");
            }
        }

}