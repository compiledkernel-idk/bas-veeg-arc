use crate::data::characters::CharacterId;
use macroquad::prelude::*;

/// Boss system for advanced boss mechanics
pub struct BossManager {
    pub active_boss: Option<BossController>,
    pub boss_music_playing: bool,
    pub intro_played: bool,
    pub phase_transitions: Vec<PhaseTransition>,
}

/// Boss controller for individual bosses
pub struct BossController {
    pub boss_type: BossType,
    pub current_phase: u32,
    pub max_phases: u32,
    pub health_percent: f32,
    pub phase_thresholds: Vec<f32>,
    pub attack_pattern: AttackPattern,
    pub enrage_timer: f32,
    pub invulnerable: bool,
    pub summoning_minions: bool,
    pub special_move_cooldown: f32,
    pub ai_state: BossAIState,
    pub position: Vec2,
}

/// Types of bosses
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossType {
    // Existing bosses
    Bastiaan,
    KeizerBomTaha,
    Mees,

    // New bosses
    PrincipalVanDerBerg,
    JanitorKing,
    HeadChef,
}

/// Boss AI states
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossAIState {
    Idle,
    Approaching,
    Attacking,
    Defensive,
    Summoning,
    PhaseTransition,
    Enraged,
    Retreating,
}

/// Attack pattern for bosses
#[derive(Clone, Debug)]
pub struct AttackPattern {
    pub attacks: Vec<BossAttack>,
    pub current_attack_index: usize,
    pub attack_cooldown: f32,
    pub pattern_type: PatternType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PatternType {
    Sequential,    // Attacks in sequence
    Random,        // Random attacks
    Adaptive,      // Based on player behavior
    PhaseBased,    // Changes per phase
}

/// Individual boss attack
#[derive(Clone, Debug)]
pub struct BossAttack {
    pub name: String,
    pub damage: f32,
    pub telegraph_time: f32,
    pub execution_time: f32,
    pub recovery_time: f32,
    pub attack_type: AttackType,
    pub hitbox_data: Vec<AttackHitbox>,
    pub can_be_interrupted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttackType {
    Melee,
    Projectile,
    AOE,
    Grab,
    Summon,
    Buff,
}

/// Hitbox for attacks
#[derive(Clone, Debug)]
pub struct AttackHitbox {
    pub position: Vec2,
    pub size: Vec2,
    pub duration: f32,
    pub damage_multiplier: f32,
}

/// Phase transition data
pub struct PhaseTransition {
    pub from_phase: u32,
    pub to_phase: u32,
    pub cutscene_text: String,
    pub effects: Vec<PhaseEffect>,
    pub duration: f32,
}

#[derive(Clone, Debug)]
pub enum PhaseEffect {
    HealPercent(f32),
    SpeedBoost(f32),
    DamageBoost(f32),
    SummonMinions(u32),
    Invulnerability(f32),
    AreaHazard,
}

impl BossManager {
    pub fn new() -> Self {
        Self {
            active_boss: None,
            boss_music_playing: false,
            intro_played: false,
            phase_transitions: Vec::new(),
        }
    }

    pub fn spawn_boss(&mut self, boss_type: BossType) {
        let boss = Self::create_boss(boss_type);
        self.active_boss = Some(boss);
        self.intro_played = false;
    }

    fn create_boss(boss_type: BossType) -> BossController {
        match boss_type {
            BossType::PrincipalVanDerBerg => Self::create_principal(),
            BossType::JanitorKing => Self::create_janitor_king(),
            BossType::HeadChef => Self::create_head_chef(),
            _ => Self::create_basic_boss(boss_type),
        }
    }

