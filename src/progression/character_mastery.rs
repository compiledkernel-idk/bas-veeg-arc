use crate::data::characters::CharacterId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages character mastery levels for all characters
pub struct MasteryManager {
    masteries: HashMap<CharacterId, CharacterMastery>,
}

/// Mastery data for a single character
#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterMastery {
    pub character: CharacterId,
    pub level: u32,
    pub xp: f32,
    pub rank: MasteryRank,
    pub games_played: u32,
    pub games_won: u32,
    pub total_damage: f32,
    pub total_kills: u32,
    pub total_deaths: u32,
    pub highest_combo: u32,
    pub unlocked_skins: Vec<String>,
    pub unlocked_titles: Vec<String>,
    pub milestones: Vec<MasteryMilestone>,
}

/// Mastery rank tiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MasteryRank {
    Bronze,    // Level 1-10
    Silver,    // Level 11-25
    Gold,      // Level 26-50
    Platinum,  // Level 51-75
    Diamond,   // Level 76-99
    Master,    // Level 100
}

/// Mastery milestones that grant rewards
#[derive(Clone, Serialize, Deserialize)]
pub struct MasteryMilestone {
    pub level: u32,
    pub name: String,
    pub description: String,
    pub reward: MasteryReward,
    pub unlocked: bool,
}

/// Rewards from mastery milestones
#[derive(Clone, Serialize, Deserialize)]
pub enum MasteryReward {
    Skin(String),
    Title(String),
    SkillPoints(u32),
    Currency(f32),
    Cosmetic(String),
}

impl MasteryManager {
    pub fn new() -> Self {
        let mut manager = Self {
            masteries: HashMap::new(),
        };

        // Initialize mastery for all characters
        manager.init_all_masteries();

        manager
    }

    fn init_all_masteries(&mut self) {
        let characters = vec![
            CharacterId::Bas,
            CharacterId::Berkay,
            CharacterId::Luca,
            // Add more characters
        ];

        for character in characters {
            self.masteries.insert(character, CharacterMastery::new(character));
        }
    }

    /// Get mastery for a character
    pub fn get_mastery(&self, character: CharacterId) -> Option<&CharacterMastery> {
        self.masteries.get(&character)
    }

    /// Get mutable mastery for a character
    pub fn get_mastery_mut(&mut self, character: CharacterId) -> Option<&mut CharacterMastery> {
        self.masteries.get_mut(&character)
    }

    /// Add XP to a character
    pub fn add_xp(&mut self, character: CharacterId, xp: f32) -> Vec<MasteryReward> {
        let mut rewards = Vec::new();

        if let Some(mastery) = self.masteries.get_mut(&character) {
            rewards = mastery.add_xp(xp);
        }

        rewards
    }

    /// Record a game played
    pub fn record_game(&mut self, character: CharacterId, won: bool, stats: GameStats) {
        if let Some(mastery) = self.masteries.get_mut(&character) {
            mastery.record_game(won, stats);
        }
    }

    /// Get total mastery level across all characters
    pub fn get_total_mastery_level(&self) -> u32 {
        self.masteries.values().map(|m| m.level).sum()
    }

    /// Get most played character
    pub fn get_most_played(&self) -> Option<CharacterId> {
        self.masteries
            .iter()
            .max_by_key(|(_, m)| m.games_played)
            .map(|(c, _)| *c)
    }
}

impl CharacterMastery {
    pub fn new(character: CharacterId) -> Self {
        Self {
            character,
            level: 1,
            xp: 0.0,
            rank: MasteryRank::Bronze,
            games_played: 0,
            games_won: 0,
            total_damage: 0.0,
            total_kills: 0,
            total_deaths: 0,
            highest_combo: 0,
            unlocked_skins: vec!["Default".to_string()],
            unlocked_titles: vec![],
            milestones: Self::create_milestones(),
        }
    }

