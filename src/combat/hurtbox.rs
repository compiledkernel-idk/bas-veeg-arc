use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Hurtbox {
    pub offset: Vec2,
    pub size: Vec2,
    pub hurtbox_type: HurtboxType,
    pub vulnerability: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HurtboxType {
    Standing,
    Crouching,
    Airborne,
    Invulnerable,
}

impl Hurtbox {
    pub fn new_standing() -> Self {
        Self {
            offset: Vec2::ZERO,
            size: Vec2::new(60.0, 120.0),
            hurtbox_type: HurtboxType::Standing,
            vulnerability: 1.0,
        }
    }

    pub fn new_crouching() -> Self {
        Self {
            offset: Vec2::new(0.0, 30.0),
            size: Vec2::new(60.0, 60.0),
            hurtbox_type: HurtboxType::Crouching,
            vulnerability: 0.8,
        }
    }

    pub fn new_airborne() -> Self {
        Self {
            offset: Vec2::ZERO,
            size: Vec2::new(60.0, 100.0),
            hurtbox_type: HurtboxType::Airborne,
            vulnerability: 1.2,
        }
    }

    pub fn new_invulnerable() -> Self {
        Self {
            offset: Vec2::ZERO,
            size: Vec2::ZERO,
            hurtbox_type: HurtboxType::Invulnerable,
            vulnerability: 0.0,
        }
    }
}
