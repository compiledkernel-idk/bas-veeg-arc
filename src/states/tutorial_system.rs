use macroquad::prelude::*;
use crate::ecs::CharacterType;
use crate::combat::{MoveId, CharacterMoveset, StyleRank};

/// Complete tutorial and character guide system
pub struct TutorialSystem {
    pub current_tutorial: Option<TutorialId>,
    pub completed_tutorials: Vec<TutorialId>,
    pub tutorial_progress: TutorialProgress,
    pub character_guides: Vec<CharacterGuide>,
    pub move_list_display: MoveListDisplay,
    pub tutorial_challenges: Vec<TutorialChallenge>,
    pub hint_system: HintSystem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TutorialId {
    // Basic tutorials
    Movement,
    BasicAttacks,
    Blocking,
    Jumping,
    Dashing,

    // Intermediate tutorials
    Combos,
    SpecialMoves,
    Launchers,
    AirCombos,
    Grabs,

    // Advanced tutorials
    Cancels,
    Parries,
    CounterHits,
    MeterManagement,
    StyleRanking,

    // Character-specific tutorials
    KeizerPlaneMechanics,
    PrincipalAuthoritySystem,
    PetraFoodPrep,

    // System tutorials
    SkillTrees,
    Progression,
    CoopPlay,
    GameModes,
    TrainingMode,
}

/// Tutorial progression state
pub struct TutorialProgress {
    pub tutorial_id: TutorialId,
    pub current_step: usize,
    pub steps: Vec<TutorialStep>,
    pub success_count: u32,
    pub required_successes: u32,
    pub time_elapsed: f32,
    pub hints_shown: Vec<String>,
}

/// Individual tutorial step
#[derive(Clone)]
pub struct TutorialStep {
    pub step_id: usize,
    pub title: &'static str,
    pub description: &'static str,
    pub objective: TutorialObjective,
    pub demonstration: Option<Vec<DemoAction>>,
    pub time_limit: Option<f32>,
    pub hints: Vec<&'static str>,
    pub reward: Option<TutorialReward>,
}

#[derive(Clone, Debug)]
pub enum TutorialObjective {
    /// Perform a specific move
    PerformMove(MoveId),

    /// Land X hits
    LandHits(u32),

    /// Block X attacks
    BlockAttacks(u32),

    /// Reach a combo count
    ReachCombo(u32),

    /// Achieve style rank
    AchieveStyleRank(StyleRank),

    /// Deal X damage
    DealDamage(f32),

    /// Complete in time limit
    CompleteInTime(f32),

    /// Execute combo string
    ExecuteComboString(Vec<MoveId>),

    /// Parry an attack
    ParryAttack,

    /// Counter hit opponent
    CounterHit,

    /// Use meter ability
    UseMeterAbility,

