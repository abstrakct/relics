use bevy_ecs::world::World;

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
