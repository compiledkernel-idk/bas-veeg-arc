pub mod player_manager;
pub mod shared_systems;
pub mod input_handler;
pub mod ui_coop;

pub use player_manager::{CoopPlayerManager, CoopPlayer, PlayerSlot};
pub use shared_systems::{SharedComboSystem, ReviveSystem};
pub use input_handler::{CoopInputHandler, InputDevice};
pub use ui_coop::CoopUI;
