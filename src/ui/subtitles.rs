use macroquad::prelude::*;
use std::collections::VecDeque;

pub struct SubtitleManager {
    pub active_subtitle: Option<Subtitle>,
    pub subtitle_queue: VecDeque<Subtitle>,
    pub enabled: bool,
    pub background_opacity: f32,
}

#[derive(Clone, Debug)]
pub struct Subtitle {
    pub speaker: String,
    pub text: String,
    pub duration: f32,
    pub timer: f32,
    pub color: Color,
}

impl SubtitleManager {
    pub fn new() -> Self {
        Self {
            active_subtitle: None,
            subtitle_queue: VecDeque::new(),
            enabled: true,
            background_opacity: 0.8,
        }
    }

    pub fn add_subtitle(&mut self, speaker: String, text: String, duration: f32) {
        let color = Self::get_speaker_color(&speaker);
        self.subtitle_queue.push_back(Subtitle {
            speaker,
            text,
            duration,
            timer: 0.0,
            color,
        });
    }

    pub fn update(&mut self, dt: f32) {
        if !self.enabled {
            return;
        }

        if let Some(ref mut subtitle) = self.active_subtitle {
            subtitle.timer += dt;
            if subtitle.timer >= subtitle.duration {
                self.active_subtitle = None;
            }
        }

        if self.active_subtitle.is_none() && !self.subtitle_queue.is_empty() {
            self.active_subtitle = self.subtitle_queue.pop_front();
        }
    }

    pub fn render(&self) {
        if !self.enabled || self.active_subtitle.is_none() {
            return;
        }

        if let Some(ref subtitle) = self.active_subtitle {
            let box_height = 120.0;
            let box_y = screen_height() - box_height - 50.0;

            draw_rectangle(
                0.0,
                box_y,
                screen_width(),
                box_height,
                Color::new(0.0, 0.0, 0.0, self.background_opacity),
            );

            let speaker_size = 25.0;
            let text_size = 20.0;

            draw_text(
                &subtitle.speaker,
                50.0,
                box_y + 30.0,
                speaker_size,
                subtitle.color,
            );

            draw_text(&subtitle.text, 50.0, box_y + 60.0, text_size, WHITE);

            let translation = Self::get_translation(&subtitle.text);
            if !translation.is_empty() {
                draw_text(
                    &translation,
                    50.0,
                    box_y + 85.0,
                    text_size * 0.8,
                    Color::new(0.7, 0.7, 0.7, 0.8),
                );
            }
        }
    }

    fn get_speaker_color(speaker: &str) -> Color {
        match speaker {
            "Bas" => Color::new(0.2, 0.4, 1.0, 1.0),
            "Wolters" => Color::new(1.0, 0.2, 0.2, 1.0),
            "Berkay" => Color::new(0.2, 1.0, 0.2, 1.0),
            "Luca" => Color::new(0.8, 0.8, 1.0, 1.0),
            "Nitin" => Color::new(1.0, 0.6, 0.2, 1.0),
            "Hadi" => Color::new(0.6, 0.2, 1.0, 1.0),
            "Bastiaan" => Color::new(1.0, 1.0, 0.2, 1.0),
            _ => WHITE,
        }
    }

    fn get_translation(dutch_text: &str) -> String {
        match dutch_text {
            "Bas, vegen!" => "[Clean up, Bas!]".to_string(),
            "Bas! Vegen, nu meteen!" => "[Bas! Clean up, right now!]".to_string(),
            "Kom dan! Ik veeg niks, bro!" => {
                "[Come on! I'm not cleaning anything, bro!]".to_string()
            }
            "Nee bro, ze gaan zien." => "[No bro, they'll see.]".to_string(),
            "Wacht maar… ik heb een winter arc plan." => {
                "[Just wait... I have a winter arc plan.]".to_string()
            }
            "Ik ga m'n barras in hun stoppen." => {
                "[I'm going to stuff my barras in them.]".to_string()
            }
            "Aina broeg… ze gaan zien." => "[Aina bro... they'll see.]".to_string(),
            "Je hebt alles verpest, Bas! Mijn kunst was perfect!" => {
                "[You ruined everything, Bas! My art was perfect!]".to_string()
            }
            _ => String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.active_subtitle = None;
        self.subtitle_queue.clear();
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
