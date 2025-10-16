use crate::states::{State, StateType};
use macroquad::prelude::*;

pub struct MenuState {
    selected_option: usize,
    options: Vec<String>,
    background_offset: f32,
    transition_to: Option<StateType>,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "START STORY".to_string(),
                "ENDLESS MODE".to_string(),
                "CONTROLS".to_string(),
                "EXIT".to_string(),
            ],
            background_offset: 0.0,
            transition_to: None,
        }
    }
}

impl State for MenuState {
    fn enter(&mut self) {
        self.selected_option = 0;
        self.transition_to = None;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.background_offset += dt * 20.0;
        if self.background_offset > 100.0 {
            self.background_offset -= 100.0;
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

        // Calculate scale factor based on screen size
        let scale_factor = (screen_width() / 1920.0).min(screen_height() / 1080.0).max(0.5);
        let sw = screen_width();
        let sh = screen_height();

        // Animated background with scaling
        for i in 0..20 {
            for j in 0..12 {
                let x = i as f32 * 100.0 * scale_factor - self.background_offset;
                let y = j as f32 * 100.0 * scale_factor;
                let size = 2.0 * scale_factor;
                draw_circle(x, y, size, Color::new(0.3, 0.2, 0.4, 0.3));
            }
        }

        let title = "BAS VEEG ARC";
        let title_size = (80.0 * scale_factor).min(100.0).max(40.0);
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            sw * 0.5 - title_dims.width * 0.5,
            sh * 0.15,
            title_size,
            WHITE,
        );

        let subtitle = "ULTIMATE EDITION";
        let subtitle_size = (25.0 * scale_factor).min(30.0).max(15.0);
        let subtitle_dims = measure_text(subtitle, None, subtitle_size as u16, 1.0);
        draw_text(
            subtitle,
            sw * 0.5 - subtitle_dims.width * 0.5,
            sh * 0.2,
            subtitle_size,
            Color::new(0.8, 0.8, 0.8, 0.8),
        );

        // Menu options with adaptive positioning
        let option_start_y = sh * 0.4;
        let option_spacing = sh * 0.08;

        for (i, option) in self.options.iter().enumerate() {
            let y = option_start_y + i as f32 * option_spacing;
            let size = (35.0 * scale_factor).min(40.0).max(20.0);
            let color = if i == self.selected_option {
                YELLOW
            } else {
                WHITE
            };

            let text_dims = measure_text(option, None, size as u16, 1.0);
            let x = sw * 0.5 - text_dims.width * 0.5;

            if i == self.selected_option {
                draw_rectangle(
                    x - 20.0 * scale_factor,
                    y - size * 0.8,
                    text_dims.width + 40.0 * scale_factor,
                    size + 10.0 * scale_factor,
                    Color::new(1.0, 1.0, 0.0, 0.2),
                );
            }

            draw_text(option, x, y, size, color);
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            if self.selected_option > 0 {
                self.selected_option -= 1;
            } else {
                self.selected_option = self.options.len() - 1;
            }
        }

        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.selected_option = (self.selected_option + 1) % self.options.len();
        }

        if is_key_pressed(KeyCode::J) || is_key_pressed(KeyCode::Enter) {
            match self.selected_option {
                0 => self.transition_to = Some(StateType::CharacterSelect),
                1 => self.transition_to = Some(StateType::EndlessMode),
                2 => self.transition_to = Some(StateType::Controls),
                3 => std::process::exit(0),
                _ => {}
            }
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