    /// NEW BOSS: Principal Van Der Berg
    /// Multi-phase boss with prefect summoning and office-based attacks
    fn create_principal() -> BossController {
        BossController {
            boss_type: BossType::PrincipalVanDerBerg,
            current_phase: 1,
            max_phases: 3,
            health_percent: 100.0,
            phase_thresholds: vec![75.0, 50.0, 25.0],
            attack_pattern: AttackPattern {
                attacks: vec![
                    // Phase 1: Disciplinary actions
                    BossAttack {
                        name: "Ruler Slam".to_string(),
                        damage: 30.0,
                        telegraph_time: 0.5,
                        execution_time: 0.3,
                        recovery_time: 1.0,
                        attack_type: AttackType::Melee,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(50.0, 0.0),
                            size: Vec2::new(80.0, 60.0),
                            duration: 0.3,
                            damage_multiplier: 1.0,
                        }],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Whistle Blow".to_string(),
                        damage: 20.0,
                        telegraph_time: 0.8,
                        execution_time: 0.2,
                        recovery_time: 1.5,
                        attack_type: AttackType::AOE,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::ZERO,
                            size: Vec2::new(200.0, 200.0),
                            duration: 0.5,
                            damage_multiplier: 1.0,
                        }],
                        can_be_interrupted: true,
                    },
                    BossAttack {
                        name: "Summon Prefects".to_string(),
                        damage: 0.0,
                        telegraph_time: 1.0,
                        execution_time: 1.0,
                        recovery_time: 2.0,
                        attack_type: AttackType::Summon,
                        hitbox_data: vec![],
                        can_be_interrupted: true,
                    },
                    // Phase 2: Aggressive tactics
                    BossAttack {
                        name: "Detention Dash".to_string(),
                        damage: 40.0,
                        telegraph_time: 0.3,
                        execution_time: 0.8,
                        recovery_time: 1.2,
                        attack_type: AttackType::Melee,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(100.0, 0.0),
                            size: Vec2::new(150.0, 80.0),
                            duration: 0.8,
                            damage_multiplier: 1.5,
                        }],
                        can_be_interrupted: false,
                    },
                    // Phase 3: Desperate measures
                    BossAttack {
                        name: "Office Furniture Throw".to_string(),
                        damage: 35.0,
                        telegraph_time: 0.6,
                        execution_time: 0.4,
                        recovery_time: 0.8,
                        attack_type: AttackType::Projectile,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(0.0, -50.0),
                            size: Vec2::new(60.0, 60.0),
                            duration: 3.0,
                            damage_multiplier: 1.0,
                        }],
                        can_be_interrupted: false,
                    },
                ],
                current_attack_index: 0,
                attack_cooldown: 0.0,
                pattern_type: PatternType::PhaseBased,
            },
            enrage_timer: 0.0,
            invulnerable: false,
            summoning_minions: false,
            special_move_cooldown: 0.0,
            ai_state: BossAIState::Idle,
            position: Vec2::new(960.0, 300.0),
        }
    }

    /// NEW BOSS: Janitor King
    /// Slow but powerful boss with mop-based attacks and water hazards
    fn create_janitor_king() -> BossController {
        BossController {
            boss_type: BossType::JanitorKing,
            current_phase: 1,
            max_phases: 3,
            health_percent: 100.0,
            phase_thresholds: vec![70.0, 40.0],
            attack_pattern: AttackPattern {
                attacks: vec![
                    BossAttack {
                        name: "Mop Swing".to_string(),
                        damage: 45.0,
                        telegraph_time: 0.7,
                        execution_time: 0.5,
                        recovery_time: 1.5,
                        attack_type: AttackType::Melee,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(60.0, 0.0),
                            size: Vec2::new(120.0, 80.0),
                            duration: 0.5,
                            damage_multiplier: 1.2,
                        }],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Bucket Slam".to_string(),
                        damage: 55.0,
                        telegraph_time: 1.0,
                        execution_time: 0.3,
                        recovery_time: 2.0,
                        attack_type: AttackType::AOE,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::ZERO,
                            size: Vec2::new(150.0, 150.0),
                            duration: 0.3,
                            damage_multiplier: 1.5,
                        }],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Slippery Floor".to_string(),
                        damage: 0.0,
                        telegraph_time: 0.5,
                        execution_time: 1.0,
                        recovery_time: 3.0,
                        attack_type: AttackType::AOE,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::ZERO,
                            size: Vec2::new(300.0, 200.0),
                            duration: 5.0,
                            damage_multiplier: 0.0,
                        }],
                        can_be_interrupted: true,
                    },
                    BossAttack {
                        name: "Spin Cycle".to_string(),
                        damage: 25.0,
                        telegraph_time: 0.8,
                        execution_time: 2.0,
                        recovery_time: 1.5,
                        attack_type: AttackType::AOE,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::ZERO,
                            size: Vec2::new(180.0, 180.0),
                            duration: 2.0,
                            damage_multiplier: 0.8,
                        }],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Soap Bubble Barrage".to_string(),
                        damage: 20.0,
                        telegraph_time: 0.4,
                        execution_time: 1.5,
                        recovery_time: 1.0,
                        attack_type: AttackType::Projectile,
                        hitbox_data: vec![
                            AttackHitbox {
                                position: Vec2::new(50.0, 0.0),
                                size: Vec2::new(30.0, 30.0),
                                duration: 2.0,
                                damage_multiplier: 1.0,
                            },
                        ],
                        can_be_interrupted: true,
                    },
                ],
                current_attack_index: 0,
                attack_cooldown: 0.0,
                pattern_type: PatternType::Sequential,
            },
            enrage_timer: 0.0,
            invulnerable: false,
            summoning_minions: false,
            special_move_cooldown: 0.0,
            ai_state: BossAIState::Idle,
            position: Vec2::new(960.0, 400.0),
        }
    }

    /// NEW BOSS: Head Chef
    /// Fast aggressive boss with food-based projectiles and rage mode
    fn create_head_chef() -> BossController {
        BossController {
            boss_type: BossType::HeadChef,
            current_phase: 1,
            max_phases: 2,
            health_percent: 100.0,
            phase_thresholds: vec![50.0],
            attack_pattern: AttackPattern {
                attacks: vec![
                    BossAttack {
                        name: "Knife Combo".to_string(),
                        damage: 18.0,
                        telegraph_time: 0.2,
                        execution_time: 0.8,
                        recovery_time: 0.6,
                        attack_type: AttackType::Melee,
                        hitbox_data: vec![
                            AttackHitbox {
                                position: Vec2::new(40.0, 0.0),
                                size: Vec2::new(50.0, 40.0),
                                duration: 0.2,
                                damage_multiplier: 1.0,
                            },
                            AttackHitbox {
                                position: Vec2::new(45.0, 5.0),
                                size: Vec2::new(50.0, 40.0),
                                duration: 0.2,
                                damage_multiplier: 1.0,
                            },
                            AttackHitbox {
                                position: Vec2::new(50.0, -5.0),
                                size: Vec2::new(50.0, 40.0),
                                duration: 0.2,
                                damage_multiplier: 1.0,
                            },
                        ],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Hot Pan Toss".to_string(),
                        damage: 35.0,
                        telegraph_time: 0.4,
                        execution_time: 0.3,
                        recovery_time: 1.0,
                        attack_type: AttackType::Projectile,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(0.0, -30.0),
                            size: Vec2::new(50.0, 50.0),
                            duration: 2.5,
                            damage_multiplier: 1.3,
                        }],
                        can_be_interrupted: false,
                    },
                    BossAttack {
                        name: "Food Fight".to_string(),
                        damage: 22.0,
                        telegraph_time: 0.5,
                        execution_time: 1.5,
                        recovery_time: 1.2,
                        attack_type: AttackType::Projectile,
                        hitbox_data: vec![
                            AttackHitbox {
                                position: Vec2::new(0.0, -20.0),
                                size: Vec2::new(30.0, 30.0),
                                duration: 3.0,
                                damage_multiplier: 0.8,
                            },
                        ],
                        can_be_interrupted: true,
                    },
                    BossAttack {
                        name: "Flaming Grill".to_string(),
                        damage: 15.0,
                        telegraph_time: 0.8,
                        execution_time: 3.0,
                        recovery_time: 2.0,
                        attack_type: AttackType::AOE,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::ZERO,
                            size: Vec2::new(250.0, 120.0),
                            duration: 3.0,
                            damage_multiplier: 0.5,
                        }],
                        can_be_interrupted: false,
                    },
                    // Rage mode attacks (Phase 2)
                    BossAttack {
                        name: "Chef's Fury".to_string(),
                        damage: 28.0,
                        telegraph_time: 0.1,
                        execution_time: 1.5,
                        recovery_time: 0.5,
                        attack_type: AttackType::Melee,
                        hitbox_data: vec![AttackHitbox {
                            position: Vec2::new(50.0, 0.0),
                            size: Vec2::new(70.0, 50.0),
                            duration: 0.3,
                            damage_multiplier: 1.5,
                        }],
                        can_be_interrupted: false,
                    },
                ],
                current_attack_index: 0,
                attack_cooldown: 0.0,
                pattern_type: PatternType::Adaptive,
            },
            enrage_timer: 0.0,
            invulnerable: false,
            summoning_minions: false,
            special_move_cooldown: 0.0,
            ai_state: BossAIState::Idle,
            position: Vec2::new(960.0, 400.0),
        }
    }

    fn create_basic_boss(boss_type: BossType) -> BossController {
        BossController {
            boss_type,
            current_phase: 1,
            max_phases: 1,
            health_percent: 100.0,
            phase_thresholds: vec![],
            attack_pattern: AttackPattern {
                attacks: vec![],
                current_attack_index: 0,
                attack_cooldown: 0.0,
                pattern_type: PatternType::Sequential,
            },
            enrage_timer: 0.0,
            invulnerable: false,
            summoning_minions: false,
            special_move_cooldown: 0.0,
            ai_state: BossAIState::Idle,
            position: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(boss) = &mut self.active_boss {
            boss.update(dt);
        }
    }
}