    /// Custom objective for character mechanics
    CustomObjective(String),
}

#[derive(Clone, Debug)]
pub struct DemoAction {
    pub action_type: DemoActionType,
    pub delay: f32,
    pub highlight: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum DemoActionType {
    Move(MoveId),
    MoveForward,
    MoveBackward,
    Jump,
    Block,
    Dash,
    Wait(f32),
}

#[derive(Clone, Debug)]
pub struct TutorialReward {
    pub reward_type: RewardType,
    pub value: u32,
}

#[derive(Clone, Debug)]
pub enum RewardType {
    Currency,
    SkillPoints,
    Achievement(String),  // Achievement ID as string
    CharacterUnlock(CharacterType),
}

/// Character-specific guide information
pub struct CharacterGuide {
    pub character_type: CharacterType,
    pub overview: CharacterOverview,
    pub move_list: Vec<MoveDescription>,
    pub combo_routes: Vec<ComboGuide>,
    pub strategy_tips: Vec<StrategyTip>,
    pub matchup_guide: Vec<MatchupInfo>,
    pub advanced_techniques: Vec<AdvancedTechnique>,
}

pub struct CharacterOverview {
    pub name: &'static str,
    pub archetype: CharacterArchetype,
    pub difficulty: CharacterDifficulty,
    pub strengths: Vec<&'static str>,
    pub weaknesses: Vec<&'static str>,
    pub playstyle_description: &'static str,
    pub recommended_for: &'static str,
    pub stats: CharacterStatsDisplay,
}

#[derive(Clone, Copy, Debug)]
pub enum CharacterArchetype {
    Rushdown,      // Aggressive, in-your-face
    Zoner,         // Keep-away, projectiles
    Grappler,      // Command grabs, high damage
    Balanced,      // Well-rounded
    Technical,     // Complex mechanics
    Tank,          // High HP, armor
    Support,       // Buffs/debuffs, control
}

#[derive(Clone, Copy, Debug)]
pub enum CharacterDifficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Expert,
    Master,
}

pub struct CharacterStatsDisplay {
    pub health: u8,        // 1-10 rating
    pub damage: u8,        // 1-10 rating
    pub speed: u8,         // 1-10 rating
    pub range: u8,         // 1-10 rating
    pub difficulty: u8,    // 1-10 rating
}

pub struct MoveDescription {
    pub move_id: MoveId,
    pub name: &'static str,
    pub input: &'static str,
    pub description: &'static str,
    pub properties: Vec<MoveProperty>,
    pub frame_data: FrameDataDisplay,
    pub usage_tips: Vec<&'static str>,
}

#[derive(Clone, Copy, Debug)]
pub enum MoveProperty {
    Overhead,
    Low,
    Launcher,
    Projectile,
    Invincible,
    Unblockable,
    SuperArmor,
    Grab,
}

pub struct FrameDataDisplay {
    pub startup: u32,
    pub active: u32,
    pub recovery: u32,
    pub on_hit: i32,       // Frame advantage on hit
    pub on_block: i32,     // Frame advantage on block
}

pub struct ComboGuide {
    pub name: &'static str,
    pub inputs: Vec<MoveId>,
    pub damage: f32,
    pub meter_cost: f32,
    pub difficulty: ComboDifficulty,
    pub description: &'static str,
    pub tips: Vec<&'static str>,
    pub video_demo: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ComboDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    StyleOnly,
}

pub struct StrategyTip {
    pub category: StrategyCategory,
    pub tip: &'static str,
    pub situation: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum StrategyCategory {
    Offense,
    Defense,
    Neutral,
    Comeback,
    MeterUsage,
    Positioning,
}

pub struct MatchupInfo {
    pub opponent: CharacterType,
    pub difficulty: MatchupDifficulty,
    pub key_points: Vec<&'static str>,
    pub what_to_watch_for: Vec<&'static str>,
    pub recommended_strategy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchupDifficulty {
    VeryFavorable,
    Favorable,
    Even,
    Unfavorable,
    VeryUnfavorable,
}

pub struct AdvancedTechnique {
    pub name: &'static str,
    pub description: &'static str,
    pub execution: &'static str,
    pub applications: Vec<&'static str>,
    pub difficulty: u8,  // 1-10
}

/// Move list display for in-game reference
pub struct MoveListDisplay {
    pub visible: bool,
    pub selected_character: Option<CharacterType>,
    pub selected_category: MoveCategory,
    pub scroll_offset: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveCategory {
    Normals,
    Specials,
    Supers,
    Grabs,
    Universal,
}

/// Tutorial challenges for practice
pub struct TutorialChallenge {
    pub challenge_id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub objective: TutorialObjective,
    pub time_limit: Option<f32>,
    pub difficulty: ChallengeDifficulty,
    pub reward: TutorialReward,
    pub unlocked: bool,
    pub completed: bool,
    pub best_time: Option<f32>,
}

#[derive(Clone, Copy, Debug)]
pub enum ChallengeDifficulty {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

/// Dynamic hint system
pub struct HintSystem {
    pub hints_enabled: bool,
    pub current_hints: Vec<ActiveHint>,
    pub hint_cooldown: f32,
    pub hint_categories: Vec<HintCategory>,
}

pub struct ActiveHint {
    pub text: String,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub position: HintPosition,
    pub priority: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum HintPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Center,
}

pub struct HintCategory {
    pub category: String,
    pub hints: Vec<ContextualHint>,
}

pub struct ContextualHint {
    pub trigger_condition: HintTrigger,
    pub hint_text: &'static str,
    pub shown_count: u32,
    pub max_shows: u32,
}

#[derive(Clone, Debug)]
pub enum HintTrigger {
    HealthBelow(f32),
    ComboDropped,
    BlockedMultipleAttacks(u32),
    MeterFull,
    OutOfPosition,
    StuckInCorner,
    TakingTooMuchDamage,
    NotUsingMeter,
    PoorComboEfficiency,
}

impl TutorialSystem {
    pub fn new() -> Self {
        let mut system = Self {
            current_tutorial: None,
            completed_tutorials: Vec::new(),
            tutorial_progress: TutorialProgress::new(),
            character_guides: Vec::new(),
            move_list_display: MoveListDisplay {
                visible: false,
                selected_character: None,
                selected_category: MoveCategory::Normals,
                scroll_offset: 0.0,
            },
            tutorial_challenges: Vec::new(),
            hint_system: HintSystem::new(),
        };

        // Initialize character guides
        system.initialize_character_guides();

        // Initialize tutorial challenges
        system.initialize_challenges();

        system
    }

