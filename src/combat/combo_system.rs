use macroquad::prelude::*;
use std::collections::VecDeque;

/// Advanced combo system with style rankings and juggling
pub struct ComboSystem {
    pub combo_count: u32,
    pub combo_timer: f32,
    pub combo_decay_time: f32,
    pub style_rank: StyleRank,
    pub style_points: f32,
    pub max_combo_this_session: u32,
    pub combo_moves: VecDeque<ComboMove>,
    pub juggle_active: bool,
    pub juggle_hits: u32,
    pub variation_bonus: f32,
    pub timing_bonus: f32,
}

/// Style rank for combo performance
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StyleRank {
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
}

/// Individual move in a combo
#[derive(Clone, Debug)]
pub struct ComboMove {
    pub move_type: MoveType,
    pub damage: f32,
    pub timestamp: f32,
    pub is_juggle: bool,
    pub is_critical: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveType {
    LightAttack,
    HeavyAttack,
    SpecialAttack,
    Launcher,
    AirAttack,
    Ability,
    Finisher,
}

impl ComboSystem {
    pub fn new() -> Self {
        Self {
            combo_count: 0,
            combo_timer: 0.0,
            combo_decay_time: 3.0,
            style_rank: StyleRank::D,
            style_points: 0.0,
            max_combo_this_session: 0,
            combo_moves: VecDeque::with_capacity(100),
            juggle_active: false,
            juggle_hits: 0,
            variation_bonus: 1.0,
            timing_bonus: 1.0,
        }
    }

    /// Update combo system
    pub fn update(&mut self, dt: f32) {
        if self.combo_count > 0 {
            self.combo_timer += dt;

            // Combo decay
            if self.combo_timer >= self.combo_decay_time {
                self.break_combo();
            }
        }

        // Update juggle state
        if self.juggle_active && self.combo_timer > 1.0 {
            self.juggle_active = false;
            self.juggle_hits = 0;
        }
    }

    /// Register a hit in the combo
    pub fn register_hit(
        &mut self,
        move_type: MoveType,
        damage: f32,
        is_critical: bool,
    ) -> ComboResult {
        self.combo_count += 1;
        self.combo_timer = 0.0;

        // Add move to combo history
        let combo_move = ComboMove {
            move_type,
            damage,
            timestamp: self.combo_timer,
            is_juggle: self.juggle_active,
            is_critical,
        };

        self.combo_moves.push_back(combo_move);

        // Limit combo history
        if self.combo_moves.len() > 100 {
            self.combo_moves.pop_front();
        }

        // Update max combo
        if self.combo_count > self.max_combo_this_session {
            self.max_combo_this_session = self.combo_count;
        }

        // Calculate style points
        self.calculate_style_points(&move_type, damage, is_critical);

        // Update style rank
        self.update_style_rank();

        // Check for juggle
        if move_type == MoveType::Launcher {
            self.juggle_active = true;
            self.juggle_hits = 0;
        } else if self.juggle_active {
            self.juggle_hits += 1;
        }

        // Calculate variation bonus
        self.variation_bonus = self.calculate_variation_bonus();

        ComboResult {
            combo_count: self.combo_count,
            style_rank: self.style_rank,
            style_points: self.style_points,
            is_juggle: self.juggle_active,
            damage_multiplier: self.get_damage_multiplier(),
        }
    }

    /// Calculate style points for a hit
    fn calculate_style_points(&mut self, move_type: &MoveType, damage: f32, is_critical: bool) {
        let mut points = damage * 0.1;

        // Move type multiplier
        let type_mult = match move_type {
            MoveType::LightAttack => 1.0,
            MoveType::HeavyAttack => 1.5,
            MoveType::SpecialAttack => 2.0,
            MoveType::Launcher => 2.5,
            MoveType::AirAttack => 3.0,
            MoveType::Ability => 3.5,
            MoveType::Finisher => 5.0,
        };

        points *= type_mult;

        // Critical bonus
        if is_critical {
            points *= 1.5;
        }

        // Juggle bonus
        if self.juggle_active {
            points *= 1.0 + (self.juggle_hits as f32 * 0.2);
        }

        // Variation bonus
        points *= self.variation_bonus;

        // Combo length bonus
        points *= 1.0 + (self.combo_count as f32 * 0.01);

        self.style_points += points;
    }

