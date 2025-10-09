use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CharacterId {
    Berkay,
    Luca,
    Gefferinho,
    Bas,
    Hadi,
    Nitin,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbilityEffect {
    DamageBoost(f32),
    HealthBoost(f32),
    SpeedBoost(f32),
    SplashDamage(f32, f32), // damage, radius
    FireDamage(f32, f32), // damage per second, duration
}

pub struct Character {
    pub id: CharacterId,
    pub name: &'static str,
    pub ability_name: &'static str,
    pub voice_line: &'static str,
    pub effects: &'static [AbilityEffect],
    pub duration: f32,
    pub cooldown: f32,
}

const BERKAY_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::DamageBoost(1.5),
    AbilityEffect::HealthBoost(20.0),
];

const LUCA_EFFECTS: &[AbilityEffect] = &[AbilityEffect::DamageBoost(2.0)];

const GEFFERINHO_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::SpeedBoost(1.5),
    AbilityEffect::DamageBoost(1.3),
    AbilityEffect::HealthBoost(15.0),
];

const BAS_EFFECTS: &[AbilityEffect] = &[AbilityEffect::SplashDamage(30.0, 150.0)];

const HADI_EFFECTS: &[AbilityEffect] = &[AbilityEffect::SpeedBoost(2.5)];

const NITIN_EFFECTS: &[AbilityEffect] = &[AbilityEffect::FireDamage(5.0, 5.0)];

pub const CHARACTERS: [Character; 6] = [
    Character {
        id: CharacterId::Berkay,
        name: "Berkay",
        ability_name: "Special Kebab",
        voice_line: "ik kan niet stoppen!",
        effects: BERKAY_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
    Character {
        id: CharacterId::Luca,
        name: "Luca",
        ability_name: "Winter Arc",
        voice_line: "nee nu ben ik klaar ik ga in mijn winter arc",
        effects: LUCA_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
    Character {
        id: CharacterId::Gefferinho,
        name: "Gefferinho",
        ability_name: "Maar Mevrouw Rage",
        voice_line: "maar mevouw wat doe je",
        effects: GEFFERINHO_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
    Character {
        id: CharacterId::Bas,
        name: "Bas",
        ability_name: "Bas Veeg",
        voice_line: "BAS VEEG!",
        effects: BAS_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
    Character {
        id: CharacterId::Hadi,
        name: "Hadi",
        ability_name: "Dubai Emirates",
        voice_line: "Dubai Emirates!",
        effects: HADI_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
    Character {
        id: CharacterId::Nitin,
        name: "Nitin",
        ability_name: "Barra in je Kont",
        voice_line: "Barra in je kont!",
        effects: NITIN_EFFECTS,
        duration: 5.0,
        cooldown: 10.0,
    },
];

impl Character {
    pub fn get_by_id(id: CharacterId) -> &'static Character {
        &CHARACTERS[id as usize]
    }
}

pub struct AbilityState {
    pub character_id: CharacterId,
    pub active: bool,
    pub active_time: f32,
    pub cooldown_time: f32,
    pub base_damage: f32,
    pub base_speed: f32,
}

impl AbilityState {
    pub fn new(character_id: CharacterId) -> Self {
        Self {
            character_id,
            active: false,
            active_time: 0.0,
            cooldown_time: 0.0,
            base_damage: 1.0,
            base_speed: 1.0,
        }
    }

    pub fn can_activate(&self) -> bool {
        !self.active && self.cooldown_time <= 0.0
    }

    pub fn activate(&mut self) -> &'static str {
        if self.can_activate() {
            let character = Character::get_by_id(self.character_id);
            self.active = true;
            self.active_time = character.duration;
            self.cooldown_time = character.cooldown;
            return character.voice_line;
        }
        ""
    }

    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.active_time -= dt;
            if self.active_time <= 0.0 {
                self.active = false;
                self.active_time = 0.0;
            }
        }

        if self.cooldown_time > 0.0 {
            self.cooldown_time -= dt;
        }
    }

    pub fn get_damage_multiplier(&self) -> f32 {
        if !self.active {
            return 1.0;
        }

        let character = Character::get_by_id(self.character_id);
        for effect in character.effects {
            if let AbilityEffect::DamageBoost(multiplier) = effect {
                return *multiplier;
            }
        }
        1.0
    }

    pub fn get_speed_multiplier(&self) -> f32 {
        if !self.active {
            return 1.0;
        }

        let character = Character::get_by_id(self.character_id);
        for effect in character.effects {
            if let AbilityEffect::SpeedBoost(multiplier) = effect {
                return *multiplier;
            }
        }
        1.0
    }

    pub fn get_health_boost(&self) -> f32 {
        if !self.active {
            return 0.0;
        }

        let character = Character::get_by_id(self.character_id);
        for effect in character.effects {
            if let AbilityEffect::HealthBoost(boost) = effect {
                return *boost;
            }
        }
        0.0
    }

    pub fn get_splash_damage(&self) -> Option<(f32, f32)> {
        if !self.active {
            return None;
        }

        let character = Character::get_by_id(self.character_id);
        for effect in character.effects {
            if let AbilityEffect::SplashDamage(damage, radius) = effect {
                return Some((*damage, *radius));
            }
        }
        None
    }

    pub fn get_fire_damage(&self) -> Option<(f32, f32)> {
        if !self.active {
            return None;
        }

        let character = Character::get_by_id(self.character_id);
        for effect in character.effects {
            if let AbilityEffect::FireDamage(dps, duration) = effect {
                return Some((*dps, *duration));
            }
        }
        None
    }
}