impl BossController {
    pub fn update(&mut self, dt: f32) {
        // Update attack cooldown
        if self.attack_pattern.attack_cooldown > 0.0 {
            self.attack_pattern.attack_cooldown -= dt;
        }

        // Update special move cooldown
        if self.special_move_cooldown > 0.0 {
            self.special_move_cooldown -= dt;
        }

        // Check for phase transitions
        self.check_phase_transition();

        // Update enrage timer if in enraged state
        if self.ai_state == BossAIState::Enraged {
            self.enrage_timer += dt;
        }
    }

    fn check_phase_transition(&mut self) {
        for (i, threshold) in self.phase_thresholds.iter().enumerate() {
            let next_phase = (i + 2) as u32; // Phases start at 1
            if self.health_percent <= *threshold && self.current_phase < next_phase {
                self.transition_to_phase(next_phase);
                break;
            }
        }
    }

    fn transition_to_phase(&mut self, phase: u32) {
        self.current_phase = phase;
        self.invulnerable = true;
        self.ai_state = BossAIState::PhaseTransition;

        // Apply phase-specific changes
        match self.boss_type {
            BossType::PrincipalVanDerBerg => {
                if phase == 2 {
                    // Summon more prefects
                    self.summoning_minions = true;
                } else if phase == 3 {
                    // Enter enraged state
                    self.ai_state = BossAIState::Enraged;
                }
            }
            BossType::JanitorKing => {
                if phase == 2 {
                    // Create water hazards
                } else if phase == 3 {
                    // Increase attack speed
                }
            }
            BossType::HeadChef => {
                if phase == 2 {
                    // Enter rage mode - faster attacks, more damage
                    self.ai_state = BossAIState::Enraged;
                }
            }
            _ => {}
        }
    }

