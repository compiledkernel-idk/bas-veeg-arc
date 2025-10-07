use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct AIController {
    pub behavior: AIBehavior,
    pub difficulty: f32,
    pub reaction_time: f32,
    pub decision_timer: f32,
    pub target_position: Vec2,
    pub last_player_action: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AIBehavior {
    Aggressive,
    Defensive,
    Balanced,
    Evasive,
    Boss(BossPhase),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BossPhase {
    Phase1,
    Phase2,
    Phase3,
}

impl AIController {
    pub fn new(behavior: AIBehavior, difficulty: f32) -> Self {
        Self {
            behavior,
            difficulty: difficulty.clamp(0.0, 1.0),
            reaction_time: 0.3 * (1.0 - difficulty * 0.5),
            decision_timer: 0.0,
            target_position: Vec2::ZERO,
            last_player_action: None,
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec2, ai_pos: Vec2, player_health: f32) {
        self.decision_timer -= dt;

        if self.decision_timer <= 0.0 {
            self.make_decision(player_pos, ai_pos, player_health);
            self.decision_timer = self.reaction_time;
        }
    }

    fn make_decision(&mut self, player_pos: Vec2, ai_pos: Vec2, player_health: f32) {
        let distance = (player_pos - ai_pos).length();

        match self.behavior.clone() {
            AIBehavior::Aggressive => {
                if distance > 100.0 {
                    self.target_position = player_pos;
                }
            }
            AIBehavior::Defensive => {
                if distance < 150.0 {
                    self.target_position = ai_pos - (player_pos - ai_pos).normalize() * 50.0;
                }
            }
            AIBehavior::Balanced => {
                if distance > 200.0 {
                    self.target_position = player_pos;
                } else if distance < 80.0 {
                    self.target_position = ai_pos - (player_pos - ai_pos).normalize() * 30.0;
                }
            }
            AIBehavior::Evasive => {
                if distance < 200.0 {
                    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                    self.target_position = ai_pos + Vec2::new(angle.cos(), angle.sin()) * 100.0;
                }
            }
            AIBehavior::Boss(phase) => {
                self.boss_behavior(&phase, player_pos, ai_pos, player_health);
            }
        }
    }

    fn boss_behavior(
        &mut self,
        phase: &BossPhase,
        player_pos: Vec2,
        ai_pos: Vec2,
        player_health: f32,
    ) {
        let distance = (player_pos - ai_pos).length();

        match phase {
            BossPhase::Phase1 => {
                if distance > 150.0 {
                    self.target_position = player_pos;
                }
            }
            BossPhase::Phase2 => {
                let pattern_time = get_time() as f32 * 2.0;
                let offset = Vec2::new(pattern_time.cos() * 150.0, 0.0);
                self.target_position = player_pos + offset;
            }
            BossPhase::Phase3 => {
                if player_health < 30.0 {
                    self.target_position = player_pos;
                } else {
                    let angle = get_time() as f32 * 3.0;
                    self.target_position = player_pos + Vec2::new(angle.cos(), angle.sin()) * 200.0;
                }
            }
        }
    }

    pub fn get_action(&self, distance: f32) -> AIAction {
        let random = rand::gen_range(0.0, 1.0);

        match &self.behavior {
            AIBehavior::Aggressive => {
                if distance < 80.0 {
                    if random < 0.4 {
                        AIAction::LightAttack
                    } else if random < 0.7 {
                        AIAction::HeavyAttack
                    } else {
                        AIAction::Special
                    }
                } else {
                    AIAction::MoveToward
                }
            }
            AIBehavior::Defensive => {
                if distance < 100.0 {
                    if random < 0.6 {
                        AIAction::Block
                    } else if random < 0.8 {
                        AIAction::Parry
                    } else {
                        AIAction::MoveAway
                    }
                } else {
                    AIAction::Wait
                }
            }
            AIBehavior::Balanced => {
                if distance < 80.0 {
                    if random < 0.3 {
                        AIAction::LightAttack
                    } else if random < 0.5 {
                        AIAction::Block
                    } else if random < 0.7 {
                        AIAction::HeavyAttack
                    } else {
                        AIAction::Dodge
                    }
                } else if distance < 200.0 {
                    if random < 0.5 {
                        AIAction::MoveToward
                    } else {
                        AIAction::Special
                    }
                } else {
                    AIAction::MoveToward
                }
            }
            AIBehavior::Evasive => {
                if distance < 120.0 {
                    if random < 0.7 {
                        AIAction::Dodge
                    } else {
                        AIAction::MoveAway
                    }
                } else {
                    AIAction::Wait
                }
            }
            AIBehavior::Boss(_) => {
                if distance < 100.0 {
                    if random < 0.3 {
                        AIAction::Super
                    } else if random < 0.6 {
                        AIAction::Special
                    } else {
                        AIAction::HeavyAttack
                    }
                } else {
                    AIAction::MoveToward
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AIAction {
    MoveToward,
    MoveAway,
    Jump,
    LightAttack,
    HeavyAttack,
    Special,
    Super,
    Block,
    Parry,
    Dodge,
    Wait,
}
