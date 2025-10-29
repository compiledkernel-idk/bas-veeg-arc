use serde::{Deserialize, Serialize};

/// Account-wide progression system
#[derive(Clone, Serialize, Deserialize)]
pub struct AccountProgression {
    pub level: u32,
    pub xp: f32,
    pub total_games_played: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub total_damage_dealt: f64,
    pub total_kills: u64,
    pub total_deaths: u64,
    pub total_playtime_seconds: u64,
    pub lifetime_currency_earned: f64,
    pub profile_title: String,
    pub profile_banner: String,
    pub profile_icon: String,
    pub level_rewards_claimed: Vec<u32>,
}

/// Prestige system for account resets with permanent bonuses
#[derive(Clone, Serialize, Deserialize)]
pub struct PrestigeSystem {
    pub prestige_level: u32,
    pub total_prestiges: u32,
    pub prestige_currency: f32, // Special currency earned from prestiging
    pub permanent_bonuses: PrestigeBonuses,
    pub prestige_rewards_unlocked: Vec<PrestigeReward>,
}

/// Permanent bonuses from prestige
#[derive(Clone, Serialize, Deserialize)]
pub struct PrestigeBonuses {
    pub xp_multiplier: f32,
    pub currency_multiplier: f32,
    pub damage_bonus: f32,
    pub health_bonus: f32,
    pub start_with_currency: f32,
    pub extra_skill_points: u32,
    pub unlock_modifiers: Vec<String>,
}

/// Prestige rewards
#[derive(Clone, Serialize, Deserialize)]
pub struct PrestigeReward {
    pub prestige_level: u32,
    pub name: String,
    pub description: String,
    pub reward_type: PrestigeRewardType,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PrestigeRewardType {
    XPMultiplier(f32),
    CurrencyMultiplier(f32),
    DamageBonus(f32),
    HealthBonus(f32),
    StartingCurrency(f32),
    SkillPoints(u32),
    UnlockCharacter(String),
    UnlockModifier(String),
    UnlockSkin(String),
}

impl AccountProgression {
    pub fn new() -> Self {
        Self {
            level: 1,
            xp: 0.0,
            total_games_played: 0,
            total_wins: 0,
            total_losses: 0,
            total_damage_dealt: 0.0,
            total_kills: 0,
            total_deaths: 0,
            total_playtime_seconds: 0,
            lifetime_currency_earned: 0.0,
            profile_title: "Newcomer".to_string(),
            profile_banner: "Default".to_string(),
            profile_icon: "Default".to_string(),
            level_rewards_claimed: vec![],
        }
    }

    /// Add XP and handle level ups
    pub fn add_xp(&mut self, xp: f32) -> Vec<u32> {
        self.xp += xp;

        let mut levels_gained = Vec::new();

        while self.xp >= self.xp_for_next_level() {
            self.xp -= self.xp_for_next_level();
            self.level += 1;
            levels_gained.push(self.level);
        }

        levels_gained
    }

    /// Calculate XP required for next level
    pub fn xp_for_next_level(&self) -> f32 {
        // Exponential scaling
        150.0 * (self.level as f32).powf(1.6)
    }

    /// Record a completed game
    pub fn record_game(&mut self, won: bool, xp_earned: f32, currency_earned: f32) -> Vec<u32> {
        self.total_games_played += 1;

        if won {
            self.total_wins += 1;
        } else {
            self.total_losses += 1;
        }

        self.lifetime_currency_earned += currency_earned as f64;

        self.add_xp(xp_earned)
    }

    /// Record playtime
    pub fn add_playtime(&mut self, seconds: u64) {
        self.total_playtime_seconds += seconds;
    }

    /// Get win rate
    pub fn get_win_rate(&self) -> f32 {
        if self.total_games_played == 0 {
            0.0
        } else {
            (self.total_wins as f32 / self.total_games_played as f32) * 100.0
        }
    }

    /// Get K/D ratio
    pub fn get_kd_ratio(&self) -> f32 {
        if self.total_deaths == 0 {
            self.total_kills as f32
        } else {
            self.total_kills as f32 / self.total_deaths as f32
        }
    }

    /// Get playtime in hours
    pub fn get_playtime_hours(&self) -> f32 {
        self.total_playtime_seconds as f32 / 3600.0
    }

    /// Get level progress as percentage
    pub fn get_progress_percent(&self) -> f32 {
        (self.xp / self.xp_for_next_level()) * 100.0
    }

    /// Claim level reward
    pub fn claim_level_reward(&mut self, level: u32) -> Option<LevelReward> {
        if self.level_rewards_claimed.contains(&level) || level > self.level {
            return None;
        }

        self.level_rewards_claimed.push(level);
        Some(Self::get_level_reward(level))
    }

