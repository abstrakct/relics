// use crossterm::event::{KeyCode, KeyEvent};
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, time::Duration};
use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::GameEvent;
// use crate::tui::Frame;
use crate::{UIComponent, UIConfig};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<GameEvent>>,
    config: UIConfig,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UIComponent for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<GameEvent>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: UIConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, _action: GameEvent) -> Result<Option<GameEvent>> {
        // match action {
        //     Action::Tick => {}
        //     _ => {}
        // }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        f.render_widget(Paragraph::new("helloworld I'M HOME"), area);
        Ok(())
    }
}
