use macroquad::prelude::*;
use crate::ecs::CharacterType;
use crate::combat::{
    character_movesets::{CharacterMoveset, MoveId, CharacterMechanics},
    combo_system::{ComboSystem, StyleRank},
    plane_system::PlaneSystem,
    boss_system::BossManager,
};
use crate::render::{EnhancedVFXSystem, ImpactType, AuraType};
use crate::progression::{
    skill_tree::SkillTree,
    character_mastery::CharacterMastery,
    achievements::AchievementManager,
    account_level::AccountProgression,
};
use std::collections::HashMap;

/// Master combat integration system that coordinates all combat-related systems
pub struct CombatIntegrationManager {
    pub character_movesets: HashMap<CharacterType, CharacterMoveset>,
    pub character_states: HashMap<u32, CharacterCombatState>,  // entity_id -> state
    pub enhanced_vfx: EnhancedVFXSystem,
    pub combo_tracker: ComboTracker,
    pub damage_calculator: DamageCalculator,
    pub hit_detector: HitDetector,
    pub frame_data_display: FrameDataDisplay,
    pub training_mode: TrainingMode,
    pub replay_system: ReplaySystem,
    pub statistics: CombatStatistics,
    pub ai_behavior: AIBehaviorManager,
}

/// Per-character combat state
#[derive(Clone)]
pub struct CharacterCombatState {
    pub character_type: CharacterType,
    pub entity_id: u32,
    pub current_move: Option<MoveId>,
    pub move_frame: u32,
    pub hitstun_remaining: u32,
    pub blockstun_remaining: u32,
    pub invincibility_frames: u32,
    pub combo_counter: u32,
    pub style_rank: StyleRank,
    pub meter: f32,
    pub max_meter: f32,
    pub health: f32,
    pub max_health: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub facing_right: bool,
    pub in_air: bool,
    pub blocking: bool,
    pub buffered_input: Option<MoveId>,
    pub cancel_window: bool,
    pub juggle_count: u32,
    pub damage_scaling: f32,
    pub status_effects: Vec<StatusEffect>,
    pub special_state: Option<SpecialCharacterState>,
}

/// Status effects that can be applied to characters
#[derive(Clone, Debug)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub intensity: f32,
    pub tick_timer: f32,
    pub tick_rate: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatusEffectType {
    Poison,
    Burn,
    Freeze,
    Stun,
    Slow,
    Armor,
    DamageBoost,
    SpeedBoost,
    Invincibility,
    Regeneration,
}

/// Special states for unique character mechanics
#[derive(Clone, Debug)]
pub enum SpecialCharacterState {
    PlaneMode {
        plane_system: PlaneSystem,
        altitude: f32,
        bombs_remaining: u32,
    },
    AuthorityMode {
        authority_meter: f32,
        intimidation_radius: f32,
        bonus_active: bool,
    },
    FoodPrep {
        prepared_items: Vec<String>,
        prep_timer: f32,
        active_recipe: Option<String>,
    },
}

/// Combo tracking and style system
pub struct ComboTracker {
    pub active_combos: HashMap<u32, ComboData>,  // entity_id -> combo data
    pub combo_decay_rate: f32,
    pub style_thresholds: Vec<(u32, StyleRank)>,
}

#[derive(Clone)]
pub struct ComboData {
    pub hit_count: u32,
    pub total_damage: f32,
    pub style_points: f32,
    pub time_since_last_hit: f32,
    pub moves_used: Vec<MoveId>,
    pub juggle_active: bool,
    pub style_rank: StyleRank,
    pub combo_dropped: bool,
}

/// Advanced damage calculation with scaling and modifiers
pub struct DamageCalculator {
    pub base_damage_multiplier: f32,
    pub combo_scaling_factor: f32,
    pub max_damage_scaling: f32,
    pub critical_hit_chance: f32,
    pub critical_multiplier: f32,
}

impl DamageCalculator {
    pub fn new() -> Self {
        Self {
            base_damage_multiplier: 1.0,
            combo_scaling_factor: 0.9,  // Each hit in combo reduces damage by 10%
            max_damage_scaling: 0.3,    // Minimum damage is 30% of original
            critical_hit_chance: 0.1,   // 10% crit chance
            critical_multiplier: 1.5,   // 1.5x damage on crit
        }
    }

