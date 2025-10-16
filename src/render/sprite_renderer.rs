use macroquad::prelude::*;
use std::collections::HashMap;

/// High-quality sprite with advanced rendering features
#[derive(Clone)]
pub struct Sprite {
    pub texture: Texture2D,
    pub normal_map: Option<Texture2D>,
    pub specular_map: Option<Texture2D>,
    pub emission_map: Option<Texture2D>,
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub origin: Vec2,
    pub depth: f32,

    // Material properties
    pub metallic: f32,
    pub roughness: f32,
    pub emission_strength: f32,
    pub specular_intensity: f32,

    // Animation
    pub frame_rect: Option<Rect>,
    pub animation_speed: f32,
}

impl Sprite {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            normal_map: None,
            specular_map: None,
            emission_map: None,
            position: Vec2::ZERO,
            size: Vec2::new(texture.width(), texture.height()),
            rotation: 0.0,
            color: WHITE,
            flip_x: false,
            flip_y: false,
            origin: Vec2::new(0.5, 0.5),
            depth: 0.0,
            metallic: 0.0,
            roughness: 0.5,
            emission_strength: 0.0,
            specular_intensity: 1.0,
            frame_rect: None,
            animation_speed: 1.0,
        }
    }

    pub fn with_normal_map(mut self, normal_map: Texture2D) -> Self {
        self.normal_map = Some(normal_map);
        self
    }

    pub fn with_specular_map(mut self, specular_map: Texture2D) -> Self {
        self.specular_map = Some(specular_map);
        self
    }

    pub fn with_emission_map(mut self, emission_map: Texture2D) -> Self {
        self.emission_map = Some(emission_map);
        self
    }
}

/// Advanced sprite animation system
#[derive(Clone)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<AnimationFrame>,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub looping: bool,
    pub playing: bool,
    pub speed: f32,

    // Advanced animation features
    pub interpolation: InterpolationType,
    pub events: HashMap<usize, AnimationEvent>,
}

#[derive(Clone)]
pub struct AnimationFrame {
    pub rect: Rect,
    pub duration: f32,
    pub offset: Vec2,
    pub hitboxes: Vec<Rect>,
    pub attachment_points: Vec<(String, Vec2)>,
}

#[derive(Clone, Debug)]
pub enum InterpolationType {
    None,
    Linear,
    Smooth,
    Cubic,
}

#[derive(Clone, Debug)]
pub enum AnimationEvent {
    PlaySound(String),
    SpawnEffect(String, Vec2),
    TriggerHitbox(usize),
    CameraShake(f32, f32),
}

impl Animation {
    pub fn new(name: String, frames: Vec<AnimationFrame>) -> Self {
        Self {
            name,
            frames,
            current_frame: 0,
            frame_timer: 0.0,
            looping: true,
            playing: true,
            speed: 1.0,
            interpolation: InterpolationType::None,
            events: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.playing || self.frames.is_empty() {
            return;
        }

        self.frame_timer += dt * self.speed;

        let current_frame_duration = self.frames[self.current_frame].duration;

        if self.frame_timer >= current_frame_duration {
            self.frame_timer -= current_frame_duration;

            // Check for animation events
            if let Some(event) = self.events.get(&self.current_frame) {
                // Event handling would be done by the game system
                match event {
                    AnimationEvent::PlaySound(sound_id) => {
                        // Trigger sound playback
                    }
                    AnimationEvent::SpawnEffect(effect_id, offset) => {
                        // Spawn visual effect
                    }
                    AnimationEvent::TriggerHitbox(hitbox_id) => {
                        // Activate hitbox
                    }
                    AnimationEvent::CameraShake(intensity, duration) => {
                        // Trigger camera shake
                    }
                }
            }

            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    self.playing = false;
                }
            }
        }
    }

    pub fn get_current_frame(&self) -> &AnimationFrame {
        &self.frames[self.current_frame.min(self.frames.len() - 1)]
    }

    pub fn get_interpolated_position(&self) -> Vec2 {
        if self.frames.is_empty() {
            return Vec2::ZERO;
        }

        let current = &self.frames[self.current_frame];

        match self.interpolation {
            InterpolationType::None => current.offset,
            InterpolationType::Linear => {
                let next_frame = (self.current_frame + 1) % self.frames.len();
                let next = &self.frames[next_frame];
                let t = self.frame_timer / current.duration;
                current.offset.lerp(next.offset, t)
            }
            InterpolationType::Smooth => {
                let next_frame = (self.current_frame + 1) % self.frames.len();
                let next = &self.frames[next_frame];
                let t = smooth_step(self.frame_timer / current.duration);
                current.offset.lerp(next.offset, t)
            }
            InterpolationType::Cubic => {
                let next_frame = (self.current_frame + 1) % self.frames.len();
                let next = &self.frames[next_frame];
                let t = cubic_ease_in_out(self.frame_timer / current.duration);
                current.offset.lerp(next.offset, t)
            }
        }
    }
}

/// Advanced sprite rendering system with batching and depth sorting
pub struct SpriteRenderer {
    sprites: Vec<Sprite>,
    animations: HashMap<u32, Animation>,
    sprite_batch: Vec<DrawCommand>,

    // Lighting
    ambient_light: Color,
    lights: Vec<Light2D>,

