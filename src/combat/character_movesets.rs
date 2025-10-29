use macroquad::prelude::*;
use crate::ecs::CharacterType;
use crate::combat::plane_system::{PlaneSystem, BombPattern};

/// Complete character stats including all combat parameters
#[derive(Clone, Debug)]
pub struct CharacterStats {
    pub character_type: CharacterType,
    pub max_health: f32,
    pub base_damage: f32,
    pub base_speed: f32,
    pub jump_force: f32,
    pub weight: f32,           // Affects knockback and fall speed
    pub air_mobility: f32,     // Control while airborne
    pub dash_distance: f32,
    pub dash_cooldown: f32,
    pub block_strength: f32,   // Damage reduction when blocking
    pub parry_window: f32,     // Frame window for perfect parry
    pub special_traits: Vec<SpecialTrait>,
}

/// Special traits that modify character behavior
#[derive(Clone, Debug, PartialEq)]
pub enum SpecialTrait {
    HeavyArmor,              // Reduced knockback, slow movement
    LightWeight,             // Increased air mobility, more knockback
    Regeneration(f32),       // HP regen per second
    DoubleJump,              // Can jump twice
    AirDash,                 // Can dash in air
    CounterAttacks,          // Special counter move
    ProjectileAbsorb,        // Can absorb projectiles for meter
    SuperArmor,              // Can't be interrupted during certain moves
    RangedSpecialist,        // Increased projectile damage
    GrappleExpert,           // Enhanced grab moves
    ComboExtender,           // Extra juggle potential
    MeterBuilder,            // Generates meter faster
    PlaneAccess,             // Can summon plane (Keizer Bom Taha)
    CommandGrab,             // Unblockable grab moves
    Intimidation(f32),       // Reduces enemy attack speed in radius
}

