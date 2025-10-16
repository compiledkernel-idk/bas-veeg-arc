use macroquad::prelude::*;

pub struct ScreenEffects {
    // Screen shake
    shake_intensity: f32,
    shake_duration: f32,
    shake_timer: f32,
    shake_offset: Vec2,

    // Hit stop / freeze frame
    hit_stop_duration: f32,
    hit_stop_timer: f32,
    is_hit_stopped: bool,

    // Chromatic aberration
    chromatic_intensity: f32,
    chromatic_timer: f32,

    // Screen flash
    flash_color: Color,
    flash_intensity: f32,
    flash_timer: f32,
}

impl ScreenEffects {
    pub fn new() -> Self {
        Self {
            shake_intensity: 0.0,
            shake_duration: 0.0,
            shake_timer: 0.0,
            shake_offset: Vec2::ZERO,

            hit_stop_duration: 0.0,
            hit_stop_timer: 0.0,
            is_hit_stopped: false,

            chromatic_intensity: 0.0,
            chromatic_timer: 0.0,

            flash_color: WHITE,
            flash_intensity: 0.0,
            flash_timer: 0.0,
        }
    }

    /// Trigger screen shake with specified intensity and duration
    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = intensity.max(self.shake_intensity);
        self.shake_duration = duration.max(self.shake_duration);
        self.shake_timer = 0.0;
    }

    /// Trigger a light shake for small hits
    pub fn light_shake(&mut self) {
        self.shake(5.0, 0.1);
    }

    /// Trigger a medium shake for heavy hits
    pub fn medium_shake(&mut self) {
        self.shake(15.0, 0.2);
        self.chromatic_aberration(0.5, 0.1);
    }

    /// Trigger a heavy shake for critical hits and explosions
    pub fn heavy_shake(&mut self) {
        self.shake(30.0, 0.4);
        self.chromatic_aberration(1.0, 0.2);
        self.flash(Color::new(1.0, 1.0, 1.0, 0.5), 0.1);
    }

    /// Trigger an extreme shake for boss attacks
    pub fn extreme_shake(&mut self) {
        self.shake(50.0, 0.6);
        self.chromatic_aberration(2.0, 0.3);
        self.flash(Color::new(1.0, 0.8, 0.0, 0.7), 0.15);
    }

    /// Trigger hit stop (freeze frame) effect
    pub fn hit_stop(&mut self, duration: f32) {
        if !self.is_hit_stopped {
            self.hit_stop_duration = duration;
            self.hit_stop_timer = 0.0;
            self.is_hit_stopped = true;
        }
    }

    /// Light hit stop for normal attacks
    pub fn light_hit_stop(&mut self) {
        self.hit_stop(0.02);
    }

    /// Medium hit stop for heavy attacks
    pub fn medium_hit_stop(&mut self) {
        self.hit_stop(0.05);
    }

    /// Heavy hit stop for critical hits
    pub fn heavy_hit_stop(&mut self) {
        self.hit_stop(0.1);
    }

    /// Trigger chromatic aberration effect
    pub fn chromatic_aberration(&mut self, intensity: f32, duration: f32) {
        self.chromatic_intensity = intensity.max(self.chromatic_intensity);
        self.chromatic_timer = duration;
    }

    /// Trigger screen flash effect
    pub fn flash(&mut self, color: Color, duration: f32) {
        self.flash_color = color;
        self.flash_intensity = 1.0;
        self.flash_timer = duration;
    }

    /// Update all screen effects
    pub fn update(&mut self, dt: f32) -> f32 {
        // Handle hit stop - returns modified delta time
        if self.is_hit_stopped {
            self.hit_stop_timer += dt;
            if self.hit_stop_timer >= self.hit_stop_duration {
                self.is_hit_stopped = false;
                let overflow = self.hit_stop_timer - self.hit_stop_duration;
                self.update_other_effects(overflow);
                return overflow;
            }
            return 0.0; // Time is stopped
        }

        self.update_other_effects(dt);
        dt
    }

    fn update_other_effects(&mut self, dt: f32) {
        // Update screen shake
        if self.shake_timer < self.shake_duration {
            self.shake_timer += dt;
            let progress = self.shake_timer / self.shake_duration;
            let current_intensity = self.shake_intensity * (1.0 - progress).powf(2.0);

            self.shake_offset = Vec2::new(
                rand::gen_range(-current_intensity, current_intensity),
                rand::gen_range(-current_intensity, current_intensity),
            );
        } else {
            self.shake_offset = Vec2::ZERO;
        }

        // Update chromatic aberration
        if self.chromatic_timer > 0.0 {
            self.chromatic_timer -= dt;
            if self.chromatic_timer <= 0.0 {
                self.chromatic_intensity = 0.0;
            }
        }

        // Update flash
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt;
            self.flash_intensity = self.flash_timer / self.flash_timer.max(0.01);
        }
    }

    /// Get the current shake offset for camera
    pub fn get_shake_offset(&self) -> Vec2 {
        self.shake_offset
    }

    /// Check if time is currently stopped
    pub fn is_time_stopped(&self) -> bool {
        self.is_hit_stopped
    }

    /// Apply screen effects rendering
    pub fn render_effects(&self) {
        // Render chromatic aberration (simulated with colored overlays)
        if self.chromatic_intensity > 0.0 {
            let offset = self.chromatic_intensity * 2.0;

            // Red channel offset
            draw_rectangle(
                -offset, 0.0,
                screen_width() + offset * 2.0,
                screen_height(),
                Color::new(1.0, 0.0, 0.0, 0.1 * self.chromatic_intensity),
            );

            // Blue channel offset
            draw_rectangle(
                offset, 0.0,
                screen_width() + offset * 2.0,
                screen_height(),
                Color::new(0.0, 0.0, 1.0, 0.1 * self.chromatic_intensity),
            );
        }

        // Render screen flash
        if self.flash_intensity > 0.0 {
            let mut color = self.flash_color;
            color.a *= self.flash_intensity;
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);
        }

        // Render hit stop vignette
        if self.is_hit_stopped {
            // Dark vignette effect during hit stop
            let vignette_size = 100.0;
            let color = Color::new(0.0, 0.0, 0.0, 0.3);

            // Top
            draw_rectangle(0.0, 0.0, screen_width(), vignette_size, color);
            // Bottom
            draw_rectangle(0.0, screen_height() - vignette_size, screen_width(), vignette_size, color);
            // Left
            draw_rectangle(0.0, 0.0, vignette_size, screen_height(), color);
            // Right
            draw_rectangle(screen_width() - vignette_size, 0.0, vignette_size, screen_height(), color);
        }
    }

    /// Apply a combat impact effect based on damage amount
    pub fn combat_impact(&mut self, damage: f32, is_critical: bool) {
        if is_critical {
            self.heavy_shake();
            self.heavy_hit_stop();
            self.flash(Color::new(1.0, 1.0, 0.0, 0.6), 0.15);
        } else if damage > 50.0 {
            self.medium_shake();
            self.medium_hit_stop();
        } else if damage > 20.0 {
            self.light_shake();
            self.light_hit_stop();
        }
    }
}