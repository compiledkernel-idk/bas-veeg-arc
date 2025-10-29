use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterId {
    Berkay,
    Luca,
    Gefferinho,
    Bas,
    Hadi,
    Nitin,
    PalaBaba,
    Fufinho,
    EfeAbi,
    Jad,
    Umut,
    KeizerBomTaha,      // Military commander with plane mechanics
    GoonLordTobese,     // Special milk power
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbilityEffect {
    DamageBoost(f32),
    HealthBoost(f32),
    SpeedBoost(f32),
    SplashDamage(f32, f32), // damage, radius
    FireDamage(f32, f32),   // damage per second, duration
    ProjectileDamage(f32),  // For Fufinho's fufu throw
    PlaneSummon,            // Keizer Bom Taha: Enter plane mode
    ArmorBoost(f32),        // Damage reduction percentage
    Stun(f32),              // Stun duration in seconds
    Slow(f32, f32),         // Slow amount (0.0-1.0), duration
    DamageReflect(f32),     // Reflect damage percentage
    Invincibility,          // Temporary invulnerability
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
    AbilityEffect::DamageBoost(2.2), // Buffed from 1.5
    AbilityEffect::HealthBoost(35.0), // Buffed from 20.0
];

const LUCA_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::DamageBoost(2.8), // Buffed from 2.0
    AbilityEffect::HealthBoost(15.0), // Added health boost
];

const GEFFERINHO_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::SpeedBoost(2.0), // Buffed from 1.5
    AbilityEffect::DamageBoost(2.0), // Buffed from 1.3
    AbilityEffect::HealthBoost(25.0), // Buffed from 15.0
];

const BAS_EFFECTS: &[AbilityEffect] = &[AbilityEffect::SplashDamage(45.0, 180.0)]; // Buffed damage and radius

const HADI_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::SpeedBoost(3.2), // Buffed from 2.5
    AbilityEffect::DamageBoost(1.5), // Added moderate damage boost
];

const NITIN_EFFECTS: &[AbilityEffect] = &[AbilityEffect::FireDamage(8.0, 6.0)]; // Buffed from 5.0 DPS and 5.0s duration

const PALA_BABA_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::DamageBoost(3.0),
    AbilityEffect::SpeedBoost(3.0),
    AbilityEffect::HealthBoost(30.0),
];

const FUFINHO_EFFECTS: &[AbilityEffect] = &[AbilityEffect::ProjectileDamage(80.0)]; // Fufu projectile damage

const EFE_ABI_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::SpeedBoost(2.5),
    AbilityEffect::DamageBoost(2.0),
];

const JAD_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::SpeedBoost(2.2),
    AbilityEffect::DamageBoost(2.5),
];

const UMUT_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::DamageBoost(2.3),
];

const KEIZER_BOM_TAHA_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::PlaneSummon,     // Enter plane mode with bombs
];

const GOON_LORD_TOBESE_EFFECTS: &[AbilityEffect] = &[
    AbilityEffect::HealthBoost(40.0),  // Special Milk gives health
    AbilityEffect::SpeedBoost(1.8),    // Milk power speed
    AbilityEffect::DamageBoost(1.6),   // Milk power strength
];

