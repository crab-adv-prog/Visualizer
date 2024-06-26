use std::fmt::Alignment;
use std::thread::sleep;

use bevy::{reflect::TypePath, render::render_resource::{AsBindGroup, ShaderRef}, sprite};
use bevy::a11y::AccessibilityNode;
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::asset::AssetContainer;
use bevy::ecs::bundle::DynamicBundle;
use bevy::ecs::query::QuerySingleError;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::ui::*;
use bevy::ui::MaxTrackSizingFunction::Px;
use bevy::ui::Val::Percent;
use bevy_inspector_egui::__macro_exports::bevy_reflect::{TypeRegistry, TypeRegistryArc};
use oxagaudiotool::sound_config::OxAgSoundConfig;
use rstykrab_cache::Action;

use crate::tilemap_plugin::Explorable;
use crate::visualizer;
use crate::visualizer::{AudioRes, CacheForRobot, CacheSize, TileSize};

pub(crate) struct RobotPlugin;

#[derive(Resource, Component, Default)]
struct TimerCache {
    timer: Timer,
}

#[derive(Component)]
struct Robot;

#[derive(Component, Debug, Reflect)]
pub(crate) struct ID {
    pub(crate) id: i32,
}

#[derive(Resource, DerefMut, Deref)]
struct UserActions {
    actions: Vec<String>,
}

#[derive(Component)]
struct Log;

const SLEEP_TIME_MILLIS: u64 = 0;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UserActions { actions: vec![] })
            .add_systems(Startup, add_text)
            .add_systems(PostUpdate, robot);
    }
}

fn spawn_robot(
    mut commands: &mut Commands,
    sprite: &Res<visualizer::SpriteSheetRust>,
    tile_size: &Res<TileSize>,
    pos: &(usize, usize),
    id: i32,
) {
    //println!("Creating robot {} in {:?}", id, pos);
    let mut robot = TextureAtlasSprite::new(11);
    let tile_size = tile_size.as_ref().tile_size;
    let x = pos.0 as f32 * tile_size;
    let y = pos.1 as f32 * tile_size;

    let robot_position = (x, y);

    robot.custom_size = Some(Vec2::splat(tile_size));

    let robot_name = "Robot_".to_string() + &*id.to_string();

    commands.spawn((SpriteSheetBundle {
        texture_atlas: sprite.0.clone(),
        sprite: robot,
        transform: Transform::from_xyz(robot_position.0, robot_position.1, 1.0),
        ..Default::default()
    }, Name::new(robot_name), Robot, ID { id }));
}

fn move_robot_with_id(
    mut robot_query: &mut Query<(&Robot, &mut Transform, Option<&ID>)>,
    new_position: &(u32, u32),
    id: &i32,
    tile_size: &Res<TileSize>,
    audio: &mut ResMut<AudioRes>,
) {

    //println!("Moving robot {} to {:?}", id, new_position);
    for (_, mut position_iter, id_iter) in robot_query {
        //println!("Robot in pos {:?} and id {:?}", position_iter, id_iter);
        if (*id == id_iter.unwrap().id) {
            //println!("Found the robot with id {}. Moving it to {:?}", id, new_position);
            let tile_size = tile_size.as_ref().tile_size;
            let x = new_position.0 as f32 * tile_size;
            let y = new_position.1 as f32 * tile_size;

            *position_iter = Transform::from_xyz(x, y, 1.0);
        }
    }
}

fn remove_content(
    mut robot_query: &mut Query<(&Explorable, &mut Visibility, &mut TextureAtlasSprite)>,
    position_to_remove: &(usize, usize),
) {
    //println!("Removing content at {:?}", position_to_remove);
    for (content_tile, _, mut sprite) in robot_query {
        let x_check = (content_tile.position.0 as usize == position_to_remove.0);
        let y_check = (content_tile.position.1 as usize == position_to_remove.1);
        if (x_check && y_check && content_tile.isContent) {
            *sprite = TextureAtlasSprite::new(27)
        }
    }
}

fn explore_tile(
    mut robot_query: &mut Query<(&Explorable, &mut Visibility, &mut TextureAtlasSprite)>,
    left_bottom_angle: &(isize, isize),
    right_top_angle: &(isize, isize),
) {
    //println!("Exploring map from {:?} to {:?}", left_bottom_angle, right_top_angle);
    for (content_tile, mut visibility, _) in robot_query {
        let high_x_check = (content_tile.position.0 as isize >= left_bottom_angle.0);
        let high_y_check = (content_tile.position.1 as isize >= left_bottom_angle.1);
        let right_top_check = high_x_check && high_y_check;

        let low_x_check = (content_tile.position.0 as isize <= right_top_angle.0);
        let low_y_check = (content_tile.position.1 as isize <= right_top_angle.1);
        let left_bottom_check = low_x_check && low_y_check;

        if (right_top_check && left_bottom_check) {
            *visibility = Visibility::Visible
        }
    }
}

