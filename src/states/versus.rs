use crate::states::State;
use macroquad::prelude::*;

pub struct VersusState {
    player1_hp: f32,
    player2_hp: f32,
    player1_pos: Vec2,
    player2_pos: Vec2,
    round_timer: f32,
}

impl VersusState {
    pub fn new() -> Self {
        Self {
            player1_hp: 100.0,
            player2_hp: 100.0,
            player1_pos: Vec2::new(400.0, 500.0),
            player2_pos: Vec2::new(800.0, 500.0),
            round_timer: 99.0,
        }
    }
}

impl State for VersusState {
    fn enter(&mut self) {
        self.player1_hp = 100.0;
        self.player2_hp = 100.0;
        self.round_timer = 99.0;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.round_timer -= dt;
        if self.round_timer < 0.0 {
            self.round_timer = 0.0;
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.15, 0.1, 0.2, 1.0));

        draw_rectangle(
            self.player1_pos.x - 30.0,
            self.player1_pos.y - 60.0,
            60.0,
            120.0,
            BLUE,
        );
        draw_rectangle(
            self.player2_pos.x - 30.0,
            self.player2_pos.y - 60.0,
            60.0,
            120.0,
            RED,
        );

        draw_rectangle(50.0, 50.0, 300.0, 30.0, Color::new(0.2, 0.0, 0.0, 0.8));
        draw_rectangle(
            50.0,
            50.0,
            300.0 * (self.player1_hp / 100.0),
            30.0,
            Color::new(0.8, 0.0, 0.0, 1.0),
        );
        draw_rectangle_lines(50.0, 50.0, 300.0, 30.0, 2.0, WHITE);
        draw_text("P1", 20.0, 70.0, 20.0, WHITE);

        draw_rectangle(
            screen_width() - 350.0,
            50.0,
            300.0,
            30.0,
            Color::new(0.2, 0.0, 0.0, 0.8),
        );
        let p2_hp_width = 300.0 * (self.player2_hp / 100.0);
        draw_rectangle(
            screen_width() - 50.0 - p2_hp_width,
            50.0,
            p2_hp_width,
            30.0,
            Color::new(0.8, 0.0, 0.0, 1.0),
        );
        draw_rectangle_lines(screen_width() - 350.0, 50.0, 300.0, 30.0, 2.0, WHITE);
        draw_text("P2", screen_width() - 45.0, 70.0, 20.0, WHITE);

        let timer_text = format!("{:02}", self.round_timer as i32);
        let timer_dims = measure_text(&timer_text, None, 60, 1.0);
        draw_text(
            &timer_text,
            screen_width() * 0.5 - timer_dims.width * 0.5,
            80.0,
            60.0,
            YELLOW,
        );
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::A) {
            self.player1_pos.x -= 5.0;
        }
        if is_key_down(KeyCode::D) {
            self.player1_pos.x += 5.0;
        }
    }
}
