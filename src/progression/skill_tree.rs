use crate::data::characters::CharacterId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages skill trees for all characters
pub struct SkillTreeManager {
    trees: HashMap<CharacterId, SkillTree>,
    skill_points: HashMap<CharacterId, u32>,
}

/// Skill tree for a character with 3 specialization branches
#[derive(Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub character: CharacterId,
    pub branch_offense: SkillBranch,
    pub branch_defense: SkillBranch,
    pub branch_utility: SkillBranch,
}

/// A branch of skills in a skill tree
#[derive(Clone, Serialize, Deserialize)]
pub struct SkillBranch {
    pub name: String,
    pub description: String,
    pub nodes: Vec<SkillNode>,
}

/// Individual skill node in the tree
#[derive(Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: u32,  // 1-5, higher tiers require more points in branch
    pub max_level: u32,
    pub current_level: u32,
    pub skill_point_cost: u32,
    pub prerequisites: Vec<String>, // IDs of required skills
    pub skill_type: SkillType,
    pub unlocked: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum SkillType {
    // Offense
    DamageIncrease(f32),          // % damage increase per level
    CriticalChance(f32),          // % crit chance per level
    CriticalDamage(f32),          // % crit damage multiplier per level
    AttackSpeed(f32),             // % attack speed increase per level
    ComboExtension(u32),          // Extra hits in combo chains
    AreaOfEffect(f32),            // % AoE radius increase
    Penetration(f32),             // Armor penetration %

    // Defense
    MaxHealthIncrease(f32),       // % max health per level
    DamageReduction(f32),         // % damage reduction per level
    DodgeChance(f32),             // % dodge chance per level
    BlockEfficiency(f32),         // % block efficiency per level
    HealthRegen(f32),             // HP/sec regeneration
    Thorns(f32),                  // % damage reflected
    Resistance(f32),              // % resistance to effects

    // Utility
    MovementSpeed(f32),           // % movement speed per level
    CooldownReduction(f32),       // % cooldown reduction per level
    AbilityDuration(f32),         // % ability duration increase
    ResourceGain(f32),            // % currency/XP gain increase
    Lifesteal(f32),               // % lifesteal per level
    Multicast(f32),               // % chance to cast ability twice
    LuckBonus(f32),               // % better loot/rewards

    // Special/Unique
    UnlockAbility(String),        // Unlock new ability
    PassiveBuff(String, f32),     // Passive buff with value
    SynergyBoost(String, f32),    // Boost to specific mechanics
}

