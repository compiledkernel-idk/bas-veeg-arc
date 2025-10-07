use crate::audio::mixer::AudioMixer;
use crate::data::save::SaveManager;
use crate::render::camera::GameCamera;
use crate::states::{StateManager, StateType};
use macroquad::prelude::*;

pub const VIRTUAL_WIDTH: f32 = 1920.0;
pub const VIRTUAL_HEIGHT: f32 = 1080.0;
pub const FIXED_TIMESTEP: f64 = 1.0 / 120.0;

pub struct Application {
    state_manager: StateManager,
    camera: GameCamera,
    audio_mixer: AudioMixer,
    save_manager: SaveManager,
    accumulator: f64,
    fullscreen: bool,
    borderless: bool,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state_manager: StateManager::new(),
            camera: GameCamera::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            audio_mixer: AudioMixer::new(),
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
            self.state_manager.update(frame_time as f32);

            clear_background(BLACK);

            self.camera.apply_transform();
            self.state_manager.render(interpolation as f32);
            self.camera.reset_transform();

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
}
