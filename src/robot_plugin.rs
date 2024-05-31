use bevy::asset::AssetContainer;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use rstykrab_cache::Action;

use crate::visualizer;
use crate::visualizer::{CacheForRobot, TileSize};

pub(crate) struct RobotPlugin;

#[derive(Resource, Component, Default)]
struct TimerCache {
    timer: Timer,
}
#[derive(Component)]
struct Robot;

#[derive(Component)]
struct ID{
    id: i32
}

const TIMER_TIME: f32 = 1.0;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, robot)
            .insert_resource(TimerCache { timer: Timer::from_seconds(TIMER_TIME, TimerMode::Repeating) });
    }
}

fn spawn_robot(
    mut commands: &mut Commands,
    sprite: &Res<visualizer::SpriteSheetRust>,
    tile_size: &Res<TileSize>,
    pos: &(usize, usize),
    id: i32
) {
    let mut robot = TextureAtlasSprite::new(11);
    let tile_size = tile_size.as_ref().tile_size;
    let x = pos.0 as f32 * tile_size;
    let y = pos.1 as f32 * tile_size;

    let robot_position = (x, y);

    robot.custom_size = Some(Vec2::splat(tile_size));

    let robot_name = "Robot_".to_string() + &*id.to_string();

    let robot = commands.spawn((SpriteSheetBundle {
        texture_atlas: sprite.0.clone(),
        sprite: robot,
        transform: Transform::from_xyz(robot_position.0, robot_position.1, 1.0),
        ..Default::default()
    }, Name::new(robot_name), Robot, ID{id}));

}

fn move_robot_with_id(
    mut robot_query: &mut Query<(&Robot, &mut Transform, Option<&ID>)>,
    new_position: &(u32, u32),
    id: &i32,
    tile_size: &Res<TileSize>
){
    for (_, mut position_iter, id_iter) in robot_query{
        if(*id == id_iter.unwrap().id){
            let tile_size = tile_size.as_ref().tile_size;
            let x = new_position.0 as f32 * tile_size;
            let y = new_position.1 as f32 * tile_size;

            *position_iter = Transform::from_xyz(x, y, 1.0);
        }
    }
}

fn robot(
    mut timer: ResMut<TimerCache>,
    time: Res<Time>,
    cache: Res<CacheForRobot>,
    mut commands: Commands,
    sprite: Res<visualizer::SpriteSheetRust>,
    tile_size: Res<TileSize>,
    mut robot_query: Query<(&Robot, &mut Transform, Option<&ID>)>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        let mut history = cache.as_ref().cache.lock().unwrap();
        if let Ok(mut recent_actions) = history.get_recent_actions(5) {
            recent_actions.reverse();
            for record in recent_actions {
                println!("Action is:  {:?}", record);
                match &record.action {
                    Action::Other(record_string) => {
                        let record_string: Vec<&str> = record_string.split(' ').collect();
                        match record_string[0] {
                            "spawn_robot" => { spawn_robot(&mut commands, &sprite, &tile_size, &record.position, 0) }
                            "spawn_robot_with_id" => {
                                let id: i32 = record_string[1].parse().unwrap();
                                spawn_robot(&mut commands, &sprite, &tile_size, &record.position, id)
                            }
                            "move_robot" => {
                                let x: u32 = record_string[1].parse().unwrap();
                                let y: u32 = record_string[2].parse().unwrap();
                                move_robot_with_id( &mut robot_query, &(x,y), &0, &tile_size)
                            }
                            "move_robot_multiple" => {
                                let id: i32 = record_string[1].parse().unwrap();
                                let x: u32 = record_string[2].parse().unwrap();
                                let y: u32 = record_string[3].parse().unwrap();
                                move_robot_with_id(&mut robot_query, &(x, y), &id, &tile_size)
                            }
                            "destroy_content" => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            history.set_size(0);
            history.set_size(25);
        } else {
            println!("Error: Invalid count specified");
        }
    }
}

