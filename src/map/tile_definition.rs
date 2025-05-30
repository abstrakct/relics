use super::{Tile, TileType};

//////////////////////
// Tile definitions //
//////////////////////

pub const WALL_TILE: Tile = Tile {
    tile_type: TileType::Wall,
    tile_revealed: false,
    tile_visible: false,
    tile_blocked: false,
    tile_blocks_view: true,
    tile_walkable: false,
    tile_destructable: false,
    tile_hitpoints: 0,
};

pub const BORDER_TILE: Tile = Tile {
    tile_type: TileType::Wall,
    tile_revealed: false,
    tile_visible: false,
    tile_blocked: false,
    tile_blocks_view: true,
    tile_walkable: false,
    tile_destructable: false,
    tile_hitpoints: 0,
};

pub const FLOOR_TILE: Tile = Tile {
    tile_type: TileType::Floor,
    tile_revealed: false,
    tile_visible: false,
    tile_blocked: false,
    tile_blocks_view: false,
    tile_walkable: true,
    tile_destructable: false,
    tile_hitpoints: 0,
};

pub const DUNGEON_ENTRY_TILE: Tile = Tile {
    tile_type: TileType::DungeonEntry,
    tile_revealed: false,
    tile_visible: false,
    tile_blocked: false,
    tile_blocks_view: false,
    tile_walkable: true,
    tile_destructable: false,
    tile_hitpoints: 0,
};
