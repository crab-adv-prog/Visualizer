use bevy::prelude::*;
use robotics_lib::world::tile::{Content, TileType};

use crate::visualizer;
use crate::visualizer::{Map, TileSize};

pub(crate) struct TileMapPlugin;

#[derive(Component)]
pub(crate) struct ContentTile{
    pub(crate) position: (u32, u32)
}

impl Plugin for TileMapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_map)
            .add_systems(Startup, create_content);
    }
}

fn create_map(mut commands: Commands, sprite: Res<visualizer::SpriteSheetRust>, map: Res<Map>, tile_size: Res<TileSize>) {
    let world = map.as_ref().map.clone();
    let tile_size = tile_size.as_ref().tile_size;

    for i in 0.. world.len(){
        for j in 0..world[0].len(){
            let tile = world[i][j].tile_type;
            let mut imageTile = match tile {
                TileType::DeepWater => {TextureAtlasSprite::new(0)},
                TileType::ShallowWater => {TextureAtlasSprite::new(1)}
                TileType::Sand => {TextureAtlasSprite::new(2)}
                TileType::Grass => {TextureAtlasSprite::new(3)}
                TileType::Street => {TextureAtlasSprite::new(4)}
                TileType::Hill => {TextureAtlasSprite::new(5)}
                TileType::Mountain => {TextureAtlasSprite::new(6)}
                TileType::Snow => {TextureAtlasSprite::new(7)}
                TileType::Lava => {TextureAtlasSprite::new(8)}
                TileType::Teleport(_) => {TextureAtlasSprite::new(9)}
                TileType::Wall => { TextureAtlasSprite::new(10)} };
            imageTile.custom_size = Some(Vec2::splat(tile_size));
            commands.spawn(SpriteSheetBundle {
                texture_atlas: sprite.0.clone(),
                sprite: imageTile,
                transform: Transform::from_xyz((i as f32) * tile_size, (j as f32) * tile_size, -2.0),
                ..Default::default()
            });
        }
    }
}


fn create_content(mut commands: Commands, sprite: Res<visualizer::SpriteSheetRust>, map: Res<Map>, tile_size: Res<TileSize>) {
    let world = map.as_ref().map.clone();
    let tile_size = tile_size.as_ref().tile_size;

    for i in 0.. world.len(){
        for j in 0..world[0].len(){
            let tile = world[i][j].content.clone();
            let mut imageTile = match tile {
                Content::Rock(_) => {TextureAtlasSprite::new(12)}
                Content::Tree(_) => {TextureAtlasSprite::new(13)}
                Content::Garbage(_) => {TextureAtlasSprite::new(14)}
                Content::Fire => {TextureAtlasSprite::new(15)}
                Content::Coin(_) => {TextureAtlasSprite::new(16)}
                Content::Bin(_) => {TextureAtlasSprite::new(17)}
                Content::Crate(_) => {TextureAtlasSprite::new(18)}
                Content::Bank(_) => {TextureAtlasSprite::new(19)}
                Content::Water(_) => {TextureAtlasSprite::new(27)}
                Content::Market(_) => {TextureAtlasSprite::new(20)}
                Content::Fish(_) => {TextureAtlasSprite::new(21)}
                Content::Building => {TextureAtlasSprite::new(22)}
                Content::Bush(_) => {TextureAtlasSprite::new(23)}
                Content::JollyBlock(_) => {TextureAtlasSprite::new(24)}
                Content::Scarecrow => {TextureAtlasSprite::new(25)}
                Content::None => {TextureAtlasSprite::new(27)}
            };
            imageTile.custom_size = Some(Vec2::splat(tile_size));
            commands.spawn((SpriteSheetBundle {
                texture_atlas: sprite.0.clone(),
                sprite: imageTile,
                transform: Transform::from_xyz((i as f32) * tile_size, (j as f32) * tile_size, -1.0),
                ..Default::default()
            }, ContentTile{ position: (i as u32, j as u32)} ));
        }
    }
}