    /// Create mastery milestones
    fn create_milestones() -> Vec<MasteryMilestone> {
        vec![
            MasteryMilestone {
                level: 5,
                name: "Novice".to_string(),
                description: "Reach mastery level 5".to_string(),
                reward: MasteryReward::SkillPoints(1),
                unlocked: false,
            },
            MasteryMilestone {
                level: 10,
                name: "Bronze Master".to_string(),
                description: "Reach Bronze rank".to_string(),
                reward: MasteryReward::Skin("Bronze Variant".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 15,
                name: "Apprentice".to_string(),
                description: "Reach mastery level 15".to_string(),
                reward: MasteryReward::SkillPoints(2),
                unlocked: false,
            },
            MasteryMilestone {
                level: 25,
                name: "Silver Master".to_string(),
                description: "Reach Silver rank".to_string(),
                reward: MasteryReward::Skin("Silver Variant".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 30,
                name: "Journeyman".to_string(),
                description: "Reach mastery level 30".to_string(),
                reward: MasteryReward::Title("Journeyman".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 40,
                name: "Veteran".to_string(),
                description: "Reach mastery level 40".to_string(),
                reward: MasteryReward::SkillPoints(3),
                unlocked: false,
            },
            MasteryMilestone {
                level: 50,
                name: "Gold Master".to_string(),
                description: "Reach Gold rank".to_string(),
                reward: MasteryReward::Skin("Gold Variant".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 60,
                name: "Expert".to_string(),
                description: "Reach mastery level 60".to_string(),
                reward: MasteryReward::Title("Expert".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 75,
                name: "Platinum Master".to_string(),
                description: "Reach Platinum rank".to_string(),
                reward: MasteryReward::Skin("Platinum Variant".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 85,
                name: "Master Fighter".to_string(),
                description: "Reach mastery level 85".to_string(),
                reward: MasteryReward::SkillPoints(5),
                unlocked: false,
            },
            MasteryMilestone {
                level: 99,
                name: "Diamond Master".to_string(),
                description: "Reach Diamond rank".to_string(),
                reward: MasteryReward::Skin("Diamond Variant".to_string()),
                unlocked: false,
            },
            MasteryMilestone {
                level: 100,
                name: "True Master".to_string(),
                description: "Reach maximum mastery".to_string(),
                reward: MasteryReward::Skin("Master Skin".to_string()),
                unlocked: false,
            },
        ]
    }

    /// Add XP and handle level ups
    pub fn add_xp(&mut self, xp: f32) -> Vec<MasteryReward> {
        self.xp += xp;

        let mut rewards = Vec::new();

        // Check for level up
        while self.xp >= self.xp_for_next_level() && self.level < 100 {
            self.xp -= self.xp_for_next_level();
            self.level += 1;

            // Update rank
            self.update_rank();

            // Check for milestone rewards
            for milestone in &mut self.milestones {
                if milestone.level == self.level && !milestone.unlocked {
                    milestone.unlocked = true;
                    rewards.push(milestone.reward.clone());

                    // Apply rewards
                    match &milestone.reward {
                        MasteryReward::Skin(skin) => {
                            self.unlocked_skins.push(skin.clone());
                        }
                        MasteryReward::Title(title) => {
                            self.unlocked_titles.push(title.clone());
                        }
                        _ => {}
                    }
                }
            }
        }

        rewards
    }

    /// Calculate XP needed for next level
    pub fn xp_for_next_level(&self) -> f32 {
        // Exponential scaling: base * (level^1.5)
        100.0 * (self.level as f32).powf(1.5)
    }

    /// Update mastery rank based on level
    fn update_rank(&mut self) {
        self.rank = match self.level {
            1..=10 => MasteryRank::Bronze,
            11..=25 => MasteryRank::Silver,
            26..=50 => MasteryRank::Gold,
            51..=75 => MasteryRank::Platinum,
            76..=99 => MasteryRank::Diamond,
            100.. => MasteryRank::Master,
            _ => MasteryRank::Bronze,
        };
    }

    /// Record a game
    pub fn record_game(&mut self, won: bool, stats: GameStats) {
        self.games_played += 1;
        if won {
            self.games_won += 1;
        }

        self.total_damage += stats.damage_dealt;
        self.total_kills += stats.kills;
        self.total_deaths += stats.deaths;

        if stats.highest_combo > self.highest_combo {
            self.highest_combo = stats.highest_combo;
        }
    }

    /// Get win rate
    pub fn get_win_rate(&self) -> f32 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f32 / self.games_played as f32) * 100.0
        }
    }

    /// Get KDA ratio
    pub fn get_kda(&self) -> f32 {
        if self.total_deaths == 0 {
            self.total_kills as f32
        } else {
            self.total_kills as f32 / self.total_deaths as f32
        }
    }

    /// Get progress to next level as percentage
    pub fn get_progress_percent(&self) -> f32 {
        if self.level >= 100 {
            100.0
        } else {
            (self.xp / self.xp_for_next_level()) * 100.0
        }
    }
}

impl MasteryRank {
    pub fn to_string(&self) -> &str {
        match self {
            MasteryRank::Bronze => "Bronze",
            MasteryRank::Silver => "Silver",
            MasteryRank::Gold => "Gold",
            MasteryRank::Platinum => "Platinum",
            MasteryRank::Diamond => "Diamond",
            MasteryRank::Master => "Master",
        }
    }

    pub fn to_color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::Color;
        match self {
            MasteryRank::Bronze => Color::new(0.8, 0.5, 0.2, 1.0),
            MasteryRank::Silver => Color::new(0.75, 0.75, 0.75, 1.0),
            MasteryRank::Gold => Color::new(1.0, 0.84, 0.0, 1.0),
            MasteryRank::Platinum => Color::new(0.9, 0.95, 1.0, 1.0),
            MasteryRank::Diamond => Color::new(0.4, 0.8, 1.0, 1.0),
            MasteryRank::Master => Color::new(1.0, 0.2, 1.0, 1.0),
        }
    }
}

/// Stats from a completed game
pub struct GameStats {
    pub damage_dealt: f32,
    pub kills: u32,
    pub deaths: u32,
    pub highest_combo: u32,
}