    /// Get reward for a specific level
    fn get_level_reward(level: u32) -> LevelReward {
        match level {
            // Every 5 levels
            5 | 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50 => LevelReward {
                level,
                currency: 100.0 * (level as f32 / 5.0),
                skill_points: 1,
                cosmetic: None,
            },
            // Every 10 levels - extra rewards
            60 | 70 | 80 | 90 => LevelReward {
                level,
                currency: 500.0,
                skill_points: 2,
                cosmetic: Some(format!("Banner Lvl {}", level)),
            },
            // Milestone level 100
            100 => LevelReward {
                level,
                currency: 2000.0,
                skill_points: 5,
                cosmetic: Some("Master Banner".to_string()),
            },
            // Regular milestones every 25 after 100
            _ if level >= 100 && level % 25 == 0 => LevelReward {
                level,
                currency: 1000.0,
                skill_points: 3,
                cosmetic: Some(format!("Legendary Banner {}", level)),
            },
            // Default
            _ => LevelReward {
                level,
                currency: 50.0,
                skill_points: 0,
                cosmetic: None,
            },
        }
    }

    /// Get all pending rewards
    pub fn get_pending_rewards(&self) -> Vec<LevelReward> {
        (1..=self.level)
            .filter(|l| !self.level_rewards_claimed.contains(l))
            .map(|l| Self::get_level_reward(l))
            .collect()
    }
}

/// Reward for reaching an account level
pub struct LevelReward {
    pub level: u32,
    pub currency: f32,
    pub skill_points: u32,
    pub cosmetic: Option<String>,
}

impl PrestigeSystem {
    pub fn new() -> Self {
        Self {
            prestige_level: 0,
            total_prestiges: 0,
            prestige_currency: 0.0,
            permanent_bonuses: PrestigeBonuses::default(),
            prestige_rewards_unlocked: vec![],
        }
    }

    /// Check if eligible to prestige
    pub fn can_prestige(&self, account_level: u32) -> bool {
        account_level >= 100
    }

    /// Perform prestige (resets account level but grants bonuses)
    pub fn prestige(&mut self, account_progression: &mut AccountProgression) -> PrestigeResult {
        if !self.can_prestige(account_progression.level) {
            return PrestigeResult::NotEligible;
        }

        self.prestige_level += 1;
        self.total_prestiges += 1;

        // Grant prestige currency
        let currency_gained = 100.0 * self.prestige_level as f32;
        self.prestige_currency += currency_gained;

        // Reset account level
        account_progression.level = 1;
        account_progression.xp = 0.0;
        account_progression.level_rewards_claimed.clear();

        // Apply new prestige bonuses
        self.apply_prestige_bonuses();

        // Check for prestige rewards
        let reward = self.check_prestige_rewards();

        PrestigeResult::Success {
            prestige_level: self.prestige_level,
            currency_gained,
            reward,
        }
    }

    /// Apply bonuses based on prestige level
    fn apply_prestige_bonuses(&mut self) {
        let level = self.prestige_level;

        // XP multiplier increases by 5% per prestige
        self.permanent_bonuses.xp_multiplier = 1.0 + (level as f32 * 0.05);

        // Currency multiplier increases by 10% per prestige
        self.permanent_bonuses.currency_multiplier = 1.0 + (level as f32 * 0.10);

        // Damage bonus: 2% per prestige
        self.permanent_bonuses.damage_bonus = level as f32 * 0.02;

        // Health bonus: 3% per prestige
        self.permanent_bonuses.health_bonus = level as f32 * 0.03;

        // Starting currency: 100 per prestige
        self.permanent_bonuses.start_with_currency = level as f32 * 100.0;

        // Extra skill points: 1 per 5 prestiges
        self.permanent_bonuses.extra_skill_points = level / 5;
    }

    /// Check for prestige milestone rewards
    fn check_prestige_rewards(&mut self) -> Option<PrestigeReward> {
        let level = self.prestige_level;

        let reward = match level {
            1 => Some(PrestigeReward {
                prestige_level: 1,
                name: "First Prestige".to_string(),
                description: "You've ascended for the first time".to_string(),
                reward_type: PrestigeRewardType::UnlockModifier("Prestige Aura".to_string()),
            }),
            5 => Some(PrestigeReward {
                prestige_level: 5,
                name: "Prestige Veteran".to_string(),
                description: "Five times and counting".to_string(),
                reward_type: PrestigeRewardType::UnlockSkin("Prestige Skin Tier 1".to_string()),
            }),
            10 => Some(PrestigeReward {
                prestige_level: 10,
                name: "Prestige Master".to_string(),
                description: "A true master of the grind".to_string(),
                reward_type: PrestigeRewardType::UnlockSkin("Prestige Skin Tier 2".to_string()),
            }),
            25 => Some(PrestigeReward {
                prestige_level: 25,
                name: "Prestige Legend".to_string(),
                description: "Legendary dedication".to_string(),
                reward_type: PrestigeRewardType::UnlockSkin("Prestige Skin Tier 3".to_string()),
            }),
            50 => Some(PrestigeReward {
                prestige_level: 50,
                name: "Prestige God".to_string(),
                description: "Unmatched commitment".to_string(),
                reward_type: PrestigeRewardType::UnlockCharacter("Secret Character".to_string()),
            }),
            _ => None,
        };

        if let Some(ref r) = reward {
            self.prestige_rewards_unlocked.push(r.clone());
        }

        reward
    }

