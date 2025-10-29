use crate::data::characters::CharacterId;
use crate::ecs::entity::EntityId;
use macroquad::prelude::*;

/// Maximum number of players in co-op
pub const MAX_PLAYERS: usize = 4;

/// Player colors for visual distinction
pub const PLAYER_COLORS: [Color; 4] = [
    Color::new(0.2, 0.5, 1.0, 1.0), // Blue (P1)
    Color::new(1.0, 0.2, 0.2, 1.0), // Red (P2)
    Color::new(0.2, 1.0, 0.2, 1.0), // Green (P3)
    Color::new(1.0, 1.0, 0.2, 1.0), // Yellow (P4)
];

/// Manages all players in a co-op session
pub struct CoopPlayerManager {
    players: [Option<CoopPlayer>; MAX_PLAYERS],
    active_player_count: usize,
    shared_currency: f32,
    shared_combo_meter: f32,
    team_multiplier: f32,
    friendly_fire_enabled: bool,
}

/// Represents a single player in co-op
#[derive(Clone)]
pub struct CoopPlayer {
    pub slot: PlayerSlot,
    pub entity_id: Option<EntityId>,
    pub character_type: CharacterId,
    pub color: Color,
    pub input_device: InputDeviceType,
    pub is_active: bool,
    pub is_downed: bool,
    pub revive_timer: f32,
    pub revive_progress: f32,
    pub kills: u32,
    pub deaths: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
    pub combo_contribution: u32,
    pub spawn_position: Vec2,
}

/// Player slot identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerSlot {
    Player1,
    Player2,
    Player3,
    Player4,
}

/// Input device type for each player
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputDeviceType {
    KeyboardMouse,
    Gamepad(u8), // Gamepad ID
}

impl CoopPlayerManager {
    pub fn new() -> Self {
        Self {
            players: [None, None, None, None],
            active_player_count: 0,
            shared_currency: 0.0,
            shared_combo_meter: 0.0,
            team_multiplier: 1.0,
            friendly_fire_enabled: false,
        }
    }

    /// Initialize for single-player mode
    pub fn init_single_player(&mut self, character: CharacterId) {
        self.players[0] = Some(CoopPlayer::new(
            PlayerSlot::Player1,
            character,
            InputDeviceType::KeyboardMouse,
        ));
        self.active_player_count = 1;
    }

    /// Initialize for co-op mode with multiple players
    pub fn init_coop(&mut self, player_configs: Vec<(CharacterId, InputDeviceType)>) {
        self.players = [None, None, None, None];
        self.active_player_count = player_configs.len().min(MAX_PLAYERS);

        for (i, (character, input_device)) in player_configs.iter().enumerate().take(MAX_PLAYERS) {
            let slot = match i {
                0 => PlayerSlot::Player1,
                1 => PlayerSlot::Player2,
                2 => PlayerSlot::Player3,
                3 => PlayerSlot::Player4,
                _ => break,
            };

            self.players[i] = Some(CoopPlayer::new(slot, *character, *input_device));
        }

        // Calculate team multiplier based on player count
        self.team_multiplier = 1.0 + (self.active_player_count as f32 - 1.0) * 0.15;
    }

    /// Add a player during gameplay (drop-in)
    pub fn add_player(
        &mut self,
        character: CharacterId,
        input_device: InputDeviceType,
    ) -> Option<PlayerSlot> {
        if self.active_player_count >= MAX_PLAYERS {
            return None;
        }

        // Find first empty slot
        for i in 0..MAX_PLAYERS {
            if self.players[i].is_none() {
                let slot = match i {
                    0 => PlayerSlot::Player1,
                    1 => PlayerSlot::Player2,
                    2 => PlayerSlot::Player3,
                    3 => PlayerSlot::Player4,
                    _ => unreachable!(),
                };

                self.players[i] = Some(CoopPlayer::new(slot, character, input_device));
                self.active_player_count += 1;
                self.recalculate_team_multiplier();

                return Some(slot);
            }
        }

        None
    }