impl SkillTreeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            trees: HashMap::new(),
            skill_points: HashMap::new(),
        };

        // Initialize skill trees for all characters
        manager.init_all_character_trees();

        manager
    }

    /// Initialize skill trees for all characters
    fn init_all_character_trees(&mut self) {
        // Create tree for Bas
        self.trees.insert(CharacterId::Bas, Self::create_bas_tree());

        // Create tree for Berkay
        self.trees.insert(CharacterId::Berkay, Self::create_berkay_tree());

        // Create tree for Luca
        self.trees.insert(CharacterId::Luca, Self::create_luca_tree());

        // Add more characters as needed
        // self.trees.insert(CharacterId::Gefferinho, Self::create_gefferinho_tree());
        // etc.

        // Initialize skill points (0 for all characters)
        for character in self.trees.keys() {
            self.skill_points.insert(*character, 0);
        }
    }

    /// Create Bas's skill tree
    fn create_bas_tree() -> SkillTree {
        SkillTree {
            character: CharacterId::Bas,
            branch_offense: SkillBranch {
                name: "Veeg Master".to_string(),
                description: "Master the art of the veeg with devastating sweeping attacks".to_string(),
                nodes: vec![
                    // Tier 1
                    SkillNode {
                        id: "bas_off_1".to_string(),
                        name: "Sweeping Strike".to_string(),
                        description: "Increase damage by 5% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::DamageIncrease(5.0),
                        unlocked: true,
                    },
                    SkillNode {
                        id: "bas_off_2".to_string(),
                        name: "Precision Sweeping".to_string(),
                        description: "Gain 2% critical hit chance per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::CriticalChance(2.0),
                        unlocked: true,
                    },
                    // Tier 2
                    SkillNode {
                        id: "bas_off_3".to_string(),
                        name: "Veeg Fury".to_string(),
                        description: "Increase attack speed by 8% per level".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_off_1".to_string()],
                        skill_type: SkillType::AttackSpeed(8.0),
                        unlocked: false,
                    },
                    SkillNode {
                        id: "bas_off_4".to_string(),
                        name: "Splash Damage".to_string(),
                        description: "Increase AoE radius by 15% per level (enhances Bas Veeg ability)".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_off_1".to_string()],
                        skill_type: SkillType::AreaOfEffect(15.0),
                        unlocked: false,
                    },
                    // Tier 3
                    SkillNode {
                        id: "bas_off_5".to_string(),
                        name: "Brutal Cleaning".to_string(),
                        description: "Critical hits deal 20% more damage per level".to_string(),
                        tier: 3,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 3,
                        prerequisites: vec!["bas_off_2".to_string(), "bas_off_3".to_string()],
                        skill_type: SkillType::CriticalDamage(20.0),
                        unlocked: false,
                    },
                    // Tier 4
                    SkillNode {
                        id: "bas_off_6".to_string(),
                        name: "Chain Sweeping".to_string(),
                        description: "Add 1 extra hit to combo chains per level".to_string(),
                        tier: 4,
                        max_level: 2,
                        current_level: 0,
                        skill_point_cost: 4,
                        prerequisites: vec!["bas_off_5".to_string()],
                        skill_type: SkillType::ComboExtension(1),
                        unlocked: false,
                    },
                    // Tier 5 - Ultimate
                    SkillNode {
                        id: "bas_off_7".to_string(),
                        name: "Ultimate Veeg".to_string(),
                        description: "Bas Veeg ability now creates a vortex that pulls enemies".to_string(),
                        tier: 5,
                        max_level: 1,
                        current_level: 0,
                        skill_point_cost: 5,
                        prerequisites: vec!["bas_off_6".to_string()],
                        skill_type: SkillType::UnlockAbility("VeegVortex".to_string()),
                        unlocked: false,
                    },
                ],
            },
            branch_defense: SkillBranch {
                name: "Janitor's Fortitude".to_string(),
                description: "Become as sturdy as the school buildings you clean".to_string(),
                nodes: vec![
                    // Tier 1
                    SkillNode {
                        id: "bas_def_1".to_string(),
                        name: "Sturdy Apron".to_string(),
                        description: "Increase max health by 8% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MaxHealthIncrease(8.0),
                        unlocked: true,
                    },
                    SkillNode {
                        id: "bas_def_2".to_string(),
                        name: "Tough Cleaning".to_string(),
                        description: "Reduce damage taken by 3% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::DamageReduction(3.0),
                        unlocked: true,
                    },
                    // Tier 2
                    SkillNode {
                        id: "bas_def_3".to_string(),
                        name: "Nimble Janitor".to_string(),
                        description: "Gain 5% dodge chance per level".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_def_1".to_string()],
                        skill_type: SkillType::DodgeChance(5.0),
                        unlocked: false,
                    },
                    SkillNode {
                        id: "bas_def_4".to_string(),
                        name: "Regeneration".to_string(),
                        description: "Regenerate 2 HP per second per level".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_def_1".to_string()],
                        skill_type: SkillType::HealthRegen(2.0),
                        unlocked: false,
                    },
                    // Tier 3
                    SkillNode {
                        id: "bas_def_5".to_string(),
                        name: "Slippery Floor".to_string(),
                        description: "Attackers have a chance to slip, taking 10% of damage dealt per level".to_string(),
                        tier: 3,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 3,
                        prerequisites: vec!["bas_def_2".to_string()],
                        skill_type: SkillType::Thorns(10.0),
                        unlocked: false,
                    },
                    // Tier 4
                    SkillNode {
                        id: "bas_def_6".to_string(),
                        name: "Unyielding".to_string(),
                        description: "Gain 10% resistance to stun and knockback per level".to_string(),
                        tier: 4,
                        max_level: 2,
                        current_level: 0,
                        skill_point_cost: 4,
                        prerequisites: vec!["bas_def_5".to_string()],
                        skill_type: SkillType::Resistance(10.0),
                        unlocked: false,
                    },
                    // Tier 5 - Ultimate
                    SkillNode {
                        id: "bas_def_7".to_string(),
                        name: "Janitor's Last Stand".to_string(),
                        description: "When health drops below 25%, gain 50% damage reduction for 5 seconds (60s cooldown)".to_string(),
                        tier: 5,
                        max_level: 1,
                        current_level: 0,
                        skill_point_cost: 5,
                        prerequisites: vec!["bas_def_6".to_string()],
                        skill_type: SkillType::UnlockAbility("LastStand".to_string()),
                        unlocked: false,
                    },
                ],
            },
            branch_utility: SkillBranch {
                name: "Efficient Worker".to_string(),
                description: "Work smarter, not harder - utility and support skills".to_string(),
                nodes: vec![
                    // Tier 1
                    SkillNode {
                        id: "bas_util_1".to_string(),
                        name: "Quick Pace".to_string(),
                        description: "Increase movement speed by 5% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MovementSpeed(5.0),
                        unlocked: true,
                    },
                    SkillNode {
                        id: "bas_util_2".to_string(),
                        name: "Efficient Cleaning".to_string(),
                        description: "Reduce ability cooldowns by 5% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::CooldownReduction(5.0),
                        unlocked: true,
                    },
                    // Tier 2
                    SkillNode {
                        id: "bas_util_3".to_string(),
                        name: "Extended Effect".to_string(),
                        description: "Increase ability duration by 10% per level".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_util_2".to_string()],
                        skill_type: SkillType::AbilityDuration(10.0),
                        unlocked: false,
                    },
                    SkillNode {
                        id: "bas_util_4".to_string(),
                        name: "Hard Worker".to_string(),
                        description: "Gain 10% more currency and XP per level".to_string(),
                        tier: 2,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 2,
                        prerequisites: vec!["bas_util_1".to_string()],
                        skill_type: SkillType::ResourceGain(10.0),
                        unlocked: false,
                    },
                    // Tier 3
                    SkillNode {
                        id: "bas_util_5".to_string(),
                        name: "Life Drain".to_string(),
                        description: "Gain 5% lifesteal per level".to_string(),
                        tier: 3,
                        max_level: 3,
                        current_level: 0,
                        skill_point_cost: 3,
                        prerequisites: vec!["bas_util_3".to_string()],
                        skill_type: SkillType::Lifesteal(5.0),
                        unlocked: false,
                    },
                    // Tier 4
                    SkillNode {
                        id: "bas_util_6".to_string(),
                        name: "Lucky Break".to_string(),
                        description: "Increase luck by 15% per level (better drops, rare events)".to_string(),
                        tier: 4,
                        max_level: 2,
                        current_level: 0,
                        skill_point_cost: 4,
                        prerequisites: vec!["bas_util_4".to_string()],
                        skill_type: SkillType::LuckBonus(15.0),
                        unlocked: false,
                    },
                    // Tier 5 - Ultimate
                    SkillNode {
                        id: "bas_util_7".to_string(),
                        name: "Double Time".to_string(),
                        description: "10% chance to cast abilities twice".to_string(),
                        tier: 5,
                        max_level: 1,
                        current_level: 0,
                        skill_point_cost: 5,
                        prerequisites: vec!["bas_util_5".to_string(), "bas_util_6".to_string()],
                        skill_type: SkillType::Multicast(10.0),
                        unlocked: false,
                    },
                ],
            },
        }
    }

    /// Create Berkay's skill tree
    fn create_berkay_tree() -> SkillTree {
        SkillTree {
            character: CharacterId::Berkay,
            branch_offense: SkillBranch {
                name: "Kebab Connoisseur".to_string(),
                description: "Harness the power of the special kebab".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "berkay_off_1".to_string(),
                        name: "Spicy Strikes".to_string(),
                        description: "Increase damage by 6% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::DamageIncrease(6.0),
                        unlocked: true,
                    },
                    // Add more nodes for Berkay...
                ],
            },
            branch_defense: SkillBranch {
                name: "Meat Shield".to_string(),
                description: "As filling as a good kebab".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "berkay_def_1".to_string(),
                        name: "Kebab Armor".to_string(),
                        description: "Increase max health by 10% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MaxHealthIncrease(10.0),
                        unlocked: true,
                    },
                    // Add more nodes...
                ],
            },
            branch_utility: SkillBranch {
                name: "Turkish Efficiency".to_string(),
                description: "Work with the speed and precision of a kebab chef".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "berkay_util_1".to_string(),
                        name: "Swift Service".to_string(),
                        description: "Increase movement speed by 6% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MovementSpeed(6.0),
                        unlocked: true,
                    },
                    // Add more nodes...
                ],
            },
        }
    }

    /// Create Luca's skill tree
    fn create_luca_tree() -> SkillTree {
        SkillTree {
            character: CharacterId::Luca,
            branch_offense: SkillBranch {
                name: "Winter Arc Warrior".to_string(),
                description: "Embrace the cold and unleash devastating power".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "luca_off_1".to_string(),
                        name: "Frozen Fury".to_string(),
                        description: "Increase damage by 7% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::DamageIncrease(7.0),
                        unlocked: true,
                    },
                    // Add more nodes...
                ],
            },
            branch_defense: SkillBranch {
                name: "Ice Armor".to_string(),
                description: "Cold as ice, hard to break".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "luca_def_1".to_string(),
                        name: "Frostbite Resistance".to_string(),
                        description: "Increase max health by 8% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MaxHealthIncrease(8.0),
                        unlocked: true,
                    },
                    // Add more nodes...
                ],
            },
            branch_utility: SkillBranch {
                name: "Arctic Agility".to_string(),
                description: "Move with the speed of a winter wind".to_string(),
                nodes: vec![
                    SkillNode {
                        id: "luca_util_1".to_string(),
                        name: "Blizzard Sprint".to_string(),
                        description: "Increase movement speed by 7% per level".to_string(),
                        tier: 1,
                        max_level: 5,
                        current_level: 0,
                        skill_point_cost: 1,
                        prerequisites: vec![],
                        skill_type: SkillType::MovementSpeed(7.0),
                        unlocked: true,
                    },
                    // Add more nodes...
                ],
            },
        }
    }

    /// Get skill tree for character
    pub fn get_tree(&self, character: CharacterId) -> Option<&SkillTree> {
        self.trees.get(&character)
    }

    /// Get mutable skill tree for character
    pub fn get_tree_mut(&mut self, character: CharacterId) -> Option<&mut SkillTree> {
        self.trees.get_mut(&character)
    }

    /// Unlock a skill node
    pub fn unlock_skill(
        &mut self,
        character: CharacterId,
        skill_id: &str,
    ) -> Result<(), String> {
        let available_points = self.skill_points.get(&character).copied().unwrap_or(0);

        if let Some(tree) = self.trees.get_mut(&character) {
            // Find the skill across all branches
            let skill = tree
                .find_skill_mut(skill_id)
                .ok_or_else(|| format!("Skill {} not found", skill_id))?;

            // Check if already maxed
            if skill.current_level >= skill.max_level {
                return Err("Skill already at max level".to_string());
            }

            // Clone prerequisites to avoid borrow issues
            let prerequisites = skill.prerequisites.clone();
            let skill_point_cost = skill.skill_point_cost;

            // Check if prerequisites are met
            for prereq_id in &prerequisites {
                if let Some(prereq) = tree.find_skill(prereq_id) {
                    if prereq.current_level == 0 {
                        return Err(format!("Prerequisite {} not unlocked", prereq_id));
                    }
                }
            }

            // Check if player has enough skill points
            if available_points < skill_point_cost {
                return Err("Not enough skill points".to_string());
            }

            // Get the skill again mutably
            let skill = tree.find_skill_mut(skill_id).unwrap();

            // Unlock the skill
            skill.current_level += 1;
            skill.unlocked = true;

            // Deduct skill points
            if let Some(points) = self.skill_points.get_mut(&character) {
                *points -= skill_point_cost;
            }

            Ok(())
        } else {
            Err(format!("No skill tree found for character {:?}", character))
        }
    }

    /// Add skill points for a character
    pub fn add_skill_points(&mut self, character: CharacterId, points: u32) {
        *self.skill_points.entry(character).or_insert(0) += points;
    }

    /// Get available skill points for a character
    pub fn get_skill_points(&self, character: CharacterId) -> u32 {
        self.skill_points.get(&character).copied().unwrap_or(0)
    }

    /// Reset skill tree for a character (costs currency)
    pub fn reset_tree(&mut self, character: CharacterId) -> u32 {
        let mut refunded_points = 0;

        if let Some(tree) = self.trees.get_mut(&character) {
            refunded_points += tree.reset();
        }

        if let Some(points) = self.skill_points.get_mut(&character) {
            *points += refunded_points;
        }

        refunded_points
    }

    /// Calculate total stat bonuses from skill tree
    pub fn calculate_bonuses(&self, character: CharacterId) -> SkillBonuses {
        let mut bonuses = SkillBonuses::default();

        if let Some(tree) = self.trees.get(&character) {
            bonuses = tree.calculate_bonuses();
        }

        bonuses
    }
}

