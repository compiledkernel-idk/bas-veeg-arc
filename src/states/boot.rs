use crate::states::State;
use macroquad::prelude::*;

pub struct BootState {
    timer: f32,
    logo_alpha: f32,
    transition_to_menu: bool,
}

impl BootState {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            logo_alpha: 0.0,
            transition_to_menu: false,
        }
    }
}

impl State for BootState {
    fn enter(&mut self) {
        self.timer = 0.0;
        self.logo_alpha = 0.0;
        self.transition_to_menu = false;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.timer += dt;

        if self.timer < 1.0 {
            self.logo_alpha = self.timer;
        } else if self.timer < 2.0 {
            self.logo_alpha = 1.0;
        } else if self.timer < 3.0 {
            self.logo_alpha = 3.0 - self.timer;
        } else {
            self.transition_to_menu = true;
        }
    }

    fn should_transition(&self) -> Option<crate::states::StateType> {
        if self.transition_to_menu {
            Some(crate::states::StateType::Menu)
        } else {
            None
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

        let text = "BAS VEEG ARC";
        let font_size = 80.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);

        draw_text(
            text,
            screen_width() * 0.5 - text_dims.width * 0.5,
            screen_height() * 0.5,
            font_size,
            Color::new(1.0, 1.0, 1.0, self.logo_alpha),
        );
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            self.timer = 3.0;
        }
    }
}