    /// Update style rank based on points
    fn update_style_rank(&mut self) {
        self.style_rank = if self.style_points >= 10000.0 {
            StyleRank::SSS
        } else if self.style_points >= 7000.0 {
            StyleRank::SS
        } else if self.style_points >= 5000.0 {
            StyleRank::S
        } else if self.style_points >= 3000.0 {
            StyleRank::A
        } else if self.style_points >= 1500.0 {
            StyleRank::B
        } else if self.style_points >= 500.0 {
            StyleRank::C
        } else {
            StyleRank::D
        };
    }

    /// Calculate variation bonus based on move diversity
    fn calculate_variation_bonus(&self) -> f32 {
        if self.combo_moves.len() < 3 {
            return 1.0;
        }

        // Count unique move types in last 10 moves
        let recent_moves: Vec<_> = self
            .combo_moves
            .iter()
            .rev()
            .take(10)
            .map(|m| m.move_type)
            .collect();

        let mut unique_types = vec![recent_moves[0]];
        for move_type in &recent_moves[1..] {
            if !unique_types.contains(move_type) {
                unique_types.push(*move_type);
            }
        }

        let variation = unique_types.len() as f32 / recent_moves.len() as f32;
        1.0 + (variation * 0.5)
    }

    /// Get damage multiplier based on combo
    pub fn get_damage_multiplier(&self) -> f32 {
        let base = 1.0;
        let combo_mult = (self.combo_count as f32 * 0.02).min(1.0);
        let rank_mult = self.style_rank.to_multiplier();
        let variation_mult = self.variation_bonus * 0.5 + 0.5;

        base + combo_mult + rank_mult + variation_mult
    }

    /// Break the combo
    pub fn break_combo(&mut self) {
        self.combo_count = 0;
        self.combo_timer = 0.0;
        self.style_points = 0.0;
        self.style_rank = StyleRank::D;
        self.combo_moves.clear();
        self.juggle_active = false;
        self.juggle_hits = 0;
        self.variation_bonus = 1.0;
        self.timing_bonus = 1.0;
    }

    /// Check if combo is active
    pub fn is_active(&self) -> bool {
        self.combo_count > 0
    }

    /// Get time remaining before combo drops
    pub fn get_time_remaining(&self) -> f32 {
        (self.combo_decay_time - self.combo_timer).max(0.0)
    }

    /// Get combo finisher availability
    pub fn can_use_finisher(&self) -> bool {
        self.combo_count >= 10 && self.style_rank >= StyleRank::B
    }

    /// Use combo finisher
    pub fn use_finisher(&mut self) -> Option<ComboFinisher> {
        if !self.can_use_finisher() {
            return None;
        }

        let finisher = match self.style_rank {
            StyleRank::B => ComboFinisher::Basic,
            StyleRank::A => ComboFinisher::Advanced,
            StyleRank::S => ComboFinisher::Super,
            StyleRank::SS => ComboFinisher::Ultra,
            StyleRank::SSS => ComboFinisher::Ultimate,
            _ => return None,
        };

        // Consume combo
        let combo_bonus = self.combo_count;
        self.break_combo();

        Some(finisher)
    }

    /// Get combo analysis
    pub fn get_combo_analysis(&self) -> ComboAnalysis {
        let total_damage: f32 = self.combo_moves.iter().map(|m| m.damage).sum();
        let critical_hits = self.combo_moves.iter().filter(|m| m.is_critical).count() as u32;
        let juggle_hits = self.combo_moves.iter().filter(|m| m.is_juggle).count() as u32;

        let move_breakdown = self.get_move_breakdown();

        ComboAnalysis {
            total_hits: self.combo_count,
            total_damage,
            critical_hits,
            juggle_hits,
            style_rank: self.style_rank,
            style_points: self.style_points,
            variation_bonus: self.variation_bonus,
            damage_multiplier: self.get_damage_multiplier(),
            move_breakdown,
        }
    }