    fn initialize_character_guides(&mut self) {
        // Keizer Bom Taha Guide
        self.character_guides.push(CharacterGuide {
            character_type: CharacterType::KeizerBomTaha,
            overview: CharacterOverview {
                name: "Keizer Bom Taha",
                archetype: CharacterArchetype::Technical,
                difficulty: CharacterDifficulty::Hard,
                strengths: vec![
                    "Unique plane mechanics for area control",
                    "High damage potential with bombs",
                    "Strong zoning and keep-away game",
                    "Excellent combo potential",
                ],
                weaknesses: vec![
                    "Vulnerable during plane transitions",
                    "Limited options without plane mode",
                    "Requires good meter management",
                    "High execution barrier",
                ],
                playstyle_description: "A technical character that excels at controlling space with aerial bombardment. Use ground combos to build meter, then activate plane mode for massive damage and area denial.",
                recommended_for: "Advanced players who enjoy unique mechanics and strategic gameplay",
                stats: CharacterStatsDisplay {
                    health: 8,
                    damage: 9,
                    speed: 7,
                    range: 10,
                    difficulty: 9,
                },
            },
            move_list: vec![
                MoveDescription {
                    move_id: MoveId::MilitaryStrike,
                    name: "Military Strike",
                    input: "Light Punch",
                    description: "Quick jab with good range. Primary combo starter.",
                    properties: vec![],
                    frame_data: FrameDataDisplay {
                        startup: 5,
                        active: 3,
                        recovery: 8,
                        on_hit: 4,
                        on_block: 0,
                    },
                    usage_tips: vec![
                        "Use as poke in neutral",
                        "Cancellable into special moves",
                        "Safe on block",
                    ],
                },
                MoveDescription {
                    move_id: MoveId::AerialAssault,
                    name: "Aerial Assault",
                    input: "Quarter-circle Forward + Heavy",
                    description: "Summons plane for 15 seconds. Gain access to bombs and strafing runs.",
                    properties: vec![MoveProperty::Invincible],
                    frame_data: FrameDataDisplay {
                        startup: 45,
                        active: 900,
                        recovery: 90,
                        on_hit: 0,
                        on_block: 0,
                    },
                    usage_tips: vec![
                        "Invincible during startup",
                        "Use after landing combo",
                        "Manage duration carefully",
                        "Exit with slam for bonus damage",
                    ],
                },
            ],
            combo_routes: vec![
                ComboGuide {
                    name: "Basic Military Combo",
                    inputs: vec![
                        MoveId::MilitaryStrike,
                        MoveId::BayonetThrust,
                        MoveId::CommanderKick,
                    ],
                    damage: 165.0,
                    meter_cost: 0.0,
                    difficulty: ComboDifficulty::Easy,
                    description: "Standard ground combo leading to launcher. Safe and reliable.",
                    tips: vec![
                        "Can be hitconfirmed from first hit",
                        "Launch sets up air combos",
                    ],
                    video_demo: false,
                },
                ComboGuide {
                    name: "Aerial Bombardment",
                    inputs: vec![
                        MoveId::MilitaryStrike,
                        MoveId::CommanderKick,
                        MoveId::AerialAssault,
                        MoveId::BombDrop,
                        MoveId::AerialSlam,
                    ],
                    damage: 395.0,
                    meter_cost: 50.0,
                    difficulty: ComboDifficulty::Hard,
                    description: "Maximum damage combo utilizing plane mechanics. Stylish and devastating.",
                    tips: vec![
                        "Timing on bomb drop is crucial",
                        "Position plane carefully",
                        "Slam is unblockable finish",
                    ],
                    video_demo: true,
                },
            ],
            strategy_tips: vec![
                StrategyTip {
                    category: StrategyCategory::Offense,
                    tip: "Use ground combos to build meter before activating plane mode",
                    situation: "Start of round or after plane cooldown",
                },
                StrategyTip {
                    category: StrategyCategory::Defense,
                    tip: "Plane mode provides invincibility during activation - use as emergency escape",
                    situation: "Under heavy pressure or in corner",
                },
                StrategyTip {
                    category: StrategyCategory::MeterUsage,
                    tip: "Save meter for plane mode rather than burning on supers",
                    situation: "General gameplay",
                },
            ],
            matchup_guide: vec![],
            advanced_techniques: vec![
                AdvancedTechnique {
                    name: "Bomb Loop",
                    description: "Chain multiple bomb patterns for extended pressure",
                    execution: "Drop bombs in sequence while maintaining altitude",
                    applications: vec![
                        "Corner pressure",
                        "Okizeme setups",
                        "Chip damage",
                    ],
                    difficulty: 8,
                },
            ],
        });
    }