impl SkillTree {
    /// Find a skill by ID across all branches
    fn find_skill(&self, skill_id: &str) -> Option<&SkillNode> {
        self.branch_offense
            .nodes
            .iter()
            .chain(&self.branch_defense.nodes)
            .chain(&self.branch_utility.nodes)
            .find(|node| node.id == skill_id)
    }

    /// Find a skill by ID (mutable)
    fn find_skill_mut(&mut self, skill_id: &str) -> Option<&mut SkillNode> {
        if let Some(node) = self.branch_offense.nodes.iter_mut().find(|n| n.id == skill_id) {
            return Some(node);
        }
        if let Some(node) = self.branch_defense.nodes.iter_mut().find(|n| n.id == skill_id) {
            return Some(node);
        }
        if let Some(node) = self.branch_utility.nodes.iter_mut().find(|n| n.id == skill_id) {
            return Some(node);
        }
        None
    }

    /// Reset all skills in the tree
    fn reset(&mut self) -> u32 {
        let mut refunded_points = 0;

        for branch in [&mut self.branch_offense, &mut self.branch_defense, &mut self.branch_utility] {
            for node in &mut branch.nodes {
                refunded_points += node.skill_point_cost * node.current_level;
                node.current_level = 0;
                if node.tier > 1 {
                    node.unlocked = false;
                }
            }
        }

        refunded_points
    }

