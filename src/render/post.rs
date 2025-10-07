use macroquad::prelude::*;

pub struct PostProcessor {
    pub bloom_intensity: f32,
    pub chromatic_aberration: f32,
    pub vignette_intensity: f32,
    pub heat_haze_intensity: f32,
    pub screen_shake: f32,
    pub color_grading: ColorGrading,
}

#[derive(Clone, Debug)]
pub struct ColorGrading {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub tint: Color,
}

impl PostProcessor {
    pub fn new() -> Self {
        Self {
            bloom_intensity: 0.5,
            chromatic_aberration: 0.0,
            vignette_intensity: 0.3,
            heat_haze_intensity: 0.0,
            screen_shake: 0.0,
            color_grading: ColorGrading {
                brightness: 1.0,
                contrast: 1.0,
                saturation: 1.0,
                tint: WHITE,
            },
        }
    }

    pub fn apply_effects(&self) {
        if self.vignette_intensity > 0.0 {
            self.apply_vignette();
        }
    }

    fn apply_vignette(&self) {
        let width = screen_width();
        let height = screen_height();
        let center = Vec2::new(width * 0.5, height * 0.5);

        for i in 0..20 {
            let radius = width.max(height) * (1.0 - i as f32 * 0.05);
            let alpha = self.vignette_intensity * 0.02;
            draw_circle_lines(
                center.x,
                center.y,
                radius,
                2.0,
                Color::new(0.0, 0.0, 0.0, alpha),
            );
        }
    }
}
