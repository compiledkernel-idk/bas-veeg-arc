use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Hitbox {
    pub offset: Vec2,
    pub size: Vec2,
    pub damage: f32,
    pub hitstun: f32,
    pub blockstun: f32,
    pub pushback: Vec2,
    pub launch_power: Vec2,
    pub hit_type: HitType,
    pub can_juggle: bool,
    pub armor_break: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HitType {
    Light,
    Medium,
    Heavy,
    Launcher,
    Sweep,
    Overhead,
    Special,
    Super,
    Projectile,
}

impl Hitbox {
    pub fn new_light() -> Self {
        Self {
            offset: Vec2::new(40.0, 0.0),
            size: Vec2::new(60.0, 40.0),
            damage: 5.0,
            hitstun: 0.2,
            blockstun: 0.1,
            pushback: Vec2::new(50.0, 0.0),
            launch_power: Vec2::ZERO,
            hit_type: HitType::Light,
            can_juggle: false,
            armor_break: false,
        }
    }

    pub fn new_heavy() -> Self {
        Self {
            offset: Vec2::new(50.0, 0.0),
            size: Vec2::new(80.0, 50.0),
            damage: 12.0,
            hitstun: 0.4,
            blockstun: 0.3,
            pushback: Vec2::new(100.0, 0.0),
            launch_power: Vec2::ZERO,
            hit_type: HitType::Heavy,
            can_juggle: false,
            armor_break: true,
        }
    }

    pub fn new_launcher() -> Self {
        Self {
            offset: Vec2::new(30.0, -20.0),
            size: Vec2::new(60.0, 80.0),
            damage: 10.0,
            hitstun: 0.5,
            blockstun: 0.2,
            pushback: Vec2::new(30.0, 0.0),
            launch_power: Vec2::new(0.0, -500.0),
            hit_type: HitType::Launcher,
            can_juggle: true,
            armor_break: false,
        }
    }

    pub fn new_special(special_type: SpecialType) -> Self {
        match special_type {
            SpecialType::Paintbrush => Self {
                offset: Vec2::new(60.0, 0.0),
                size: Vec2::new(120.0, 40.0),
                damage: 15.0,
                hitstun: 0.6,
                blockstun: 0.4,
                pushback: Vec2::new(150.0, 0.0),
                launch_power: Vec2::ZERO,
                hit_type: HitType::Special,
                can_juggle: false,
                armor_break: true,
            },
            SpecialType::EraserBomb => Self {
                offset: Vec2::ZERO,
                size: Vec2::new(200.0, 200.0),
                damage: 20.0,
                hitstun: 0.8,
                blockstun: 0.5,
                pushback: Vec2::new(200.0, -100.0),
                launch_power: Vec2::new(0.0, -300.0),
                hit_type: HitType::Special,
                can_juggle: true,
                armor_break: true,
            },
            SpecialType::MarkerBlast => Self {
                offset: Vec2::new(80.0, 0.0),
                size: Vec2::new(40.0, 20.0),
                damage: 8.0,
                hitstun: 0.3,
                blockstun: 0.2,
                pushback: Vec2::new(80.0, 0.0),
                launch_power: Vec2::ZERO,
                hit_type: HitType::Projectile,
                can_juggle: false,
                armor_break: false,
            },
        }
    }

    pub fn new_super() -> Self {
        Self {
            offset: Vec2::ZERO,
            size: Vec2::new(300.0, 150.0),
            damage: 35.0,
            hitstun: 1.2,
            blockstun: 0.8,
            pushback: Vec2::new(300.0, -200.0),
            launch_power: Vec2::new(0.0, -600.0),
            hit_type: HitType::Super,
            can_juggle: true,
            armor_break: true,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SpecialType {
    Paintbrush,
    EraserBomb,
    MarkerBlast,
}
