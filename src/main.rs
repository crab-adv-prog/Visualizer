use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use robotics_lib::world::world_generator::Generator;
use rstykrab_cache::{Action, Cache};
use worldgen_unwrap::public::WorldgeneratorUnwrap;

mod visualizer;
mod robot_plugin;
mod tilemap_plugin;
mod camera_plugin;

fn main() {
    let mut map_creation = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("./assets/maps/map4"))).gen().0;
    let map = Arc::new(Mutex::new(map_creation));

    let cache_creation = Cache::new(25);
    let cache = Arc::new(Mutex::new(cache_creation));

    let visulizer_map = Arc::clone(&map);
    let fun_map = Arc::clone(&map);

    let visualizer_cache = Arc::clone(&cache);
    let fun_cache = Arc::clone(&cache);


    let cache_handle = std::thread::spawn(move || {
        println!("Starting to use cache");
        cache_usage(fun_cache);
    });

    println!("Starting visualizer");
    visualizer::start(visulizer_map.clone(), visualizer_cache);
}


fn cache_usage(cache: Arc<Mutex<Cache>>){

    println!("I am using the cache, yippie!");

    sleep(std::time::Duration::from_secs(2));
    {
        let mut cache = cache.lock().unwrap();
        println!("spawn_robot 0,0");
        cache.add_record(Action::Other("spawn_robot".to_string()), (0, 0));
    }

    sleep(std::time::Duration::from_secs(2));
    {
        let mut cache = cache.lock().unwrap();
        println!("move_robot 1,0");
        cache.add_record(Action::Other("move_robot 1 0".to_string()), (0,0));
    }

    sleep(std::time::Duration::from_secs(2));
    {
        let mut cache = cache.lock().unwrap();
        println!("move_robot 2,0");
        cache.add_record(Action::Other("move_robot 2 0".to_string()), (0,0));
    }

    sleep(std::time::Duration::from_secs(2));
    {
        let mut cache = cache.lock().unwrap();
        println!("move_robot 2,1");
        cache.add_record(Action::Other("move_robot 2 1".to_string()), (0,0));
    }

    sleep(std::time::Duration::from_secs(2));
    {
        let mut cache = cache.lock().unwrap();
        println!("move_robot 2,2");
        cache.add_record(Action::Other("move_robot 2 2".to_string()), (0,0));
    }

}