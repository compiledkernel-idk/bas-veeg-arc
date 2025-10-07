use macroquad::prelude::*;

pub struct HUD {
    health_bar: HealthBar,
    meter_bar: MeterBar,
    combo_display: ComboDisplay,
    damage_numbers: Vec<DamageNumber>,
    round_timer: RoundTimer,
}

#[derive(Clone)]
pub struct HealthBar {
    pub position: Vec2,
    pub size: Vec2,
    pub current: f32,
    pub maximum: f32,
    pub red_health: f32,
    pub player_index: u8,
}

#[derive(Clone)]
pub struct MeterBar {
    pub position: Vec2,
    pub size: Vec2,
    pub current: f32,
    pub maximum: f32,
    pub segments: u32,
}

#[derive(Clone)]
pub struct ComboDisplay {
    pub position: Vec2,
    pub hits: u32,
    pub damage: f32,
    pub timer: f32,
    pub max_timer: f32,
}

#[derive(Clone)]
pub struct DamageNumber {
    pub position: Vec2,
    pub value: f32,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub color: Color,
}

#[derive(Clone)]
pub struct RoundTimer {
    pub time_remaining: f32,
    pub position: Vec2,
}

impl HUD {
    pub fn new() -> Self {
        Self {
            health_bar: HealthBar {
                position: Vec2::new(50.0, 50.0),
                size: Vec2::new(400.0, 40.0),
                current: 100.0,
                maximum: 100.0,
                red_health: 100.0,
                player_index: 0,
            },
            meter_bar: MeterBar {
                position: Vec2::new(50.0, 100.0),
                size: Vec2::new(300.0, 25.0),
                current: 0.0,
                maximum: 100.0,
                segments: 3,
            },
            combo_display: ComboDisplay {
                position: Vec2::new(screen_width() - 200.0, 100.0),
                hits: 0,
                damage: 0.0,
                timer: 0.0,
                max_timer: 2.0,
            },
            damage_numbers: Vec::new(),
            round_timer: RoundTimer {
                time_remaining: 99.0,
                position: Vec2::new(screen_width() * 0.5, 50.0),
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.health_bar.red_health > self.health_bar.current {
            self.health_bar.red_health -= dt * 20.0;
            if self.health_bar.red_health < self.health_bar.current {
                self.health_bar.red_health = self.health_bar.current;
            }
        }

        if self.combo_display.hits > 0 {
            self.combo_display.timer -= dt;
            if self.combo_display.timer <= 0.0 {
                self.combo_display.hits = 0;
                self.combo_display.damage = 0.0;
            }
        }

        for damage_num in &mut self.damage_numbers {
            damage_num.lifetime -= dt;
            damage_num.velocity.y += 200.0 * dt;
            damage_num.position += damage_num.velocity * dt;
        }

        self.damage_numbers.retain(|d| d.lifetime > 0.0);

        self.round_timer.time_remaining -= dt;
        if self.round_timer.time_remaining < 0.0 {
            self.round_timer.time_remaining = 0.0;
        }
    }

    pub fn render(&self) {
        self.render_health_bar();
        self.render_meter_bar();
        self.render_combo_display();
        self.render_damage_numbers();
        self.render_round_timer();
    }

    fn render_health_bar(&self) {
        let pos = self.health_bar.position;
        let size = self.health_bar.size;

        draw_rectangle(pos.x, pos.y, size.x, size.y, Color::new(0.2, 0.0, 0.0, 0.8));

        let red_width = size.x * (self.health_bar.red_health / self.health_bar.maximum);
        draw_rectangle(
            pos.x,
            pos.y,
            red_width,
            size.y,
            Color::new(0.5, 0.0, 0.0, 0.8),
        );

        let health_width = size.x * (self.health_bar.current / self.health_bar.maximum);
        draw_rectangle(
            pos.x,
            pos.y,
            health_width,
            size.y,
            Color::new(0.8, 0.0, 0.0, 1.0),
        );

        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, 2.0, WHITE);

        let health_text = format!("{:.0}", self.health_bar.current);
        draw_text(
            &health_text,
            pos.x + 10.0,
            pos.y + size.y * 0.7,
            25.0,
            WHITE,
        );
    }

    fn render_meter_bar(&self) {
        let pos = self.meter_bar.position;
        let size = self.meter_bar.size;

        draw_rectangle(pos.x, pos.y, size.x, size.y, Color::new(0.0, 0.0, 0.2, 0.8));

        let meter_width = size.x * (self.meter_bar.current / self.meter_bar.maximum);
        draw_rectangle(
            pos.x,
            pos.y,
            meter_width,
            size.y,
            Color::new(0.0, 0.4, 0.8, 1.0),
        );

        let segment_width = size.x / self.meter_bar.segments as f32;
        for i in 0..self.meter_bar.segments {
            let x = pos.x + segment_width * i as f32;
            draw_line(
                x,
                pos.y,
                x,
                pos.y + size.y,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
        }

        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, 2.0, WHITE);
    }

    fn render_combo_display(&self) {
        if self.combo_display.hits > 0 {
            let pos = self.combo_display.position;
            let alpha = self.combo_display.timer / self.combo_display.max_timer;

            let combo_text = format!("{} HITS", self.combo_display.hits);
            let size = 40.0 + (self.combo_display.hits as f32 * 2.0).min(20.0);
            draw_text(
                &combo_text,
                pos.x,
                pos.y,
                size,
                Color::new(1.0, 1.0, 0.0, alpha),
            );

            let damage_text = format!("{:.0} DMG", self.combo_display.damage);
            draw_text(
                &damage_text,
                pos.x,
                pos.y + 40.0,
                25.0,
                Color::new(1.0, 0.8, 0.0, alpha),
            );
        }
    }

    fn render_damage_numbers(&self) {
        for damage_num in &self.damage_numbers {
            let alpha = damage_num.lifetime;
            let mut color = damage_num.color;
            color.a *= alpha;

            let size = 30.0 + (damage_num.value * 0.5).min(20.0);
            let text = format!("{:.0}", damage_num.value);
            draw_text(
                &text,
                damage_num.position.x,
                damage_num.position.y,
                size,
                color,
            );
        }
    }

    fn render_round_timer(&self) {
        let time_text = format!("{:02}", self.round_timer.time_remaining as i32);
        let pos = self.round_timer.position;

        draw_text(&time_text, pos.x - 30.0, pos.y, 50.0, YELLOW);
    }

    pub fn spawn_damage_number(&mut self, position: Vec2, damage: f32, critical: bool) {
        let color = if critical { GOLD } else { YELLOW };
        let velocity = Vec2::new(rand::gen_range(-50.0, 50.0), -150.0);

        self.damage_numbers.push(DamageNumber {
            position,
            value: damage,
            velocity,
            lifetime: 1.0,
            color,
        });
    }

    pub fn update_combo(&mut self, hits: u32, damage: f32) {
        self.combo_display.hits = hits;
        self.combo_display.damage = damage;
        self.combo_display.timer = self.combo_display.max_timer;
    }
}
