use bevy::dev_tools::states::*;
use bevy::log::*;
// use bevy::remote::RemotePlugin;
// use bevy::remote::http::RemoteHttpPlugin;
use bevy::{app::ScheduleRunnerPlugin, prelude::*, state::app::StatesPlugin};
use bevy_ratatui::{RatatuiPlugins, event::KeyEvent};
use clap::Parser;
// use component::Position;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod component;
mod config;
mod event;
mod game;
mod game_event;
mod gamelogic;
mod map;
mod player;
mod rng;
mod system;
mod ui;
mod ui_component;
mod utils;
mod worldgen;

use cli::CliArgs;
pub use component::*;
pub use config::*;
use event::*;
use game::CurrentGameData;
pub use game_event::*;
use gamelogic::Rollable;
pub use player::*;
pub use rng::*;
use system::ui_render::ui_render_system;
pub use ui::*;
pub use ui_component::*;
use worldgen::generate_world;

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

pub const MAIN_MENU_NAME: &str = "main_menu";
pub const GAME_UI_NAME: &str = "game_ui";

// Later maybe we want a Menu state with various SubStates for the different menus?
#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    ApplicationStart,
    Menu,
    WorldGen,
    NewGame,
    InGame,
}

#[derive(SubStates, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[source(GameState = GameState::Menu)]
// #[states(scoped_entities)]
pub enum MenuState {
    #[default]
    MainMenu,
    SomeOtherMenu,
}

#[derive(SubStates, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[source(GameState = GameState::InGame)]
// #[states(scoped_entities)]
pub enum TurnState {
    #[default]
    PlayersTurn,
    NotPlayersTurn,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Player,
    NonPlayer,
}

fn main() {
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

    if args.stats {
        do_stats();
        exit(0);
    }

    ///// Build Bevy App and run
    let frame_time = Duration::from_secs_f32(1.0 / 60.0);

    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
            // RemotePlugin::default(),
            // RemoteHttpPlugin::default(),
            StatesPlugin,
        ))
        // .add_plugins(DefaultPlugins)
        // States
        .init_state::<GameState>()
        .add_sub_state::<MenuState>()
        .add_sub_state::<TurnState>()
        // Resources
        .init_resource::<UIConfig>()
        .init_resource::<UIComponents>()
        .init_resource::<CurrentGameData>()
        .insert_resource(seed)
        // Events
        .add_event::<GameEvent>()
        .add_event::<PlayerMoveRelativeEvent>()
        .add_event::<PlayerSpentEnergy>()
        // Startup schedule
        .add_systems(PreStartup, setup_ui_components)
        .add_systems(Startup, enter_main_menu)
        // Update schedule
        .add_systems(First, update_player_pos)
        .add_systems(PreUpdate, cleanup_component_system::<Intent>.before(keyboard_input_system))
        .add_systems(PreUpdate, keyboard_input_system)
        // .add_systems(PreUpdate, log_positions)
        .add_systems(Update, ui_render_system)
        .add_systems(Update, game_event_handler)
        .add_systems(
            Update,
            intent_system
                .run_if(in_state(TurnState::PlayersTurn))
                .after(game_event_handler),
        )
        .add_systems(
            Update,
            (player_move_system, player_spent_energy_system, update_player_pos).run_if(in_state(TurnState::PlayersTurn)),
        )
        // .add_systems(
        //     Update,
        //     (update_player_pos)
        //         .run_if(in_state(TurnState::PlayersTurn))
        //         .in_set(GameplaySet::NonPlayer),
        // )
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(Update, log_transitions::<MenuState>)
        .add_systems(Update, log_transitions::<TurnState>)
        .add_systems(PostUpdate, update_map.run_if(in_state(GameState::InGame))) // TODO: only run on some Map Update event?
        // State transition schedules
        .add_systems(OnEnter(MenuState::MainMenu), show_main_menu)
        .add_systems(OnExit(MenuState::MainMenu), hide_main_menu)
        .add_systems(OnEnter(GameState::InGame), show_game_ui)
        .add_systems(OnExit(GameState::InGame), hide_game_ui)
        .add_systems(OnEnter(GameState::WorldGen), generate_world)
        .add_systems(OnEnter(GameState::NewGame), setup_new_game)
        .run();
}

