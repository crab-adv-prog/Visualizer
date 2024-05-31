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
    ///Punto 1
    let mut map_creation = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("./assets/maps/map1"))).gen().0;
    println!("Starting visualizer, World big as {}", map_creation.len());
    //let map = Arc::new(Mutex::new(map_creation));

    ///Punto 2
    /// COMANDI DA MANDARE CON LA CACHE:
    /// cache.add_record("spawn_robot", (x,y)), per spawnare un robot in (x,y)
    /// cache.add_record("spawn_robot_with_id A", (x,y)), per spawnare un robot in (x,y) con id A
    /// cache.add_record("move_robot x1 y1", (x,y)), per muovere il robot da (x,y) a (x1,y1)
    /// cache.add_record("move_robot_multiple A x1 y1", (x,y)), per muovere il robot situato in (x,y) con id A a (x1,y1) [Serve per differenziare facilmente il caso di un robot o più]
    /// cache.add_record("destroy_content", (x,y)), per rimuovere il content in (x,y)
    let cache_creation = Cache::new(25);
    let cache = Arc::new(Mutex::new(cache_creation));

    ///Punto 3
    //let visulizer_map = Arc::clone(&map);
    let visualizer_cache = Arc::clone(&cache);

    //let fun_map = Arc::clone(&map);
    let fun_cache = Arc::clone(&cache);


    let cache_handle = std::thread::spawn(move || {
        println!("Starting to use cache");
        cache_usage(fun_cache);
    });

    ///Per far partire il visualizer, assicurarsi di avere un Arc<Mutex<>> sia della mappa che della cache(Punto 1 e 2).
    /// Successivamente clonarli(Punto 3) e passarli al visualizer
    /// La mappa clonarla (non ricordo perchè lo ho fatto e dovrebbe funzionare anche senza il clone, ma tanto non viene mai cambiato dopo l'init dal punto di vista del visualizer)
    /// E' necessario metterlo come ultima parte fatta partire, e assicurarsi che tutto il resto (come il cache_usage nell'esempio), siano dentro thread separati
    visualizer::start(map_creation.clone(), visualizer_cache);
}


fn cache_usage(cache: Arc<Mutex<Cache>>){

    println!("I am using the cache, yippie!");
    let timer_time = 1;

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "spawn_robot_with_id 1".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (23, 25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "spawn_robot_with_id 2".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (27,25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command  = "move_robot_multiple 1 23 26".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (23,25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "move_robot_multiple 2 27 26".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (27,25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "move_robot_multiple 1 23 27".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (23,26));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "move_robot_multiple 2 27 27".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (27,26));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "spawn_robot".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (25,25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "move_robot 25 24".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (25,25));
    }

    sleep(std::time::Duration::from_secs(timer_time));
    {
        let mut cache = cache.lock().unwrap();
        let command = "move_robot 25 23".to_string();
        println!("{}", command);
        cache.add_record(Action::Other(command), (25,24));
    }

}