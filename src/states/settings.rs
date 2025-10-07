use crate::states::State;
use macroquad::prelude::*;

pub struct SettingsState {
    selected_option: usize,
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
    fx_intensity: f32,
    vsync_enabled: bool,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 0.8,
            fx_intensity: 1.0,
            vsync_enabled: true,
        }
    }
}

impl State for SettingsState {
    fn enter(&mut self) {
        self.selected_option = 0;
    }

    fn exit(&mut self) {}

    fn update(&mut self, _dt: f32) {}

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        draw_text("SETTINGS", screen_width() * 0.5 - 100.0, 100.0, 50.0, WHITE);

        let options = vec![
            format!("Master Volume: {:.0}%", self.master_volume * 100.0),
            format!("SFX Volume: {:.0}%", self.sfx_volume * 100.0),
            format!("Music Volume: {:.0}%", self.music_volume * 100.0),
            format!("FX Intensity: {:.0}%", self.fx_intensity * 100.0),
            format!("VSync: {}", if self.vsync_enabled { "ON" } else { "OFF" }),
            "Back".to_string(),
        ];

        for (i, option) in options.iter().enumerate() {
            let y = 250.0 + i as f32 * 50.0;
            let color = if i == self.selected_option {
                YELLOW
            } else {
                WHITE
            };

            draw_text(option, 300.0, y, 25.0, color);

            if i < 4 && i == self.selected_option {
                let bar_width = 200.0;
                let bar_x = 600.0;
                draw_rectangle(
                    bar_x,
                    y - 20.0,
                    bar_width,
                    10.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );

                let fill_width = bar_width
                    * match i {
                        0 => self.master_volume,
                        1 => self.sfx_volume,
                        2 => self.music_volume,
                        3 => self.fx_intensity,
                        _ => 0.0,
                    };
                draw_rectangle(bar_x, y - 20.0, fill_width, 10.0, YELLOW);
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            if self.selected_option > 0 {
                self.selected_option -= 1;
            }
        }

        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            if self.selected_option < 5 {
                self.selected_option += 1;
            }
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            match self.selected_option {
                0 => self.master_volume = (self.master_volume - 0.01).max(0.0),
                1 => self.sfx_volume = (self.sfx_volume - 0.01).max(0.0),
                2 => self.music_volume = (self.music_volume - 0.01).max(0.0),
                3 => self.fx_intensity = (self.fx_intensity - 0.01).max(0.0),
                _ => {}
            }
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            match self.selected_option {
                0 => self.master_volume = (self.master_volume + 0.01).min(1.0),
                1 => self.sfx_volume = (self.sfx_volume + 0.01).min(1.0),
                2 => self.music_volume = (self.music_volume + 0.01).min(1.0),
                3 => self.fx_intensity = (self.fx_intensity + 0.01).min(1.0),
                _ => {}
            }
        }

        if is_key_pressed(KeyCode::J) || is_key_pressed(KeyCode::Enter) {
            match self.selected_option {
                4 => self.vsync_enabled = !self.vsync_enabled,
                5 => {}
                _ => {}
            }
        }
    }
}
