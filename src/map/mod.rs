use bevy::prelude::*;
use rand::random;

use crate::map::chunks::*;
use crate::map::weather::*;
pub mod assets;
pub mod chunks;
pub mod generate;
pub mod math;
pub mod noise;
pub mod weather;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .insert_resource(MapSeed(random()))
            .insert_resource(MapTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Update, update_chunks)
            .add_systems(Update, tick_map_timer)
            .add_systems(
                Update,
                (rotate_seed, update_chunks).run_if(map_timer_finished),
            );
    }
}

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Weather>()
            .add_systems(Startup, change_weather);
    }
}
