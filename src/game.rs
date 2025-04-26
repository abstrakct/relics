use bevy_ecs::{resource::Resource, world::World};

// todo: consider moving to a file like resources.rs
#[derive(Resource, Default)]
pub struct Seed(pub u64);

pub struct Game {
    pub world: World,
    pub tick_rate: f64,
    pub frame_rate: f64,
}

impl Game {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Game {
        Game {
            world: World::default(),
            tick_rate,
            frame_rate,
        }
    }
}