    /// Spend prestige currency on permanent upgrades
    pub fn buy_prestige_upgrade(&mut self, upgrade: PrestigeUpgrade) -> bool {
        let cost = upgrade.cost();

        if self.prestige_currency < cost {
            return false;
        }

        self.prestige_currency -= cost;
        upgrade.apply(&mut self.permanent_bonuses);

        true
    }

    /// Get all available prestige upgrades
    pub fn get_available_upgrades(&self) -> Vec<PrestigeUpgrade> {
        vec![
            PrestigeUpgrade::XPBoost,
            PrestigeUpgrade::CurrencyBoost,
            PrestigeUpgrade::DamageBoost,
            PrestigeUpgrade::HealthBoost,
            PrestigeUpgrade::StartingCurrency,
            PrestigeUpgrade::SkillPoint,
        ]
    }
}

/// Result of a prestige attempt
pub enum PrestigeResult {
    Success {
        prestige_level: u32,
        currency_gained: f32,
        reward: Option<PrestigeReward>,
    },
    NotEligible,
}

impl Default for PrestigeBonuses {
    fn default() -> Self {
        Self {
            xp_multiplier: 1.0,
            currency_multiplier: 1.0,
            damage_bonus: 0.0,
            health_bonus: 0.0,
            start_with_currency: 0.0,
            extra_skill_points: 0,
            unlock_modifiers: vec![],
        }
    }
}

/// Upgrades purchasable with prestige currency
#[derive(Clone)]
pub enum PrestigeUpgrade {
    XPBoost,         // +5% XP gain
    CurrencyBoost,   // +10% currency gain
    DamageBoost,     // +2% damage
    HealthBoost,     // +3% health
    StartingCurrency, // +100 starting currency
    SkillPoint,      // +1 skill point for all characters
}

impl PrestigeUpgrade {
    pub fn cost(&self) -> f32 {
        match self {
            PrestigeUpgrade::XPBoost => 50.0,
            PrestigeUpgrade::CurrencyBoost => 75.0,
            PrestigeUpgrade::DamageBoost => 100.0,
            PrestigeUpgrade::HealthBoost => 100.0,
            PrestigeUpgrade::StartingCurrency => 50.0,
            PrestigeUpgrade::SkillPoint => 200.0,
        }
    }

    pub fn apply(&self, bonuses: &mut PrestigeBonuses) {
        match self {
            PrestigeUpgrade::XPBoost => bonuses.xp_multiplier += 0.05,
            PrestigeUpgrade::CurrencyBoost => bonuses.currency_multiplier += 0.10,
            PrestigeUpgrade::DamageBoost => bonuses.damage_bonus += 0.02,
            PrestigeUpgrade::HealthBoost => bonuses.health_bonus += 0.03,
            PrestigeUpgrade::StartingCurrency => bonuses.start_with_currency += 100.0,
            PrestigeUpgrade::SkillPoint => bonuses.extra_skill_points += 1,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            PrestigeUpgrade::XPBoost => "XP Boost",
            PrestigeUpgrade::CurrencyBoost => "Currency Boost",
            PrestigeUpgrade::DamageBoost => "Damage Boost",
            PrestigeUpgrade::HealthBoost => "Health Boost",
            PrestigeUpgrade::StartingCurrency => "Starting Currency",
            PrestigeUpgrade::SkillPoint => "Skill Point",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PrestigeUpgrade::XPBoost => "Increase XP gain by 5% (stacks)",
            PrestigeUpgrade::CurrencyBoost => "Increase currency gain by 10% (stacks)",
            PrestigeUpgrade::DamageBoost => "Increase base damage by 2% (stacks)",
            PrestigeUpgrade::HealthBoost => "Increase base health by 3% (stacks)",
            PrestigeUpgrade::StartingCurrency => "Start each run with +100 currency (stacks)",
            PrestigeUpgrade::SkillPoint => "Gain +1 skill point for all characters (stacks)",
        }
    }
}