    // Post-processing
    bloom_intensity: f32,
    contrast: f32,
    saturation: f32,
    brightness: f32,

    // Performance
    max_batch_size: usize,
    culling_enabled: bool,
    viewport: Rect,
}

#[derive(Clone)]
struct DrawCommand {
    texture: Texture2D,
    source: Rect,
    dest: Rect,
    color: Color,
    rotation: f32,
    depth: f32,
}

#[derive(Clone)]
pub struct Light2D {
    pub position: Vec2,
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub falloff: f32,
    pub cast_shadows: bool,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        Self {
            sprites: Vec::with_capacity(1000),
            animations: HashMap::new(),
            sprite_batch: Vec::with_capacity(1000),
            ambient_light: Color::new(0.2, 0.2, 0.2, 1.0),
            lights: Vec::new(),
            bloom_intensity: 0.5,
            contrast: 1.0,
            saturation: 1.0,
            brightness: 1.0,
            max_batch_size: 1000,
            culling_enabled: true,
            viewport: Rect::new(0.0, 0.0, screen_width(), screen_height()),
        }
    }

    pub fn add_light(&mut self, light: Light2D) {
        self.lights.push(light);
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }

    pub fn set_ambient_light(&mut self, color: Color) {
        self.ambient_light = color;
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        // Frustum culling
        if self.culling_enabled {
            let sprite_bounds = Rect::new(
                sprite.position.x - sprite.size.x * sprite.origin.x,
                sprite.position.y - sprite.size.y * sprite.origin.y,
                sprite.size.x,
                sprite.size.y,
            );

            if !self.viewport.overlaps(&sprite_bounds) {
                return;
            }
        }

        let source = sprite.frame_rect.unwrap_or(Rect::new(
            0.0,
            0.0,
            sprite.texture.width(),
            sprite.texture.height(),
        ));

        let dest = Rect::new(
            sprite.position.x,
            sprite.position.y,
            sprite.size.x * if sprite.flip_x { -1.0 } else { 1.0 },
            sprite.size.y * if sprite.flip_y { -1.0 } else { 1.0 },
        );

        // Calculate lighting
        let mut final_color = sprite.color;
        if !self.lights.is_empty() {
            final_color = self.calculate_lighting(sprite.position, sprite.color);
        }

        self.sprite_batch.push(DrawCommand {
            texture: sprite.texture,
            source,
            dest,
            color: final_color,
            rotation: sprite.rotation,
            depth: sprite.depth,
        });
    }

    fn calculate_lighting(&self, position: Vec2, base_color: Color) -> Color {
        let mut light_accumulation = self.ambient_light;

        for light in &self.lights {
            let distance = light.position.distance(position);

            if distance < light.radius {
                let attenuation = 1.0 - (distance / light.radius).powf(light.falloff);
                let light_contribution = Color::new(
                    light.color.r * light.intensity * attenuation,
                    light.color.g * light.intensity * attenuation,
                    light.color.b * light.intensity * attenuation,
                    1.0,
                );

                light_accumulation.r = (light_accumulation.r + light_contribution.r).min(1.0);
                light_accumulation.g = (light_accumulation.g + light_contribution.g).min(1.0);
                light_accumulation.b = (light_accumulation.b + light_contribution.b).min(1.0);
            }
        }

        Color::new(
            base_color.r * light_accumulation.r,
            base_color.g * light_accumulation.g,
            base_color.b * light_accumulation.b,
            base_color.a,
        )
    }

    pub fn flush(&mut self) {
        // Sort by depth for proper layering
        self.sprite_batch.sort_by(|a, b| {
            a.depth.partial_cmp(&b.depth).unwrap()
        });

        // Batch rendering by texture
        let mut current_texture: Option<Texture2D> = None;
        let mut batch_commands = Vec::new();

        for command in &self.sprite_batch {
            if current_texture.is_none() || current_texture.unwrap() != command.texture {
                // Flush current batch
                self.render_batch(&batch_commands);
                batch_commands.clear();
                current_texture = Some(command.texture);
            }

            batch_commands.push(command.clone());

            if batch_commands.len() >= self.max_batch_size {
                self.render_batch(&batch_commands);
                batch_commands.clear();
            }
        }

        // Flush remaining commands
        if !batch_commands.is_empty() {
            self.render_batch(&batch_commands);
        }

        self.sprite_batch.clear();
    }

    fn render_batch(&self, commands: &[DrawCommand]) {
        if commands.is_empty() {
            return;
        }

        for command in commands {
            draw_texture_ex(
                &command.texture,
                command.dest.x,
                command.dest.y,
                command.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(command.dest.w, command.dest.h)),
                    source: Some(command.source),
                    rotation: command.rotation,
                    ..Default::default()
                },
            );
        }
    }

    pub fn apply_post_processing(&self, render_target: &RenderTarget) {
        // This would apply post-processing effects using shaders
        // For now, we'll apply simple color adjustments

        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            ..Default::default()
        };

        // Apply brightness, contrast, saturation adjustments
        let adjusted_color = Color::new(
            self.brightness,
            self.brightness,
            self.brightness,
            1.0,
        );

        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            adjusted_color,
            params,
        );
    }
}

// Utility functions
fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn cubic_ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powf(3.0) / 2.0
    }
}