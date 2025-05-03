// use crossterm::event::{KeyCode, KeyEvent};
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, time::Duration};
use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::UIComponent;
use crate::game_event::GameEvent;
use crate::ui::centered_rect;

#[derive(Default)]
pub struct Menu {
    command_tx: Option<UnboundedSender<GameEvent>>,
    // config: UIConfig, // this isn't used anywhere...
    items: Vec<(String, Option<GameEvent>)>,
    state: ListState,
    index: usize,
    title: String,
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item<T: ToString>(&mut self, item: (T, Option<GameEvent>)) -> &mut Self {
        let o: (String, Option<GameEvent>) = (item.0.to_string(), item.1);
        self.items.push(o);
        self.index = 0;
        self.state.select(Some(self.index));
        self
    }

    pub fn set_title<T: ToString>(&mut self, title: T) -> &mut Self {
        self.title = title.to_string();
        self
    }

    fn next_item(&mut self) -> Result<()> {
        self.index = (self.index + 1) % self.items.len();
        self.state.select(Some(self.index));
        Ok(())
    }

    fn prev_item(&mut self) -> Result<()> {
        self.index = (self.index + self.items.len() - 1) % self.items.len();
        self.state.select(Some(self.index));
        Ok(())
    }
}

impl UIComponent for Menu {
    fn register_action_handler(&mut self, tx: UnboundedSender<GameEvent>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    // fn register_config_handler(&mut self, config: UIConfig) -> Result<()> {
    //     self.config = config;
    //     Ok(())
    // }

    fn update(&mut self, action: GameEvent) -> Result<Option<GameEvent>> {
        // match action {
        //     Action::Tick => {}
        //     Action::NextMenuItem => {
        //         self.index = (self.index + 1) % self.options.len();
        //         self.state.select(Some(self.index));
        //     }
        //     _ => {}
        // }
        if let GameEvent::NextMenuItem = action {
            self.next_item()?
        }
        if let GameEvent::PrevMenuItem = action {
            self.prev_item()?
        }
        if let GameEvent::SelectMenuItem = action {
            let index = self.state.selected().unwrap();
            let action = self.items[index].1.clone();
            return Ok(action);
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, _area: Rect) -> Result<()> {
        // get all the strings in items vector and store as a vector of strings
        let menu_items: Vec<String> = self.items.iter().map(|(s, _)| s.clone()).collect();
        let menu = List::new(menu_items)
            .block(
                Block::default()
                    .title(self.title.clone())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always)
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        f.render_stateful_widget(menu, centered_rect(f.area(), 20, 20), &mut self.state);

        Ok(())
    }
}
