use crate::combat::hitbox::Hitbox;
use crate::combat::hurtbox::Hurtbox;
use crate::ecs::{entity::EntityId, world::Component};
use bitflags::bitflags;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Component for Transform {}

#[derive(Clone, Debug)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32,
}

impl Component for Velocity {}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub texture: Texture2D,
    pub source_rect: Rect,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Component for Sprite {}

#[derive(Clone, Debug)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub armor: f32,
}

impl Component for Health {}

#[derive(Clone, Debug)]
pub struct Fighter {
    pub character_type: CharacterType,
    pub state: FighterState,
    pub combo_counter: u32,
    pub meter: f32,
    pub max_meter: f32,
    pub hitstun: f32,
    pub blockstun: f32,
    pub invulnerable: bool,
    pub facing: f32,
    pub attack_timer: f32,
    pub team: Team,
    pub consecutive_hits_taken: u32,
    pub hit_decay_timer: f32,
}

impl Component for Fighter {}

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterType {
    // Playable characters
    Bas,
    Berkay,
    Gefferinho,
    Hadi,
    Luca,
    Nitin,
    YigitBaba,
    // NPCs and enemies
    Wolters,
    PrefectA,
    PrefectB,
    Chef,
    Librarian,
    Coach,
    // Bosses
    Bastiaan,
    KeizerBomTaha,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FighterState {
    Idle,
    Walking,
    Jumping,
    Falling,
    Crouching,
    LightAttack,
    HeavyAttack,
    Launcher,
    Special,
    Super,
    Blocking,
    Dodging,
    Parrying,
    Hitstun,
    Blockstun,
    KnockedDown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Player,
    Ally,
    Enemy,
}

impl Team {
    pub fn is_allied(self, other: Team) -> bool {
        matches!(
            (self, other),
            (Team::Player, Team::Player)
                | (Team::Player, Team::Ally)
                | (Team::Ally, Team::Player)
                | (Team::Ally, Team::Ally)
                | (Team::Enemy, Team::Enemy)
        )
    }
}

#[derive(Clone, Debug)]
pub struct AnimationController {
    pub current_animation: String,
    pub frame: usize,
    pub timer: f32,
    pub frame_duration: f32,
    pub looping: bool,
}

impl Component for AnimationController {}

#[derive(Clone, Debug)]
pub struct CollisionBox {
    pub offset: Vec2,
    pub size: Vec2,
    pub active: bool,
}

impl Component for CollisionBox {}

#[derive(Clone, Debug)]
pub struct HitboxComponent {
    pub hitbox: Hitbox,
    pub active: bool,
    pub hits_registered: Vec<u32>,
}

impl Component for HitboxComponent {}

#[derive(Clone, Debug)]
pub struct HurtboxComponent {
    pub hurtbox: Hurtbox,
    pub active: bool,
}

impl Component for HurtboxComponent {}

#[derive(Clone, Debug)]
pub struct AIController {
    pub behavior: AIBehavior,
    pub target_entity: Option<EntityId>,
    pub state_timer: f32,
    pub reaction_delay: f32,
    pub difficulty: f32,
}

impl Component for AIController {}

#[derive(Clone, Debug, PartialEq)]
pub enum AIBehavior {
    Aggressive,
    Defensive,
    Balanced,
    Evasive,
    Support,
    Boss(BossPhase),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BossPhase {
    Phase1,
    Phase2,
    Phase3,
}

#[derive(Clone, Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage_multiplier: f32,
    pub range: f32,
    pub durability: Option<f32>,
}

impl Component for Weapon {}

#[derive(Clone, Debug, PartialEq)]
pub enum WeaponType {
    PaintbrushStaff,
    RulerSword,
    CompassDagger,
    EraserBomb,
    MarkerBlaster,
    Mop,
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub color_start: Color,
    pub color_end: Color,
    pub size_start: f32,
    pub size_end: f32,
}

impl Component for Particle {}

#[derive(Clone, Debug, PartialEq)]
pub enum ParticleType {
    Paint,
    Dust,
    Spark,
    Blood,
    Energy,
    Smoke,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CollisionLayer: u32 {
        const PLAYER = 1 << 0;
        const ENEMY = 1 << 1;
        const PROJECTILE = 1 << 2;
        const WALL = 1 << 3;
        const PLATFORM = 1 << 4;
        const INTERACTABLE = 1 << 5;
        const TRIGGER = 1 << 6;
    }
}

#[derive(Clone, Debug)]
pub struct PhysicsBody {
    pub layer: CollisionLayer,
    pub mask: CollisionLayer,
    pub gravity_scale: f32,
    pub friction: f32,
    pub restitution: f32,
    pub mass: f32,
}

impl Component for PhysicsBody {}

#[derive(Clone, Debug)]
pub struct Interactable {
    pub interaction_type: InteractionType,
    pub interaction_range: f32,
    pub cooldown: f32,
}

impl Component for Interactable {}

#[derive(Clone, Debug, PartialEq)]
pub enum InteractionType {
    Desk,
    Locker,
    Book,
    PaintBucket,
    Alarm,
}

#[derive(Clone, Debug)]
pub struct AudioEmitter {
    pub sound_id: String,
    pub volume: f32,
    pub pitch: f32,
    pub looping: bool,
    pub spatial: bool,
}

impl Component for AudioEmitter {}

#[derive(Clone, Debug)]
pub struct Bomb {
    pub damage: f32,
    pub owner_id: u32,
}

impl Component for Bomb {}
