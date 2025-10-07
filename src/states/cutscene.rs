use crate::states::State;
use macroquad::prelude::*;

pub struct CutsceneState {
    current_scene: usize,
    timer: f32,
    dialogue_index: usize,
    dialogues: Vec<(String, String)>,
}

impl CutsceneState {
    pub fn new() -> Self {
        Self {
            current_scene: 0,
            timer: 0.0,
            dialogue_index: 0,
            dialogues: vec![
                ("Bas".to_string(), "Kom dan! Ik veeg niks, bro!".to_string()),
                ("Meneer Wolters".to_string(), "Bas, vegen!".to_string()),
                ("Bas".to_string(), "Nee!".to_string()),
                (
                    "Meneer Wolters".to_string(),
                    "Bas! Vegen, nu meteen!".to_string(),
                ),
            ],
        }
    }
}

impl State for CutsceneState {
    fn enter(&mut self) {
        self.current_scene = 0;
        self.timer = 0.0;
        self.dialogue_index = 0;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.timer += dt;
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

        draw_rectangle(
            0.0,
            screen_height() - 200.0,
            screen_width(),
            200.0,
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        if self.dialogue_index < self.dialogues.len() {
            let (speaker, text) = &self.dialogues[self.dialogue_index];

            draw_text(speaker, 50.0, screen_height() - 150.0, 30.0, YELLOW);
            draw_text(text, 50.0, screen_height() - 100.0, 25.0, WHITE);
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            self.dialogue_index += 1;
        }
    }
}