    /// Get breakdown of move types
    fn get_move_breakdown(&self) -> Vec<(MoveType, u32)> {
        let mut breakdown = vec![
            (MoveType::LightAttack, 0),
            (MoveType::HeavyAttack, 0),
            (MoveType::SpecialAttack, 0),
            (MoveType::Launcher, 0),
            (MoveType::AirAttack, 0),
            (MoveType::Ability, 0),
            (MoveType::Finisher, 0),
        ];

        for combo_move in &self.combo_moves {
            for (move_type, count) in &mut breakdown {
                if *move_type == combo_move.move_type {
                    *count += 1;
                    break;
                }
            }
        }

        breakdown.retain(|(_, count)| *count > 0);
        breakdown
    }
}

/// Result of registering a hit
pub struct ComboResult {
    pub combo_count: u32,
    pub style_rank: StyleRank,
    pub style_points: f32,
    pub is_juggle: bool,
    pub damage_multiplier: f32,
}

/// Combo finisher types
#[derive(Clone, Copy, Debug)]
pub enum ComboFinisher {
    Basic,      // 2x damage
    Advanced,   // 3x damage + knockback
    Super,      // 5x damage + AOE
    Ultra,      // 8x damage + AOE + stun
    Ultimate,   // 12x damage + screen-clearing AOE
}

impl ComboFinisher {
    pub fn get_damage_multiplier(&self) -> f32 {
        match self {
            ComboFinisher::Basic => 2.0,
            ComboFinisher::Advanced => 3.0,
            ComboFinisher::Super => 5.0,
            ComboFinisher::Ultra => 8.0,
            ComboFinisher::Ultimate => 12.0,
        }
    }

    pub fn get_aoe_radius(&self) -> Option<f32> {
        match self {
            ComboFinisher::Basic | ComboFinisher::Advanced => None,
            ComboFinisher::Super => Some(150.0),
            ComboFinisher::Ultra => Some(250.0),
            ComboFinisher::Ultimate => Some(500.0),
        }
    }

    pub fn applies_stun(&self) -> bool {
        matches!(self, ComboFinisher::Ultra | ComboFinisher::Ultimate)
    }

    pub fn to_string(&self) -> &str {
        match self {
            ComboFinisher::Basic => "Basic Finisher",
            ComboFinisher::Advanced => "Advanced Finisher",
            ComboFinisher::Super => "Super Finisher",
            ComboFinisher::Ultra => "Ultra Finisher",
            ComboFinisher::Ultimate => "Ultimate Finisher",
        }
    }
}

/// Analysis of a combo
pub struct ComboAnalysis {
    pub total_hits: u32,
    pub total_damage: f32,
    pub critical_hits: u32,
    pub juggle_hits: u32,
    pub style_rank: StyleRank,
    pub style_points: f32,
    pub variation_bonus: f32,
    pub damage_multiplier: f32,
    pub move_breakdown: Vec<(MoveType, u32)>,
}

impl StyleRank {
    pub fn to_string(&self) -> &str {
        match self {
            StyleRank::D => "D",
            StyleRank::C => "C",
            StyleRank::B => "B",
            StyleRank::A => "A",
            StyleRank::S => "S",
            StyleRank::SS => "SS",
            StyleRank::SSS => "SSS",
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            StyleRank::D => Color::new(0.5, 0.5, 0.5, 1.0),     // Gray
            StyleRank::C => Color::new(1.0, 1.0, 1.0, 1.0),     // White
            StyleRank::B => Color::new(0.3, 0.7, 1.0, 1.0),     // Blue
            StyleRank::A => Color::new(0.2, 1.0, 0.2, 1.0),     // Green
            StyleRank::S => Color::new(1.0, 0.8, 0.0, 1.0),     // Gold
            StyleRank::SS => Color::new(1.0, 0.3, 0.0, 1.0),    // Orange
            StyleRank::SSS => Color::new(1.0, 0.0, 1.0, 1.0),   // Magenta
        }
    }