    pub fn get_current_attack(&self) -> Option<&BossAttack> {
        self.attack_pattern.attacks.get(self.attack_pattern.current_attack_index)
    }

    pub fn advance_attack_pattern(&mut self) {
        match self.attack_pattern.pattern_type {
            PatternType::Sequential => {
                self.attack_pattern.current_attack_index =
                    (self.attack_pattern.current_attack_index + 1) % self.attack_pattern.attacks.len();
            }
            PatternType::Random => {
                let rand_index = (rand::gen_range(0.0, 1.0) * self.attack_pattern.attacks.len() as f32) as usize;
                self.attack_pattern.current_attack_index = rand_index % self.attack_pattern.attacks.len();
            }
            PatternType::Adaptive => {
                // Would analyze player behavior
                self.attack_pattern.current_attack_index =
                    (self.attack_pattern.current_attack_index + 1) % self.attack_pattern.attacks.len();
            }
            PatternType::PhaseBased => {
                // Filter attacks based on current phase
                self.attack_pattern.current_attack_index =
                    (self.attack_pattern.current_attack_index + 1) % self.attack_pattern.attacks.len();
            }
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        if self.invulnerable {
            return;
        }

        let max_health = 1000.0; // Would be based on boss type
        self.health_percent -= (damage / max_health) * 100.0;
        self.health_percent = self.health_percent.max(0.0);
    }

    pub fn is_defeated(&self) -> bool {
        self.health_percent <= 0.0
    }
}

impl BossType {
    pub fn to_string(&self) -> &str {
        match self {
            BossType::Bastiaan => "Bastiaan",
            BossType::KeizerBomTaha => "Keizer Bom Taha",
            BossType::Mees => "Mees",
            BossType::PrincipalVanDerBerg => "Principal Van Der Berg",
            BossType::JanitorKing => "The Janitor King",
            BossType::HeadChef => "Head Chef Ramsey",
        }
    }

    pub fn get_intro_text(&self) -> &str {
        match self {
            BossType::PrincipalVanDerBerg => "Principal Van Der Berg blocks your path!\n'Detention for all of you!'",
            BossType::JanitorKing => "The Janitor King emerges from his domain!\n'This school will be SPOTLESS!'",
            BossType::HeadChef => "Head Chef Ramsey guards the cafeteria!\n'YOU'RE DONE! THIS KITCHEN IS CLOSED!'",
            _ => "A powerful foe appears!",
        }
    }

    pub fn get_max_health(&self) -> f32 {
        match self {
            BossType::Bastiaan => 800.0,
            BossType::KeizerBomTaha => 900.0,
            BossType::Mees => 750.0,
            BossType::PrincipalVanDerBerg => 1200.0,
            BossType::JanitorKing => 1500.0,
            BossType::HeadChef => 1000.0,
        }
    }
}
