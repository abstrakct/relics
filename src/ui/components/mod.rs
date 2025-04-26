// mod home;
// mod hud;
mod menu;

// pub use home::*;
// pub use hud::*;
pub use menu::*;

use crate::UIComponent;

pub struct UIComponentData {
    pub component: Box<dyn UIComponent>,
    pub visible: bool,
}
