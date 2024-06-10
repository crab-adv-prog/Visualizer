use std::ops::Deref;
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use robotics_lib::world::environmental_conditions::WeatherType;

use crate::visualizer::{Map, TileSize, WeatherForRobot};

pub(crate) struct BgPlugin;

struct RGB {
    r: f32,
    g: f32,
    b: f32
}

impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, change_weather);

    }
}

fn change_weather(meteo: Res<WeatherForRobot>, mut background: ResMut<ClearColor>) {
    let mut meteo = meteo.as_ref().weather.lock().unwrap();

    if let Ok(meteoTicks) = meteo.ticks_until_weather_change(100) {
        if let Ok(weather_prediction) = meteo.predict(meteoTicks){
            if let Ok(weather_current) = meteo.predict(0){
                let rgb  = calculate_color(weather_current, weather_prediction, meteoTicks);
                background.0 = Color::rgb(rgb.r, rgb.g, rgb.b);
            }
        }
    }

}


fn color_weather(weather: WeatherType) -> RGB{
    match weather {
        WeatherType::Sunny => RGB { r: 1.0, g: 0.843, b: 0.0 },
        WeatherType::Rainy => RGB { r: 0.439, g: 0.502, b: 0.565 },
        WeatherType::Foggy => RGB { r: 0.827, g: 0.827, b: 0.827 },
        WeatherType::TropicalMonsoon => RGB { r: 0.133, g: 0.545, b: 0.133 },
        WeatherType::TrentinoSnow => RGB { r: 0.878, g: 1.0, b: 1.0 },
    }
}

fn calculate_color(current: WeatherType, prediction: WeatherType, ticks: usize) -> RGB{
    let color_current = color_weather(current);
    let color_prediction = color_weather(prediction);

    let t: f32 = (ticks as f32)/10.0;

    RGB {
        r: color_current.r * (1.0-t) + color_prediction.r * t,
        g: color_current.g * (1.0-t) + color_prediction.g * t,
        b: color_current.b * (1.0-t) + color_prediction.b * t,
    }
}

