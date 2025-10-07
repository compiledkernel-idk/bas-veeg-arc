use crate::states::State;
use macroquad::prelude::*;

pub struct ResultsState {
    winner: String,
    score: u32,
    time_bonus: u32,
    combo_bonus: u32,
    total_score: u32,
    display_timer: f32,
}

impl ResultsState {
    pub fn new() -> Self {
        Self {
            winner: "BAS".to_string(),
            score: 10000,
            time_bonus: 5000,
            combo_bonus: 2500,
            total_score: 17500,
            display_timer: 0.0,
        }
    }
}

impl State for ResultsState {
    fn enter(&mut self) {
        self.display_timer = 0.0;
        self.total_score = self.score + self.time_bonus + self.combo_bonus;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.display_timer += dt;
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let victory_text = format!("{} WINS!", self.winner);
        let victory_size = 80.0;
        let victory_dims = measure_text(&victory_text, None, victory_size as u16, 1.0);
        draw_text(
            &victory_text,
            screen_width() * 0.5 - victory_dims.width * 0.5,
            150.0,
            victory_size,
            GOLD,
        );

        if self.display_timer > 1.0 {
            draw_text("Base Score:", 300.0, 300.0, 30.0, WHITE);
            draw_text(&format!("{:06}", self.score), 600.0, 300.0, 30.0, YELLOW);
        }

        if self.display_timer > 1.5 {
            draw_text("Time Bonus:", 300.0, 350.0, 30.0, WHITE);
            draw_text(
                &format!("{:06}", self.time_bonus),
                600.0,
                350.0,
                30.0,
                YELLOW,
            );
        }

        if self.display_timer > 2.0 {
            draw_text("Combo Bonus:", 300.0, 400.0, 30.0, WHITE);
            draw_text(
                &format!("{:06}", self.combo_bonus),
                600.0,
                400.0,
                30.0,
                YELLOW,
            );
        }

        if self.display_timer > 2.5 {
            draw_line(300.0, 450.0, 700.0, 450.0, 2.0, WHITE);
            draw_text("TOTAL:", 300.0, 500.0, 40.0, WHITE);
            draw_text(
                &format!("{:06}", self.total_score),
                600.0,
                500.0,
                40.0,
                GOLD,
            );
        }

        if self.display_timer > 3.0 {
            draw_text(
                "Press SPACE to continue",
                screen_width() * 0.5 - 150.0,
                600.0,
                20.0,
                Color::new(1.0, 1.0, 1.0, 0.6),
            );
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            if self.display_timer > 3.0 {}
        }
    }
}
