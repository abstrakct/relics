use anyhow::Result;
use bevy_ecs::{resource::Resource, world::World};
use std::collections::HashMap;

use crate::ui_mode::UiMode;
pub use crate::{
    Action, UIComponent,
    config::UIConfig,
    ui::{self, UIComponentData},
};

// todo: consider moving to a file like resources.rs
#[derive(Resource, Default)]
pub struct Seed(pub u64);

pub struct Game {
    pub world: World,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub ui_config: UIConfig,
    pub ui_mode: UiMode,
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
            ui_config,
            ui_mode: UiMode::Menu,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
