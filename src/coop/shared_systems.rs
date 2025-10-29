use super::player_manager::{CoopPlayerManager, PlayerSlot};
use crate::ecs::entity::EntityId;
use macroquad::prelude::*;

/// Shared combo system for cooperative gameplay
pub struct SharedComboSystem {
    combo_count: u32,
    combo_timer: f32,
    combo_decay_time: f32,
    last_attacker: Option<PlayerSlot>,
    combo_multiplier: f32,
    combo_rank: ComboRank,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComboRank {
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
}

impl SharedComboSystem {
    pub fn new() -> Self {
        Self {
            combo_count: 0,
            combo_timer: 0.0,
            combo_decay_time: 2.0,
            last_attacker: None,
            combo_multiplier: 1.0,
            combo_rank: ComboRank::D,
        }
    }

    /// Update combo system
    pub fn update(&mut self, dt: f32) {
        if self.combo_count > 0 {
            self.combo_timer += dt;

            if self.combo_timer >= self.combo_decay_time {
                // Combo dropped
                self.reset_combo();
            }
        }
    }

    /// Register a hit in the combo
    pub fn register_hit(&mut self, attacker: PlayerSlot, is_different_from_last: bool) {
        self.combo_count += 1;
        self.combo_timer = 0.0;
        self.last_attacker = Some(attacker);

        // Bonus for switching between players
        if is_different_from_last {
            self.combo_multiplier += 0.1;
        }

        // Update combo rank
        self.update_combo_rank();
    }

    /// Get current combo count
    pub fn get_combo_count(&self) -> u32 {
        self.combo_count
    }

    /// Get combo multiplier
    pub fn get_combo_multiplier(&self) -> f32 {
        self.combo_multiplier
    }

    /// Get combo rank
    pub fn get_combo_rank(&self) -> ComboRank {
        self.combo_rank
    }

    /// Reset combo
    pub fn reset_combo(&mut self) {
        self.combo_count = 0;
        self.combo_timer = 0.0;
        self.combo_multiplier = 1.0;
        self.combo_rank = ComboRank::D;
        self.last_attacker = None;
    }

    /// Update combo rank based on count
    fn update_combo_rank(&mut self) {
        self.combo_rank = match self.combo_count {
            0..=4 => ComboRank::D,
            5..=9 => ComboRank::C,
            10..=19 => ComboRank::B,
            20..=39 => ComboRank::A,
            40..=69 => ComboRank::S,
            70..=99 => ComboRank::SS,
            _ => ComboRank::SSS,
        };

        // Update multiplier based on rank
        self.combo_multiplier = match self.combo_rank {
            ComboRank::D => 1.0,
            ComboRank::C => 1.2,
            ComboRank::B => 1.5,
            ComboRank::A => 2.0,
            ComboRank::S => 2.5,
            ComboRank::SS => 3.0,
            ComboRank::SSS => 4.0,
        };
    }

    /// Check if combo is active
    pub fn is_combo_active(&self) -> bool {
        self.combo_count > 0
    }

    /// Get time remaining before combo drops
    pub fn get_time_remaining(&self) -> f32 {
        (self.combo_decay_time - self.combo_timer).max(0.0)
    }
}

impl ComboRank {
    pub fn to_string(&self) -> &str {
        match self {
            ComboRank::D => "D",
            ComboRank::C => "C",
            ComboRank::B => "B",
            ComboRank::A => "A",
            ComboRank::S => "S",
            ComboRank::SS => "SS",
            ComboRank::SSS => "SSS",
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            ComboRank::D => GRAY,
            ComboRank::C => WHITE,
            ComboRank::B => Color::new(0.5, 0.8, 1.0, 1.0), // Light blue
            ComboRank::A => Color::new(1.0, 0.8, 0.2, 1.0), // Gold
            ComboRank::S => Color::new(1.0, 0.4, 0.0, 1.0), // Orange
            ComboRank::SS => Color::new(1.0, 0.2, 0.2, 1.0), // Red
            ComboRank::SSS => Color::new(1.0, 0.0, 1.0, 1.0), // Magenta
        }
    }
}

/// Revive system for downed players
pub struct ReviveSystem {
    active_revives: Vec<ActiveRevive>,
    revive_time: f32, // Time needed to complete revive
    revive_range: f32, // Distance within which players can revive
}

#[derive(Clone)]
struct ActiveRevive {
    downed_player: PlayerSlot,
    reviver: PlayerSlot,
    progress: f32,
    position: Vec2,
}

impl ReviveSystem {
    pub fn new() -> Self {
        Self {
            active_revives: Vec::new(),
            revive_time: 3.0, // 3 seconds to revive
            revive_range: 100.0,
        }
    }

    /// Start a revive action
    pub fn start_revive(
        &mut self,
        downed_player: PlayerSlot,
        reviver: PlayerSlot,
        position: Vec2,
    ) {
        // Check if this revive is already in progress
        if self.active_revives.iter().any(|r| r.downed_player == downed_player) {
            return;
        }

        self.active_revives.push(ActiveRevive {
            downed_player,
            reviver,
            progress: 0.0,
            position,
        });
    }