    /// Remove a player during gameplay (drop-out)
    pub fn remove_player(&mut self, slot: PlayerSlot) {
        let index = slot.to_index();
        if self.players[index].is_some() {
            self.players[index] = None;
            self.active_player_count = self.active_player_count.saturating_sub(1);
            self.recalculate_team_multiplier();
        }
    }

    /// Get a player by slot
    pub fn get_player(&self, slot: PlayerSlot) -> Option<&CoopPlayer> {
        self.players[slot.to_index()].as_ref()
    }

    /// Get a mutable player by slot
    pub fn get_player_mut(&mut self, slot: PlayerSlot) -> Option<&mut CoopPlayer> {
        self.players[slot.to_index()].as_mut()
    }

    /// Get all active players
    pub fn get_active_players(&self) -> Vec<&CoopPlayer> {
        self.players
            .iter()
            .filter_map(|p| p.as_ref())
            .filter(|p| p.is_active)
            .collect()
    }

    /// Get all active players (mutable)
    pub fn get_active_players_mut(&mut self) -> Vec<&mut CoopPlayer> {
        self.players
            .iter_mut()
            .filter_map(|p| p.as_mut())
            .filter(|p| p.is_active)
            .collect()
    }

    /// Update all players
    pub fn update(&mut self, dt: f32) {
        // Update downed players' revive timers
        for player in self.players.iter_mut().filter_map(|p| p.as_mut()) {
            if player.is_downed {
                player.revive_timer += dt;

                // Auto-respawn after 30 seconds if not revived
                if player.revive_timer > 30.0 {
                    player.is_downed = false;
                    player.revive_timer = 0.0;
                    player.deaths += 1;
                }
            }
        }

        // Decay shared combo meter
        if self.shared_combo_meter > 0.0 {
            self.shared_combo_meter -= dt * 0.5;
            self.shared_combo_meter = self.shared_combo_meter.max(0.0);
        }
    }

    /// Add to shared currency
    pub fn add_currency(&mut self, amount: f32) {
        self.shared_currency += amount;
    }

    /// Spend shared currency
    pub fn spend_currency(&mut self, amount: f32) -> bool {
        if self.shared_currency >= amount {
            self.shared_currency -= amount;
            true
        } else {
            false
        }
    }

    /// Get shared currency
    pub fn get_currency(&self) -> f32 {
        self.shared_currency
    }

    /// Add to shared combo meter
    pub fn add_combo(&mut self, amount: f32, contributing_player: PlayerSlot) {
        self.shared_combo_meter = (self.shared_combo_meter + amount * self.team_multiplier).min(100.0);

        // Track contribution
        if let Some(player) = self.get_player_mut(contributing_player) {
            player.combo_contribution += 1;
        }
    }

    /// Get shared combo meter
    pub fn get_combo_meter(&self) -> f32 {
        self.shared_combo_meter
    }

    /// Spend combo meter for team super
    pub fn spend_combo_meter(&mut self, amount: f32) -> bool {
        if self.shared_combo_meter >= amount {
            self.shared_combo_meter -= amount;
            true
        } else {
            false
        }
    }

    /// Mark a player as downed
    pub fn down_player(&mut self, slot: PlayerSlot) {
        if let Some(player) = self.get_player_mut(slot) {
            player.is_downed = true;
            player.revive_timer = 0.0;
            player.revive_progress = 0.0;
        }
    }

    /// Revive a downed player
    pub fn revive_player(&mut self, slot: PlayerSlot) {
        if let Some(player) = self.get_player_mut(slot) {
            player.is_downed = false;
            player.revive_timer = 0.0;
            player.revive_progress = 0.0;
        }
    }

    /// Update revive progress
    pub fn update_revive_progress(&mut self, slot: PlayerSlot, progress: f32) {
        if let Some(player) = self.get_player_mut(slot) {
            player.revive_progress = progress.min(1.0);

            // Automatically revive when progress reaches 100%
            if player.revive_progress >= 1.0 {
                self.revive_player(slot);
            }
        }
    }

    /// Check if friendly fire is enabled
    pub fn is_friendly_fire_enabled(&self) -> bool {
        self.friendly_fire_enabled
    }

