use anyhow::Result;
use bevy_ecs::{resource::Resource, world::World};
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub use crate::{
    Action, UIComponent,
    config::UIConfig,
    ui::{self, UIComponentData},
};
use crate::{tui, ui_mode::UiMode};

// todo: consider moving to a file like resources.rs
#[derive(Resource, Default)]
pub struct Seed(pub u64);

pub struct Game {
    pub world: World,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub ui_config: UIConfig,
    pub ui_components: HashMap<String, UIComponentData>,
    pub ui_mode: UiMode,
    pub should_quit: bool,
}

impl Game {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let ui_config = UIConfig::new()?;
        let mut ui_components = HashMap::new();

        let mut main_menu = ui::components::Menu::new();
        main_menu
            .set_title("Main Menu")
            .add_item(("Generate World", Some(Action::GenerateWorld)))
            .add_item(("New Game", Some(Action::StartNewGame)))
            .add_item(("Load Game", None))
            .add_item(("Quit", Some(Action::Quit)));

        ui_components.insert(
            "main_menu".to_string(),
            UIComponentData {
                component: Box::new(main_menu) as Box<dyn UIComponent>,
                visible: true,
            },
        );

        Ok(Self {
            world: World::default(),
            tick_rate,
            frame_rate,
            last_tick_key_events: Vec::new(),
            ui_config,
            ui_components,
            ui_mode: UiMode::Menu,
            should_quit: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?.tick_rate(self.tick_rate).frame_rate(self.frame_rate);
        tui.enter()?;

        for (_, uicomponent) in self.ui_components.iter_mut() {
            uicomponent.component.register_action_handler(action_tx.clone())?;
        }

        for (_, uicomponent) in self.ui_components.iter_mut() {
            uicomponent.component.register_config_handler(self.ui_config.clone())?;
        }

        for (_, uicomponent) in self.ui_components.iter_mut() {
            uicomponent.component.init(tui.size()?)?;
        }

        // The Main Loop
        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        log::debug!("key event received: {key:?}");
                        if let Some(keymap) = self.ui_config.keybindings.get(&self.ui_mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::debug!("Key pressed for action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action, then consider it for multi-key combinations
                                self.last_tick_key_events.push(key);
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::debug!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        }
                    }
                    _ => {}
                }

                for (_, uicomponent) in self.ui_components.iter_mut() {
                    if let Some(action) = uicomponent.component.handle_events(Some(e.clone()))? {
                        log::debug!("Sending action: {action:?}");
                        action_tx.send(action)?;
                    }
                }

                while let Ok(action) = action_rx.try_recv() {
                    log::debug!("Received action: {action:?}");
                    match action {
                        Action::Quit => self.should_quit = true,
                        Action::Tick => {
                            self.last_tick_key_events.drain(..);
                        }
                        Action::Render => {
                            tui.draw(|f| {
                                for (_component_name, uicomponent) in self.ui_components.iter_mut().filter(|x| x.1.visible) {
                                    // log::debug!("Drawing component: {}", component_name);
                                    let r = uicomponent.component.draw(f, f.area());
                                    if let Err(e) = r {
                                        action_tx.send(Action::Error(format!("Failed to draw: {:?}", e))).unwrap();
                                    }
                                }
                            })?;
                        }
                        _ => {}
                    }

                    for (_, uicomponent) in self.ui_components.iter_mut() {
                        if let Some(action) = uicomponent.component.update(action.clone())? {
                            action_tx.send(action)?;
                        }
                    }
                }
            }

            if self.should_quit {
                log::debug!("We should quit. Quitting.");
                tui.stop()?;
                break;
            }
        }

        Ok(())
    }
}