    /// Update revive progress
    pub fn update(
        &mut self,
        dt: f32,
        player_manager: &mut CoopPlayerManager,
        player_positions: &[(PlayerSlot, Vec2)],
    ) {
        let mut completed_revives = Vec::new();
        let mut cancelled_revives = Vec::new();

        for (i, revive) in self.active_revives.iter_mut().enumerate() {
            // Check if reviver is still in range
            let reviver_pos = player_positions
                .iter()
                .find(|(slot, _)| *slot == revive.reviver)
                .map(|(_, pos)| *pos);

            let downed_pos = player_positions
                .iter()
                .find(|(slot, _)| *slot == revive.downed_player)
                .map(|(_, pos)| *pos);

            if let (Some(rpos), Some(dpos)) = (reviver_pos, downed_pos) {
                let distance = rpos.distance(dpos);

                if distance <= self.revive_range {
                    // In range, continue revive
                    revive.progress += dt / self.revive_time;

                    // Update player manager with progress
                    player_manager.update_revive_progress(revive.downed_player, revive.progress);

                    if revive.progress >= 1.0 {
                        // Revive complete
                        completed_revives.push(i);
                        player_manager.revive_player(revive.downed_player);
                    }
                } else {
                    // Out of range, cancel revive
                    cancelled_revives.push(i);
                }
            } else {
                // Players not found, cancel revive
                cancelled_revives.push(i);
            }
        }

        // Remove completed and cancelled revives
        for i in completed_revives.iter().chain(&cancelled_revives).rev() {
            self.active_revives.remove(*i);
        }
    }

    /// Cancel a revive
    pub fn cancel_revive(&mut self, reviver: PlayerSlot) {
        self.active_revives.retain(|r| r.reviver != reviver);
    }

    /// Get active revive for a player
    pub fn get_active_revive(&self, downed_player: PlayerSlot) -> Option<(PlayerSlot, f32)> {
        self.active_revives
            .iter()
            .find(|r| r.downed_player == downed_player)
            .map(|r| (r.reviver, r.progress))
    }

    /// Check if player is being revived
    pub fn is_being_revived(&self, player: PlayerSlot) -> bool {
        self.active_revives.iter().any(|r| r.downed_player == player)
    }

    /// Get revive range
    pub fn get_revive_range(&self) -> f32 {
        self.revive_range
    }
}

/// Team super move system
pub struct TeamSuperSystem {
    super_active: bool,
    super_duration: f32,
    super_timer: f32,
    super_type: TeamSuperType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TeamSuperType {
    InvincibilityAura,  // All players invincible for duration
    DamageBoost,        // Massive damage multiplier
    TimeStop,           // Freeze all enemies
    HealingWave,        // Restore health to all players
    UltimateCombo,      // Massive combined attack
}

impl TeamSuperSystem {
    pub fn new() -> Self {
        Self {
            super_active: false,
            super_duration: 10.0,
            super_timer: 0.0,
            super_type: TeamSuperType::DamageBoost,
        }
    }

    /// Activate team super
    pub fn activate(&mut self, super_type: TeamSuperType) {
        self.super_active = true;
        self.super_timer = 0.0;
        self.super_type = super_type;
        self.super_duration = match super_type {
            TeamSuperType::InvincibilityAura => 8.0,
            TeamSuperType::DamageBoost => 10.0,
            TeamSuperType::TimeStop => 5.0,
            TeamSuperType::HealingWave => 0.0, // Instant
            TeamSuperType::UltimateCombo => 3.0,
        };
    }

    /// Update team super
    pub fn update(&mut self, dt: f32) {
        if self.super_active {
            self.super_timer += dt;

            if self.super_timer >= self.super_duration {
                self.super_active = false;
            }
        }
    }

    /// Check if super is active
    pub fn is_active(&self) -> bool {
        self.super_active
    }

    /// Get active super type
    pub fn get_super_type(&self) -> Option<TeamSuperType> {
        if self.super_active {
            Some(self.super_type)
        } else {
            None
        }
    }

    /// Get time remaining
    pub fn get_time_remaining(&self) -> f32 {
        if self.super_active {
            (self.super_duration - self.super_timer).max(0.0)
        } else {
            0.0
        }
    }

    /// Get damage multiplier from super
    pub fn get_damage_multiplier(&self) -> f32 {
        if self.super_active && self.super_type == TeamSuperType::DamageBoost {
            5.0
        } else if self.super_active && self.super_type == TeamSuperType::UltimateCombo {
            10.0
        } else {
            1.0
        }
    }

    /// Check if players are invincible
    pub fn is_invincible(&self) -> bool {
        self.super_active && self.super_type == TeamSuperType::InvincibilityAura
    }
}
