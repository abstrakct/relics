use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum TileType {
    #[default]
    Wall,
    Floor,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub tile_revealed: bool,
    pub tile_visible: bool,
    pub tile_blocked: bool,
    pub tile_blocks_view: bool,
    pub tile_walkable: bool,
    pub tile_destructable: bool,
    pub tile_hitpoints: i32,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::default(),
            tile_revealed: false,
            tile_visible: false,
            tile_blocked: false,
            tile_blocks_view: true,
            tile_walkable: false,
            tile_destructable: false,
            tile_hitpoints: 0,
        }
    }
}
