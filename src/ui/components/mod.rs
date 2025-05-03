mod game_ui;
mod home;
mod menu;

use crate::UIComponent;
pub use game_ui::*;
pub use home::*;
pub use menu::*;

use bevy_ecs::resource::Resource;
use std::collections::HashMap;

pub struct UIComponentData {
    pub component: Box<dyn UIComponent>,
    pub visible: bool,
}

impl Default for UIComponentData {
    fn default() -> Self {
        Self {
            component: Box::new(Home::new()) as Box<dyn UIComponent>,
            visible: false,
        }
    }
}

#[derive(Default, Resource)]
pub struct UIComponents {
    pub comps: HashMap<String, UIComponentData>,
}