    pub fn to_multiplier(&self) -> f32 {
        match self {
            StyleRank::D => 0.0,
            StyleRank::C => 0.1,
            StyleRank::B => 0.25,
            StyleRank::A => 0.5,
            StyleRank::S => 1.0,
            StyleRank::SS => 1.5,
            StyleRank::SSS => 2.0,
        }
    }

    pub fn get_title(&self) -> &str {
        match self {
            StyleRank::D => "Dull",
            StyleRank::C => "Decent",
            StyleRank::B => "Badass",
            StyleRank::A => "Awesome",
            StyleRank::S => "Stylish",
            StyleRank::SS => "Smoking Sexy Style",
            StyleRank::SSS => "Sensational!",
        }
    }
}

impl MoveType {
    pub fn to_string(&self) -> &str {
        match self {
            MoveType::LightAttack => "Light Attack",
            MoveType::HeavyAttack => "Heavy Attack",
            MoveType::SpecialAttack => "Special Attack",
            MoveType::Launcher => "Launcher",
            MoveType::AirAttack => "Air Attack",
            MoveType::Ability => "Ability",
            MoveType::Finisher => "Finisher",
        }
    }
}

/// Combo challenges for advanced players
pub struct ComboChallenge {
    pub name: String,
    pub description: String,
    pub requirements: ChallengeRequirements,
    pub reward: String,
}

pub struct ChallengeRequirements {
    pub min_combo: u32,
    pub required_rank: StyleRank,
    pub must_include_juggle: bool,
    pub must_include_finisher: bool,
    pub min_variation: f32,
}

impl ComboChallenge {
    pub fn create_challenges() -> Vec<Self> {
        vec![
            ComboChallenge {
                name: "Combo Novice".to_string(),
                description: "Achieve a 20-hit combo".to_string(),
                requirements: ChallengeRequirements {
                    min_combo: 20,
                    required_rank: StyleRank::C,
                    must_include_juggle: false,
                    must_include_finisher: false,
                    min_variation: 0.0,
                },
                reward: "100 Currency".to_string(),
            },
            ComboChallenge {
                name: "Stylish Fighter".to_string(),
                description: "Achieve an S rank combo".to_string(),
                requirements: ChallengeRequirements {
                    min_combo: 30,
                    required_rank: StyleRank::S,
                    must_include_juggle: false,
                    must_include_finisher: false,
                    min_variation: 0.5,
                },
                reward: "Stylish Title".to_string(),
            },
            ComboChallenge {
                name: "Juggle Master".to_string(),
                description: "Perform a 15-hit juggle combo".to_string(),
                requirements: ChallengeRequirements {
                    min_combo: 15,
                    required_rank: StyleRank::A,
                    must_include_juggle: true,
                    must_include_finisher: false,
                    min_variation: 0.0,
                },
                reward: "Juggle Master Title".to_string(),
            },
            ComboChallenge {
                name: "SSS Rank Legend".to_string(),
                description: "Achieve an SSS rank combo with a finisher".to_string(),
                requirements: ChallengeRequirements {
                    min_combo: 50,
                    required_rank: StyleRank::SSS,
                    must_include_juggle: true,
                    must_include_finisher: true,
                    min_variation: 0.7,
                },
                reward: "Combo God Title + Legendary Skin".to_string(),
            },
        ]
    }

    pub fn check_completion(&self, analysis: &ComboAnalysis, had_finisher: bool) -> bool {
        analysis.total_hits >= self.requirements.min_combo
            && analysis.style_rank >= self.requirements.required_rank
            && (!self.requirements.must_include_juggle || analysis.juggle_hits > 0)
            && (!self.requirements.must_include_finisher || had_finisher)
            && analysis.variation_bonus >= self.requirements.min_variation
    }
}
