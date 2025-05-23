use bevy::log::debug_once;
use grid::Grid;
use ratatui::layout::Position as RatatuiPosition;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Color, widgets::Widget};

use super::Map;
use crate::component::{Position, Render};

pub struct Camera {
    /// player_pos: currently not in use!
    player_pos: Position,
    map: Map,
    entities: Vec<(Position, Render)>,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(player_pos: Position, map: Map, entities: Vec<(Position, Render)>) -> Self {
        Self {
            player_pos,
            map,
            entities,
        }
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    pub fn update(&mut self, player_pos: Position, map: Map) {
        self.player_pos = player_pos;
        self.map = map;
    }
}

impl Widget for Camera {
    fn render(self, area: Rect, buf: &mut Buffer) {
        debug_once!("Rendering map on screen area: {:?}", area);
        let rendered_map = render_map(&self.player_pos, self.map, area, self.entities);
        // log::debug!("{:?}", rendered_map);
        for ((y, x), _) in rendered_map.indexed_iter() {
            buf[RatatuiPosition {
                x: x as u16,
                y: y as u16,
            }]
            .set_char(rendered_map[(y, x)].glyph)
            .set_bg(rendered_map[(y, x)].bg)
            .set_fg(rendered_map[(y, x)].fg);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RenderedTile {
    glyph: char,
    fg: Color,
    bg: Color,
}

impl Default for RenderedTile {
    fn default() -> Self {
        Self {
            glyph: ' ',
            fg: Color::Black,
            bg: Color::Black,
        }
    }
}

pub fn render_map(_player_pos: &Position, map: Map, _area: Rect, entities: Vec<(Position, Render)>) -> Grid<RenderedTile> {
    let mut rendered_map = Grid::init(map.height, map.width, RenderedTile::default());

    for ((y, x), tile) in map.tiles.indexed_iter() {
        if tile.tile_revealed {
            let fg = if tile.tile_visible { Color::White } else { Color::Gray };
            let bg = Color::Black;
            rendered_map[(y, x)] = RenderedTile {
                glyph: map.glyph(x, y),
                fg,
                bg,
            };
        }
    }

    for (pos, render) in entities {
        rendered_map[(pos.y as usize, pos.x as usize)] = RenderedTile {
            glyph: render.glyph,
            fg: render.fg,
            bg: render.bg,
        };
    }

    rendered_map
}