/// Individual move data with frame-perfect timing
#[derive(Clone, Debug)]
pub struct MoveData {
    pub move_id: MoveId,
    pub name: &'static str,
    pub startup_frames: u32,   // Frames before hitbox becomes active
    pub active_frames: u32,    // Frames hitbox is active
    pub recovery_frames: u32,  // Frames before can act again
    pub damage: f32,
    pub hitstun_frames: u32,   // Frames enemy is stunned
    pub blockstun_frames: u32, // Frames enemy is in blockstun
    pub knockback: Vec2,       // Knockback vector
    pub hitbox_offset: Vec2,   // Hitbox position relative to character
    pub hitbox_size: Vec2,     // Hitbox dimensions
    pub can_cancel: Vec<MoveId>, // Moves this can cancel into
    pub meter_gain: f32,       // Meter gained on hit
    pub meter_cost: f32,       // Meter required to perform
    pub properties: Vec<MoveProperty>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveId {
    // Universal moves
    LightPunch,
    LightKick,
    HeavyPunch,
    HeavyKick,
    Launcher,
    AirLight,
    AirHeavy,
    Grab,

    // Keizer Bom Taha specific
    MilitaryStrike,
    BayonetThrust,
    CommanderKick,
    OrderBarrage,
    TacticalRetreat,
    AerialAssault,      // Summon plane
    BombDrop,
    StrafeRun,
    AerialSlam,

    // Principal Van Der Berg specific
    RulerSlap,
    DeskSlam,
    AuthorityStrike,
    DisciplineGrab,
    ExecutiveOrder,     // Super armor stance
    DetentionSentence,  // Command grab
    PrincipalPresence,
    OfficeDecree,

    // Lunchroom Lady Petra specific
    SoupSplash,
    TraySmash,
    LadleSwing,
    FoodThrow,
    SteamBlast,
    FoodFight,          // AoE debuff
    NutritionalValue,   // Self-heal
    ServingTime,        // Projectile barrage
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveProperty {
    Overhead,           // Must be blocked high
    Low,                // Must be blocked low
    Unblockable,        // Cannot be blocked
    Invincible,         // Invincible during move
    SuperArmor,         // Can't be interrupted
    Juggle,             // Launches opponent
    GroundBounce,       // Bounces opponent off ground
    WallBounce,         // Bounces opponent off wall
    Projectile,         // Is a projectile
    MultiHit(u32),      // Multiple hits
    ChargeMeter,        // Generates extra meter
    Slow(f32),          // Slows opponent
    Stun(f32),          // Stuns opponent
    DOT(f32, f32),      // Damage over time (dps, duration)
}

/// Complete moveset for a character
pub struct CharacterMoveset {
    pub character_type: CharacterType,
    pub stats: CharacterStats,
    pub moves: Vec<MoveData>,
    pub combo_routes: Vec<ComboRoute>,
    pub special_mechanics: CharacterMechanics,
}

/// Predefined combo routes with optimal damage
#[derive(Clone, Debug)]
pub struct ComboRoute {
    pub name: &'static str,
    pub moves: Vec<MoveId>,
    pub total_damage: f32,
    pub difficulty: ComboDifficulty,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComboDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    StyleOnly,  // Style combos that don't necessarily do most damage
}

/// Character-specific mechanics and systems
#[derive(Clone, Debug)]
pub enum CharacterMechanics {
    /// Keizer Bom Taha: Plane system with aerial combat
    PlaneSystem {
        plane_system: PlaneSystem,
        ground_combo_limit: u32,  // Max combo hits before must use plane
        aerial_damage_bonus: f32, // Damage multiplier while in plane
    },

    /// Principal Van Der Berg: Authority meter that increases with hits landed
    AuthoritySystem {
        authority_meter: f32,
        max_authority: f32,
        authority_decay_rate: f32,
        authority_bonuses: Vec<AuthorityBonus>,
    },

    /// Lunchroom Lady Petra: Food preparation system
    FoodSystem {
        prepared_food: Vec<FoodType>,
        max_food_slots: u32,
        prep_time: f32,
        current_recipe: Option<Recipe>,
    },

    /// Standard mechanics for other characters
    Standard,
}

#[derive(Clone, Debug)]
pub struct AuthorityBonus {
    pub threshold: f32,  // Authority meter threshold
    pub bonus_type: BonusType,
}

#[derive(Clone, Debug)]
pub enum BonusType {
    DamageIncrease(f32),
    SpeedIncrease(f32),
    ArmorIncrease(f32),
    UnblockableGrabs,
    InstantDetention,  // One-hit stun
}

#[derive(Clone, Debug, PartialEq)]
pub enum FoodType {
    Soup,           // DOT damage
    Salad,          // Healing
    MainCourse,     // Damage boost
    Dessert,        // Speed boost
    Mystery,        // Random effect
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub ingredients: Vec<FoodType>,
    pub effect: RecipeEffect,
}

#[derive(Clone, Debug)]
pub enum RecipeEffect {
    MassiveHealing(f32),
    MassiveDamage(f32),
    TeamBuff(BuffType),
    EnemyDebuff(DebuffType),
}

#[derive(Clone, Debug)]
pub enum BuffType {
    AttackUp(f32, f32),      // amount, duration
    DefenseUp(f32, f32),
    SpeedUp(f32, f32),
    MeterGen(f32, f32),
}

#[derive(Clone, Debug)]
pub enum DebuffType {
    AttackDown(f32, f32),
    DefenseDown(f32, f32),
    SlowDown(f32, f32),
    Poison(f32, f32),        // dps, duration
}

impl CharacterMoveset {
    /// Get moveset for Keizer Bom Taha
    pub fn keizer_bom_taha() -> Self {
        let stats = CharacterStats {
            character_type: CharacterType::KeizerBomTaha,
            max_health: 950.0,
            base_damage: 1.2,
            base_speed: 250.0,
            jump_force: 420.0,
            weight: 1.3,
            air_mobility: 0.8,
            dash_distance: 180.0,
            dash_cooldown: 0.8,
            block_strength: 0.75,
            parry_window: 0.15,
            special_traits: vec![
                SpecialTrait::PlaneAccess,
                SpecialTrait::RangedSpecialist,
                SpecialTrait::ComboExtender,
            ],
        };

        let moves = vec![
            // Ground normals
            MoveData {
                move_id: MoveId::MilitaryStrike,
                name: "Military Strike",
                startup_frames: 5,
                active_frames: 3,
                recovery_frames: 8,
                damage: 45.0,
                hitstun_frames: 12,
                blockstun_frames: 8,
                knockback: Vec2::new(20.0, 0.0),
                hitbox_offset: Vec2::new(40.0, 0.0),
                hitbox_size: Vec2::new(60.0, 80.0),
                can_cancel: vec![MoveId::BayonetThrust, MoveId::CommanderKick],
                meter_gain: 8.0,
                meter_cost: 0.0,
                properties: vec![],
            },
            MoveData {
                move_id: MoveId::BayonetThrust,
                name: "Bayonet Thrust",
                startup_frames: 8,
                active_frames: 4,
                recovery_frames: 12,
                damage: 65.0,
                hitstun_frames: 16,
                blockstun_frames: 10,
                knockback: Vec2::new(35.0, 0.0),
                hitbox_offset: Vec2::new(60.0, 0.0),
                hitbox_size: Vec2::new(80.0, 40.0),
                can_cancel: vec![MoveId::OrderBarrage],
                meter_gain: 12.0,
                meter_cost: 0.0,
                properties: vec![MoveProperty::ChargeMeter],
            },
            MoveData {
                move_id: MoveId::CommanderKick,
                name: "Commander's Kick",
                startup_frames: 7,
                active_frames: 5,
                recovery_frames: 10,
                damage: 55.0,
                hitstun_frames: 18,
                blockstun_frames: 12,
                knockback: Vec2::new(15.0, -30.0),
                hitbox_offset: Vec2::new(35.0, -20.0),
                hitbox_size: Vec2::new(70.0, 70.0),
                can_cancel: vec![MoveId::AerialAssault],
                meter_gain: 10.0,
                meter_cost: 0.0,
                properties: vec![MoveProperty::Juggle],
            },
            MoveData {
                move_id: MoveId::OrderBarrage,
                name: "Order Barrage",
                startup_frames: 12,
                active_frames: 20,
                recovery_frames: 18,
                damage: 120.0,
                hitstun_frames: 25,
                blockstun_frames: 15,
                knockback: Vec2::new(50.0, -20.0),
                hitbox_offset: Vec2::new(50.0, 0.0),
                hitbox_size: Vec2::new(100.0, 90.0),
                can_cancel: vec![],
                meter_gain: 20.0,
                meter_cost: 25.0,
                properties: vec![
                    MoveProperty::MultiHit(6),
                    MoveProperty::SuperArmor,
                ],
            },
            // Plane mode moves
            MoveData {
                move_id: MoveId::AerialAssault,
                name: "Aerial Assault",
                startup_frames: 45,
                active_frames: 900,  // 15 seconds at 60fps
                recovery_frames: 90,
                damage: 0.0,  // Doesn't directly damage
                hitstun_frames: 0,
                blockstun_frames: 0,
                knockback: Vec2::ZERO,
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::ZERO,
                can_cancel: vec![],
                meter_gain: 0.0,
                meter_cost: 50.0,
                properties: vec![MoveProperty::Invincible],
            },
            MoveData {
                move_id: MoveId::BombDrop,
                name: "Bomb Drop",
                startup_frames: 3,
                active_frames: 180,  // Bomb fall time
                recovery_frames: 5,
                damage: 80.0,
                hitstun_frames: 30,
                blockstun_frames: 20,
                knockback: Vec2::new(0.0, 50.0),
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::new(100.0, 100.0),  // Explosion radius
                can_cancel: vec![],
                meter_gain: 15.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Projectile,
                    MoveProperty::GroundBounce,
                ],
            },
            MoveData {
                move_id: MoveId::StrafeRun,
                name: "Strafe Run",
                startup_frames: 2,
                active_frames: 4,
                recovery_frames: 8,
                damage: 25.0,
                hitstun_frames: 8,
                blockstun_frames: 6,
                knockback: Vec2::new(10.0, 0.0),
                hitbox_offset: Vec2::new(80.0, 0.0),
                hitbox_size: Vec2::new(40.0, 30.0),
                can_cancel: vec![],
                meter_gain: 5.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Projectile,
                    MoveProperty::MultiHit(3),
                ],
            },
            MoveData {
                move_id: MoveId::AerialSlam,
                name: "Aerial Slam",
                startup_frames: 60,
                active_frames: 10,
                recovery_frames: 30,
                damage: 150.0,
                hitstun_frames: 45,
                blockstun_frames: 30,
                knockback: Vec2::new(0.0, 80.0),
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::new(150.0, 150.0),
                can_cancel: vec![],
                meter_gain: 30.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Unblockable,
                    MoveProperty::GroundBounce,
                    MoveProperty::Invincible,
                ],
            },
        ];