    /// Toggle friendly fire
    pub fn toggle_friendly_fire(&mut self) {
        self.friendly_fire_enabled = !self.friendly_fire_enabled;
    }

    /// Get active player count
    pub fn get_active_count(&self) -> usize {
        self.active_player_count
    }

    /// Get team multiplier for difficulty scaling
    pub fn get_team_multiplier(&self) -> f32 {
        self.team_multiplier
    }

    /// Recalculate team multiplier based on active players
    fn recalculate_team_multiplier(&mut self) {
        self.team_multiplier = 1.0 + (self.active_player_count as f32 - 1.0) * 0.15;
    }

    /// Check if all players are downed (game over condition)
    pub fn are_all_players_downed(&self) -> bool {
        let active_players = self.get_active_players();
        !active_players.is_empty() && active_players.iter().all(|p| p.is_downed)
    }

    /// Get spawn position for a player slot
    pub fn get_spawn_position(&self, slot: PlayerSlot) -> Vec2 {
        // Spread players horizontally based on slot
        let base_x = 960.0; // Center of screen
        let spacing = 150.0;
        let offset = match slot {
            PlayerSlot::Player1 => -1.5 * spacing,
            PlayerSlot::Player2 => -0.5 * spacing,
            PlayerSlot::Player3 => 0.5 * spacing,
            PlayerSlot::Player4 => 1.5 * spacing,
        };

        Vec2::new(base_x + offset, 540.0)
    }

    /// Reset all player stats (for new game/wave)
    pub fn reset_stats(&mut self) {
        for player in self.players.iter_mut().filter_map(|p| p.as_mut()) {
            player.kills = 0;
            player.deaths = 0;
            player.damage_dealt = 0.0;
            player.damage_taken = 0.0;
            player.combo_contribution = 0;
            player.is_downed = false;
            player.revive_timer = 0.0;
            player.revive_progress = 0.0;
        }

        self.shared_combo_meter = 0.0;
    }
}

impl CoopPlayer {
    pub fn new(slot: PlayerSlot, character: CharacterId, input_device: InputDeviceType) -> Self {
        Self {
            slot,
            entity_id: None,
            character_type: character,
            color: PLAYER_COLORS[slot.to_index()],
            input_device,
            is_active: true,
            is_downed: false,
            revive_timer: 0.0,
            revive_progress: 0.0,
            kills: 0,
            deaths: 0,
            damage_dealt: 0.0,
            damage_taken: 0.0,
            combo_contribution: 0,
            spawn_position: Vec2::ZERO,
        }
    }

    /// Set the entity ID for this player
    pub fn set_entity(&mut self, entity_id: EntityId) {
        self.entity_id = Some(entity_id);
    }

    /// Get the entity ID for this player
    pub fn get_entity(&self) -> Option<EntityId> {
        self.entity_id
    }

    /// Record a kill
    pub fn add_kill(&mut self) {
        self.kills += 1;
    }

    /// Record damage dealt
    pub fn add_damage_dealt(&mut self, damage: f32) {
        self.damage_dealt += damage;
    }

    /// Record damage taken
    pub fn add_damage_taken(&mut self, damage: f32) {
        self.damage_taken += damage;
    }

    /// Get kill/death ratio
    pub fn get_kd_ratio(&self) -> f32 {
        if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        }
    }
}

impl PlayerSlot {
    pub fn to_index(&self) -> usize {
        match self {
            PlayerSlot::Player1 => 0,
            PlayerSlot::Player2 => 1,
            PlayerSlot::Player3 => 2,
            PlayerSlot::Player4 => 3,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(PlayerSlot::Player1),
            1 => Some(PlayerSlot::Player2),
            2 => Some(PlayerSlot::Player3),
            3 => Some(PlayerSlot::Player4),
            _ => None,
        }
    }

    pub fn next(&self) -> Option<Self> {
        Self::from_index(self.to_index() + 1)
    }

    pub fn prev(&self) -> Option<Self> {
        if self.to_index() > 0 {
            Self::from_index(self.to_index() - 1)
        } else {
            None
        }
    }
}
