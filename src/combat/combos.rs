use crate::combat::inputs::InputAction;

#[derive(Clone, Debug)]
pub struct Combo {
    pub name: String,
    pub sequence: Vec<InputAction>,
    pub window: f64,
    pub damage_scaling: f32,
    pub meter_gain: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComboRank {
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
    Legendary,
}

impl ComboRank {
    pub fn from_hits(hits: u32) -> Self {
        match hits {
            0..=2 => ComboRank::D,
            3..=5 => ComboRank::C,
            6..=9 => ComboRank::B,
            10..=14 => ComboRank::A,
            15..=19 => ComboRank::S,
            20..=29 => ComboRank::SS,
            30..=49 => ComboRank::SSS,
            _ => ComboRank::Legendary,
        }
    }

    pub fn color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::*;
        match self {
            ComboRank::D => GRAY,
            ComboRank::C => WHITE,
            ComboRank::B => GREEN,
            ComboRank::A => BLUE,
            ComboRank::S => PURPLE,
            ComboRank::SS => ORANGE,
            ComboRank::SSS => GOLD,
            ComboRank::Legendary => Color::new(1.0, 0.0, 1.0, 1.0),
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            ComboRank::D => 1.0,
            ComboRank::C => 1.1,
            ComboRank::B => 1.25,
            ComboRank::A => 1.5,
            ComboRank::S => 2.0,
            ComboRank::SS => 2.5,
            ComboRank::SSS => 3.0,
            ComboRank::Legendary => 5.0,
        }
    }
}

pub struct ComboManager {
    pub active_combo: Option<ActiveCombo>,
    pub combo_list: Vec<Combo>,
    pub damage_scaling: f32,
    pub hitstun_decay: f32,
    pub combo_meter: f32,
    pub combo_meter_max: f32,
    pub special_cancel_available: bool,
    pub juggle_count: u32,
    pub max_juggle: u32,
}

#[derive(Clone, Debug)]
pub struct ActiveCombo {
    pub hits: u32,
    pub total_damage: f32,
    pub timer: f32,
    pub max_timer: f32,
    pub rank: ComboRank,
    pub score_multiplier: f32,
    pub highest_rank_achieved: ComboRank,
    pub special_moves_used: u32,
    pub perfect_timing_count: u32,
}

impl ComboManager {
    pub fn new() -> Self {
        Self {
            active_combo: None,
            combo_list: Self::create_combo_list(),
            damage_scaling: 1.0,
            hitstun_decay: 1.0,
            combo_meter: 0.0,
            combo_meter_max: 100.0,
            special_cancel_available: true,
            juggle_count: 0,
            max_juggle: 5,
        }
    }

    fn create_combo_list() -> Vec<Combo> {
        vec![
            // Basic combos
            Combo {
                name: "Basic Chain".to_string(),
                sequence: vec![
                    InputAction::LightAttack,
                    InputAction::LightAttack,
                    InputAction::HeavyAttack,
                ],
                window: 1.0,
                damage_scaling: 0.9,
                meter_gain: 10.0,
            },
            Combo {
                name: "Speed Blitz".to_string(),
                sequence: vec![
                    InputAction::LightAttack,
                    InputAction::LightAttack,
                    InputAction::LightAttack,
                    InputAction::LightAttack,
                ],
                window: 0.8,
                damage_scaling: 0.8,
                meter_gain: 15.0,
            },
            // Launcher combos
            Combo {
                name: "Launcher".to_string(),
                sequence: vec![InputAction::Down, InputAction::HeavyAttack],
                window: 0.5,
                damage_scaling: 1.0,
                meter_gain: 15.0,
            },
            Combo {
                name: "Air Juggle".to_string(),
                sequence: vec![InputAction::Up, InputAction::LightAttack, InputAction::HeavyAttack],
                window: 0.6,
                damage_scaling: 1.1,
                meter_gain: 20.0,
            },
            // Special combos
            Combo {
                name: "Paint Rush".to_string(),
                sequence: vec![
                    InputAction::Special,
                    InputAction::LightAttack,
                    InputAction::Special,
                ],
                window: 1.2,
                damage_scaling: 0.85,
                meter_gain: 20.0,
            },
            Combo {
                name: "Veeg Buster".to_string(),
                sequence: vec![
                    InputAction::HeavyAttack,
                    InputAction::HeavyAttack,
                    InputAction::Special,
                ],
                window: 1.0,
                damage_scaling: 1.2,
                meter_gain: 25.0,
            },
            Combo {
                name: "Dutch Destroyer".to_string(),
                sequence: vec![
                    InputAction::Left,
                    InputAction::Right,
                    InputAction::LightAttack,
                    InputAction::HeavyAttack,
                ],
                window: 0.8,
                damage_scaling: 1.3,
                meter_gain: 30.0,
            },
            // Ultimate combos
            Combo {
                name: "Ultimate Veeg".to_string(),
                sequence: vec![InputAction::Down, InputAction::Down, InputAction::Super],
                window: 1.0,
                damage_scaling: 1.5,
                meter_gain: 0.0,
            },
            Combo {
                name: "Bas Rage Mode".to_string(),
                sequence: vec![
                    InputAction::Special,
                    InputAction::Special,
                    InputAction::Super,
                ],
                window: 1.0,
                damage_scaling: 2.0,
                meter_gain: 0.0,
            },
        ]
    }

