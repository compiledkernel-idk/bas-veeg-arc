use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages all achievements in the game
pub struct AchievementManager {
    achievements: HashMap<String, Achievement>,
    unlocked_count: u32,
    total_count: u32,
}

/// Individual achievement
#[derive(Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub difficulty: AchievementDifficulty,
    pub requirement: AchievementRequirement,
    pub reward: AchievementReward,
    pub progress: f32,
    pub max_progress: f32,
    pub unlocked: bool,
    pub unlock_timestamp: Option<u64>,
    pub hidden: bool, // Secret achievements
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AchievementCategory {
    Combat,
    Progression,
    Exploration,
    Mastery,
    Social, // Co-op related
    Challenge,
    Collection,
    Secret,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AchievementDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    Legendary,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AchievementRequirement {
    // Combat
    DealDamage(f32),
    DefeatEnemies(u32),
    DefeatBosses(u32),
    ReachCombo(u32),
    WinWithoutDying,
    CompleteWave(u32),

    // Progression
    ReachLevel(u32),
    UnlockCharacter(String),
    UnlockAllCharacters,
    MasteryLevel(String, u32), // Character, level
    SkillTreeComplete(String), // Character

    // Challenge
    CompleteTimeAttack(f32), // Under X seconds
    CompleteNoHit,
    CompleteSoloRun,
    CompleteEndlessWave(u32),

    // Collection
    UnlockAllSkins,
    UnlockAllTitles,
    BuyAllUpgrades,
    CollectCurrency(f32),

    // Social/Co-op
    PlayCoopGames(u32),
    RevivePlayers(u32),
    WinCoopGame,

    // Misc
    PlayGames(u32),
    PlayTime(f32), // Hours

    // Complex/Compound
    Multiple(Vec<AchievementRequirement>),
    Either(Vec<AchievementRequirement>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AchievementReward {
    Currency(f32),
    SkillPoints(u32),
    Title(String),
    Skin(String),
    None,
}

impl AchievementManager {
    pub fn new() -> Self {
        let mut manager = Self {
            achievements: HashMap::new(),
            unlocked_count: 0,
            total_count: 0,
        };

        manager.init_all_achievements();
        manager.total_count = manager.achievements.len() as u32;

        manager
    }

    fn init_all_achievements(&mut self) {
        let achievements = vec![
            // === COMBAT ACHIEVEMENTS ===
            Achievement {
                id: "first_blood".to_string(),
                name: "First Blood".to_string(),
                description: "Defeat your first enemy".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::DefeatEnemies(1),
                reward: AchievementReward::Currency(50.0),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "killing_spree".to_string(),
                name: "Killing Spree".to_string(),
                description: "Defeat 100 enemies".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::DefeatEnemies(100),
                reward: AchievementReward::Currency(200.0),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "executioner".to_string(),
                name: "Executioner".to_string(),
                description: "Defeat 500 enemies".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::DefeatEnemies(500),
                reward: AchievementReward::SkillPoints(2),
                progress: 0.0,
                max_progress: 500.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "genocide".to_string(),
                name: "Genocide".to_string(),
                description: "Defeat 2000 enemies".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::DefeatEnemies(2000),
                reward: AchievementReward::Title("The Destroyer".to_string()),
                progress: 0.0,
                max_progress: 2000.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "combo_novice".to_string(),
                name: "Combo Novice".to_string(),
                description: "Achieve a 10-hit combo".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::ReachCombo(10),
                reward: AchievementReward::Currency(75.0),
                progress: 0.0,
                max_progress: 10.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "combo_expert".to_string(),
                name: "Combo Expert".to_string(),
                description: "Achieve a 50-hit combo".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::ReachCombo(50),
                reward: AchievementReward::Currency(150.0),
                progress: 0.0,
                max_progress: 50.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "combo_master".to_string(),
                name: "Combo Master".to_string(),
                description: "Achieve a 100-hit combo".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::ReachCombo(100),
                reward: AchievementReward::SkillPoints(3),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "combo_god".to_string(),
                name: "Combo God".to_string(),
                description: "Achieve a 250-hit combo".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Legendary,
                requirement: AchievementRequirement::ReachCombo(250),
                reward: AchievementReward::Title("Combo God".to_string()),
                progress: 0.0,
                max_progress: 250.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: true,
            },
            Achievement {
                id: "boss_killer".to_string(),
                name: "Boss Killer".to_string(),
                description: "Defeat your first boss".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::DefeatBosses(1),
                reward: AchievementReward::Currency(150.0),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "boss_slayer".to_string(),
                name: "Boss Slayer".to_string(),
                description: "Defeat 10 bosses".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::DefeatBosses(10),
                reward: AchievementReward::SkillPoints(2),
                progress: 0.0,
                max_progress: 10.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "million_damage".to_string(),
                name: "Heavy Hitter".to_string(),
                description: "Deal 1 million total damage".to_string(),
                category: AchievementCategory::Combat,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::DealDamage(1000000.0),
                reward: AchievementReward::Title("Heavy Hitter".to_string()),
                progress: 0.0,
                max_progress: 1000000.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === PROGRESSION ACHIEVEMENTS ===
            Achievement {
                id: "getting_started".to_string(),
                name: "Getting Started".to_string(),
                description: "Complete wave 1".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::CompleteWave(1),
                reward: AchievementReward::Currency(50.0),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "wave_warrior".to_string(),
                name: "Wave Warrior".to_string(),
                description: "Complete wave 10".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::CompleteWave(10),
                reward: AchievementReward::SkillPoints(1),
                progress: 0.0,
                max_progress: 10.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "survivor".to_string(),
                name: "Survivor".to_string(),
                description: "Complete wave 25".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::CompleteWave(25),
                reward: AchievementReward::Title("Survivor".to_string()),
                progress: 0.0,
                max_progress: 25.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "perfect_run".to_string(),
                name: "Perfect Run".to_string(),
                description: "Win a game without dying".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::WinWithoutDying,
                reward: AchievementReward::SkillPoints(3),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "unlock_all_chars".to_string(),
                name: "Gotta Catch 'Em All".to_string(),
                description: "Unlock all playable characters".to_string(),
                category: AchievementCategory::Collection,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::UnlockAllCharacters,
                reward: AchievementReward::Title("Collector".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === MASTERY ACHIEVEMENTS ===
            Achievement {
                id: "bas_master".to_string(),
                name: "Bas Mastery".to_string(),
                description: "Reach mastery level 100 with Bas".to_string(),
                category: AchievementCategory::Mastery,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::MasteryLevel("Bas".to_string(), 100),
                reward: AchievementReward::Skin("Bas Master Skin".to_string()),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "berkay_master".to_string(),
                name: "Berkay Mastery".to_string(),
                description: "Reach mastery level 100 with Berkay".to_string(),
                category: AchievementCategory::Mastery,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::MasteryLevel("Berkay".to_string(), 100),
                reward: AchievementReward::Skin("Berkay Master Skin".to_string()),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === CHALLENGE ACHIEVEMENTS ===
            Achievement {
                id: "speed_runner".to_string(),
                name: "Speed Runner".to_string(),
                description: "Complete a run in under 10 minutes".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::CompleteTimeAttack(600.0),
                reward: AchievementReward::Title("Speed Demon".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "untouchable".to_string(),
                name: "Untouchable".to_string(),
                description: "Complete a run without taking damage".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Legendary,
                requirement: AchievementRequirement::CompleteNoHit,
                reward: AchievementReward::Title("Untouchable".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: true,
            },
            Achievement {
                id: "solo_artist".to_string(),
                name: "Solo Artist".to_string(),
                description: "Complete a run solo (no allies)".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::CompleteSoloRun,
                reward: AchievementReward::SkillPoints(3),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "endless_warrior".to_string(),
                name: "Endless Warrior".to_string(),
                description: "Reach wave 50 in Endless mode".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::CompleteEndlessWave(50),
                reward: AchievementReward::Title("Endless Warrior".to_string()),
                progress: 0.0,
                max_progress: 50.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "endless_legend".to_string(),
                name: "Endless Legend".to_string(),
                description: "Reach wave 100 in Endless mode".to_string(),
                category: AchievementCategory::Challenge,
                difficulty: AchievementDifficulty::Legendary,
                requirement: AchievementRequirement::CompleteEndlessWave(100),
                reward: AchievementReward::Title("Endless Legend".to_string()),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: true,
            },

            // === CO-OP ACHIEVEMENTS ===
            Achievement {
                id: "team_player".to_string(),
                name: "Team Player".to_string(),
                description: "Win a co-op game".to_string(),
                category: AchievementCategory::Social,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::WinCoopGame,
                reward: AchievementReward::Currency(100.0),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "coop_veteran".to_string(),
                name: "Co-op Veteran".to_string(),
                description: "Play 50 co-op games".to_string(),
                category: AchievementCategory::Social,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::PlayCoopGames(50),
                reward: AchievementReward::SkillPoints(2),
                progress: 0.0,
                max_progress: 50.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "medic".to_string(),
                name: "Field Medic".to_string(),
                description: "Revive teammates 25 times".to_string(),
                category: AchievementCategory::Social,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::RevivePlayers(25),
                reward: AchievementReward::Title("Medic".to_string()),
                progress: 0.0,
                max_progress: 25.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === COLLECTION ACHIEVEMENTS ===
            Achievement {
                id: "fashionista".to_string(),
                name: "Fashionista".to_string(),
                description: "Unlock all character skins".to_string(),
                category: AchievementCategory::Collection,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::UnlockAllSkins,
                reward: AchievementReward::Title("Fashionista".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "title_collector".to_string(),
                name: "Title Collector".to_string(),
                description: "Unlock all titles".to_string(),
                category: AchievementCategory::Collection,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::UnlockAllTitles,
                reward: AchievementReward::Title("Completionist".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "fully_upgraded".to_string(),
                name: "Fully Upgraded".to_string(),
                description: "Purchase all shop upgrades".to_string(),
                category: AchievementCategory::Collection,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::BuyAllUpgrades,
                reward: AchievementReward::SkillPoints(5),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "rich".to_string(),
                name: "Moneybags".to_string(),
                description: "Collect 10,000 currency".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::CollectCurrency(10000.0),
                reward: AchievementReward::Currency(500.0),
                progress: 0.0,
                max_progress: 10000.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "millionaire".to_string(),
                name: "Millionaire".to_string(),
                description: "Collect 100,000 currency".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::CollectCurrency(100000.0),
                reward: AchievementReward::Title("Millionaire".to_string()),
                progress: 0.0,
                max_progress: 100000.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === MISC ACHIEVEMENTS ===
            Achievement {
                id: "first_game".to_string(),
                name: "First Steps".to_string(),
                description: "Play your first game".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Easy,
                requirement: AchievementRequirement::PlayGames(1),
                reward: AchievementReward::Currency(25.0),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "hundred_games".to_string(),
                name: "Dedicated Player".to_string(),
                description: "Play 100 games".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::PlayGames(100),
                reward: AchievementReward::SkillPoints(3),
                progress: 0.0,
                max_progress: 100.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },
            Achievement {
                id: "thousand_games".to_string(),
                name: "Addicted".to_string(),
                description: "Play 1000 games".to_string(),
                category: AchievementCategory::Progression,
                difficulty: AchievementDifficulty::Expert,
                requirement: AchievementRequirement::PlayGames(1000),
                reward: AchievementReward::Title("Addicted".to_string()),
                progress: 0.0,
                max_progress: 1000.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: false,
            },

            // === SECRET ACHIEVEMENTS ===
            Achievement {
                id: "secret_room".to_string(),
                name: "???".to_string(),
                description: "Find the secret room".to_string(),
                category: AchievementCategory::Secret,
                difficulty: AchievementDifficulty::Hard,
                requirement: AchievementRequirement::Multiple(vec![]),
                reward: AchievementReward::Title("Explorer".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: true,
            },
            Achievement {
                id: "konami_code".to_string(),
                name: "???".to_string(),
                description: "Enter the Konami code".to_string(),
                category: AchievementCategory::Secret,
                difficulty: AchievementDifficulty::Medium,
                requirement: AchievementRequirement::Multiple(vec![]),
                reward: AchievementReward::Skin("Retro Skin".to_string()),
                progress: 0.0,
                max_progress: 1.0,
                unlocked: false,
                unlock_timestamp: None,
                hidden: true,
            },
        ];

        for achievement in achievements {
            self.achievements.insert(achievement.id.clone(), achievement);
        }
    }

    /// Update achievement progress
    pub fn update_progress(&mut self, id: &str, progress: f32) -> Option<AchievementReward> {
        if let Some(achievement) = self.achievements.get_mut(id) {
            if achievement.unlocked {
                return None;
            }

            achievement.progress = achievement.progress.max(progress);

            if achievement.progress >= achievement.max_progress {
                achievement.unlocked = true;
                achievement.unlock_timestamp = Some(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs());
                self.unlocked_count += 1;
                return Some(achievement.reward.clone());
            }
        }

        None
    }

    /// Get achievement by ID
    pub fn get(&self, id: &str) -> Option<&Achievement> {
        self.achievements.get(id)
    }

    /// Get all achievements
    pub fn get_all(&self) -> Vec<&Achievement> {
        self.achievements.values().collect()
    }

    /// Get achievements by category
    pub fn get_by_category(&self, category: AchievementCategory) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.category == category)
            .collect()
    }

    /// Get unlocked achievements
    pub fn get_unlocked(&self) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.unlocked)
            .collect()
    }

    /// Get completion percentage
    pub fn get_completion_percent(&self) -> f32 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.unlocked_count as f32 / self.total_count as f32) * 100.0
        }
    }
}

impl AchievementCategory {
    pub fn to_string(&self) -> &str {
        match self {
            AchievementCategory::Combat => "Combat",
            AchievementCategory::Progression => "Progression",
            AchievementCategory::Exploration => "Exploration",
            AchievementCategory::Mastery => "Mastery",
            AchievementCategory::Social => "Social",
            AchievementCategory::Challenge => "Challenge",
            AchievementCategory::Collection => "Collection",
            AchievementCategory::Secret => "Secret",
        }
    }
}

impl AchievementDifficulty {
    pub fn to_color(&self) -> macroquad::prelude::Color {
        use macroquad::prelude::Color;
        match self {
            AchievementDifficulty::Easy => Color::new(0.5, 0.8, 0.5, 1.0), // Green
            AchievementDifficulty::Medium => Color::new(0.5, 0.7, 1.0, 1.0), // Blue
            AchievementDifficulty::Hard => Color::new(0.9, 0.5, 1.0, 1.0), // Purple
            AchievementDifficulty::Expert => Color::new(1.0, 0.5, 0.0, 1.0), // Orange
            AchievementDifficulty::Legendary => Color::new(1.0, 0.8, 0.0, 1.0), // Gold
        }
    }
}
