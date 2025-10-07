use macroquad::prelude::*;

pub struct GameCamera {
    pub position: Vec2,
    pub target: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub shake_intensity: f32,
    pub shake_duration: f32,
    pub follow_speed: f32,
    pub bounds: Option<CameraBounds>,
}

#[derive(Clone, Debug)]
pub struct CameraBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl GameCamera {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            target: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            virtual_width,
            virtual_height,
            shake_intensity: 0.0,
            shake_duration: 0.0,
            follow_speed: 0.1,
            bounds: None,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.position = self.position.lerp(self.target, self.follow_speed);

        if self.shake_duration > 0.0 {
            self.shake_duration -= dt;
            let shake_offset = Vec2::new(
                rand::gen_range(-1.0, 1.0) * self.shake_intensity,
                rand::gen_range(-1.0, 1.0) * self.shake_intensity,
            );
            self.position += shake_offset;
        }

        if let Some(bounds) = &self.bounds {
            self.position.x = self.position.x.clamp(bounds.min.x, bounds.max.x);
            self.position.y = self.position.y.clamp(bounds.min.y, bounds.max.y);
        }
    }

    pub fn follow(&mut self, target: Vec2) {
        self.target = target;
    }

    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = intensity;
        self.shake_duration = duration;
    }

    pub fn set_bounds(&mut self, min: Vec2, max: Vec2) {
        self.bounds = Some(CameraBounds { min, max });
    }

    pub fn apply_transform(&self) {
        set_default_camera();
    }

    pub fn reset_transform(&self) {
        set_default_camera();
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let scale_x = screen_width / self.virtual_width;
        let scale_y = screen_height / self.virtual_height;
        let scale = scale_x.min(scale_y);

        let relative_pos = world_pos - self.position;
        let screen_x = relative_pos.x * scale + screen_width * 0.5;
        let screen_y = relative_pos.y * scale + screen_height * 0.5;

        Vec2::new(screen_x, screen_y)
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let scale_x = screen_width / self.virtual_width;
        let scale_y = screen_height / self.virtual_height;
        let scale = scale_x.min(scale_y);

        let world_x = (screen_pos.x - screen_width * 0.5) / scale + self.position.x;
        let world_y = (screen_pos.y - screen_height * 0.5) / scale + self.position.y;

        Vec2::new(world_x, world_y)
    }
}
