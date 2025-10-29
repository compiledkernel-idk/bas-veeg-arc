use crate::data::characters::CharacterId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Daily and weekly challenge system
pub struct ChallengeManager {
    pub daily_challenges: Vec<Challenge>,
    pub weekly_challenges: Vec<Challenge>,
    pub completed_daily: Vec<String>,
    pub completed_weekly: Vec<String>,
    pub last_daily_reset: u64,
    pub last_weekly_reset: u64,
    pub streak_days: u32,
    pub total_challenges_completed: u32,
}

/// Individual challenge
#[derive(Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub challenge_type: ChallengeType,
    pub difficulty: ChallengeDifficulty,
    pub progress: f32,
    pub max_progress: f32,
    pub reward: ChallengeReward,
    pub is_daily: bool,
    pub expires_at: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    // Combat challenges
    DefeatEnemies {
        count: u32,
        enemy_type: Option<String>,
    },
    DealDamage {
        amount: f32,
    },
    ReachCombo {
        combo: u32,
    },
    DefeatBoss {
        boss: CharacterId,
        time_limit: Option<f32>,
    },
    WinWithoutDying,
    PerfectParries {
        count: u32,
    },
    CriticalHits {
        count: u32,
    },

    // Character specific
    WinWithCharacter {
        character: CharacterId,
    },
    UseAbility {
        character: CharacterId,
        times: u32,
    },

    // Mode specific
    CompleteWave {
        wave: u32,
        mode: String,
    },
    SurviveMinutes {
        minutes: u32,
        mode: String,
    },
    CompleteMode {
        mode: String,
    },

    // Co-op challenges
    WinCoopGame,
    RevivePlayers {
        count: u32,
    },
    TeamCombo {
        combo: u32,
    },

    // Special challenges
    NoUpgrades,
    OnlyMeleeAttacks,
    SpeedRun {
        time_limit: f32,
    },
    FlawlessVictory,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ChallengeDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ChallengeReward {
    Currency(f32),
    XP(f32),
    SkillPoints(u32),
    Cosmetic(String),
    Multiple(Vec<ChallengeReward>),
}

