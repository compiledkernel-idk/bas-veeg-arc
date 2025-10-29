use crate::audio::mixer::AudioMixer;
use crate::data::save::SaveManager;
use crate::render::camera::GameCamera;
use crate::render::graphics_enhancement::{GraphicsEnhancement, QualityPreset};
use crate::states::{StateManager, StateType};
use macroquad::prelude::*;

pub const VIRTUAL_WIDTH: f32 = 1920.0;
pub const VIRTUAL_HEIGHT: f32 = 1080.0;
pub const FIXED_TIMESTEP: f64 = 1.0 / 120.0;

pub struct Application {
    state_manager: StateManager,
    camera: GameCamera,
    audio_mixer: AudioMixer,
    graphics_enhancement: GraphicsEnhancement,
    #[allow(dead_code)] // Future use: save/load system
    save_manager: SaveManager,
    accumulator: f64,
    fullscreen: bool,
    #[allow(dead_code)] // Future use: borderless fullscreen mode
    borderless: bool,
}

impl Application {
    pub fn new() -> Self {
        let mut graphics = GraphicsEnhancement::new();
        // Start with more moderate settings to avoid black screen
        graphics.set_quality_preset(QualityPreset::Medium);
        graphics.bloom_intensity = 0.3;
        graphics.chromatic_aberration = 0.05;
        graphics.vignette_strength = 0.15;
        graphics.ambient_light = Color::new(0.8, 0.8, 0.85, 1.0); // Much brighter ambient

        Self {
            state_manager: StateManager::new(),
            camera: GameCamera::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            audio_mixer: AudioMixer::new(),
            graphics_enhancement: graphics,
            save_manager: SaveManager::new(),
            accumulator: 0.0,
            fullscreen: false,
            borderless: false,
        }
    }

    pub async fn run(&mut self) {
        self.state_manager.push_state(StateType::Boot);

        let mut last_time = get_time();

        loop {
            let current_time = get_time();
            let frame_time = (current_time - last_time).min(0.25);
            last_time = current_time;

            self.accumulator += frame_time;

            self.handle_global_input();

            while self.accumulator >= FIXED_TIMESTEP {
                self.state_manager.fixed_update(FIXED_TIMESTEP);
                self.accumulator -= FIXED_TIMESTEP;
            }

            let interpolation = self.accumulator / FIXED_TIMESTEP;

            self.camera.update();
            self.audio_mixer.update(frame_time as f32);

            // Apply time scale for slow-motion effects
            let adjusted_frame_time = frame_time as f32 * self.graphics_enhancement.get_time_scale();
            self.state_manager.update(adjusted_frame_time);

            // Enable graphics enhancement (fixed to not use render targets)
            self.graphics_enhancement.begin_frame(frame_time as f32);

            clear_background(BLACK);

            self.camera.apply_transform();
            self.state_manager.render(interpolation as f32);
            self.camera.reset_transform();

            // Render post-processing effects (vignette, lights, flash)
            self.graphics_enhancement.end_frame();

            self.render_letterbox();

            if self.state_manager.should_quit() {
                break;
            }

            next_frame().await;
        }
    }

    fn handle_global_input(&mut self) {
        if is_key_pressed(KeyCode::F11) {
            self.toggle_fullscreen();
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state_manager.handle_escape();
        }
    }

    fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        if self.fullscreen {
            request_new_screen_size(screen_width(), screen_height());
        } else {
            request_new_screen_size(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        }
    }

    fn render_letterbox(&self) {}

    pub fn get_graphics_enhancement_mut(&mut self) -> &mut GraphicsEnhancement {
        &mut self.graphics_enhancement
    }
}
