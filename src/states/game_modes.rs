use crate::data::characters::CharacterId;
use crate::ecs::entity::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Game mode types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Story,          // Original story mode
    Endless,        // Survive as long as possible
    BossRush,       // Fight all bosses back-to-back
    Horde,          // Massive enemy waves
    TimeAttack,     // Complete objectives quickly
    Survival,       // One life challenge
    Training,       // Practice mode
    Versus,         // PvP mode
}

/// Boss Rush mode - fight all bosses consecutively
pub struct BossRushMode {
    pub current_boss_index: usize,
    pub bosses_defeated: Vec<CharacterId>,
    pub boss_order: Vec<CharacterId>,
    pub time_elapsed: f32,
    pub no_damage_run: bool,
    pub health_restored_between_bosses: bool,
    pub difficulty_multiplier: f32,
}

impl BossRushMode {
    pub fn new(difficulty: BossRushDifficulty) -> Self {
        let boss_order = vec![
            CharacterId::Bastiaan,
            // Add more bosses as they're implemented
        ];

        let (health_restored, difficulty_mult) = match difficulty {
            BossRushDifficulty::Normal => (true, 1.0),
            BossRushDifficulty::Hard => (false, 1.5),
            BossRushDifficulty::Nightmare => (false, 2.0),
        };

        Self {
            current_boss_index: 0,
            bosses_defeated: Vec::new(),
            boss_order,
            time_elapsed: 0.0,
            no_damage_run: true,
            health_restored_between_bosses: health_restored,
            difficulty_multiplier: difficulty_mult,
        }
    }

    pub fn get_current_boss(&self) -> Option<CharacterId> {
        self.boss_order.get(self.current_boss_index).copied()
    }

