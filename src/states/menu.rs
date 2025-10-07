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
            options: vec!["START STORY".to_string(), "EXIT".to_string()],
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
        clear_background(Color::new(0.1, 0.05, 0.15, 1.0));

        for i in 0..20 {
            for j in 0..12 {
                let x = i as f32 * 100.0 - self.background_offset;
                let y = j as f32 * 100.0;
                let size = 2.0;
                draw_circle(x, y, size, Color::new(0.3, 0.2, 0.4, 0.3));
            }
        }

        let title = "BAS VEEG ARC";
        let title_size = 100.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() * 0.5 - title_dims.width * 0.5,
            150.0,
            title_size,
            WHITE,
        );

        let subtitle = "ULTIMATE EDITION";
        let subtitle_size = 30.0;
        let subtitle_dims = measure_text(subtitle, None, subtitle_size as u16, 1.0);
        draw_text(
            subtitle,
            screen_width() * 0.5 - subtitle_dims.width * 0.5,
            200.0,
            subtitle_size,
            Color::new(0.8, 0.8, 0.8, 0.8),
        );

        for (i, option) in self.options.iter().enumerate() {
            let y = 350.0 + i as f32 * 60.0;
            let size = 40.0;
            let color = if i == self.selected_option {
                YELLOW
            } else {
                WHITE
            };

            let text_dims = measure_text(option, None, size as u16, 1.0);
            let x = screen_width() * 0.5 - text_dims.width * 0.5;

            if i == self.selected_option {
                draw_rectangle(
                    x - 20.0,
                    y - size * 0.8,
                    text_dims.width + 40.0,
                    size + 10.0,
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
                0 => self.transition_to = Some(StateType::Gameplay),
                1 => std::process::exit(0),
                _ => {}
            }
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
