use crate::data::CharacterId;
use std::sync::Mutex;

// Global game state to pass data between states
static SELECTED_CHARACTER: Mutex<Option<CharacterId>> = Mutex::new(None);
static COOP_PLAYERS: Mutex<Option<Vec<CharacterId>>> = Mutex::new(None);

pub fn set_selected_character(character: CharacterId) {
    if let Ok(mut selected) = SELECTED_CHARACTER.lock() {
        *selected = Some(character);
    }
}

pub fn get_selected_character() -> CharacterId {
    if let Ok(selected) = SELECTED_CHARACTER.lock() {
        selected.unwrap_or(CharacterId::Bas)
    } else {
        CharacterId::Bas
    }
}

pub fn set_coop_players(players: Vec<CharacterId>) {
    if let Ok(mut coop) = COOP_PLAYERS.lock() {
        *coop = Some(players);
    }
}

pub fn get_coop_players() -> Option<Vec<CharacterId>> {
    if let Ok(coop) = COOP_PLAYERS.lock() {
        coop.clone()
    } else {
        None
    }
}

pub fn clear_coop_players() {
    if let Ok(mut coop) = COOP_PLAYERS.lock() {
        *coop = None;
    }
}