impl ChallengeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            daily_challenges: Vec::new(),
            weekly_challenges: Vec::new(),
            completed_daily: Vec::new(),
            completed_weekly: Vec::new(),
            last_daily_reset: Self::get_current_timestamp(),
            last_weekly_reset: Self::get_current_timestamp(),
            streak_days: 0,
            total_challenges_completed: 0,
        };

        manager.refresh_daily_challenges();
        manager.refresh_weekly_challenges();

        manager
    }

    /// Update challenges and check for resets
    pub fn update(&mut self) {
        let current_time = Self::get_current_timestamp();

        // Check for daily reset (24 hours)
        if current_time >= self.last_daily_reset + 86400 {
            self.reset_daily_challenges();
        }

        // Check for weekly reset (7 days)
        if current_time >= self.last_weekly_reset + 604800 {
            self.reset_weekly_challenges();
        }
    }

    /// Reset daily challenges
    fn reset_daily_challenges(&mut self) {
        // Check if any daily challenges were completed
        let completed_any = !self.completed_daily.is_empty();

        if completed_any {
            self.streak_days += 1;
        } else {
            self.streak_days = 0;
        }

        self.completed_daily.clear();
        self.refresh_daily_challenges();
        self.last_daily_reset = Self::get_current_timestamp();
    }

    /// Reset weekly challenges
    fn reset_weekly_challenges(&mut self) {
        self.completed_weekly.clear();
        self.refresh_weekly_challenges();
        self.last_weekly_reset = Self::get_current_timestamp();
    }

    /// Generate new daily challenges
    fn refresh_daily_challenges(&mut self) {
        self.daily_challenges.clear();

        let templates = Self::get_daily_challenge_templates();
        let selected = Self::select_random_challenges(&templates, 3);

        for template in selected {
            self.daily_challenges.push(template.clone());
        }
    }

    /// Generate new weekly challenges
    fn refresh_weekly_challenges(&mut self) {
        self.weekly_challenges.clear();

        let templates = Self::get_weekly_challenge_templates();
        let selected = Self::select_random_challenges(&templates, 5);

        for template in selected {
            self.weekly_challenges.push(template.clone());
        }
    }

    /// Get daily challenge templates
    fn get_daily_challenge_templates() -> Vec<Challenge> {
        let current_time = Self::get_current_timestamp();
        let expires_at = current_time + 86400; // 24 hours

        vec![
            Challenge {
                id: "daily_defeat_30".to_string(),
                name: "Slayer".to_string(),
                description: "Defeat 30 enemies".to_string(),
                challenge_type: ChallengeType::DefeatEnemies {
                    count: 30,
                    enemy_type: None,
                },
                difficulty: ChallengeDifficulty::Easy,
                progress: 0.0,
                max_progress: 30.0,
                reward: ChallengeReward::Currency(150.0),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_combo_50".to_string(),
                name: "Combo Master".to_string(),
                description: "Achieve a 50-hit combo".to_string(),
                challenge_type: ChallengeType::ReachCombo { combo: 50 },
                difficulty: ChallengeDifficulty::Medium,
                progress: 0.0,
                max_progress: 50.0,
                reward: ChallengeReward::Currency(200.0),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_wave_10".to_string(),
                name: "Wave Warrior".to_string(),
                description: "Complete wave 10".to_string(),
                challenge_type: ChallengeType::CompleteWave {
                    wave: 10,
                    mode: "Story".to_string(),
                },
                difficulty: ChallengeDifficulty::Medium,
                progress: 0.0,
                max_progress: 10.0,
                reward: ChallengeReward::XP(500.0),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_no_damage".to_string(),
                name: "Flawless".to_string(),
                description: "Win a game without taking damage".to_string(),
                challenge_type: ChallengeType::FlawlessVictory,
                difficulty: ChallengeDifficulty::Hard,
                progress: 0.0,
                max_progress: 1.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(300.0),
                    ChallengeReward::XP(750.0),
                ]),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_damage_5k".to_string(),
                name: "Heavy Hitter".to_string(),
                description: "Deal 5000 damage in a single game".to_string(),
                challenge_type: ChallengeType::DealDamage { amount: 5000.0 },
                difficulty: ChallengeDifficulty::Medium,
                progress: 0.0,
                max_progress: 5000.0,
                reward: ChallengeReward::Currency(175.0),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_perfect_parries".to_string(),
                name: "Perfect Defense".to_string(),
                description: "Perform 10 perfect parries".to_string(),
                challenge_type: ChallengeType::PerfectParries { count: 10 },
                difficulty: ChallengeDifficulty::Hard,
                progress: 0.0,
                max_progress: 10.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(250.0),
                    ChallengeReward::SkillPoints(1),
                ]),
                is_daily: true,
                expires_at,
            },
            Challenge {
                id: "daily_coop".to_string(),
                name: "Team Player".to_string(),
                description: "Win a co-op game".to_string(),
                challenge_type: ChallengeType::WinCoopGame,
                difficulty: ChallengeDifficulty::Easy,
                progress: 0.0,
                max_progress: 1.0,
                reward: ChallengeReward::Currency(200.0),
                is_daily: true,
                expires_at,
            },
        ]
    }

    /// Get weekly challenge templates
    fn get_weekly_challenge_templates() -> Vec<Challenge> {
        let current_time = Self::get_current_timestamp();
        let expires_at = current_time + 604800; // 7 days

        vec![
            Challenge {
                id: "weekly_defeat_500".to_string(),
                name: "Executioner".to_string(),
                description: "Defeat 500 enemies this week".to_string(),
                challenge_type: ChallengeType::DefeatEnemies {
                    count: 500,
                    enemy_type: None,
                },
                difficulty: ChallengeDifficulty::Hard,
                progress: 0.0,
                max_progress: 500.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(1000.0),
                    ChallengeReward::XP(2000.0),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_boss_rush".to_string(),
                name: "Boss Crusher".to_string(),
                description: "Complete Boss Rush mode".to_string(),
                challenge_type: ChallengeType::CompleteMode {
                    mode: "BossRush".to_string(),
                },
                difficulty: ChallengeDifficulty::Expert,
                progress: 0.0,
                max_progress: 1.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(1500.0),
                    ChallengeReward::SkillPoints(3),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_wave_25".to_string(),
                name: "Endurance".to_string(),
                description: "Reach wave 25 in any mode".to_string(),
                challenge_type: ChallengeType::CompleteWave {
                    wave: 25,
                    mode: "Any".to_string(),
                },
                difficulty: ChallengeDifficulty::Hard,
                progress: 0.0,
                max_progress: 25.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(800.0),
                    ChallengeReward::SkillPoints(2),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_combo_150".to_string(),
                name: "Combo God".to_string(),
                description: "Achieve a 150-hit combo".to_string(),
                challenge_type: ChallengeType::ReachCombo { combo: 150 },
                difficulty: ChallengeDifficulty::Expert,
                progress: 0.0,
                max_progress: 150.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(1200.0),
                    ChallengeReward::Cosmetic("Combo Master Title".to_string()),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_survival_10min".to_string(),
                name: "Survivor".to_string(),
                description: "Survive 10 minutes in Survival mode".to_string(),
                challenge_type: ChallengeType::SurviveMinutes {
                    minutes: 10,
                    mode: "Survival".to_string(),
                },
                difficulty: ChallengeDifficulty::Hard,
                progress: 0.0,
                max_progress: 600.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(900.0),
                    ChallengeReward::XP(1500.0),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_character_mastery".to_string(),
                name: "Character Master".to_string(),
                description: "Win 10 games with the same character".to_string(),
                challenge_type: ChallengeType::WinWithCharacter {
                    character: CharacterId::Bas,
                },
                difficulty: ChallengeDifficulty::Medium,
                progress: 0.0,
                max_progress: 10.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(700.0),
                    ChallengeReward::XP(1000.0),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_speedrun".to_string(),
                name: "Speed Demon".to_string(),
                description: "Complete a run in under 15 minutes".to_string(),
                challenge_type: ChallengeType::SpeedRun { time_limit: 900.0 },
                difficulty: ChallengeDifficulty::Expert,
                progress: 0.0,
                max_progress: 1.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(1500.0),
                    ChallengeReward::Cosmetic("Speed Runner Title".to_string()),
                ]),
                is_daily: false,
                expires_at,
            },
            Challenge {
                id: "weekly_coop_revives".to_string(),
                name: "Field Medic".to_string(),
                description: "Revive teammates 15 times".to_string(),
                challenge_type: ChallengeType::RevivePlayers { count: 15 },
                difficulty: ChallengeDifficulty::Medium,
                progress: 0.0,
                max_progress: 15.0,
                reward: ChallengeReward::Multiple(vec![
                    ChallengeReward::Currency(600.0),
                    ChallengeReward::Cosmetic("Medic Title".to_string()),
                ]),
                is_daily: false,
                expires_at,
            },
        ]
    }

    /// Select random challenges from templates
    fn select_random_challenges(templates: &[Challenge], count: usize) -> Vec<Challenge> {
        let mut selected = Vec::new();
        let mut indices: Vec<usize> = (0..templates.len()).collect();

        // Simple shuffle
        for i in 0..indices.len() {
            let j = (rand::gen_range(0.0, 1.0) * (indices.len() - i) as f32) as usize + i;
            indices.swap(i, j);
        }

        for i in 0..count.min(templates.len()) {
            selected.push(templates[indices[i]].clone());
        }

        selected
    }

    /// Update challenge progress
    pub fn update_progress(&mut self, challenge_type: &ChallengeType, progress: f32) -> Vec<ChallengeReward> {
        let mut rewards = Vec::new();

        // Update daily challenges
        for challenge in &mut self.daily_challenges {
            if Self::challenge_types_match(&challenge.challenge_type, challenge_type) {
                if !self.completed_daily.contains(&challenge.id) {
                    challenge.progress = (challenge.progress + progress).min(challenge.max_progress);

                    if challenge.progress >= challenge.max_progress {
                        self.completed_daily.push(challenge.id.clone());
                        self.total_challenges_completed += 1;
                        rewards.push(challenge.reward.clone());
                    }
                }
            }
        }

        // Update weekly challenges
        for challenge in &mut self.weekly_challenges {
            if Self::challenge_types_match(&challenge.challenge_type, challenge_type) {
                if !self.completed_weekly.contains(&challenge.id) {
                    challenge.progress = (challenge.progress + progress).min(challenge.max_progress);

                    if challenge.progress >= challenge.max_progress {
                        self.completed_weekly.push(challenge.id.clone());
                        self.total_challenges_completed += 1;
                        rewards.push(challenge.reward.clone());
                    }
                }
            }
        }

        rewards
    }

    /// Check if challenge types match
    fn challenge_types_match(a: &ChallengeType, b: &ChallengeType) -> bool {
        match (a, b) {
            (ChallengeType::DefeatEnemies { .. }, ChallengeType::DefeatEnemies { .. }) => true,
            (ChallengeType::DealDamage { .. }, ChallengeType::DealDamage { .. }) => true,
            (ChallengeType::ReachCombo { .. }, ChallengeType::ReachCombo { .. }) => true,
            (ChallengeType::DefeatBoss { .. }, ChallengeType::DefeatBoss { .. }) => true,
            (ChallengeType::WinWithoutDying, ChallengeType::WinWithoutDying) => true,
            (ChallengeType::PerfectParries { .. }, ChallengeType::PerfectParries { .. }) => true,
            (ChallengeType::CriticalHits { .. }, ChallengeType::CriticalHits { .. }) => true,
            (ChallengeType::WinCoopGame, ChallengeType::WinCoopGame) => true,
            (ChallengeType::RevivePlayers { .. }, ChallengeType::RevivePlayers { .. }) => true,
            (ChallengeType::FlawlessVictory, ChallengeType::FlawlessVictory) => true,
            _ => false,
        }
    }

    /// Get all active challenges
    pub fn get_active_challenges(&self) -> Vec<&Challenge> {
        self.daily_challenges
            .iter()
            .chain(&self.weekly_challenges)
            .filter(|c| !self.is_completed(&c.id, c.is_daily))
            .collect()
    }

    /// Check if challenge is completed
    pub fn is_completed(&self, challenge_id: &str, is_daily: bool) -> bool {
        if is_daily {
            self.completed_daily.contains(&challenge_id.to_string())
        } else {
            self.completed_weekly.contains(&challenge_id.to_string())
        }
    }

    /// Get completion percentage
    pub fn get_completion_percent(&self, is_daily: bool) -> f32 {
        if is_daily {
            let total = self.daily_challenges.len() as f32;
            let completed = self.completed_daily.len() as f32;
            (completed / total) * 100.0
        } else {
            let total = self.weekly_challenges.len() as f32;
            let completed = self.completed_weekly.len() as f32;
            (completed / total) * 100.0
        }
    }

    /// Get streak bonus multiplier
    pub fn get_streak_bonus(&self) -> f32 {
        1.0 + (self.streak_days.min(30) as f32 * 0.05)
    }

    /// Get current timestamp
    fn get_current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Get time until daily reset
    pub fn time_until_daily_reset(&self) -> u64 {
        let current = Self::get_current_timestamp();
        let next_reset = self.last_daily_reset + 86400;
        next_reset.saturating_sub(current)
    }

    /// Get time until weekly reset
    pub fn time_until_weekly_reset(&self) -> u64 {
        let current = Self::get_current_timestamp();
        let next_reset = self.last_weekly_reset + 604800;
        next_reset.saturating_sub(current)
    }
}

impl ChallengeDifficulty {
    pub fn to_string(&self) -> &str {
        match self {
            ChallengeDifficulty::Easy => "Easy",
            ChallengeDifficulty::Medium => "Medium",
            ChallengeDifficulty::Hard => "Hard",
            ChallengeDifficulty::Expert => "Expert",
        }
    }

    pub fn to_color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::Color;
        match self {
            ChallengeDifficulty::Easy => Color::new(0.5, 1.0, 0.5, 1.0),
            ChallengeDifficulty::Medium => Color::new(0.5, 0.7, 1.0, 1.0),
            ChallengeDifficulty::Hard => Color::new(1.0, 0.5, 0.0, 1.0),
            ChallengeDifficulty::Expert => Color::new(1.0, 0.2, 0.2, 1.0),
        }
    }
}
