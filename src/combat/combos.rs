use crate::combat::inputs::InputAction;

#[derive(Clone, Debug)]
pub struct Combo {
    pub name: String,
    pub sequence: Vec<InputAction>,
    pub window: f64,
    pub damage_scaling: f32,
    pub meter_gain: f32,
}

pub struct ComboManager {
    pub active_combo: Option<ActiveCombo>,
    pub combo_list: Vec<Combo>,
    pub damage_scaling: f32,
    pub hitstun_decay: f32,
}

#[derive(Clone, Debug)]
pub struct ActiveCombo {
    pub hits: u32,
    pub total_damage: f32,
    pub timer: f32,
    pub max_timer: f32,
}

impl ComboManager {
    pub fn new() -> Self {
        Self {
            active_combo: None,
            combo_list: Self::create_combo_list(),
            damage_scaling: 1.0,
            hitstun_decay: 1.0,
        }
    }

    fn create_combo_list() -> Vec<Combo> {
        vec![
            Combo {
                name: "Basic Chain".to_string(),
                sequence: vec![
                    InputAction::LightAttack,
                    InputAction::LightAttack,
                    InputAction::HeavyAttack,
                ],
                window: 1.0,
                damage_scaling: 0.9,
                meter_gain: 10.0,
            },
            Combo {
                name: "Launcher Combo".to_string(),
                sequence: vec![InputAction::Down, InputAction::HeavyAttack],
                window: 0.5,
                damage_scaling: 1.0,
                meter_gain: 15.0,
            },
            Combo {
                name: "Paint Rush".to_string(),
                sequence: vec![
                    InputAction::Special,
                    InputAction::LightAttack,
                    InputAction::Special,
                ],
                window: 1.2,
                damage_scaling: 0.85,
                meter_gain: 20.0,
            },
            Combo {
                name: "Ultimate Veeg".to_string(),
                sequence: vec![InputAction::Down, InputAction::Down, InputAction::Super],
                window: 1.0,
                damage_scaling: 1.5,
                meter_gain: 0.0,
            },
        ]
    }

    pub fn register_hit(&mut self, damage: f32) {
        if let Some(ref mut combo) = self.active_combo {
            combo.hits += 1;
            combo.total_damage += damage * self.damage_scaling;
            combo.timer = combo.max_timer;

            self.damage_scaling *= 0.95;
            self.hitstun_decay *= 0.98;

            if self.damage_scaling < 0.3 {
                self.damage_scaling = 0.3;
            }
            if self.hitstun_decay < 0.5 {
                self.hitstun_decay = 0.5;
            }
        } else {
            self.active_combo = Some(ActiveCombo {
                hits: 1,
                total_damage: damage,
                timer: 2.0,
                max_timer: 2.0,
            });
            self.damage_scaling = 1.0;
            self.hitstun_decay = 1.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(ref mut combo) = self.active_combo {
            combo.timer -= dt;

            if combo.timer <= 0.0 {
                self.reset();
            }
        }
    }

    pub fn reset(&mut self) {
        self.active_combo = None;
        self.damage_scaling = 1.0;
        self.hitstun_decay = 1.0;
    }

    pub fn check_combo(&self, inputs: &[InputAction]) -> Option<&Combo> {
        for combo in &self.combo_list {
            if Self::matches_sequence(inputs, &combo.sequence) {
                return Some(combo);
            }
        }
        None
    }

    fn matches_sequence(inputs: &[InputAction], sequence: &[InputAction]) -> bool {
        if inputs.len() < sequence.len() {
            return false;
        }

        let start = inputs.len() - sequence.len();
        &inputs[start..] == sequence
    }

    pub fn get_current_multiplier(&self) -> f32 {
        self.damage_scaling
    }

    pub fn get_hitstun_multiplier(&self) -> f32 {
        self.hitstun_decay
    }
}
