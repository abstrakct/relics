use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
}