        let combo_routes = vec![
            ComboRoute {
                name: "Basic Military Combo",
                moves: vec![
                    MoveId::MilitaryStrike,
                    MoveId::BayonetThrust,
                    MoveId::CommanderKick,
                ],
                total_damage: 165.0,
                difficulty: ComboDifficulty::Easy,
                description: "Standard ground combo leading to launcher",
            },
            ComboRoute {
                name: "Aerial Bombardment",
                moves: vec![
                    MoveId::MilitaryStrike,
                    MoveId::CommanderKick,
                    MoveId::AerialAssault,
                    MoveId::BombDrop,
                    MoveId::AerialSlam,
                ],
                total_damage: 395.0,
                difficulty: ComboDifficulty::Hard,
                description: "Launch into plane mode, bomb, and slam finish",
            },
            ComboRoute {
                name: "Order Execution",
                moves: vec![
                    MoveId::BayonetThrust,
                    MoveId::OrderBarrage,
                ],
                total_damage: 185.0,
                difficulty: ComboDifficulty::Medium,
                description: "Meter-burning combo with super armor",
            },
        ];

        Self {
            character_type: CharacterType::KeizerBomTaha,
            stats,
            moves,
            combo_routes,
            special_mechanics: CharacterMechanics::PlaneSystem {
                plane_system: PlaneSystem::new(),
                ground_combo_limit: 8,
                aerial_damage_bonus: 1.5,
            },
        }
    }

    /// Get moveset for Principal Van Der Berg
    pub fn principal_van_der_berg() -> Self {
        let stats = CharacterStats {
            character_type: CharacterType::PrincipalVanDerBerg,
            max_health: 1100.0,  // Tank character
            base_damage: 1.3,
            base_speed: 200.0,   // Slow movement
            jump_force: 350.0,   // Low jump
            weight: 1.8,         // Very heavy
            air_mobility: 0.5,   // Poor air control
            dash_distance: 120.0,
            dash_cooldown: 1.2,
            block_strength: 0.85,  // Excellent blocking
            parry_window: 0.2,
            special_traits: vec![
                SpecialTrait::HeavyArmor,
                SpecialTrait::CommandGrab,
                SpecialTrait::Intimidation(150.0),
                SpecialTrait::SuperArmor,
            ],
        };

        let moves = vec![
            MoveData {
                move_id: MoveId::RulerSlap,
                name: "Ruler Slap",
                startup_frames: 6,
                active_frames: 4,
                recovery_frames: 10,
                damage: 50.0,
                hitstun_frames: 14,
                blockstun_frames: 10,
                knockback: Vec2::new(25.0, 0.0),
                hitbox_offset: Vec2::new(45.0, 0.0),
                hitbox_size: Vec2::new(70.0, 80.0),
                can_cancel: vec![MoveId::DeskSlam, MoveId::AuthorityStrike],
                meter_gain: 9.0,
                meter_cost: 0.0,
                properties: vec![MoveProperty::Overhead],
            },
            MoveData {
                move_id: MoveId::DeskSlam,
                name: "Desk Slam",
                startup_frames: 15,
                active_frames: 6,
                recovery_frames: 20,
                damage: 95.0,
                hitstun_frames: 25,
                blockstun_frames: 18,
                knockback: Vec2::new(0.0, 40.0),
                hitbox_offset: Vec2::new(60.0, 0.0),
                hitbox_size: Vec2::new(120.0, 100.0),
                can_cancel: vec![],
                meter_gain: 18.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Unblockable,
                    MoveProperty::GroundBounce,
                    MoveProperty::Stun(1.5),
                ],
            },
            MoveData {
                move_id: MoveId::AuthorityStrike,
                name: "Authority Strike",
                startup_frames: 9,
                active_frames: 5,
                recovery_frames: 12,
                damage: 70.0,
                hitstun_frames: 20,
                blockstun_frames: 14,
                knockback: Vec2::new(30.0, -15.0),
                hitbox_offset: Vec2::new(50.0, 0.0),
                hitbox_size: Vec2::new(85.0, 90.0),
                can_cancel: vec![MoveId::DisciplineGrab],
                meter_gain: 14.0,
                meter_cost: 0.0,
                properties: vec![MoveProperty::SuperArmor],
            },
            MoveData {
                move_id: MoveId::DisciplineGrab,
                name: "Discipline Grab",
                startup_frames: 10,
                active_frames: 3,
                recovery_frames: 25,
                damage: 85.0,
                hitstun_frames: 35,
                blockstun_frames: 0,  // Can't be blocked
                knockback: Vec2::ZERO,  // Cinematic grab
                hitbox_offset: Vec2::new(40.0, 0.0),
                hitbox_size: Vec2::new(60.0, 100.0),
                can_cancel: vec![],
                meter_gain: 20.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Unblockable,
                    MoveProperty::GroundBounce,
                ],
            },
            MoveData {
                move_id: MoveId::ExecutiveOrder,
                name: "Executive Order",
                startup_frames: 20,
                active_frames: 180,  // 3 seconds
                recovery_frames: 15,
                damage: 0.0,  // Buff move
                hitstun_frames: 0,
                blockstun_frames: 0,
                knockback: Vec2::ZERO,
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::ZERO,
                can_cancel: vec![],
                meter_gain: 0.0,
                meter_cost: 30.0,
                properties: vec![
                    MoveProperty::Invincible,
                    MoveProperty::ChargeMeter,
                ],
            },
            MoveData {
                move_id: MoveId::DetentionSentence,
                name: "Detention Sentence",
                startup_frames: 8,
                active_frames: 5,
                recovery_frames: 30,
                damage: 140.0,
                hitstun_frames: 50,
                blockstun_frames: 0,
                knockback: Vec2::new(60.0, -30.0),
                hitbox_offset: Vec2::new(55.0, 0.0),
                hitbox_size: Vec2::new(80.0, 110.0),
                can_cancel: vec![],
                meter_gain: 35.0,
                meter_cost: 50.0,
                properties: vec![
                    MoveProperty::Unblockable,
                    MoveProperty::SuperArmor,
                    MoveProperty::WallBounce,
                ],
            },
            MoveData {
                move_id: MoveId::PrincipalPresence,
                name: "Principal's Presence",
                startup_frames: 1,
                active_frames: 240,  // 4 seconds
                recovery_frames: 10,
                damage: 0.0,
                hitstun_frames: 0,
                blockstun_frames: 0,
                knockback: Vec2::ZERO,
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::new(300.0, 300.0),  // Large AoE
                can_cancel: vec![],
                meter_gain: 0.0,
                meter_cost: 25.0,
                properties: vec![MoveProperty::Slow(0.3)],  // Slows enemies by 30%
            },
        ];

        let combo_routes = vec![
            ComboRoute {
                name: "Disciplinary Action",
                moves: vec![
                    MoveId::RulerSlap,
                    MoveId::AuthorityStrike,
                    MoveId::DisciplineGrab,
                ],
                total_damage: 205.0,
                difficulty: ComboDifficulty::Medium,
                description: "Standard command grab combo",
            },
            ComboRoute {
                name: "Permanent Detention",
                moves: vec![
                    MoveId::ExecutiveOrder,
                    MoveId::RulerSlap,
                    MoveId::DetentionSentence,
                ],
                total_damage: 190.0,
                difficulty: ComboDifficulty::Hard,
                description: "Buff setup into super command grab",
            },
            ComboRoute {
                name: "Zero Tolerance",
                moves: vec![
                    MoveId::DeskSlam,
                    MoveId::AuthorityStrike,
                ],
                total_damage: 165.0,
                difficulty: ComboDifficulty::Easy,
                description: "Heavy damage with ground bounce",
            },
        ];

        Self {
            character_type: CharacterType::PrincipalVanDerBerg,
            stats,
            moves,
            combo_routes,
            special_mechanics: CharacterMechanics::AuthoritySystem {
                authority_meter: 0.0,
                max_authority: 100.0,
                authority_decay_rate: 5.0,  // Loses 5 authority per second
                authority_bonuses: vec![
                    AuthorityBonus {
                        threshold: 25.0,
                        bonus_type: BonusType::DamageIncrease(1.2),
                    },
                    AuthorityBonus {
                        threshold: 50.0,
                        bonus_type: BonusType::ArmorIncrease(0.2),
                    },
                    AuthorityBonus {
                        threshold: 75.0,
                        bonus_type: BonusType::UnblockableGrabs,
                    },
                    AuthorityBonus {
                        threshold: 100.0,
                        bonus_type: BonusType::InstantDetention,
                    },
                ],
            },
        }
    }

    /// Get moveset for Lunchroom Lady Petra
    pub fn lunchroom_lady_petra() -> Self {
        let stats = CharacterStats {
            character_type: CharacterType::LunchroomLadyPetra,
            max_health: 900.0,
            base_damage: 1.0,
            base_speed: 220.0,
            jump_force: 380.0,
            weight: 1.1,
            air_mobility: 0.7,
            dash_distance: 160.0,
            dash_cooldown: 0.9,
            block_strength: 0.7,
            parry_window: 0.12,
            special_traits: vec![
                SpecialTrait::Regeneration(3.0),  // 3 HP/sec
                SpecialTrait::RangedSpecialist,
                SpecialTrait::MeterBuilder,
                SpecialTrait::ProjectileAbsorb,
            ],
        };

        let moves = vec![
            MoveData {
                move_id: MoveId::SoupSplash,
                name: "Soup Splash",
                startup_frames: 7,
                active_frames: 60,
                recovery_frames: 12,
                damage: 40.0,
                hitstun_frames: 10,
                blockstun_frames: 8,
                knockback: Vec2::new(15.0, 0.0),
                hitbox_offset: Vec2::new(70.0, 0.0),
                hitbox_size: Vec2::new(50.0, 50.0),
                can_cancel: vec![MoveId::LadleSwing],
                meter_gain: 7.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Projectile,
                    MoveProperty::DOT(5.0, 3.0),  // 5 DPS for 3 seconds
                ],
            },
            MoveData {
                move_id: MoveId::TraySmash,
                name: "Tray Smash",
                startup_frames: 8,
                active_frames: 4,
                recovery_frames: 11,
                damage: 60.0,
                hitstun_frames: 16,
                blockstun_frames: 11,
                knockback: Vec2::new(20.0, -10.0),
                hitbox_offset: Vec2::new(50.0, 0.0),
                hitbox_size: Vec2::new(80.0, 70.0),
                can_cancel: vec![MoveId::FoodThrow],
                meter_gain: 11.0,
                meter_cost: 0.0,
                properties: vec![MoveProperty::Overhead],
            },
            MoveData {
                move_id: MoveId::LadleSwing,
                name: "Ladle Swing",
                startup_frames: 6,
                active_frames: 3,
                recovery_frames: 9,
                damage: 45.0,
                hitstun_frames: 12,
                blockstun_frames: 8,
                knockback: Vec2::new(18.0, 0.0),
                hitbox_offset: Vec2::new(55.0, 0.0),
                hitbox_size: Vec2::new(75.0, 60.0),
                can_cancel: vec![MoveId::SoupSplash, MoveId::TraySmash],
                meter_gain: 8.0,
                meter_cost: 0.0,
                properties: vec![],
            },
            MoveData {
                move_id: MoveId::FoodThrow,
                name: "Food Throw",
                startup_frames: 10,
                active_frames: 90,
                recovery_frames: 8,
                damage: 35.0,
                hitstun_frames: 8,
                blockstun_frames: 6,
                knockback: Vec2::new(12.0, 0.0),
                hitbox_offset: Vec2::new(100.0, 0.0),
                hitbox_size: Vec2::new(40.0, 40.0),
                can_cancel: vec![],
                meter_gain: 6.0,
                meter_cost: 0.0,
                properties: vec![
                    MoveProperty::Projectile,
                    MoveProperty::Slow(0.2),  // 20% slow
                ],
            },
            MoveData {
                move_id: MoveId::SteamBlast,
                name: "Steam Blast",
                startup_frames: 12,
                active_frames: 30,
                recovery_frames: 15,
                damage: 75.0,
                hitstun_frames: 22,
                blockstun_frames: 16,
                knockback: Vec2::new(35.0, 0.0),
                hitbox_offset: Vec2::new(80.0, 0.0),
                hitbox_size: Vec2::new(100.0, 100.0),
                can_cancel: vec![],
                meter_gain: 16.0,
                meter_cost: 20.0,
                properties: vec![
                    MoveProperty::MultiHit(5),
                    MoveProperty::DOT(8.0, 2.0),
                ],
            },
            MoveData {
                move_id: MoveId::FoodFight,
                name: "Food Fight",
                startup_frames: 18,
                active_frames: 120,  // 2 seconds
                recovery_frames: 20,
                damage: 90.0,
                hitstun_frames: 20,
                blockstun_frames: 15,
                knockback: Vec2::new(25.0, 0.0),
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::new(200.0, 200.0),  // Large AoE
                can_cancel: vec![],
                meter_gain: 25.0,
                meter_cost: 40.0,
                properties: vec![
                    MoveProperty::MultiHit(8),
                    MoveProperty::Slow(0.5),
                    MoveProperty::DOT(6.0, 4.0),
                ],
            },
            MoveData {
                move_id: MoveId::NutritionalValue,
                name: "Nutritional Value",
                startup_frames: 30,
                active_frames: 1,
                recovery_frames: 25,
                damage: -150.0,  // Negative damage = healing
                hitstun_frames: 0,
                blockstun_frames: 0,
                knockback: Vec2::ZERO,
                hitbox_offset: Vec2::ZERO,
                hitbox_size: Vec2::ZERO,
                can_cancel: vec![],
                meter_gain: 0.0,
                meter_cost: 35.0,
                properties: vec![],
            },
            MoveData {
                move_id: MoveId::ServingTime,
                name: "Serving Time",
                startup_frames: 15,
                active_frames: 180,  // 3 seconds
                recovery_frames: 30,
                damage: 180.0,
                hitstun_frames: 15,
                blockstun_frames: 12,
                knockback: Vec2::new(20.0, -20.0),
                hitbox_offset: Vec2::new(90.0, 0.0),
                hitbox_size: Vec2::new(60.0, 60.0),
                can_cancel: vec![],
                meter_gain: 40.0,
                meter_cost: 50.0,
                properties: vec![
                    MoveProperty::Projectile,
                    MoveProperty::MultiHit(12),
                    MoveProperty::Juggle,
                ],
            },
        ];

        let combo_routes = vec![
            ComboRoute {
                name: "Lunch Rush",
                moves: vec![
                    MoveId::LadleSwing,
                    MoveId::TraySmash,
                    MoveId::SoupSplash,
                ],
                total_damage: 145.0,
                difficulty: ComboDifficulty::Easy,
                description: "Basic combo with DOT finish",
            },
            ComboRoute {
                name: "Cafeteria Chaos",
                moves: vec![
                    MoveId::TraySmash,
                    MoveId::FoodThrow,
                    MoveId::SteamBlast,
                    MoveId::ServingTime,
                ],
                total_damage: 350.0,
                difficulty: ComboDifficulty::Expert,
                description: "Full combo with super finish",
            },
            ComboRoute {
                name: "Hot Meal",
                moves: vec![
                    MoveId::SoupSplash,
                    MoveId::LadleSwing,
                    MoveId::SteamBlast,
                ],
                total_damage: 160.0,
                difficulty: ComboDifficulty::Medium,
                description: "DOT-focused combo",
            },
        ];

        Self {
            character_type: CharacterType::LunchroomLadyPetra,
            stats,
            moves,
            combo_routes,
            special_mechanics: CharacterMechanics::FoodSystem {
                prepared_food: Vec::new(),
                max_food_slots: 3,
                prep_time: 2.0,
                current_recipe: None,
            },
        }
    }

    /// Get appropriate moveset for any character
    pub fn for_character(character_type: CharacterType) -> Self {
        match character_type {
            CharacterType::KeizerBomTaha => Self::keizer_bom_taha(),
            CharacterType::PrincipalVanDerBerg => Self::principal_van_der_berg(),
            CharacterType::LunchroomLadyPetra => Self::lunchroom_lady_petra(),
            _ => Self::default_moveset(character_type),
        }
    }

    /// Default moveset for characters without custom movesets
    fn default_moveset(character_type: CharacterType) -> Self {
        let stats = CharacterStats {
            character_type,
            max_health: 1000.0,
            base_damage: 1.0,
            base_speed: 250.0,
            jump_force: 400.0,
            weight: 1.0,
            air_mobility: 1.0,
            dash_distance: 150.0,
            dash_cooldown: 1.0,
            block_strength: 0.75,
            parry_window: 0.13,
            special_traits: vec![],
        };

        Self {
            character_type,
            stats,
            moves: Vec::new(),
            combo_routes: Vec::new(),
            special_mechanics: CharacterMechanics::Standard,
        }
    }

    /// Find move data by ID
    pub fn get_move(&self, move_id: MoveId) -> Option<&MoveData> {
        self.moves.iter().find(|m| m.move_id == move_id)
    }

    /// Check if a move can be performed
    pub fn can_perform_move(&self, move_id: MoveId, current_meter: f32) -> bool {
        if let Some(move_data) = self.get_move(move_id) {
            return current_meter >= move_data.meter_cost;
        }
        false
    }

    /// Get total frames for a move
    pub fn get_total_frames(&self, move_id: MoveId) -> u32 {
        if let Some(move_data) = self.get_move(move_id) {
            return move_data.startup_frames + move_data.active_frames + move_data.recovery_frames;
        }
        0
    }

    /// Check if move has specific property
    pub fn move_has_property(&self, move_id: MoveId, property: &MoveProperty) -> bool {
        if let Some(move_data) = self.get_move(move_id) {
            return move_data.properties.iter().any(|p| {
                // Compare discriminants for enum matching
                std::mem::discriminant(p) == std::mem::discriminant(property)
            });
        }
        false
    }
}