    pub fn register_hit(&mut self, damage: f32, is_special: bool, perfect_timing: bool) {
        if let Some(ref mut combo) = self.active_combo {
            combo.hits += 1;

            // Update rank based on hits
            combo.rank = ComboRank::from_hits(combo.hits);

            // Track highest rank
            if combo.rank.multiplier() > combo.highest_rank_achieved.multiplier() {
                combo.highest_rank_achieved = combo.rank.clone();
            }

            // Apply rank multiplier to damage
            let rank_multiplier = combo.rank.multiplier();
            let scaled_damage = damage * self.damage_scaling * rank_multiplier;
            combo.total_damage += scaled_damage;

            // Update score multiplier
            combo.score_multiplier = rank_multiplier * (1.0 + combo.special_moves_used as f32 * 0.1);

            // Reset timer with bonus for higher ranks
            combo.timer = combo.max_timer * (1.0 + combo.rank.multiplier() * 0.1);

            // Track special moves and perfect timing
            if is_special {
                combo.special_moves_used += 1;
                self.combo_meter = (self.combo_meter + 20.0).min(self.combo_meter_max);
            }
            if perfect_timing {
                combo.perfect_timing_count += 1;
                self.combo_meter = (self.combo_meter + 10.0).min(self.combo_meter_max);
            }

            // Update scaling with less harsh reduction for higher ranks
            let scaling_reduction = 0.95 + (combo.rank.multiplier() - 1.0) * 0.01;
            self.damage_scaling *= scaling_reduction;
            self.hitstun_decay *= 0.98;

            // Minimum scaling improves with rank
            let min_scaling = 0.3 + (combo.rank.multiplier() - 1.0) * 0.05;
            if self.damage_scaling < min_scaling {
                self.damage_scaling = min_scaling;
            }
            if self.hitstun_decay < 0.5 {
                self.hitstun_decay = 0.5;
            }

            // Build combo meter
            self.combo_meter = (self.combo_meter + 5.0 * rank_multiplier).min(self.combo_meter_max);
        } else {
            self.active_combo = Some(ActiveCombo {
                hits: 1,
                total_damage: damage,
                timer: 2.0,
                max_timer: 2.0,
                rank: ComboRank::D,
                score_multiplier: 1.0,
                highest_rank_achieved: ComboRank::D,
                special_moves_used: if is_special { 1 } else { 0 },
                perfect_timing_count: if perfect_timing { 1 } else { 0 },
            });
            self.damage_scaling = 1.0;
            self.hitstun_decay = 1.0;
            self.combo_meter = 5.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(ref mut combo) = self.active_combo {
            combo.timer -= dt;

            if combo.timer <= 0.0 {
                self.reset();
            }
        }
    }

    pub fn reset(&mut self) {
        self.active_combo = None;
        self.damage_scaling = 1.0;
        self.hitstun_decay = 1.0;
    }

    pub fn check_combo(&self, inputs: &[InputAction]) -> Option<&Combo> {
        for combo in &self.combo_list {
            if Self::matches_sequence(inputs, &combo.sequence) {
                return Some(combo);
            }
        }
        None
    }

    fn matches_sequence(inputs: &[InputAction], sequence: &[InputAction]) -> bool {
        if inputs.len() < sequence.len() {
            return false;
        }

        let start = inputs.len() - sequence.len();
        &inputs[start..] == sequence
    }

    pub fn get_current_multiplier(&self) -> f32 {
        self.damage_scaling
    }

    pub fn get_hitstun_multiplier(&self) -> f32 {
        self.hitstun_decay
    }

    pub fn get_combo_rank(&self) -> Option<&ComboRank> {
        self.active_combo.as_ref().map(|c| &c.rank)
    }

    pub fn get_combo_hits(&self) -> u32 {
        self.active_combo.as_ref().map(|c| c.hits).unwrap_or(0)
    }

    pub fn get_combo_damage(&self) -> f32 {
        self.active_combo.as_ref().map(|c| c.total_damage).unwrap_or(0.0)
    }

    pub fn get_score_multiplier(&self) -> f32 {
        self.active_combo.as_ref().map(|c| c.score_multiplier).unwrap_or(1.0)
    }

    pub fn is_combo_active(&self) -> bool {
        self.active_combo.is_some()
    }

    pub fn can_special_cancel(&self) -> bool {
        self.special_cancel_available && self.combo_meter >= 25.0
    }

    pub fn use_special_cancel(&mut self) {
        if self.can_special_cancel() {
            self.combo_meter -= 25.0;
            self.special_cancel_available = false;
        }
    }

    pub fn register_juggle(&mut self) {
        self.juggle_count += 1;
        if self.juggle_count > self.max_juggle {
            self.reset();
        }
    }

    pub fn get_combo_meter_percentage(&self) -> f32 {
        (self.combo_meter / self.combo_meter_max) * 100.0
    }

    pub fn activate_combo_burst(&mut self) -> bool {
        if self.combo_meter >= self.combo_meter_max {
            self.combo_meter = 0.0;
            self.special_cancel_available = true;
            if let Some(ref mut combo) = self.active_combo {
                combo.timer += 2.0;
            }
            true
        } else {
            false
        }
    }
}