pub const CHARACTERS: [Character; 13] = [
    Character {
        id: CharacterId::Berkay,
        name: "Berkay",
        ability_name: "Special Kebab",
        voice_line: "ik kan niet stoppen!",
        effects: BERKAY_EFFECTS,
        duration: 6.0, // Increased from 5.0
        cooldown: 12.0, // Increased from 10.0
    },
    Character {
        id: CharacterId::Luca,
        name: "Luca",
        ability_name: "Winter Arc",
        voice_line: "nee nu ben ik klaar ik ga in mijn winter arc",
        effects: LUCA_EFFECTS,
        duration: 5.0,
        cooldown: 12.0, // Increased from 10.0 due to buff
    },
    Character {
        id: CharacterId::Gefferinho,
        name: "Gefferinho",
        ability_name: "Maar Mevrouw Rage",
        voice_line: "maar mevouw wat doe je",
        effects: GEFFERINHO_EFFECTS,
        duration: 6.0, // Increased from 5.0
        cooldown: 13.0, // Increased from 10.0
    },
    Character {
        id: CharacterId::Bas,
        name: "Bas",
        ability_name: "Bas Veeg",
        voice_line: "BAS VEEG!",
        effects: BAS_EFFECTS,
        duration: 0.1, // Instant AOE burst
        cooldown: 11.0, // Increased from 10.0
    },
    Character {
        id: CharacterId::Hadi,
        name: "Hadi",
        ability_name: "Dubai Emirates",
        voice_line: "Dubai Emirates!",
        effects: HADI_EFFECTS,
        duration: 6.0, // Increased from 5.0
        cooldown: 12.0, // Increased from 10.0
    },
    Character {
        id: CharacterId::Nitin,
        name: "Nitin",
        ability_name: "Barra in je Kont",
        voice_line: "Barra in je kont!",
        effects: NITIN_EFFECTS,
        duration: 1.0, // Apply DOT effect
        cooldown: 11.0, // Increased from 10.0
    },
    Character {
        id: CharacterId::PalaBaba,
        name: "Yigit Baba",
        ability_name: "Sivas Rage",
        voice_line: "TURKIYEEEE",
        effects: PALA_BABA_EFFECTS,
        duration: 10.0,
        cooldown: 30.0,
    },
    Character {
        id: CharacterId::Fufinho,
        name: "Fufinho",
        ability_name: "Fufu Throw",
        voice_line: "ik eet fufu!",
        effects: FUFINHO_EFFECTS,
        duration: 0.1, // Instant projectile
        cooldown: 8.0,
    },
    Character {
        id: CharacterId::EfeAbi,
        name: "Efe abi",
        ability_name: "Lahmacun Power",
        voice_line: "Ik eet lahmacun!",
        effects: EFE_ABI_EFFECTS,
        duration: 5.0,
        cooldown: 15.0,
    },
    Character {
        id: CharacterId::Jad,
        name: "Jad",
        ability_name: "KFC Rage",
        voice_line: "ik eet", // Will trigger special dialogue sequence
        effects: JAD_EFFECTS,
        duration: 6.0,
        cooldown: 20.0,
    },
    Character {
        id: CharacterId::Umut,
        name: "Umut",
        ability_name: "Terraria Arc",
        voice_line: "Ik ga nu in mijn terraria arc",
        effects: UMUT_EFFECTS,
        duration: 5.0,
        cooldown: 13.0,
    },
    Character {
        id: CharacterId::KeizerBomTaha,
        name: "Keizer Bom Taha",
        ability_name: "Lucht Aanval",
        voice_line: "ik gooi bommen!",
        effects: KEIZER_BOM_TAHA_EFFECTS,
        duration: 15.0, // Max plane duration
        cooldown: 25.0,  // Long cooldown for powerful ability
    },
    Character {
        id: CharacterId::GoonLordTobese,
        name: "Goon Lord Tobese",
        ability_name: "Speciale Melk",
        voice_line: "Ik hou van padme!",
        effects: GOON_LORD_TOBESE_EFFECTS,
        duration: 7.0,
        cooldown: 14.0,
    },
];

impl Character {
    pub fn get_by_id(id: CharacterId) -> &'static Character {
        &CHARACTERS[id as usize]
    }
}

impl CharacterId {
    /// Convert CharacterId to ECS CharacterType for use in game entities
    pub fn to_character_type(self) -> crate::ecs::CharacterType {
        use crate::ecs::CharacterType as CT;
        match self {
            CharacterId::Berkay => CT::Berkay,
            CharacterId::Luca => CT::Luca,
            CharacterId::Gefferinho => CT::Gefferinho,
            CharacterId::Bas => CT::Bas,
            CharacterId::Hadi => CT::Hadi,
            CharacterId::Nitin => CT::Nitin,
            CharacterId::PalaBaba => CT::YigitBaba,
            CharacterId::Fufinho => CT::Luca, // Temporarily map to existing type
            CharacterId::EfeAbi => CT::Hadi,   // Temporarily map to existing type
            CharacterId::Jad => CT::Berkay,    // Temporarily map to existing type
            CharacterId::Umut => CT::Nitin,    // Temporarily map to existing type
            CharacterId::KeizerBomTaha => CT::KeizerBomTaha,
            CharacterId::GoonLordTobese => CT::Luca, // Temporarily map to existing type
        }
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
