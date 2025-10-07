use macroquad::prelude::*;

pub struct SFXManager {
    impact_sounds: Vec<String>,
    voice_lines: Vec<VoiceLine>,
    environment_sounds: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct VoiceLine {
    pub character: String,
    pub line: String,
    pub audio_id: String,
    pub subtitle_key: String,
}

impl SFXManager {
    pub fn new() -> Self {
        Self {
            impact_sounds: vec![
                "hit_light".to_string(),
                "hit_heavy".to_string(),
                "block".to_string(),
                "parry".to_string(),
            ],
            voice_lines: Self::create_voice_lines(),
            environment_sounds: vec![
                "desk_crash".to_string(),
                "locker_slam".to_string(),
                "paint_splash".to_string(),
                "alarm_bell".to_string(),
            ],
        }
    }

    fn create_voice_lines() -> Vec<VoiceLine> {
        vec![
            VoiceLine {
                character: "Wolters".to_string(),
                line: "Bas, vegen!".to_string(),
                audio_id: "wolters_vegen_1".to_string(),
                subtitle_key: "wolters_sweep_command".to_string(),
            },
            VoiceLine {
                character: "Wolters".to_string(),
                line: "Bas! Vegen, nu meteen!".to_string(),
                audio_id: "wolters_vegen_2".to_string(),
                subtitle_key: "wolters_sweep_now".to_string(),
            },
            VoiceLine {
                character: "Bas".to_string(),
                line: "Kom dan! Ik veeg niks, bro!".to_string(),
                audio_id: "bas_refuse".to_string(),
                subtitle_key: "bas_refuse_sweep".to_string(),
            },
            VoiceLine {
                character: "Berkay".to_string(),
                line: "Nee bro, ze gaan zien.".to_string(),
                audio_id: "berkay_warning".to_string(),
                subtitle_key: "berkay_they_will_see".to_string(),
            },
            VoiceLine {
                character: "Luca".to_string(),
                line: "Wacht maar… ik heb een winter arc plan.".to_string(),
                audio_id: "luca_plan".to_string(),
                subtitle_key: "luca_winter_arc".to_string(),
            },
            VoiceLine {
                character: "Nitin".to_string(),
                line: "Ik ga m'n barras in hun stoppen.".to_string(),
                audio_id: "nitin_threat".to_string(),
                subtitle_key: "nitin_barras".to_string(),
            },
            VoiceLine {
                character: "Hadi".to_string(),
                line: "Aina broeg… ze gaan zien.".to_string(),
                audio_id: "hadi_warning".to_string(),
                subtitle_key: "hadi_aina".to_string(),
            },
            VoiceLine {
                character: "Bastiaan".to_string(),
                line: "Je hebt alles verpest, Bas! Mijn kunst was perfect!".to_string(),
                audio_id: "bastiaan_angry".to_string(),
                subtitle_key: "bastiaan_ruined_art".to_string(),
            },
        ]
    }

    pub fn get_impact_sound(&self, hit_type: ImpactType) -> &str {
        match hit_type {
            ImpactType::Light => &self.impact_sounds[0],
            ImpactType::Heavy => &self.impact_sounds[1],
            ImpactType::Block => &self.impact_sounds[2],
            ImpactType::Parry => &self.impact_sounds[3],
        }
    }

    pub fn get_voice_line(&self, character: &str) -> Option<&VoiceLine> {
        self.voice_lines.iter().find(|v| v.character == character)
    }

    pub fn get_random_voice_line(&self, character: &str) -> Option<&VoiceLine> {
        let character_lines: Vec<_> = self
            .voice_lines
            .iter()
            .filter(|v| v.character == character)
            .collect();

        if !character_lines.is_empty() {
            let index = rand::gen_range(0, character_lines.len());
            Some(character_lines[index])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum ImpactType {
    Light,
    Heavy,
    Block,
    Parry,
}
