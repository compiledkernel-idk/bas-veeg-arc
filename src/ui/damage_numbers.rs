use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct DamageNumber {
    pub position: Vec2,
    pub value: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
    pub is_critical: bool,
    pub is_heal: bool,
    pub combo_multiplier: f32,
    pub velocity: Vec2,
    pub scale: f32,
    pub text: String,
}

pub struct DamageNumberManager {
    active_numbers: VecDeque<DamageNumber>,
    max_numbers: usize,
    font_size: f32,
    critical_font_size: f32,
}

impl DamageNumberManager {
    pub fn new() -> Self {
        Self {
            active_numbers: VecDeque::with_capacity(100),
            max_numbers: 100,
            font_size: 24.0,
            critical_font_size: 36.0,
        }
    }

    pub fn spawn_damage(
        &mut self,
        position: Vec2,
        damage: f32,
        is_critical: bool,
        combo_multiplier: f32,
    ) {
        if self.active_numbers.len() >= self.max_numbers {
            self.active_numbers.pop_front();
        }

        let color = if is_critical {
            Color::new(1.0, 1.0, 0.0, 1.0) // Yellow for crits
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0) // White for normal
        };

        let velocity = Vec2::new(
            rand::gen_range(-50.0, 50.0),
            rand::gen_range(-200.0, -150.0),
        );

        let text = if is_critical {
            format!("CRIT! {:.0}", damage)
        } else {
            format!("{:.0}", damage)
        };

        let number = DamageNumber {
            position,
            value: damage,
            lifetime: 0.0,
            max_lifetime: if is_critical { 1.5 } else { 1.0 },
            color,
            is_critical,
            is_heal: false,
            combo_multiplier,
            velocity,
            scale: if is_critical { 1.5 } else { 1.0 },
            text,
        };

        self.active_numbers.push_back(number);
    }

    pub fn spawn_heal(&mut self, position: Vec2, heal_amount: f32) {
        if self.active_numbers.len() >= self.max_numbers {
            self.active_numbers.pop_front();
        }

        let velocity = Vec2::new(
            rand::gen_range(-30.0, 30.0),
            rand::gen_range(-100.0, -80.0),
        );

        let number = DamageNumber {
            position,
            value: heal_amount,
            lifetime: 0.0,
            max_lifetime: 1.2,
            color: Color::new(0.0, 1.0, 0.0, 1.0), // Green for heals
            is_critical: false,
            is_heal: true,
            combo_multiplier: 1.0,
            velocity,
            scale: 1.0,
            text: format!("+{:.0}", heal_amount),
        };

        self.active_numbers.push_back(number);
    }

    pub fn spawn_text(
        &mut self,
        position: Vec2,
        text: String,
        color: Color,
        duration: f32,
    ) {
        if self.active_numbers.len() >= self.max_numbers {
            self.active_numbers.pop_front();
        }

        let velocity = Vec2::new(0.0, -100.0);

        let number = DamageNumber {
            position,
            value: 0.0,
            lifetime: 0.0,
            max_lifetime: duration,
            color,
            is_critical: false,
            is_heal: false,
            combo_multiplier: 1.0,
            velocity,
            scale: 1.2,
            text,
        };

        self.active_numbers.push_back(number);
    }

    pub fn spawn_combo_text(&mut self, position: Vec2, combo_count: u32, rank: &str) {
        let color = match rank {
            "SSS" | "Legendary" => Color::new(1.0, 0.0, 1.0, 1.0),
            "SS" => GOLD,
            "S" => ORANGE,
            "A" => PURPLE,
            "B" => BLUE,
            "C" => GREEN,
            _ => WHITE,
        };

        let text = format!("{} HIT COMBO! {}", combo_count, rank);
        self.spawn_text(position, text, color, 2.0);
    }

    pub fn update(&mut self, dt: f32) {
        self.active_numbers.retain_mut(|number| {
            number.lifetime += dt;

            // Update position
            number.velocity.y += 200.0 * dt; // Gravity
            number.position += number.velocity * dt;

            // Keep if still alive
            number.lifetime < number.max_lifetime
        });
    }

    pub fn render(&self) {
        for number in &self.active_numbers {
            let alpha = 1.0 - (number.lifetime / number.max_lifetime).powf(2.0);
            let mut color = number.color;
            color.a *= alpha;

            // Calculate oscillation for critical hits
            let scale = if number.is_critical {
                let oscillation = (number.lifetime * 10.0).sin() * 0.2 + 1.0;
                number.scale * oscillation
            } else {
                number.scale
            };

            // Calculate font size
            let font_size = if number.is_critical {
                self.critical_font_size * scale
            } else {
                self.font_size * scale
            };

            // Add outline for better visibility
            let outline_color = Color::new(0.0, 0.0, 0.0, color.a * 0.8);

            // Draw outline (by drawing the text multiple times with offset)
            for x_offset in -2..=2 {
                for y_offset in -2..=2 {
                    if x_offset != 0 || y_offset != 0 {
                        draw_text(
                            &number.text,
                            number.position.x + x_offset as f32,
                            number.position.y + y_offset as f32,
                            font_size,
                            outline_color,
                        );
                    }
                }
            }

            // Draw main text
            draw_text(
                &number.text,
                number.position.x,
                number.position.y,
                font_size,
                color,
            );

            // Draw combo multiplier if significant
            if number.combo_multiplier > 1.1 && !number.is_heal {
                let multiplier_text = format!("x{:.1}", number.combo_multiplier);
                draw_text(
                    &multiplier_text,
                    number.position.x + 40.0,
                    number.position.y - 10.0,
                    font_size * 0.6,
                    Color::new(1.0, 0.5, 0.0, color.a),
                );
            }
        }
    }

    pub fn clear(&mut self) {
        self.active_numbers.clear();
    }

    pub fn spawn_status_effect(&mut self, position: Vec2, effect: &str) {
        let (text, color) = match effect {
            "BURN" => ("BURNING!", Color::new(1.0, 0.3, 0.0, 1.0)),
            "FREEZE" => ("FROZEN!", Color::new(0.5, 0.8, 1.0, 1.0)),
            "POISON" => ("POISONED!", Color::new(0.0, 0.8, 0.2, 1.0)),
            "STUN" => ("STUNNED!", Color::new(1.0, 1.0, 0.0, 1.0)),
            "SLOW" => ("SLOWED!", Color::new(0.5, 0.5, 1.0, 1.0)),
            "RAGE" => ("ENRAGED!", Color::new(1.0, 0.0, 0.0, 1.0)),
            "SHIELD" => ("SHIELDED!", Color::new(0.0, 0.5, 1.0, 1.0)),
            _ => (effect, WHITE),
        };

        self.spawn_text(position, text.to_string(), color, 1.5);
    }

    pub fn spawn_perfect_timing(&mut self, position: Vec2) {
        self.spawn_text(
            position,
            "PERFECT!".to_string(),
            Color::new(1.0, 0.0, 1.0, 1.0),
            0.8,
        );
    }

    pub fn spawn_counter(&mut self, position: Vec2) {
        self.spawn_text(
            position,
            "COUNTER!".to_string(),
            Color::new(0.0, 1.0, 1.0, 1.0),
            1.0,
        );
    }

    pub fn spawn_parry(&mut self, position: Vec2) {
        self.spawn_text(
            position,
            "PARRY!".to_string(),
            Color::new(1.0, 0.5, 0.0, 1.0),
            1.0,
        );
    }
}