use bevy::dev_tools::states::*;
use bevy::log::*;
use bevy::{app::ScheduleRunnerPlugin, prelude::*, state::app::StatesPlugin};
use bevy_ratatui::{RatatuiPlugins, event::KeyEvent, terminal::RatatuiContext};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
// use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod action;
mod cli;
mod component;
mod config;
mod gameevent;
mod map;
mod rng;
mod system;
mod ui;
mod ui_component;
mod ui_mode;

pub use action::*;
use cli::CliArgs;
pub use config::*;
pub use gameevent::GameEvent;
pub use rng::*;
pub use ui::*;
pub use ui_component::*;

#[macro_use]
extern crate lazy_static;

pub const VERSION_STRING: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

// Later maybe we want a Menu state with various SubStates for the different menus?
#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    ApplicationStart,
    Menu,
    InGame,
}

#[derive(SubStates, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[source(GameState = GameState::Menu)]
// #[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    SomeOtherMenu,
}

fn main() {
    setup_logging();

    ///// Load config
    debug!("Loading config files");
    config::load_config(None, None);

    ///// Parse CLI args
    let args = CliArgs::parse();

    let seed;
    if args.seed == 0 {
        info!("No RNG seed specified - using current unix epoch time.");
        seed = Seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    } else {
        seed = Seed(args.seed);
        info!("RNG seed specified on command line: {}", seed.0);
    }
    info!("Setting RNG seed to {}", seed.0);
    rng::reseed(seed.0);

    ///// Build Bevy App and run
    let frame_time = Duration::from_secs_f32(1.0 / 60.0);

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            // bevy::log::LogPlugin {
            //     // Uncomment this to override the default log settings:
            //     // level: bevy::log::Level::TRACE,
            //     // filter: "wgpu=warn,bevy_ecs=info".to_string(),
            //     ..default()
            // },
            RatatuiPlugins::default(),
            StatesPlugin,
        ))
        // States
        .init_state::<GameState>()
        .add_sub_state::<MenuState>()
        // Resources
        .init_resource::<UIConfig>()
        .init_resource::<UIComponents>()
        .insert_resource(seed)
        // Events
        .add_event::<GameEvent>()
        // Startup schedule
        .add_systems(PreStartup, setup_ui_components)
        .add_systems(Startup, enter_main_menu)
        // Update schedule
        .add_systems(PreUpdate, keyboard_input_system)
        .add_systems(Update, draw_ui_system)
        .add_systems(Update, game_event_handler)
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(Update, log_transitions::<MenuState>)
        // State transition schedules
        .add_systems(OnEnter(MenuState::MainMenu), show_main_menu)
        .run();
}

fn setup_logging() {
    ////// Start logger
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let log_file = format!("{}_{}.log", env!("CARGO_PKG_NAME"), timestamp);
    let log_file_path = std::path::Path::new("logs").join(log_file.clone());
    let file_appender =
        tracing_appender::rolling::RollingFileAppender::new(tracing_appender::rolling::Rotation::NEVER, "logs", log_file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Create symlink to current log file
    let symlink_path = std::path::Path::new("current-log");
    if symlink_path.exists() {
        std::fs::remove_file(symlink_path).unwrap_or_else(|e| warn!("Failed to remove old symlink: {}", e));
    }
    std::os::unix::fs::symlink(log_file_path, symlink_path).unwrap_or_else(|e| warn!("Failed to create symlink: {}", e));

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    info!("{} {} starting", env!("CARGO_PKG_NAME"), VERSION_STRING);
}

fn game_event_handler(mut game_events: EventReader<GameEvent>) {
    for event in game_events.read() {
        info!("Received: {:?}", event);
    }
}

fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    // mut app_exit: EventWriter<AppExit>,
    mut game_events: EventWriter<GameEvent>,
    uiconfig: Res<UIConfig>,
    state: Res<State<GameState>>,
) {
    for event in events.read() {
        debug!("key event received: {event:?}");
        if let Some(keymap) = uiconfig.keybindings.get(&state) {
            // debug!("keymap found: {keymap:?}");
            if let Some(ge) = keymap.get(&vec![crossterm::event::KeyEvent::new(event.code, event.modifiers)]) {
                debug!("Key pressed for action/gameevent: {ge:?}");
                game_events.write(ge.clone());
                //     // action_tx.send(action.clone())?;
                // } else {
                //     // If the key was not handled as a single key action, then consider it for multi-key combinations
                //     // self.last_tick_key_events.push(key);
                //     // if let Some(action) = keymap.get(&self.last_tick_key_events) {
                //     //     log::info!("Got action: {action:?}");
                //     //     action_tx.send(action.clone())?;
                //     // }
            }
        }
    }
}

fn enter_main_menu(mut next_game_state: ResMut<NextState<GameState>>, mut next_menu_state: ResMut<NextState<MenuState>>) {
    bevy::log::info!("Entering main menu");
    next_game_state.set(GameState::Menu);
    next_menu_state.set(MenuState::MainMenu);
}

fn show_main_menu(mut uicomps: ResMut<UIComponents>) {
    let c = uicomps
        .comps
        .get_mut("main_menu")
        .unwrap_or_else(|| panic!("Couldn't find main_menu UI component."));
    c.visible = true;
}

fn setup_ui_components(mut uiconfig: ResMut<UIConfig>, mut uicomps: ResMut<UIComponents>) {
    info!("Setting up UI components...");

    info!("Loading UI config...");
    let load_uiconfig = UIConfig::new();
    match load_uiconfig {
        Ok(load_uiconfig) => {
            uiconfig.config = load_uiconfig.config;
            uiconfig.keybindings = load_uiconfig.keybindings;
            uiconfig.styles = load_uiconfig.styles;
        }
        Err(e) => panic!("Error while loading ui config: {e}"),
    }

    info!("Creating initial UI components...");

    // Main Menu UI component
    let mut main_menu = ui::components::Menu::new();
    main_menu
        .set_title("Main Menu")
        .add_item(("Generate World", Some(Action::GenerateWorld)))
        .add_item(("New Game", Some(Action::StartNewGame)))
        .add_item(("Load Game", None))
        .add_item(("Quit", Some(Action::Quit)));

    // UIMap UI component
    let hud = ui::components::Hud::new();

    uicomps.comps.insert(
        "main_menu".to_string(),
        UIComponentData {
            component: Box::new(main_menu) as Box<dyn UIComponent>,
            visible: false,
        },
    );
    uicomps.comps.insert(
        "hud".to_string(),
        UIComponentData {
            component: Box::new(hud) as Box<dyn UIComponent>,
            visible: false,
        },
    );
}

fn draw_ui_system(mut context: ResMut<RatatuiContext>, mut ui_components: ResMut<UIComponents>) -> Result {
    context.draw(|f| {
        for (_component_name, uicomponent) in ui_components.comps.iter_mut().filter(|x| x.1.visible) {
            // log::debug!("Drawing component: {}", component_name);
            let r = uicomponent.component.draw(f, f.area());
            if let Err(e) = r {
                error!("Failed to draw: {:?}", e);
                // action_tx.send(Action::Error(format!("Failed to draw: {:?}", e))).unwrap();
            }
        }
    })?;

    Ok(())
}
