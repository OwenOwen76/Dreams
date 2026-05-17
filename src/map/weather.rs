use bevy::prelude::*;
use rand::RngExt;

#[derive(Default, States, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
pub enum Weather {
    #[default]
    Sunny, //50%
    Rainy,  //30%
    Stormy, //15%
    Cloudy, //5%
}

impl Weather {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let roll = rng.random_range(0..100);

        match roll {
            0..=49 => Weather::Sunny,
            50..=79 => Weather::Cloudy,
            80..=94 => Weather::Rainy,
            _ => Weather::Stormy,
        }
    }
}

pub fn change_weather(mut next_state: ResMut<NextState<Weather>>) {
    next_state.set(Weather::new());
}