fn robot(
    cache: Res<CacheForRobot>,
    mut commands: Commands,
    sprite: Res<visualizer::SpriteSheetRust>,
    tile_size: Res<TileSize>,
    mut robot_query: Query<(&Robot, &mut Transform, Option<&ID>)>,
    mut content_query: Query<(&Explorable, &mut Visibility, &mut TextureAtlasSprite)>,
    cache_size: Res<CacheSize>,
    mut actions: ResMut<UserActions>,
    mut text_query: Query<&mut Text, With<Log>>,
    mut audio: ResMut<AudioRes>
) {
    let mut history = cache.as_ref().cache.lock().unwrap();
    if let Ok(mut recent_actions) = history.get_recent_actions(cache_size.cache_size) {
        recent_actions.reverse();
        //println!("Cache contains {:?}", recent_actions);
        for record in recent_actions {
            //println!("Action in explore_map is:  {:?}", record);
            match &record.action {
                Action::Other(record_string) => {
                    actions.actions.insert(0, record_string.clone());
                    modify_text(&mut text_query, &actions);
                    let record_string: Vec<&str> = record_string.split(' ').collect();
                    sleep(std::time::Duration::from_millis(SLEEP_TIME_MILLIS));
                    match record_string[0] {
                        "spawn_robot" => { spawn_robot(&mut commands, &sprite, &tile_size, &record.position, 0) }
                        "spawn_robot_with_id" => {
                            let id: i32 = record_string[1].parse().unwrap();
                            spawn_robot(&mut commands, &sprite, &tile_size, &record.position, id)
                        }
                        "move_robot" => {
                            let x: u32 = record_string[1].parse().unwrap();
                            let y: u32 = record_string[2].parse().unwrap();
                            move_robot_with_id(&mut robot_query, &(x, y), &0, &tile_size, &mut audio)
                        }
                        "move_robot_multiple" => {
                            let id: i32 = record_string[1].parse().unwrap();
                            let x: u32 = record_string[2].parse().unwrap();
                            let y: u32 = record_string[3].parse().unwrap();
                            move_robot_with_id(&mut robot_query, &(x, y), &id, &tile_size, &mut audio)
                        }
                        "destroy_content" => { remove_content(&mut content_query, &record.position) }
                        "explore_map" => {
                            let right_top: (isize, isize) = (record_string[1].parse().unwrap(), record_string[2].parse().unwrap());
                            let left_top: (isize, isize) = (record_string[3].parse().unwrap(), record_string[4].parse().unwrap());
                            explore_tile(&mut content_query, &right_top, &left_top)
                        }
                        "start_audio" => {
                            //println!("assets/music/{}.ogg",record_string[1]);
                            let volume: f64 = record_string[2].parse().unwrap();
                            let background_music = OxAgSoundConfig::new_with_volume(&format!("assets/music/{}.ogg",record_string[1]), 2.0);
                            audio.audio.play_audio(&background_music).expect("Panico, panico, panico paura!");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        history.set_size(0);
        history.set_size(cache_size.cache_size);
    } else {
        //println!("Error: Invalid count specified");
    }
}

fn modify_text(
    mut text_query: &mut Query<&mut Text, With<Log>>,
    action: &ResMut<UserActions>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = action.actions.clone().join("\n");
    }
}

///CERCARE SU INTERNET COME INSERIRE RETTANGOLI COME ELEMENTI DI UI
fn add_text(mut commands: Commands) {
    let text = ((
        TextBundle::from("FPS: ").with_style(
            Style {
                border: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                width: Percent(90.0),
                height: Percent(90.0),
                position_type: PositionType::Absolute,
                ..default()
            }
        ),
        Log,
    ));

    commands.spawn(NodeBundle {
        border_color: Color::WHITE.into(),
        background_color: Color::BLACK.into(),
        style: Style {
            border: UiRect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            width: Percent(90.0),
            height: Percent(20.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Percent(5.0),
            right: Percent(5.0),
            overflow: Overflow::clip_y(),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(text);
    });
}
