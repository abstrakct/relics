use bevy_ecs::prelude::Resource;
use grid::Grid;
use serde::{Deserialize, Serialize};

mod builders;
mod rect;
mod tile;
pub use builders::*;
pub use rect::*;
pub use tile::*;

#[derive(Resource)]
pub struct Maps {
    pub map: Vec<Map>,
}

impl Maps {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Map {
    pub id: i32,
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tile_type: Grid<TileType>,
    pub tile_revealed: Grid<bool>,
    pub tile_visible: Grid<bool>,
    pub tile_blocked: Grid<bool>,
    pub tile_blocks_view: Grid<bool>,
    pub tile_walkable: Grid<bool>,
    pub tile_destructable: Grid<bool>,
    pub tile_hitpoints: Grid<i32>,
    // of course a different approach is to have walls and stuff be entities.
    // and systems which update these grids according to components
    // but then, transformations of the grids would be more complex and require transforming entities as well.
}

#[allow(dead_code)]
impl Map {
    pub fn new(id: i32, name: &str, width: usize, height: usize) -> Self {
        Self {
            id,
            name: name.into(),
            width,
            height,
            tile_type: Grid::new(height, width),
            tile_revealed: Grid::init(height, width, false),
            tile_visible: Grid::init(height, width, false),
            tile_blocked: Grid::init(height, width, false),
            tile_blocks_view: Grid::init(height, width, false),
            tile_walkable: Grid::init(height, width, false),
            tile_destructable: Grid::init(height, width, false),
            tile_hitpoints: Grid::init(height, width, 0),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }

    #[inline]
    pub fn set_tiletype(&mut self, x: usize, y: usize, tile: TileType) {
        self.tile_type[(y, x)] = tile;
    }

    #[inline]
    pub fn get_tiletype(&mut self, x: usize, y: usize) -> TileType {
        self.tile_type[(y, x)]
    }

    #[inline]
    pub fn set_revealed(&mut self, x: usize, y: usize, revealed: bool) {
        self.tile_revealed[(y, x)] = revealed;
    }

    #[inline]
    pub fn is_revealed(&mut self, x: usize, y: usize) -> bool {
        self.tile_revealed[(y, x)]
    }

    #[inline]
    pub fn set_destructable(&mut self, x: usize, y: usize, destructable: bool, hitpoints: i32) {
        self.tile_destructable[(y, x)] = destructable;
        self.tile_hitpoints[(y, x)] = hitpoints;
    }

    #[inline]
    pub fn is_destructable(&mut self, x: usize, y: usize) -> bool {
        self.tile_destructable[(y, x)]
    }

    #[inline]
    pub fn get_hitpoints(&mut self, x: usize, y: usize) -> i32 {
        self.tile_hitpoints[(y, x)]
    }

    #[inline]
    pub fn set_walkable(&mut self, x: usize, y: usize, walkable: bool) {
        self.tile_walkable[(y, x)] = walkable;
    }

    #[inline]
    pub fn is_walkable(&mut self, x: usize, y: usize) -> bool {
        self.tile_walkable[(y, x)]
    }

    #[inline]
    pub fn set_blocked(&mut self, x: usize, y: usize, blocked: bool) {
        self.tile_blocked[(y, x)] = blocked;
    }

    #[inline]
    pub fn is_blocked(&mut self, x: usize, y: usize) -> bool {
        self.tile_blocked[(y, x)]
    }

    #[inline]
    pub fn set_blocks_view(&mut self, x: usize, y: usize, blocks_view: bool) {
        self.tile_blocks_view[(y, x)] = blocks_view;
    }

    #[inline]
    pub fn blocks_view(&mut self, x: usize, y: usize) -> bool {
        self.tile_blocks_view[(y, x)]
    }

    #[inline]
    pub fn set_visible(&mut self, x: usize, y: usize, visible: bool) {
        self.tile_visible[(y, x)] = visible;
    }

    #[inline]
    pub fn is_visible(&mut self, x: usize, y: usize) -> bool {
        self.tile_visible[(y, x)]
    }
}
