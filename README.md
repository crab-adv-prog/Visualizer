# Robot Visualizer
World visualizer

To import as dependency:

```toml
robot_visualizer = { version = "0.3.6", git = "https://github.com/crab-adv-prog/Visualizer.git"}
```

## How to use:

### Setting the visualizer

Create a cache and add it to the robot as a ```Arc<Mutex<Cache>>```

```toml
    let cache_creation = Cache::new(CACHE_SIZE);
    let cache = Arc::new(Mutex::new(cache_creation));
```

Start the visualizer giving him the correct variables inside

```toml
    visualizer::start(MAP, CACHE, CACHE_SIZE, RUNNER, TICK_AMOUNT);
```

Where:

```Map``` is the ```vec<vec<Tile>>``` from the worldGen

```Cache``` is the cache that we seen before (send it with ```Arc::clone(&Cache)```)

```CACHE_SIZE``` is the wanted size for the cache (put at least 100)

```RUNNER``` is the runner that has the robot inside

```TICK_AMOUNT``` is the amount of ticks that you want your runner to run (```-1``` makes the runner go infinitely)

### Sending data to the visualizer

Inside the runner do: ```cache.add_record(action, coordinates)```

#### Possible actions:

```
cache.add_record("spawn_robot", (x,y)), per spawnare un robot in (x,y)
cache.add_record("spawn_robot_with_id A", (x,y)), per spawnare un robot in (x,y) con id A
cache.add_record("move_robot x1 y1", (x,y)), per muovere il robot da (x,y) a (x1,y1)
cache.add_record("move_robot_multiple A x1 y1", (x,y)), per muovere il robot situato in (x,y) con id A a (x1,y1)
cache.add_record("destroy_content", (x,y)), per rimuovere il content in (x,y)
cache.add_record("explore_map x1 y1 x2 y2", (_,_)), per mostrare la mappa nel rettangolo con angoli (x1,y1) in basso a sinistra e (x2,y2) in alto a destra
cache.add_record("start_audio 'audio_file_name' x1", (_,_)), per far partire l'audio con nome 'audio_file_name' trovatosi in assets/music/'audio_file_name'.ogg a volume x (float positivo)
```


## NOTE

### To use the visualizer you will need two important files:

A file called ```SpriteSheetRust.png``` that will contain the assets. The file needs to be put inside a assets folder in the root of the project
Every asset needs to be ```32x32 pixels```. They'll need to be in a ```12x6 grid``` and have ```3 pixel``` distance between each-other

A file called ```background_music.ogg``` that will contain the background music of the app. It needs to be to put in a music folder inside the assets folder mentioned before.
Inside this folder you will need to insert the audio to call with the ```start_audio``` command. The volume of the background is ```2.0``` 