    pub fn calculate_damage(
        &self,
        base_damage: f32,
        attacker_state: &CharacterCombatState,
        defender_state: &CharacterCombatState,
        hit_number: u32,
    ) -> DamageResult {
        // Base damage
        let mut damage = base_damage * self.base_damage_multiplier;

        // Apply combo scaling
        let scaling = (self.combo_scaling_factor.powi(hit_number as i32))
            .max(self.max_damage_scaling);
        damage *= scaling;

        // Apply attacker buffs
        for effect in &attacker_state.status_effects {
            if let StatusEffectType::DamageBoost = effect.effect_type {
                damage *= 1.0 + effect.intensity;
            }
        }

        // Apply defender armor
        for effect in &defender_state.status_effects {
            if let StatusEffectType::Armor = effect.effect_type {
                damage *= 1.0 - effect.intensity;
            }
        }

        // Check for critical hit
        let is_critical = rand::gen_range(0.0, 1.0) < self.critical_hit_chance;
        if is_critical {
            damage *= self.critical_multiplier;
        }

        // Counter hit bonus (if defender was attacking)
        let is_counter = defender_state.current_move.is_some() &&
                        defender_state.move_frame < 10;
        if is_counter {
            damage *= 1.2;
        }

        DamageResult {
            damage,
            is_critical,
            is_counter,
            scaling_applied: scaling,
        }
    }

    pub fn calculate_meter_gain(&self, damage: f32, hit_type: HitType) -> f32 {
        let base_gain = damage * 0.15;
        match hit_type {
            HitType::Normal => base_gain,
            HitType::Counter => base_gain * 1.5,
            HitType::Critical => base_gain * 2.0,
            HitType::Blocked => base_gain * 0.3,
        }
    }
}

pub struct DamageResult {
    pub damage: f32,
    pub is_critical: bool,
    pub is_counter: bool,
    pub scaling_applied: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum HitType {
    Normal,
    Counter,
    Critical,
    Blocked,
}

/// Hit detection with precise hitbox/hurtbox checking
pub struct HitDetector {
    pub hit_records: Vec<HitRecord>,
    pub max_records: usize,
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub attacker_id: u32,
    pub defender_id: u32,
    pub move_id: MoveId,
    pub damage: f32,
    pub position: Vec2,
    pub frame: u64,
    pub hit_type: HitType,
}

impl HitDetector {
    pub fn new() -> Self {
        Self {
            hit_records: Vec::new(),
            max_records: 100,
        }
    }

    pub fn check_hit(
        &mut self,
        attacker: &CharacterCombatState,
        defender: &CharacterCombatState,
        hitbox_pos: Vec2,
        hitbox_size: Vec2,
        hurtbox_pos: Vec2,
        hurtbox_size: Vec2,
    ) -> Option<HitResult> {
        // Check if already invincible
        if defender.invincibility_frames > 0 {
            return None;
        }

        // Simple AABB collision
        if Self::check_aabb_collision(hitbox_pos, hitbox_size, hurtbox_pos, hurtbox_size) {
            let hit_type = if defender.current_move.is_some() {
                HitType::Counter
            } else if defender.blocking {
                HitType::Blocked
            } else {
                HitType::Normal
            };

            Some(HitResult {
                hit: true,
                hit_type,
                hit_position: hitbox_pos,
            })
        } else {
            None
        }
    }

    fn check_aabb_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
        let left1 = pos1.x - size1.x / 2.0;
        let right1 = pos1.x + size1.x / 2.0;
        let top1 = pos1.y - size1.y / 2.0;
        let bottom1 = pos1.y + size1.y / 2.0;

        let left2 = pos2.x - size2.x / 2.0;
        let right2 = pos2.x + size2.x / 2.0;
        let top2 = pos2.y - size2.y / 2.0;
        let bottom2 = pos2.y + size2.y / 2.0;

        right1 > left2 && left1 < right2 && bottom1 > top2 && top1 < bottom2
    }

    pub fn record_hit(&mut self, record: HitRecord) {
        self.hit_records.push(record);
        if self.hit_records.len() > self.max_records {
            self.hit_records.remove(0);
        }
    }
}

pub struct HitResult {
    pub hit: bool,
    pub hit_type: HitType,
    pub hit_position: Vec2,
}

