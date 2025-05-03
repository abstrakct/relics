// use crossterm::event::{KeyCode, KeyEvent};
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, time::Duration};
use anyhow::Result;
use bevy::log::debug;
use ratatui::prelude::*;
// use tokio::sync::mpsc::UnboundedSender;

use crate::UIComponent;
use crate::component::{Position, Render};
use crate::game_event::GameEvent;
use crate::map::{Map, camera::Camera};

#[derive(Default)]
pub struct GameUi {
    // command_tx: Option<UnboundedSender<GameEvent>>,
    // config: UIConfig,
    player_pos: Position,
    map: Map,
    entities: Vec<(Position, Render)>,
}

impl GameUi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_player_pos(&mut self, player_pos: Position) -> &mut Self {
        debug!("set_player_pos: {:?}", player_pos);
        self.player_pos = player_pos;
        self
    }

    pub fn set_map(&mut self, map: Map) -> &mut Self {
        debug!("set_map: name: {} / width: {} / height: {} ", map.name, map.width, map.height);
        self.map = map;
        self
    }

    pub fn set_entities(&mut self, entities: Vec<(Position, Render)>) -> &mut Self {
        self.entities = entities;
        self
    }
}

impl UIComponent for GameUi {
    // fn register_action_handler(&mut self, tx: UnboundedSender<GameEvent>) -> Result<()> {
    //     self.command_tx = Some(tx);
    //     Ok(())
    // }

    // fn register_config_handler(&mut self, config: UIConfig) -> Result<()> {
    //     self.config = config;
    //     Ok(())
    // }

    fn update(&mut self, _action: GameEvent) -> Result<Option<GameEvent>> {
        // match action {
        //     Action::Tick => {}
        //     _ => {}
        // }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
            // .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)])
            .split(area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
            .split(horizontal_layout[0]);

        let camera = Camera::new(self.player_pos, self.map.clone() /*self.entities.clone()*/);
        // log::debug!("Drawing UIMap");

        f.render_widget(camera, layout[0]);
        // f.render_widget(Paragraph::new("helloworld I'M UIMap"), layout[1]);
        Ok(())
    }
}
