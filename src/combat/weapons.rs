use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage_modifier: f32,
    pub range: f32,
    pub speed_modifier: f32,
    pub special_properties: Vec<WeaponProperty>,
}

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
pub enum WeaponProperty {
    Piercing,
    Explosive,
    Stunning,
    Poison,
    Lifesteal,
    ChainHit,
    Projectile,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        match weapon_type {
            WeaponType::PaintbrushStaff => Self {
                weapon_type,
                damage_modifier: 1.2,
                range: 120.0,
                speed_modifier: 0.9,
                special_properties: vec![WeaponProperty::ChainHit],
            },
            WeaponType::RulerSword => Self {
                weapon_type,
                damage_modifier: 1.0,
                range: 80.0,
                speed_modifier: 1.0,
                special_properties: vec![],
            },
            WeaponType::CompassDagger => Self {
                weapon_type,
                damage_modifier: 0.8,
                range: 60.0,
                speed_modifier: 1.3,
                special_properties: vec![WeaponProperty::Piercing],
            },
            WeaponType::EraserBomb => Self {
                weapon_type,
                damage_modifier: 1.5,
                range: 150.0,
                speed_modifier: 0.7,
                special_properties: vec![WeaponProperty::Explosive, WeaponProperty::Stunning],
            },
            WeaponType::MarkerBlaster => Self {
                weapon_type,
                damage_modifier: 0.7,
                range: 200.0,
                speed_modifier: 1.0,
                special_properties: vec![WeaponProperty::Projectile],
            },
            WeaponType::Mop => Self {
                weapon_type,
                damage_modifier: 1.1,
                range: 100.0,
                speed_modifier: 0.85,
                special_properties: vec![WeaponProperty::Stunning],
            },
        }
    }

    pub fn get_attack_data(&self) -> AttackData {
        AttackData {
            base_damage: 10.0 * self.damage_modifier,
            range: self.range,
            startup_frames: (8.0 / self.speed_modifier) as u32,
            active_frames: 4,
            recovery_frames: (12.0 / self.speed_modifier) as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttackData {
    pub base_damage: f32,
    pub range: f32,
    pub startup_frames: u32,
    pub active_frames: u32,
    pub recovery_frames: u32,
}