    pub fn defeat_current_boss(&mut self) {
        if let Some(boss) = self.get_current_boss() {
            self.bosses_defeated.push(boss);
            self.current_boss_index += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_boss_index >= self.boss_order.len()
    }

    pub fn get_completion_percent(&self) -> f32 {
        (self.current_boss_index as f32 / self.boss_order.len() as f32) * 100.0
    }

    pub fn record_damage_taken(&mut self) {
        self.no_damage_run = false;
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = self.bosses_defeated.len() as u32 * 1000;

        // Time bonus (faster = more points)
        let time_bonus = (10000.0 / (self.time_elapsed + 1.0)) as u32;
        score += time_bonus;

        // No damage bonus
        if self.no_damage_run {
            score += 5000;
        }

        // Difficulty multiplier
        score = (score as f32 * self.difficulty_multiplier) as u32;

        score
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BossRushDifficulty {
    Normal,   // Health restored between bosses
    Hard,     // No health restore, 1.5x damage
    Nightmare, // No health restore, 2x damage
}

/// Horde Mode - survive massive waves of enemies
pub struct HordeMode {
    pub wave: u32,
    pub enemies_killed_this_wave: u32,
    pub enemies_remaining: u32,
    pub total_enemies_killed: u32,
    pub time_elapsed: f32,
    pub spawn_rate: f32,
    pub max_enemies: u32,
    pub difficulty_scaling: f32,
    pub active_modifiers: Vec<HordeModifier>,
}

impl HordeMode {
    pub fn new() -> Self {
        Self {
            wave: 1,
            enemies_killed_this_wave: 0,
            enemies_remaining: 0,
            total_enemies_killed: 0,
            time_elapsed: 0.0,
            spawn_rate: 2.0, // Spawn every 2 seconds
            max_enemies: 50,
            difficulty_scaling: 1.0,
            active_modifiers: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_elapsed += dt;

        // Increase difficulty over time
        self.difficulty_scaling = 1.0 + (self.wave as f32 * 0.1);

        // Speed up spawn rate
        self.spawn_rate = (2.0 / (1.0 + self.wave as f32 * 0.05)).max(0.2);

        // Increase max enemies
        self.max_enemies = 50 + (self.wave * 5);
    }

    pub fn on_enemy_killed(&mut self) {
        self.enemies_killed_this_wave += 1;
        self.total_enemies_killed += 1;
        self.enemies_remaining = self.enemies_remaining.saturating_sub(1);
    }

    pub fn should_advance_wave(&self) -> bool {
        self.enemies_remaining == 0 && self.enemies_killed_this_wave >= 20 + (self.wave * 5)
    }

    pub fn advance_wave(&mut self) {
        self.wave += 1;
        self.enemies_killed_this_wave = 0;

        // Apply random modifier every 5 waves
        if self.wave % 5 == 0 {
            self.add_random_modifier();
        }
    }

    fn add_random_modifier(&mut self) {
        let modifiers = vec![
            HordeModifier::SpeedBoost,
            HordeModifier::DamageBoost,
            HordeModifier::ArmorBoost,
            HordeModifier::ExplosiveEnemies,
            HordeModifier::FastSpawns,
        ];

        let index = (rand::gen_range(0.0, 1.0) * modifiers.len() as f32) as usize % modifiers.len();
        let modifier = modifiers[index].clone();

        if !self.active_modifiers.contains(&modifier) {
            self.active_modifiers.push(modifier);
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = self.total_enemies_killed * 10;
        score += self.wave * 500;

        // Survival time bonus
        let time_bonus = (self.time_elapsed / 60.0) as u32 * 100;
        score += time_bonus;

        score
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HordeModifier {
    SpeedBoost,        // Enemies move 50% faster
    DamageBoost,       // Enemies deal 50% more damage
    ArmorBoost,        // Enemies have 50% more health
    ExplosiveEnemies,  // Enemies explode on death
    FastSpawns,        // Enemies spawn 2x faster
}

impl HordeModifier {
    pub fn to_string(&self) -> &str {
        match self {
            HordeModifier::SpeedBoost => "Speed Boost",
            HordeModifier::DamageBoost => "Damage Boost",
            HordeModifier::ArmorBoost => "Armor Boost",
            HordeModifier::ExplosiveEnemies => "Explosive Enemies",
            HordeModifier::FastSpawns => "Fast Spawns",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            HordeModifier::SpeedBoost => "Enemies move 50% faster",
            HordeModifier::DamageBoost => "Enemies deal 50% more damage",
            HordeModifier::ArmorBoost => "Enemies have 50% more health",
            HordeModifier::ExplosiveEnemies => "Enemies explode when killed",
            HordeModifier::FastSpawns => "Enemies spawn twice as fast",
        }
    }
}

/// Time Attack mode - complete objectives as fast as possible
pub struct TimeAttackMode {
    pub objective: TimeAttackObjective,
    pub time_elapsed: f32,
    pub time_limit: Option<f32>,
    pub objectives_completed: u32,
    pub combo_maintained: bool,
    pub par_time: f32,
    pub record_time: Option<f32>,
}

impl TimeAttackMode {
    pub fn new(objective: TimeAttackObjective) -> Self {
        let par_time = match &objective {
            TimeAttackObjective::DefeatEnemies(count) => *count as f32 * 2.0,
            TimeAttackObjective::DefeatBoss(_) => 120.0,
            TimeAttackObjective::ReachWave(wave) => *wave as f32 * 30.0,
            TimeAttackObjective::ComboChallenge(hits) => *hits as f32 * 0.5,
        };

        Self {
            objective,
            time_elapsed: 0.0,
            time_limit: None,
            objectives_completed: 0,
            combo_maintained: true,
            par_time,
            record_time: None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_elapsed += dt;
    }

    pub fn complete(&mut self) {
        self.objectives_completed += 1;
        if let Some(record) = self.record_time {
            if self.time_elapsed < record {
                self.record_time = Some(self.time_elapsed);
            }
        } else {
            self.record_time = Some(self.time_elapsed);
        }
    }

    pub fn get_rank(&self) -> TimeAttackRank {
        let time_ratio = self.time_elapsed / self.par_time;

        if time_ratio <= 0.5 {
            TimeAttackRank::S
        } else if time_ratio <= 0.75 {
            TimeAttackRank::A
        } else if time_ratio <= 1.0 {
            TimeAttackRank::B
        } else if time_ratio <= 1.5 {
            TimeAttackRank::C
        } else {
            TimeAttackRank::D
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let base_score = 10000;
        let time_penalty = (self.time_elapsed * 10.0) as u32;
        let rank_bonus = match self.get_rank() {
            TimeAttackRank::S => 5000,
            TimeAttackRank::A => 3000,
            TimeAttackRank::B => 1000,
            TimeAttackRank::C => 500,
            TimeAttackRank::D => 0,
        };

        base_score.saturating_sub(time_penalty) + rank_bonus
    }
}

#[derive(Clone, Debug)]
pub enum TimeAttackObjective {
    DefeatEnemies(u32),
    DefeatBoss(CharacterId),
    ReachWave(u32),
    ComboChallenge(u32), // Reach X combo
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeAttackRank {
    S,
    A,
    B,
    C,
    D,
}

impl TimeAttackRank {
    pub fn to_string(&self) -> &str {
        match self {
            TimeAttackRank::S => "S",
            TimeAttackRank::A => "A",
            TimeAttackRank::B => "B",
            TimeAttackRank::C => "C",
            TimeAttackRank::D => "D",
        }
    }

    pub fn to_color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::Color;
        match self {
            TimeAttackRank::S => Color::new(1.0, 0.8, 0.0, 1.0),    // Gold
            TimeAttackRank::A => Color::new(0.0, 1.0, 0.0, 1.0),    // Green
            TimeAttackRank::B => Color::new(0.0, 0.5, 1.0, 1.0),    // Blue
            TimeAttackRank::C => Color::new(1.0, 0.5, 0.0, 1.0),    // Orange
            TimeAttackRank::D => Color::new(0.5, 0.5, 0.5, 1.0),    // Gray
        }
    }
}

/// Survival mode - one life, see how long you can last
pub struct SurvivalMode {
    pub time_survived: f32,
    pub enemies_killed: u32,
    pub damage_dealt: f32,
    pub max_combo: u32,
    pub difficulty_tier: u32,
    pub health_pickups_collected: u32,
    pub perfect_dodges: u32,
    pub is_alive: bool,
}

impl SurvivalMode {
    pub fn new() -> Self {
        Self {
            time_survived: 0.0,
            enemies_killed: 0,
            damage_dealt: 0.0,
            max_combo: 0,
            difficulty_tier: 1,
            health_pickups_collected: 0,
            perfect_dodges: 0,
            is_alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_alive {
            self.time_survived += dt;

            // Increase difficulty every 60 seconds
            self.difficulty_tier = (self.time_survived / 60.0) as u32 + 1;
        }
    }

    pub fn on_death(&mut self) {
        self.is_alive = false;
    }

    pub fn get_difficulty_multiplier(&self) -> f32 {
        1.0 + (self.difficulty_tier as f32 * 0.2)
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;

        // Time survived (1 point per second)
        score += self.time_survived as u32;

        // Kills (10 points each)
        score += self.enemies_killed * 10;

        // Damage dealt (1 point per 10 damage)
        score += (self.damage_dealt / 10.0) as u32;

        // Max combo bonus
        score += self.max_combo * 5;

        // Perfect dodge bonus
        score += self.perfect_dodges * 50;

        score
    }

    pub fn get_rank(&self) -> SurvivalRank {
        let minutes = self.time_survived / 60.0;

        if minutes >= 30.0 {
            SurvivalRank::Legend
        } else if minutes >= 20.0 {
            SurvivalRank::Master
        } else if minutes >= 15.0 {
            SurvivalRank::Expert
        } else if minutes >= 10.0 {
            SurvivalRank::Veteran
        } else if minutes >= 5.0 {
            SurvivalRank::Survivor
        } else {
            SurvivalRank::Novice
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurvivalRank {
    Novice,    // < 5 minutes
    Survivor,  // 5-10 minutes
    Veteran,   // 10-15 minutes
    Expert,    // 15-20 minutes
    Master,    // 20-30 minutes
    Legend,    // 30+ minutes
}

impl SurvivalRank {
    pub fn to_string(&self) -> &str {
        match self {
            SurvivalRank::Novice => "Novice",
            SurvivalRank::Survivor => "Survivor",
            SurvivalRank::Veteran => "Veteran",
            SurvivalRank::Expert => "Expert",
            SurvivalRank::Master => "Master",
            SurvivalRank::Legend => "Legend",
        }
    }

    pub fn to_color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::Color;
        match self {
            SurvivalRank::Novice => Color::new(0.7, 0.7, 0.7, 1.0),
            SurvivalRank::Survivor => Color::new(0.0, 1.0, 0.0, 1.0),
            SurvivalRank::Veteran => Color::new(0.0, 0.5, 1.0, 1.0),
            SurvivalRank::Expert => Color::new(0.8, 0.0, 1.0, 1.0),
            SurvivalRank::Master => Color::new(1.0, 0.5, 0.0, 1.0),
            SurvivalRank::Legend => Color::new(1.0, 0.8, 0.0, 1.0),
        }
    }
}

/// Leaderboard entry
#[derive(Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub character: CharacterId,
    pub score: u32,
    pub time: f32,
    pub wave: Option<u32>,
    pub timestamp: u64,
}

/// Leaderboard manager
pub struct LeaderboardManager {
    pub boss_rush_leaderboard: Vec<LeaderboardEntry>,
    pub horde_leaderboard: Vec<LeaderboardEntry>,
    pub time_attack_leaderboard: HashMap<String, Vec<LeaderboardEntry>>,
    pub survival_leaderboard: Vec<LeaderboardEntry>,
    max_entries: usize,
}

impl LeaderboardManager {
    pub fn new() -> Self {
        Self {
            boss_rush_leaderboard: Vec::new(),
            horde_leaderboard: Vec::new(),
            time_attack_leaderboard: HashMap::new(),
            survival_leaderboard: Vec::new(),
            max_entries: 100,
        }
    }

    pub fn add_boss_rush_entry(&mut self, entry: LeaderboardEntry) {
        self.boss_rush_leaderboard.push(entry);
        self.boss_rush_leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
        self.boss_rush_leaderboard.truncate(self.max_entries);
    }

    pub fn add_horde_entry(&mut self, entry: LeaderboardEntry) {
        self.horde_leaderboard.push(entry);
        self.horde_leaderboard.sort_by(|a, b| b.wave.unwrap_or(0).cmp(&a.wave.unwrap_or(0)));
        self.horde_leaderboard.truncate(self.max_entries);
    }

    pub fn add_time_attack_entry(&mut self, objective_name: String, entry: LeaderboardEntry) {
        let leaderboard = self.time_attack_leaderboard.entry(objective_name).or_insert_with(Vec::new);
        leaderboard.push(entry);
        leaderboard.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        leaderboard.truncate(self.max_entries);
    }

    pub fn add_survival_entry(&mut self, entry: LeaderboardEntry) {
        self.survival_leaderboard.push(entry);
        self.survival_leaderboard.sort_by(|a, b| b.time.partial_cmp(&a.time).unwrap());
        self.survival_leaderboard.truncate(self.max_entries);
    }

    pub fn get_player_rank(&self, mode: GameMode, score: u32) -> Option<usize> {
        let leaderboard = match mode {
            GameMode::BossRush => &self.boss_rush_leaderboard,
            GameMode::Horde => &self.horde_leaderboard,
            GameMode::Survival => &self.survival_leaderboard,
            _ => return None,
        };

        leaderboard.iter().position(|entry| entry.score <= score).map(|pos| pos + 1)
    }
}
