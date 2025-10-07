use crate::combat::inputs::InputAction;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct SuperMove {
    pub name: String,
    pub character_name: String,
    pub input_sequence: Vec<InputAction>,
    pub meter_cost: f32,
    pub damage: f32,
    pub invulnerability_frames: u32,
    pub cinematic: bool,
}

pub struct SuperManager {
    pub available_supers: Vec<SuperMove>,
    pub active_super: Option<ActiveSuper>,
}

#[derive(Clone, Debug)]
pub struct ActiveSuper {
    pub super_move: SuperMove,
    pub timer: f32,
    pub total_duration: f32,
    pub phase: SuperPhase,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuperPhase {
    Startup,
    Active,
    Recovery,
}

impl SuperManager {
    pub fn new() -> Self {
        Self {
            available_supers: Self::create_super_list(),
            active_super: None,
        }
    }

    fn create_super_list() -> Vec<SuperMove> {
        vec![
            SuperMove {
                name: "Veeg Barrage".to_string(),
                character_name: "Bas".to_string(),
                input_sequence: vec![InputAction::Down, InputAction::Down, InputAction::Super],
                meter_cost: 50.0,
                damage: 40.0,
                invulnerability_frames: 60,
                cinematic: true,
            },
            SuperMove {
                name: "Winter Arc Awakening".to_string(),
                character_name: "Luca".to_string(),
                input_sequence: vec![
                    InputAction::Special,
                    InputAction::Special,
                    InputAction::Super,
                ],
                meter_cost: 75.0,
                damage: 50.0,
                invulnerability_frames: 90,
                cinematic: true,
            },
            SuperMove {
                name: "Barras Storm".to_string(),
                character_name: "Nitin".to_string(),
                input_sequence: vec![
                    InputAction::LightAttack,
                    InputAction::HeavyAttack,
                    InputAction::Super,
                ],
                meter_cost: 50.0,
                damage: 35.0,
                invulnerability_frames: 45,
                cinematic: false,
            },
            SuperMove {
                name: "Discipline Wave".to_string(),
                character_name: "Wolters".to_string(),
                input_sequence: vec![
                    InputAction::HeavyAttack,
                    InputAction::HeavyAttack,
                    InputAction::Super,
                ],
                meter_cost: 100.0,
                damage: 60.0,
                invulnerability_frames: 120,
                cinematic: true,
            },
            SuperMove {
                name: "Perfect Art".to_string(),
                character_name: "Bastiaan".to_string(),
                input_sequence: vec![
                    InputAction::Special,
                    InputAction::HeavyAttack,
                    InputAction::Super,
                ],
                meter_cost: 100.0,
                damage: 55.0,
                invulnerability_frames: 100,
                cinematic: true,
            },
        ]
    }

    pub fn activate_super(&mut self, super_move: SuperMove) {
        self.active_super = Some(ActiveSuper {
            super_move,
            timer: 0.0,
            total_duration: 3.0,
            phase: SuperPhase::Startup,
        });
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(ref mut active) = self.active_super {
            active.timer += dt;

            if active.timer < 0.5 {
                active.phase = SuperPhase::Startup;
            } else if active.timer < 2.5 {
                active.phase = SuperPhase::Active;
            } else {
                active.phase = SuperPhase::Recovery;
            }

            if active.timer >= active.total_duration {
                self.active_super = None;
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active_super.is_some()
    }

    pub fn get_phase(&self) -> Option<SuperPhase> {
        self.active_super.as_ref().map(|s| s.phase.clone())
    }

    pub fn can_activate(&self, character: &str, meter: f32) -> Option<&SuperMove> {
        self.available_supers
            .iter()
            .find(|s| s.character_name == character && s.meter_cost <= meter)
    }
}