/// Frame data display for training mode
pub struct FrameDataDisplay {
    pub enabled: bool,
    pub show_hitboxes: bool,
    pub show_hurtboxes: bool,
    pub show_frame_advantage: bool,
    pub show_input_history: bool,
    pub input_history: Vec<InputRecord>,
    pub max_history: usize,
}

#[derive(Clone, Debug)]
pub struct InputRecord {
    pub frame: u64,
    pub move_id: MoveId,
    pub character_type: CharacterType,
}

impl FrameDataDisplay {
    pub fn new() -> Self {
        Self {
            enabled: false,
            show_hitboxes: true,
            show_hurtboxes: true,
            show_frame_advantage: true,
            show_input_history: true,
            input_history: Vec::new(),
            max_history: 20,
        }
    }

    pub fn record_input(&mut self, frame: u64, move_id: MoveId, character_type: CharacterType) {
        self.input_history.push(InputRecord {
            frame,
            move_id,
            character_type,
        });

        if self.input_history.len() > self.max_history {
            self.input_history.remove(0);
        }
    }

    pub fn render(&self, state: &CharacterCombatState, moveset: &CharacterMoveset) {
        if !self.enabled {
            return;
        }

        // Draw hitboxes
        if self.show_hitboxes {
            if let Some(move_id) = state.current_move {
                if let Some(move_data) = moveset.get_move(move_id) {
                    let hitbox_pos = state.position + move_data.hitbox_offset;
                    draw_rectangle_lines(
                        hitbox_pos.x - move_data.hitbox_size.x / 2.0,
                        hitbox_pos.y - move_data.hitbox_size.y / 2.0,
                        move_data.hitbox_size.x,
                        move_data.hitbox_size.y,
                        2.0,
                        RED,
                    );
                }
            }
        }

        // Draw hurtbox
        if self.show_hurtboxes {
            draw_rectangle_lines(
                state.position.x - 30.0,
                state.position.y - 80.0,
                60.0,
                160.0,
                2.0,
                BLUE,
            );
        }

        // Display frame data
        if self.show_frame_advantage {
            if let Some(move_id) = state.current_move {
                let text = format!("Frame: {}", state.move_frame);
                draw_text(
                    &text,
                    state.position.x - 30.0,
                    state.position.y - 100.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }
}

/// Training mode with dummy settings
pub struct TrainingMode {
    pub enabled: bool,
    pub dummy_behavior: DummyBehavior,
    pub reset_position: bool,
    pub infinite_meter: bool,
    pub infinite_health: bool,
    pub show_damage_values: bool,
    pub record_mode: bool,
    pub recorded_actions: Vec<RecordedAction>,
    pub playback_mode: bool,
    pub playback_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DummyBehavior {
    Idle,
    Crouch,
    Stand,
    Jump,
    Block,
    BlockFirstHit,
    RandomBlock,
    Counterattack,
    Record,
    Playback,
}

#[derive(Clone, Debug)]
pub struct RecordedAction {
    pub frame: u64,
    pub action_type: ActionType,
    pub move_id: Option<MoveId>,
}

#[derive(Clone, Copy, Debug)]
pub enum ActionType {
    Move(MoveId),
    Block,
    Jump,
    Crouch,
    Idle,
}

impl TrainingMode {
    pub fn new() -> Self {
        Self {
            enabled: false,
            dummy_behavior: DummyBehavior::Idle,
            reset_position: false,
            infinite_meter: true,
            infinite_health: true,
            show_damage_values: true,
            record_mode: false,
            recorded_actions: Vec::new(),
            playback_mode: false,
            playback_index: 0,
        }
    }

    pub fn record_action(&mut self, frame: u64, action: ActionType) {
        if self.record_mode {
            self.recorded_actions.push(RecordedAction {
                frame,
                action_type: action,
                move_id: match action {
                    ActionType::Move(id) => Some(id),
                    _ => None,
                },
            });
        }
    }

    pub fn start_playback(&mut self) {
        self.playback_mode = true;
        self.playback_index = 0;
    }

    pub fn get_playback_action(&mut self, frame: u64) -> Option<ActionType> {
        if !self.playback_mode || self.playback_index >= self.recorded_actions.len() {
            return None;
        }

        if self.recorded_actions[self.playback_index].frame == frame {
            let action = self.recorded_actions[self.playback_index].action_type;
            self.playback_index += 1;
            Some(action)
        } else {
            None
        }
    }
}

/// Replay system for saving and viewing matches
pub struct ReplaySystem {
    pub recording: bool,
    pub recorded_frames: Vec<FrameSnapshot>,
    pub playback_mode: bool,
    pub playback_frame: usize,
    pub playback_speed: f32,
    pub max_frames: usize,
}

#[derive(Clone)]
pub struct FrameSnapshot {
    pub frame_number: u64,
    pub character_states: Vec<CharacterCombatState>,
    pub camera_position: Vec2,
    pub events: Vec<FrameEvent>,
}

#[derive(Clone, Debug)]
pub enum FrameEvent {
    Hit {
        attacker: u32,
        defender: u32,
        damage: f32,
    },
    KO {
        victim: u32,
    },
    ComboBreak {
        player: u32,
        combo_count: u32,
    },
    SuperActivated {
        player: u32,
        move_id: MoveId,
    },
}

impl ReplaySystem {
    pub fn new() -> Self {
        Self {
            recording: false,
            recorded_frames: Vec::new(),
            playback_mode: false,
            playback_frame: 0,
            playback_speed: 1.0,
            max_frames: 36000,  // 10 minutes at 60fps
        }
    }

    pub fn start_recording(&mut self) {
        self.recording = true;
        self.recorded_frames.clear();
    }

    pub fn record_frame(&mut self, snapshot: FrameSnapshot) {
        if !self.recording {
            return;
        }

        self.recorded_frames.push(snapshot);

        if self.recorded_frames.len() > self.max_frames {
            self.recorded_frames.remove(0);
        }
    }

    pub fn stop_recording(&mut self) {
        self.recording = false;
    }

    pub fn start_playback(&mut self) {
        self.playback_mode = true;
        self.playback_frame = 0;
    }

    pub fn get_current_frame(&self) -> Option<&FrameSnapshot> {
        if self.playback_mode && self.playback_frame < self.recorded_frames.len() {
            Some(&self.recorded_frames[self.playback_frame])
        } else {
            None
        }
    }

    pub fn advance_playback(&mut self, dt: f32) {
        if !self.playback_mode {
            return;
        }

        let frames_to_advance = (dt * 60.0 * self.playback_speed) as usize;
        self.playback_frame = (self.playback_frame + frames_to_advance)
            .min(self.recorded_frames.len() - 1);
    }

    pub fn seek_frame(&mut self, frame: usize) {
        self.playback_frame = frame.min(self.recorded_frames.len() - 1);
    }

    pub fn toggle_pause(&mut self) {
        if self.playback_speed > 0.0 {
            self.playback_speed = 0.0;
        } else {
            self.playback_speed = 1.0;
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.playback_speed = speed.clamp(0.0, 2.0);
    }
}

/// Combat statistics tracking
pub struct CombatStatistics {
    pub total_damage_dealt: HashMap<u32, f32>,
    pub total_damage_taken: HashMap<u32, f32>,
    pub hits_landed: HashMap<u32, u32>,
    pub hits_taken: HashMap<u32, u32>,
    pub blocks: HashMap<u32, u32>,
    pub parries: HashMap<u32, u32>,
    pub counters: HashMap<u32, u32>,
    pub critical_hits: HashMap<u32, u32>,
    pub max_combo: HashMap<u32, u32>,
    pub meters_used: HashMap<u32, f32>,
    pub ko_count: HashMap<u32, u32>,
    pub match_start_time: f64,
    pub match_end_time: Option<f64>,
}

impl CombatStatistics {
    pub fn new() -> Self {
        Self {
            total_damage_dealt: HashMap::new(),
            total_damage_taken: HashMap::new(),
            hits_landed: HashMap::new(),
            hits_taken: HashMap::new(),
            blocks: HashMap::new(),
            parries: HashMap::new(),
            counters: HashMap::new(),
            critical_hits: HashMap::new(),
            max_combo: HashMap::new(),
            meters_used: HashMap::new(),
            ko_count: HashMap::new(),
            match_start_time: get_time(),
            match_end_time: None,
        }
    }

    pub fn record_damage_dealt(&mut self, entity_id: u32, damage: f32) {
        *self.total_damage_dealt.entry(entity_id).or_insert(0.0) += damage;
    }

    pub fn record_damage_taken(&mut self, entity_id: u32, damage: f32) {
        *self.total_damage_taken.entry(entity_id).or_insert(0.0) += damage;
    }

    pub fn record_hit_landed(&mut self, entity_id: u32) {
        *self.hits_landed.entry(entity_id).or_insert(0) += 1;
    }

    pub fn record_combo(&mut self, entity_id: u32, combo_count: u32) {
        let current_max = self.max_combo.entry(entity_id).or_insert(0);
        *current_max = (*current_max).max(combo_count);
    }

    pub fn end_match(&mut self) {
        self.match_end_time = Some(get_time());
    }

    pub fn get_match_duration(&self) -> f64 {
        if let Some(end_time) = self.match_end_time {
            end_time - self.match_start_time
        } else {
            get_time() - self.match_start_time
        }
    }

    pub fn get_dps(&self, entity_id: u32) -> f32 {
        let duration = self.get_match_duration() as f32;
        if duration > 0.0 {
            self.total_damage_dealt.get(&entity_id).unwrap_or(&0.0) / duration
        } else {
            0.0
        }
    }

    pub fn get_accuracy(&self, entity_id: u32) -> f32 {
        let hits = *self.hits_landed.get(&entity_id).unwrap_or(&0) as f32;
        let total_attacks = hits + 10.0;  // Simplified - would track total attacks
        if total_attacks > 0.0 {
            (hits / total_attacks) * 100.0
        } else {
            0.0
        }
    }
}

/// AI behavior manager for single-player modes
pub struct AIBehaviorManager {
    pub difficulty: AIDifficulty,
    pub reaction_time: f32,
    pub combo_knowledge: f32,
    pub blocking_skill: f32,
    pub aggression: f32,
    pub decision_timer: f32,
    pub current_strategy: AIStrategy,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIDifficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
    Expert,
    Master,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIStrategy {
    Aggressive,      // Rushdown, pressure
    Defensive,       // Defensive, counter-based
    Balanced,        // Mix of both
    Zoning,          // Keep distance, projectiles
    Adaptive,        // Changes based on player behavior
}

impl AIBehaviorManager {
    pub fn new(difficulty: AIDifficulty) -> Self {
        let (reaction_time, combo_knowledge, blocking_skill, aggression) = match difficulty {
            AIDifficulty::VeryEasy => (0.8, 0.2, 0.1, 0.3),
            AIDifficulty::Easy => (0.5, 0.4, 0.3, 0.4),
            AIDifficulty::Medium => (0.3, 0.6, 0.5, 0.5),
            AIDifficulty::Hard => (0.2, 0.8, 0.7, 0.6),
            AIDifficulty::VeryHard => (0.1, 0.9, 0.85, 0.7),
            AIDifficulty::Expert => (0.05, 0.95, 0.95, 0.8),
            AIDifficulty::Master => (0.02, 1.0, 1.0, 0.9),
        };

        Self {
            difficulty,
            reaction_time,
            combo_knowledge,
            blocking_skill,
            aggression,
            decision_timer: 0.0,
            current_strategy: AIStrategy::Balanced,
        }
    }

    pub fn decide_action(&mut self, state: &CharacterCombatState, opponent_state: &CharacterCombatState, dt: f32) -> AIAction {
        self.decision_timer += dt;

        // React based on difficulty
        if self.decision_timer < self.reaction_time {
            return AIAction::Wait;
        }

        self.decision_timer = 0.0;

        // Check if should block
        if opponent_state.current_move.is_some() {
            let distance = (state.position - opponent_state.position).length();
            if distance < 150.0 && rand::gen_range(0.0, 1.0) < self.blocking_skill {
                return AIAction::Block;
            }
        }

        // Decide on attack or movement based on strategy
        match self.current_strategy {
            AIStrategy::Aggressive => {
                if rand::gen_range(0.0, 1.0) < self.aggression {
                    AIAction::Attack(Self::choose_aggressive_move())
                } else {
                    AIAction::MoveForward
                }
            }
            AIStrategy::Defensive => {
                if opponent_state.current_move.is_some() {
                    AIAction::Block
                } else if rand::gen_range(0.0, 1.0) < 0.3 {
                    AIAction::Attack(Self::choose_counter_move())
                } else {
                    AIAction::MoveBackward
                }
            }
            AIStrategy::Balanced => {
                if rand::gen_range(0.0, 1.0) < 0.5 {
                    AIAction::Attack(Self::choose_balanced_move())
                } else {
                    AIAction::Wait
                }
            }
            AIStrategy::Zoning => {
                let distance = (state.position - opponent_state.position).length();
                if distance < 200.0 {
                    AIAction::MoveBackward
                } else {
                    AIAction::Attack(Self::choose_projectile_move())
                }
            }
            AIStrategy::Adaptive => {
                // Adapt based on opponent's health
                if opponent_state.health < opponent_state.max_health * 0.3 {
                    self.current_strategy = AIStrategy::Aggressive;
                } else if state.health < state.max_health * 0.3 {
                    self.current_strategy = AIStrategy::Defensive;
                }
                AIAction::Wait
            }
        }
    }

    fn choose_aggressive_move() -> MoveId {
        // Would choose from aggressive moveset
        MoveId::HeavyPunch
    }

    fn choose_counter_move() -> MoveId {
        // Would choose from counter moveset
        MoveId::LightPunch
    }

    fn choose_balanced_move() -> MoveId {
        // Would choose from balanced moveset
        MoveId::LightPunch
    }

    fn choose_projectile_move() -> MoveId {
        // Would choose from projectile moveset
        MoveId::LightKick
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AIAction {
    Wait,
    Attack(MoveId),
    Block,
    MoveForward,
    MoveBackward,
    Jump,
    Dash,
}

impl CombatIntegrationManager {
    pub fn new() -> Self {
        let mut character_movesets = HashMap::new();

        // Load movesets for all characters
        for char_type in [
            CharacterType::KeizerBomTaha,
        ] {
            character_movesets.insert(char_type, CharacterMoveset::for_character(char_type));
        }

        Self {
            character_movesets,
            character_states: HashMap::new(),
            enhanced_vfx: EnhancedVFXSystem::new(),
            combo_tracker: ComboTracker::new(),
            damage_calculator: DamageCalculator::new(),
            hit_detector: HitDetector::new(),
            frame_data_display: FrameDataDisplay::new(),
            training_mode: TrainingMode::new(),
            replay_system: ReplaySystem::new(),
            statistics: CombatStatistics::new(),
            ai_behavior: AIBehaviorManager::new(AIDifficulty::Medium),
        }
    }

    pub fn register_character(&mut self, entity_id: u32, character_type: CharacterType, position: Vec2) {
        if let Some(moveset) = self.character_movesets.get(&character_type) {
            let state = CharacterCombatState {
                character_type,
                entity_id,
                current_move: None,
                move_frame: 0,
                hitstun_remaining: 0,
                blockstun_remaining: 0,
                invincibility_frames: 0,
                combo_counter: 0,
                style_rank: StyleRank::D,
                meter: 0.0,
                max_meter: 100.0,
                health: moveset.stats.max_health,
                max_health: moveset.stats.max_health,
                position,
                velocity: Vec2::ZERO,
                facing_right: true,
                in_air: false,
                blocking: false,
                buffered_input: None,
                cancel_window: false,
                juggle_count: 0,
                damage_scaling: 1.0,
                status_effects: Vec::new(),
                special_state: None,
            };

            self.character_states.insert(entity_id, state);
            self.combo_tracker.register_character(entity_id);
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update VFX
        self.enhanced_vfx.update(dt);

        // Update character states
        for state in self.character_states.values_mut() {
            // Update frame counters
            if state.current_move.is_some() {
                state.move_frame += 1;
            }

            if state.hitstun_remaining > 0 {
                state.hitstun_remaining -= 1;
            }

            if state.blockstun_remaining > 0 {
                state.blockstun_remaining -= 1;
            }

            if state.invincibility_frames > 0 {
                state.invincibility_frames -= 1;
            }

            // Update status effects
            state.status_effects.retain_mut(|effect| {
                effect.duration -= dt;
                effect.tick_timer += dt;

                // Apply tick effects
                if effect.tick_timer >= effect.tick_rate {
                    effect.tick_timer = 0.0;
                    match effect.effect_type {
                        StatusEffectType::Poison | StatusEffectType::Burn => {
                            state.health -= effect.intensity;
                        }
                        StatusEffectType::Regeneration => {
                            state.health = (state.health + effect.intensity).min(state.max_health);
                        }
                        _ => {}
                    }
                }

                effect.duration > 0.0
            });
        }

        // Update combo tracker
        self.combo_tracker.update(dt);

        // Update replay system
        if self.replay_system.playback_mode {
            self.replay_system.advance_playback(dt);
        }
    }

    pub fn process_hit(
        &mut self,
        attacker_id: u32,
        defender_id: u32,
        move_id: MoveId,
        hit_position: Vec2,
    ) {
        let attacker_state = self.character_states.get(&attacker_id).cloned();
        let defender_state = self.character_states.get(&defender_id).cloned();

        if let (Some(attacker), Some(mut defender)) = (attacker_state, defender_state) {
            // Get move data
            if let Some(moveset) = self.character_movesets.get(&attacker.character_type) {
                if let Some(move_data) = moveset.get_move(move_id) {
                    // Calculate damage
                    let combo_count = self.combo_tracker.get_combo_count(attacker_id);
                    let damage_result = self.damage_calculator.calculate_damage(
                        move_data.damage,
                        &attacker,
                        &defender,
                        combo_count,
                    );

                    // Apply damage
                    defender.health -= damage_result.damage;

                    // Record statistics
                    self.statistics.record_damage_dealt(attacker_id, damage_result.damage);
                    self.statistics.record_damage_taken(defender_id, damage_result.damage);
                    self.statistics.record_hit_landed(attacker_id);

                    // Update combo
                    self.combo_tracker.add_hit(attacker_id, move_id, damage_result.damage);

                    // Spawn VFX
                    let impact_type = if damage_result.is_critical {
                        ImpactType::Critical
                    } else if damage_result.is_counter {
                        ImpactType::Counter
                    } else {
                        ImpactType::Heavy
                    };

                    self.enhanced_vfx.spawn_impact(
                        hit_position,
                        (defender.position - attacker.position).normalize(),
                        impact_type,
                    );

                    self.enhanced_vfx.show_damage_number(
                        hit_position,
                        damage_result.damage,
                        damage_result.is_critical,
                    );

                    // Update defender state
                    self.character_states.insert(defender_id, defender);
                }
            }
        }
    }
}

impl ComboTracker {
    pub fn new() -> Self {
        Self {
            active_combos: HashMap::new(),
            combo_decay_rate: 1.0,
            style_thresholds: vec![
                (3, StyleRank::D),
                (5, StyleRank::C),
                (8, StyleRank::B),
                (12, StyleRank::A),
                (20, StyleRank::S),
                (30, StyleRank::SS),
                (50, StyleRank::SSS),
            ],
        }
    }

    pub fn register_character(&mut self, entity_id: u32) {
        self.active_combos.insert(entity_id, ComboData {
            hit_count: 0,
            total_damage: 0.0,
            style_points: 0.0,
            time_since_last_hit: 0.0,
            moves_used: Vec::new(),
            juggle_active: false,
            style_rank: StyleRank::D,
            combo_dropped: false,
        });
    }

    pub fn add_hit(&mut self, entity_id: u32, move_id: MoveId, damage: f32) {
        if let Some(combo) = self.active_combos.get_mut(&entity_id) {
            combo.hit_count += 1;
            combo.total_damage += damage;
            combo.time_since_last_hit = 0.0;
            combo.moves_used.push(move_id);
            combo.combo_dropped = false;

            // Calculate style points
            combo.style_points += damage * 0.1;

            // Update style rank
            for (threshold, rank) in &self.style_thresholds {
                if combo.hit_count >= *threshold {
                    combo.style_rank = *rank;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for combo in self.active_combos.values_mut() {
            combo.time_since_last_hit += dt;

            // Drop combo if too much time has passed
            if combo.time_since_last_hit > self.combo_decay_rate && combo.hit_count > 0 {
                combo.combo_dropped = true;
                combo.hit_count = 0;
                combo.total_damage = 0.0;
                combo.style_points = 0.0;
                combo.moves_used.clear();
                combo.style_rank = StyleRank::D;
            }
        }
    }

    pub fn get_combo_count(&self, entity_id: u32) -> u32 {
        self.active_combos.get(&entity_id).map(|c| c.hit_count).unwrap_or(0)
    }
}