    fn initialize_challenges(&mut self) {
        self.tutorial_challenges.push(TutorialChallenge {
            challenge_id: 1,
            name: "10-Hit Combo",
            description: "Land a 10-hit combo on the training dummy",
            objective: TutorialObjective::ReachCombo(10),
            time_limit: None,
            difficulty: ChallengeDifficulty::Bronze,
            reward: TutorialReward {
                reward_type: RewardType::Currency,
                value: 100,
            },
            unlocked: true,
            completed: false,
            best_time: None,
        });

        self.tutorial_challenges.push(TutorialChallenge {
            challenge_id: 2,
            name: "Style Master",
            description: "Achieve SSS rank in a combo",
            objective: TutorialObjective::AchieveStyleRank(StyleRank::SSS),
            time_limit: None,
            difficulty: ChallengeDifficulty::Platinum,
            reward: TutorialReward {
                reward_type: RewardType::SkillPoints,
                value: 5,
            },
            unlocked: true,
            completed: false,
            best_time: None,
        });

        self.tutorial_challenges.push(TutorialChallenge {
            challenge_id: 3,
            name: "Perfect Parry",
            description: "Successfully parry 5 attacks in a row",
            objective: TutorialObjective::ParryAttack,
            time_limit: Some(30.0),
            difficulty: ChallengeDifficulty::Gold,
            reward: TutorialReward {
                reward_type: RewardType::Currency,
                value: 500,
            },
            unlocked: true,
            completed: false,
            best_time: None,
        });
    }

    pub fn start_tutorial(&mut self, tutorial_id: TutorialId) {
        self.current_tutorial = Some(tutorial_id);
        self.tutorial_progress = TutorialProgress::for_tutorial(tutorial_id);
    }

    pub fn complete_current_step(&mut self) {
        self.tutorial_progress.success_count += 1;

        if self.tutorial_progress.success_count >= self.tutorial_progress.required_successes {
            self.next_step();
        }
    }

    pub fn next_step(&mut self) {
        self.tutorial_progress.current_step += 1;
        self.tutorial_progress.success_count = 0;

        if self.tutorial_progress.current_step >= self.tutorial_progress.steps.len() {
            self.complete_tutorial();
        }
    }

    pub fn complete_tutorial(&mut self) {
        if let Some(tutorial_id) = self.current_tutorial {
            self.completed_tutorials.push(tutorial_id);
            self.current_tutorial = None;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.tutorial_progress.time_elapsed += dt;
        self.hint_system.update(dt);
    }

    pub fn render(&self) {
        if let Some(_tutorial_id) = self.current_tutorial {
            self.render_tutorial_ui();
        }

        if self.move_list_display.visible {
            self.render_move_list();
        }

        self.hint_system.render();
    }

    fn render_tutorial_ui(&self) {
        // Draw tutorial overlay
        let bg_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, screen_width(), 150.0, bg_color);

        if let Some(step) = self.tutorial_progress.steps.get(self.tutorial_progress.current_step) {
            // Draw step title
            draw_text(step.title, 20.0, 40.0, 30.0, WHITE);

            // Draw description
            draw_text(step.description, 20.0, 70.0, 20.0, LIGHTGRAY);

            // Draw progress
            let progress_text = format!(
                "Progress: {}/{}",
                self.tutorial_progress.success_count,
                self.tutorial_progress.required_successes
            );
            draw_text(&progress_text, 20.0, 100.0, 24.0, GOLD);

            // Draw time if applicable
            if let Some(time_limit) = step.time_limit {
                let time_remaining = time_limit - self.tutorial_progress.time_elapsed;
                let time_text = format!("Time: {:.1}s", time_remaining);
                let color = if time_remaining < 10.0 { RED } else { WHITE };
                draw_text(&time_text, screen_width() - 150.0, 40.0, 24.0, color);
            }
        }
    }