impl Default for CharacterMoveset {
    fn default() -> Self {
        Self::default_moveset(CharacterType::Bas)
    }
}

/// Frame data calculator for competitive play
pub struct FrameDataCalculator;

impl FrameDataCalculator {
    /// Calculate frame advantage on hit
    pub fn frame_advantage_on_hit(move_data: &MoveData) -> i32 {
        let attacker_recovery = move_data.recovery_frames as i32;
        let defender_hitstun = move_data.hitstun_frames as i32;
        defender_hitstun - attacker_recovery
    }

    /// Calculate frame advantage on block
    pub fn frame_advantage_on_block(move_data: &MoveData) -> i32 {
        let attacker_recovery = move_data.recovery_frames as i32;
        let defender_blockstun = move_data.blockstun_frames as i32;
        defender_blockstun - attacker_recovery
    }

    /// Check if move is plus on block
    pub fn is_plus_on_block(move_data: &MoveData) -> bool {
        Self::frame_advantage_on_block(move_data) > 0
    }

    /// Check if move is safe on block (0 or better)
    pub fn is_safe_on_block(move_data: &MoveData) -> bool {
        Self::frame_advantage_on_block(move_data) >= 0
    }

    /// Calculate punish window (frames opponent has to punish)
    pub fn punish_window(move_data: &MoveData) -> u32 {
        let advantage = Self::frame_advantage_on_block(move_data);
        if advantage < 0 {
            advantage.abs() as u32
        } else {
            0
        }
    }
}
