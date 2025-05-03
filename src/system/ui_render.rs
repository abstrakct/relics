use bevy::{log::error, prelude::Result};
use bevy_ecs::system::ResMut;
use bevy_ratatui::terminal::RatatuiContext;

pub use crate::ui::UIComponents;

pub fn ui_render_system(mut context: ResMut<RatatuiContext>, mut ui_components: ResMut<UIComponents>) -> Result {
    context.draw(|f| {
        for (_component_name, uicomponent) in ui_components.comps.iter_mut().filter(|x| x.1.visible) {
            // log::debug!("Drawing component: {}", component_name);
            let r = uicomponent.component.draw(f, f.area());
            if let Err(e) = r {
                // action_tx.send(Action::Error(format!("Failed to draw: {:?}", e))).unwrap();
                error!("Failed to draw UI component: {:?}", e);
            }
        }
    })?;

    Ok(())
}