    /// Calculate all bonuses from unlocked skills
    fn calculate_bonuses(&self) -> SkillBonuses {
        let mut bonuses = SkillBonuses::default();

        for branch in [&self.branch_offense, &self.branch_defense, &self.branch_utility] {
            for node in &branch.nodes {
                if node.current_level > 0 {
                    bonuses.apply_skill(&node.skill_type, node.current_level);
                }
            }
        }

        bonuses
    }
}

/// Accumulated bonuses from skill tree
#[derive(Default, Clone)]
pub struct SkillBonuses {
    pub damage_multiplier: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub attack_speed: f32,
    pub combo_extension: u32,
    pub aoe_radius: f32,
    pub penetration: f32,

    pub max_health_multiplier: f32,
    pub damage_reduction: f32,
    pub dodge_chance: f32,
    pub block_efficiency: f32,
    pub health_regen: f32,
    pub thorns: f32,
    pub resistance: f32,

    pub movement_speed: f32,
    pub cooldown_reduction: f32,
    pub ability_duration: f32,
    pub resource_gain: f32,
    pub lifesteal: f32,
    pub multicast_chance: f32,
    pub luck_bonus: f32,

    pub unlocked_abilities: Vec<String>,
}

impl SkillBonuses {
    fn apply_skill(&mut self, skill_type: &SkillType, level: u32) {
        let level_f = level as f32;

        match skill_type {
            SkillType::DamageIncrease(v) => self.damage_multiplier += v * level_f / 100.0,
            SkillType::CriticalChance(v) => self.crit_chance += v * level_f,
            SkillType::CriticalDamage(v) => self.crit_damage += v * level_f / 100.0,
            SkillType::AttackSpeed(v) => self.attack_speed += v * level_f / 100.0,
            SkillType::ComboExtension(v) => self.combo_extension += v * level,
            SkillType::AreaOfEffect(v) => self.aoe_radius += v * level_f / 100.0,
            SkillType::Penetration(v) => self.penetration += v * level_f,

            SkillType::MaxHealthIncrease(v) => self.max_health_multiplier += v * level_f / 100.0,
            SkillType::DamageReduction(v) => self.damage_reduction += v * level_f,
            SkillType::DodgeChance(v) => self.dodge_chance += v * level_f,
            SkillType::BlockEfficiency(v) => self.block_efficiency += v * level_f / 100.0,
            SkillType::HealthRegen(v) => self.health_regen += v * level_f,
            SkillType::Thorns(v) => self.thorns += v * level_f / 100.0,
            SkillType::Resistance(v) => self.resistance += v * level_f,

            SkillType::MovementSpeed(v) => self.movement_speed += v * level_f / 100.0,
            SkillType::CooldownReduction(v) => self.cooldown_reduction += v * level_f,
            SkillType::AbilityDuration(v) => self.ability_duration += v * level_f / 100.0,
            SkillType::ResourceGain(v) => self.resource_gain += v * level_f / 100.0,
            SkillType::Lifesteal(v) => self.lifesteal += v * level_f,
            SkillType::Multicast(v) => self.multicast_chance += v * level_f,
            SkillType::LuckBonus(v) => self.luck_bonus += v * level_f / 100.0,

            SkillType::UnlockAbility(ability) => {
                if !self.unlocked_abilities.contains(ability) {
                    self.unlocked_abilities.push(ability.clone());
                }
            }
            _ => {}
        }
    }
}
