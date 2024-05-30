use std::collections::HashMap;
use robotics_lib::world::tile::{Content, Tile, TileType};

mod visualizer;
mod robot_plugin;
mod tilemap_plugin;
mod camera_plugin;

fn main() {
    //TODO La mappa viene girata e la spawna solo se rettangolare. Non so se sia un problema di spawnare le cose con la stringa o succede anche con la matrix
    let map_string = "
        DDDDDDDDDDDDDDDDDDDDD
        DWWWWWWWWWWWWWWWWWWWD
        DWSSSSSSSSSSSSSSSSSWD
        DWSGGGGGGGGGGGGGGGSWD
        DWSGTTTTTTTTTTTTTGSWD
        DWSGTHHHHHHHHHHHTGSWD
        DWSGTHMMMMMMMMMHTGSWD
        DWSGTHMNNNNNNNMHTGSWD
        DWSGTHMNLLLLLNMHTGSWD
        DWSGTHMNLEEELNMHTGSWD
        DWSGTHMNLEBELNMHTGSWD
        DWSGTHMNLEEELNMHTGSWD
        DWSGTHMNLLLLLNMHTGSWD
        DWSGTHMNNNNNNNMHTGSWD
        DWSGTHMMMMMMMMMHTGSWD
        DWSGTHHHHHHHHHHHTGSWD
        DWSGTTTTTTTTTTTTTGSWD
        DWSGGGGGGGGGGGGGGGSWD
        DWSSSSSSSSSSSSSSSSSWD
        DWWWWWWWWWWWWWWWWWWWD
        DDDDDDDDDDDDDDDDDDDDD
        ".trim();

    let legend = [
        ('D', TileType::DeepWater),
        ('W', TileType::ShallowWater),
        ('S', TileType::Sand),
        ('G', TileType::Grass),
        ('T', TileType::Street),
        ('H', TileType::Hill),
        ('M', TileType::Mountain),
        ('N', TileType::Snow),
        ('L', TileType::Lava),
        ('E', TileType::Teleport(true)),
        ('B', TileType::Wall),
    ].iter().cloned().collect::<HashMap<char, TileType>>();

    let mut map = Vec::new();

    for line in map_string.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            if let Some(&tile_type) = legend.get(&ch) {
                row.push(Tile {
                    tile_type,
                    content: Content::None,
                    elevation: 0,
                });
            }
        }
        map.push(row);
    }
    visualizer::start(map.clone());
}
