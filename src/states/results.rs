use crate::states::{State, StateType};
use macroquad::prelude::*;

pub struct ResultsState {
    winner: String,
    score: u32,
    time_bonus: u32,
    combo_bonus: u32,
    total_score: u32,
    display_timer: f32,
    transition_to: Option<StateType>,
    waves_completed: usize,
    enemies_defeated: u32,
}

impl ResultsState {
    pub fn new() -> Self {
        Self::with_data("PLAYER".to_string(), 0, 0, 0)
    }

    pub fn with_data(winner: String, waves_completed: usize, enemies_defeated: u32, time_survived: u32) -> Self {
        // Calculate scores based on performance
        let base_score = waves_completed as u32 * 1000 + enemies_defeated * 100;
        let time_bonus = time_survived * 10;
        let wave_bonus = if waves_completed >= 10 { 5000 } else { waves_completed as u32 * 300 };
        let combo_bonus = wave_bonus; // Simplified combo calculation
        let total_score = base_score + time_bonus + combo_bonus;

        Self {
            winner,
            score: base_score,
            time_bonus,
            combo_bonus,
            total_score,
            display_timer: 0.0,
            transition_to: None,
            waves_completed,
            enemies_defeated,
        }
    }
}

impl State for ResultsState {
    fn enter(&mut self) {
        self.display_timer = 0.0;
        self.total_score = self.score + self.time_bonus + self.combo_bonus;
        self.transition_to = None;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.display_timer += dt;
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

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
            // Show stats
            let stats_y = 550.0;
            draw_text(
                &format!("Waves Completed: {}", self.waves_completed),
                300.0,
                stats_y,
                24.0,
                Color::new(0.7, 0.9, 1.0, 1.0),
            );
            draw_text(
                &format!("Enemies Defeated: {}", self.enemies_defeated),
                300.0,
                stats_y + 35.0,
                24.0,
                Color::new(0.7, 0.9, 1.0, 1.0),
            );

            draw_text(
                "Press SPACE to continue",
                screen_width() * 0.5 - 150.0,
                650.0,
                20.0,
                Color::new(1.0, 1.0, 1.0, 0.6),
            );
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            if self.display_timer > 3.0 {
                self.transition_to = Some(StateType::Menu);
            }
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