fn do_stats() {
    for i in 1..=100 {
        println!("{i}");
        let a = Attribute {
            base: i,
            ..Default::default()
        };
        let result = a.check(50);
        println!("result: {:?}\n", result);
        let result = a.roll();
        println!("result: {:?}\n", result);
    }
}

// fn log_positions(query: Query<(Entity, &Position)>) {
//     for (entity, position) in &query {
//         info_once!(
//             "Entity {} is at position: x {}, y {}, map {}",
//             entity,
//             position.x,
//             position.y,
//             position.map
//         );
//     }
// }

/// Handle events of type GameEvent (the generic event in the application)
/// Passes events on to UI components if appropriate
fn game_event_handler(
    mut param_set: ParamSet<(EventReader<GameEvent>, EventWriter<GameEvent>)>,
    // mut intent_queue: EventWriter<IntentEvent>,
    mut app_exit: EventWriter<AppExit>,
    mut ui_components: ResMut<UIComponents>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    cgd: Res<CurrentGameData>,
) {
    let mut events_to_send = Vec::new();
    for event in param_set.p0().read() {
        debug!("Received GameEvent: {:?}", event);
        match event {
            GameEvent::Quit => {
                app_exit.write_default();
            }
            GameEvent::GenerateWorld => {
                next_state.set(GameState::WorldGen);
            }
            GameEvent::StartNewGame => {
                next_state.set(GameState::NewGame);
            }
            GameEvent::ShowMainMenu => {
                next_state.set(GameState::Menu);
            }
            GameEvent::PlayerMoveRelative { dx, dy } => {
                // intent_queue.write(IntentEvent::PlayerMoveRelative { dx: *x, dy: *y });
                commands
                    .entity(cgd.player.unwrap())
                    .insert(Intent::MoveRelative { dx: *dx, dy: *dy });
            }
            _ => {}
        }

        // Now check if any UI components should handle this Event
        for (name, uicomponent) in ui_components.comps.iter_mut() {
            if let Ok(Some(ev)) = uicomponent.component.update(event.clone()) {
                debug!("UI component '{}' produced new event '{:?}'", name, ev);
                events_to_send.push(ev.clone());
            }
        }
    }

    for event in events_to_send {
        param_set.p1().write(event);
    }
}

fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut game_events: EventWriter<GameEvent>,
    uiconfig: Res<UIConfig>,
    state: Res<State<GameState>>,
) {
    for event in events.read() {
        debug!("KeyEvent received: {event:?}");
        if let Some(keymap) = uiconfig.keybindings.get(&state) {
            // debug!("keymap found: {keymap:?}");
            if let Some(ge) = keymap.get(&vec![crossterm::event::KeyEvent::new(event.code, event.modifiers)]) {
                debug!("Key pressed for game event: {ge:?}");
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
        .get_mut(MAIN_MENU_NAME)
        .unwrap_or_else(|| panic!("Couldn't find main_menu UI component."));
    c.visible = true;
}

fn hide_main_menu(mut uicomps: ResMut<UIComponents>) {
    let c = uicomps
        .comps
        .get_mut(MAIN_MENU_NAME)
        .unwrap_or_else(|| panic!("Couldn't find main_menu UI component."));
    c.visible = false;
}

fn show_game_ui(mut uicomps: ResMut<UIComponents>) {
    let c = uicomps
        .comps
        .get_mut(GAME_UI_NAME)
        .unwrap_or_else(|| panic!("Couldn't find game_ui UI component."));
    c.visible = true;
}

fn hide_game_ui(mut uicomps: ResMut<UIComponents>) {
    let c = uicomps
        .comps
        .get_mut(GAME_UI_NAME)
        .unwrap_or_else(|| panic!("Couldn't find game_ui UI component."));
    c.visible = false;
}

fn setup_new_game(
    cgd: Res<CurrentGameData>,
    mut uicomps: ResMut<UIComponents>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Update GameUi with current map
    let mut game_ui = ui::components::GameUi::new();
    game_ui.set_map(cgd.maps.map[cgd.current_map].clone());
    uicomps.comps.insert(
        GAME_UI_NAME.to_string(),
        UIComponentData {
            component: Box::new(game_ui) as Box<dyn UIComponent>,
            visible: true,
        },
    );

    next_state.set(GameState::InGame);
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
        .add_item(("Generate World", Some(GameEvent::GenerateWorld)))
        .add_item(("New Game", Some(GameEvent::StartNewGame)))
        .add_item(("Load Game", None))
        .add_item(("Quit", Some(GameEvent::Quit)));

    // UIMap UI component
    let game_ui = ui::components::GameUi::new();

    uicomps.comps.insert(
        MAIN_MENU_NAME.to_string(),
        UIComponentData {
            component: Box::new(main_menu) as Box<dyn UIComponent>,
            visible: false,
        },
    );
    uicomps.comps.insert(
        GAME_UI_NAME.to_string(),
        UIComponentData {
            component: Box::new(game_ui) as Box<dyn UIComponent>,
            visible: false,
        },
    );
}

fn update_map(cgd: Res<CurrentGameData>, mut uicomps: ResMut<UIComponents>, query: Query<(&Position, &Render)>) {
    let mut result: Vec<(Position, Render)> = Vec::new();

    // Find renderable entities on current map
    for (position, render) in query {
        debug_once!(
            "Found renderable entity at {},{} in map {}",
            position.x,
            position.y,
            position.map
        );
        if position.map == cgd.current_map as i32 {
            result.push((*position, *render));
        }
    }

    // Sort by rendering order
    result.sort_by(|a, b| b.1.order.cmp(&a.1.order));

    // Update Game UI
    let mut game_ui = ui::components::GameUi::new();
    game_ui.set_map(cgd.maps.map[cgd.current_map].clone());
    game_ui.set_entities(result);
    uicomps.comps.insert(
        GAME_UI_NAME.to_string(),
        UIComponentData {
            component: Box::new(game_ui) as Box<dyn UIComponent>,
            visible: true,
        },
    );
}

#[allow(clippy::collapsible_if, clippy::single_match)]
fn intent_system(
    cgd: Res<CurrentGameData>,
    mut move_queue: EventWriter<PlayerMoveRelativeEvent>,
    mut energy_queue: EventWriter<PlayerSpentEnergy>,
    query: Query<(Entity, &Intent)>,
) {
    for (entity, intent) in query {
        let base_energy_cost = intent.energy_cost();
        debug!(
            "entity {} has intent {:?} - it has base cost of {} energy",
            entity, intent, base_energy_cost
        );

        match *intent {
            Intent::MoveRelative { dx, dy } => {
                if entity == cgd.player.unwrap() {
                    if cgd.maps.map[cgd.player_pos.map as usize].is_walkable(cgd.player_pos.x + dx, cgd.player_pos.y + dy) {
                        debug_once!("entity is player, sending PlayerMoveRelativeEvent");
                        move_queue.write(PlayerMoveRelativeEvent { dx, dy });
                        energy_queue.write(PlayerSpentEnergy(base_energy_cost));
                    }
                }
            }
            _ => {}
        }
    }
}

fn player_spent_energy_system(
    mut energy_queue: EventReader<PlayerSpentEnergy>,
    mut query: Query<&mut Energy>,
    player_query: Query<(&Player, &Speed)>,
) {
    if let Ok((_, speed)) = player_query.single() {
        for e in energy_queue.read() {
            debug!("{:?}", e);
            for mut comp in query.iter_mut() {
                debug!("Found entity with Energy component: {:?}", comp);
                comp.energy += (e.0 as f32 * speed.speed) as i32;
                debug!("Energy component after increase: {:?}", comp);
            }
        }
    }
}

fn player_move_system(mut player_move: EventReader<PlayerMoveRelativeEvent>, mut query: Query<(&Player, &mut Position)>) {
    for pm in player_move.read() {
        debug_once!("Got PlayerMoveRelativeEvent, moving player");
        if let Ok((_entity, mut pos)) = query.single_mut() {
            pos.x += pm.dx;
            pos.y += pm.dy;
        }
    }
}

/// Update player position in CurrentGameData resource
fn update_player_pos(mut cgd: ResMut<CurrentGameData>, query: Query<(&Player, &Position)>) {
    if let Ok((_player, pos)) = query.single() {
        cgd.player_pos = *pos;
    }
}

/// System which removes all components of type T from all entities that have them.
/// Used to clean up temporary components like Intent at the start of each frame.
fn cleanup_component_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).remove::<T>();
    }
}
