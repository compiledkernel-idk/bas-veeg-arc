use crate::states::State;
use macroquad::prelude::*;

pub struct TrainingState {
    dummy_hp: f32,
    dummy_pos: Vec2,
    player_pos: Vec2,
    input_history: Vec<String>,
}

impl TrainingState {
    pub fn new() -> Self {
        Self {
            dummy_hp: 100.0,
            dummy_pos: Vec2::new(800.0, 500.0),
            player_pos: Vec2::new(400.0, 500.0),
            input_history: Vec::new(),
        }
    }
}

impl State for TrainingState {
    fn enter(&mut self) {
        self.dummy_hp = 100.0;
        self.input_history.clear();
    }

    fn exit(&mut self) {}

    fn update(&mut self, _dt: f32) {
        if is_key_pressed(KeyCode::R) {
            self.dummy_hp = 100.0;
            self.dummy_pos = Vec2::new(800.0, 500.0);
            self.player_pos = Vec2::new(400.0, 500.0);
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        draw_rectangle(
            self.player_pos.x - 30.0,
            self.player_pos.y - 60.0,
            60.0,
            120.0,
            BLUE,
        );
        draw_rectangle(
            self.dummy_pos.x - 30.0,
            self.dummy_pos.y - 60.0,
            60.0,
            120.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );

        draw_text("TRAINING MODE", 50.0, 50.0, 40.0, WHITE);
        draw_text(
            &format!("Dummy HP: {:.0}", self.dummy_hp),
            50.0,
            100.0,
            20.0,
            WHITE,
        );
        draw_text("Press R to reset", 50.0, 130.0, 20.0, GRAY);

        for (i, input) in self.input_history.iter().rev().take(10).enumerate() {
            draw_text(
                input,
                screen_width() - 200.0,
                50.0 + i as f32 * 25.0,
                20.0,
                Color::new(1.0, 1.0, 1.0, 1.0 - i as f32 * 0.1),
            );
        }
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::A) {
            self.player_pos.x -= 5.0;
            self.input_history.push("A".to_string());
        }
        if is_key_down(KeyCode::D) {
            self.player_pos.x += 5.0;
            self.input_history.push("D".to_string());
        }
        if is_key_pressed(KeyCode::J) {
            self.input_history.push("J".to_string());
        }
        if is_key_pressed(KeyCode::K) {
            self.input_history.push("K".to_string());
        }
        if is_key_pressed(KeyCode::L) {
            self.input_history.push("L".to_string());
        }
    }
}
