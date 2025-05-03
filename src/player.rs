use crate::{component::*, rng};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    detailed_name: DetailedName,
    stats: Stats,
    render: Render,
    attributes: Attributes,
    position: Position, // Add more components here as needed
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerBundle {
    pub fn new() -> Self {
        let cfg = &super::CFG.lock().unwrap();
        let str_roll = rng::roll_str(cfg.config.player.str.clone());
        info!("Roll for str: {str_roll}");
        let dex_roll = rng::roll_str(cfg.config.player.dex.clone());
        info!("Roll for dex: {dex_roll}");
        let con_roll = rng::roll_str(cfg.config.player.con.clone());
        info!("Roll for con: {con_roll}");
        let int_roll = rng::roll_str(cfg.config.player.int.clone());
        info!("Roll for int: {int_roll}");

        Self {
            player: Player {},
            detailed_name: DetailedName {
                base: "you".to_string(),
                full: cfg.config.player.name.clone(),
            },
            stats: Stats {
                hp: Pool::new(cfg.config.player.hp),
                mp: Pool::new(cfg.config.player.mp),
                xp: 0,
                pv: 0,
                dv: 0,
                lv: 1,
            },
            render: Render {
                glyph: '@',
                fg: ratatui::style::Color::Yellow,
                bg: ratatui::style::Color::Black,
                order: 1,
                always: false,
            },
            attributes: Attributes {
                str: Attribute {
                    base: str_roll,
                    modifiers: 0,
                    bonus: 0,
                },
                dex: Attribute {
                    base: dex_roll,
                    modifiers: 0,
                    bonus: 0,
                },
                con: Attribute {
                    base: con_roll,
                    modifiers: 0,
                    bonus: 0,
                },
                int: Attribute {
                    base: int_roll,
                    modifiers: 0,
                    bonus: 0,
                },
            },
            position: Position { x: 1, y: 1, map: 1 },
        }
    }
}

pub fn spawn(world: &mut World) -> Entity {
    world.spawn(PlayerBundle::new()).id()
}