    fn render_move_list(&self) {
        // Draw move list panel
        let panel_x = 50.0;
        let panel_y = 100.0;
        let panel_width = screen_width() - 100.0;
        let panel_height = screen_height() - 200.0;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height,
            Color::new(0.1, 0.1, 0.15, 0.95));

        draw_text("MOVE LIST", panel_x + 20.0, panel_y + 40.0, 40.0, GOLD);
        draw_text("Press M to close", panel_x + panel_width - 200.0, panel_y + 40.0, 20.0, LIGHTGRAY);
    }

    pub fn get_character_guide(&self, character_type: CharacterType) -> Option<&CharacterGuide> {
        self.character_guides.iter().find(|g| g.character_type == character_type)
    }
}

impl TutorialProgress {
    pub fn new() -> Self {
        Self {
            tutorial_id: TutorialId::Movement,
            current_step: 0,
            steps: Vec::new(),
            success_count: 0,
            required_successes: 1,
            time_elapsed: 0.0,
            hints_shown: Vec::new(),
        }
    }

    pub fn for_tutorial(tutorial_id: TutorialId) -> Self {
        let steps = match tutorial_id {
            TutorialId::Movement => vec![
                TutorialStep {
                    step_id: 0,
                    title: "Basic Movement",
                    description: "Use arrow keys or WASD to move your character",
                    objective: TutorialObjective::CompleteInTime(5.0),
                    demonstration: None,
                    time_limit: None,
                    hints: vec!["Hold the direction you want to move"],
                    reward: None,
                },
            ],
            TutorialId::BasicAttacks => vec![
                TutorialStep {
                    step_id: 0,
                    title: "Light Attacks",
                    description: "Press Light Punch (J) to perform a quick attack",
                    objective: TutorialObjective::LandHits(5),
                    demonstration: None,
                    time_limit: None,
                    hints: vec!["Light attacks are fast but deal less damage"],
                    reward: Some(TutorialReward {
                        reward_type: RewardType::Currency,
                        value: 50,
                    }),
                },
            ],
            _ => Vec::new(),
        };

        Self {
            tutorial_id,
            current_step: 0,
            steps,
            success_count: 0,
            required_successes: 1,
            time_elapsed: 0.0,
            hints_shown: Vec::new(),
        }
    }
}

impl HintSystem {
    pub fn new() -> Self {
        Self {
            hints_enabled: true,
            current_hints: Vec::new(),
            hint_cooldown: 0.0,
            hint_categories: Vec::new(),
        }
    }

    pub fn show_hint(&mut self, text: String, position: HintPosition, lifetime: f32, priority: u32) {
        self.current_hints.push(ActiveHint {
            text,
            lifetime: 0.0,
            max_lifetime: lifetime,
            position,
            priority,
        });

        // Sort by priority
        self.current_hints.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Limit to 3 hints
        if self.current_hints.len() > 3 {
            self.current_hints.truncate(3);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.hint_cooldown > 0.0 {
            self.hint_cooldown -= dt;
        }

        self.current_hints.retain_mut(|hint| {
            hint.lifetime += dt;
            hint.lifetime < hint.max_lifetime
        });
    }

    pub fn render(&self) {
        for hint in &self.current_hints {
            let alpha = if hint.lifetime > hint.max_lifetime - 0.5 {
                (hint.max_lifetime - hint.lifetime) / 0.5
            } else {
                1.0
            };

            let (x, y) = match hint.position {
                HintPosition::TopCenter => (screen_width() / 2.0 - 200.0, 50.0),
                HintPosition::BottomCenter => (screen_width() / 2.0 - 200.0, screen_height() - 100.0),
                HintPosition::Center => (screen_width() / 2.0 - 200.0, screen_height() / 2.0),
                _ => (100.0, 100.0),
            };

            draw_rectangle(x - 10.0, y - 10.0, 420.0, 60.0,
                Color::new(0.0, 0.0, 0.0, 0.8 * alpha));
            draw_text(&hint.text, x, y + 30.0, 20.0,
                Color::new(1.0, 1.0, 1.0, alpha));
        }
    }
}

impl Default for TutorialSystem {
    fn default() -> Self {
        Self::new()
    }
}
