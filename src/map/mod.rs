use bevy_ecs::prelude::Resource;
use grid::Grid;
use serde::{Deserialize, Serialize};

mod builders;
pub mod camera;
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
    pub tiles: Grid<Tile>,
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
            tiles: Grid::init(height, width, WALL_TILE.clone()),
        }
    }

    pub fn glyph(&self, x: usize, y: usize) -> char {
        match self.get_tile_type(x, y) {
            TileType::Floor => '.',
            TileType::Wall => '#',
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }

    #[inline]
    pub fn set_tile_type(&mut self, x: i32, y: i32, tile: TileType) {
        self.tiles[(y as usize, x as usize)].tile_type = tile;
    }

    #[inline]
    pub fn get_tile_type(&self, x: usize, y: usize) -> TileType {
        self.tiles[(y, x)].tile_type
    }

    #[inline]
    pub fn set_revealed(&mut self, x: i32, y: i32, revealed: bool) {
        self.tiles[(y as usize, x as usize)].tile_revealed = revealed;
    }

    #[inline]
    pub fn is_revealed(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_revealed
    }

    #[inline]
    pub fn set_destructable(&mut self, x: i32, y: i32, destructable: bool, hitpoints: i32) {
        self.tiles[(y as usize, x as usize)].tile_destructable = destructable;
        self.tiles[(y as usize, x as usize)].tile_hitpoints = hitpoints;
    }

    #[inline]
    pub fn is_destructable(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_destructable
    }

    #[inline]
    pub fn get_hitpoints(&self, x: usize, y: usize) -> i32 {
        self.tiles[(y, x)].tile_hitpoints
    }

    #[inline]
    pub fn set_walkable(&mut self, x: i32, y: i32, walkable: bool) {
        self.tiles[(y as usize, x as usize)].tile_walkable = walkable;
    }

    #[inline]
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_walkable
    }

    #[inline]
    pub fn set_blocked(&mut self, x: i32, y: i32, blocked: bool) {
        self.tiles[(y as usize, x as usize)].tile_blocked = blocked;
    }

    #[inline]
    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_blocked
    }

    #[inline]
    pub fn set_blocks_view(&mut self, x: i32, y: i32, blocks_view: bool) {
        self.tiles[(y as usize, x as usize)].tile_blocks_view = blocks_view;
    }

    #[inline]
    pub fn blocks_view(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_blocks_view
    }

    #[inline]
    pub fn set_visible(&mut self, x: i32, y: i32, visible: bool) {
        self.tiles[(y as usize, x as usize)].tile_visible = visible;
    }

    #[inline]
    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        self.tiles[(y, x)].tile_visible
    }

    // pub fn insert_col(&mut self, i: usize, h: usize) {
    //     self.tile_type.insert_col(i, vec![TileType::default(); h]);
    //     self.tile_revealed.insert_col(i, vec![true; h]);
    //     self.tile_visible.insert_col(i, vec![false; h]);
    //     self.tile_blocked.insert_col(i, vec![false; h]);
    //     self.tile_blocks_view.insert_col(i, vec![true; h]);
    //     self.tile_walkable.insert_col(i, vec![false; h]);
    //     self.tile_destructable.insert_col(i, vec![false; h]);
    //     self.tile_hitpoints.insert_col(i, vec![0; h]);
    // }
}
