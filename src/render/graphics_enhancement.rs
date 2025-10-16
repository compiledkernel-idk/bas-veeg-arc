use macroquad::prelude::*;

/// Simplified advanced graphics enhancement system that works with Macroquad
pub struct GraphicsEnhancement {
    // Post-processing effects
    pub bloom_enabled: bool,
    pub bloom_intensity: f32,
    pub chromatic_aberration: f32,
    pub vignette_strength: f32,
    pub color_grading: ColorGrading,

    // Lighting
    pub ambient_light: Color,
    pub dynamic_lights: Vec<DynamicLight>,

    // Visual quality
    pub particle_quality: ParticleQuality,
    pub shadow_quality: ShadowQuality,
    pub motion_blur_enabled: bool,

    // Performance
    render_target: Option<RenderTarget>,
}

#[derive(Clone)]
pub struct ColorGrading {
    pub temperature: f32,
    pub tint: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub gamma: f32,
}

impl Default for ColorGrading {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            tint: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            brightness: 1.0,
            gamma: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct DynamicLight {
    pub position: Vec2,
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub flicker: bool,
    pub cast_shadows: bool,
}

#[derive(Clone)]
pub enum ParticleQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Clone)]
pub enum ShadowQuality {
    None,
    Low,
    Medium,
    High,
}

impl GraphicsEnhancement {
    pub fn new() -> Self {
        Self {
            bloom_enabled: true,
            bloom_intensity: 0.5,
            chromatic_aberration: 0.1,
            vignette_strength: 0.3,
            color_grading: ColorGrading::default(),
            ambient_light: Color::new(0.2, 0.2, 0.3, 1.0),
            dynamic_lights: Vec::new(),
            particle_quality: ParticleQuality::High,
            shadow_quality: ShadowQuality::Medium,
            motion_blur_enabled: true,
            render_target: None,
        }
    }

    pub fn begin_frame(&mut self) {
        // Create render target if needed
        if self.render_target.is_none() {
            self.render_target = Some(render_target(screen_width() as u32, screen_height() as u32));
        }

        // Set render target for post-processing
        if let Some(rt) = &self.render_target {
            set_camera(&Camera2D {
                render_target: Some(rt.clone()),
                ..Default::default()
            });
        }
    }

    pub fn end_frame(&mut self) {
        // Reset to screen rendering
        set_default_camera();

        if let Some(rt) = &self.render_target {
            // Apply post-processing effects
            self.apply_post_processing(&rt.texture);
        }
    }

    fn apply_post_processing(&self, texture: &Texture2D) {
        // Clear with ambient light
        clear_background(self.ambient_light);

        // Base rendering
        let mut params = DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            ..Default::default()
        };

        // Apply color grading
        let graded_color = self.apply_color_grading(WHITE);

        // Draw main texture
        draw_texture_ex(texture, 0.0, 0.0, graded_color, params.clone());

        // Apply bloom effect
        if self.bloom_enabled {
            self.render_bloom(texture);
        }

        // Apply chromatic aberration
        if self.chromatic_aberration > 0.0 {
            self.render_chromatic_aberration(texture);
        }

        // Apply vignette
        if self.vignette_strength > 0.0 {
            self.render_vignette();
        }

        // Render dynamic lights
        for light in &self.dynamic_lights {
            self.render_light(light);
        }
    }

    fn apply_color_grading(&self, color: Color) -> Color {
        let cg = &self.color_grading;

        // Temperature adjustment
        let r = color.r * (1.0 + cg.temperature * 0.1);
        let b = color.b * (1.0 - cg.temperature * 0.1);

        // Tint adjustment
        let g = color.g * (1.0 + cg.tint * 0.05);

        // Apply contrast
        let r = ((r - 0.5) * cg.contrast + 0.5).clamp(0.0, 1.0);
        let g = ((g - 0.5) * cg.contrast + 0.5).clamp(0.0, 1.0);
        let b = ((b - 0.5) * cg.contrast + 0.5).clamp(0.0, 1.0);

        // Apply saturation
        let gray = r * 0.299 + g * 0.587 + b * 0.114;
        let r = gray + (r - gray) * cg.saturation;
        let g = gray + (g - gray) * cg.saturation;
        let b = gray + (b - gray) * cg.saturation;

        // Apply brightness
        let r = (r * cg.brightness).clamp(0.0, 1.0);
        let g = (g * cg.brightness).clamp(0.0, 1.0);
        let b = (b * cg.brightness).clamp(0.0, 1.0);

        // Apply gamma
        let r = r.powf(1.0 / cg.gamma);
        let g = g.powf(1.0 / cg.gamma);
        let b = b.powf(1.0 / cg.gamma);

        Color::new(r, g, b, color.a)
    }

    fn render_bloom(&self, texture: &Texture2D) {
        // Simplified bloom effect
        let blur_steps = 3;
        let mut bloom_color = Color::new(1.0, 1.0, 1.0, self.bloom_intensity * 0.3);

        for i in 0..blur_steps {
            let offset = (i + 1) as f32 * 2.0;
            let alpha = bloom_color.a * (1.0 - i as f32 / blur_steps as f32);
            bloom_color.a = alpha;

            // Horizontal blur
            draw_texture_ex(
                texture,
                offset,
                0.0,
                bloom_color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                texture,
                -offset,
                0.0,
                bloom_color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            // Vertical blur
            draw_texture_ex(
                texture,
                0.0,
                offset,
                bloom_color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                texture,
                0.0,
                -offset,
                bloom_color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }
    }

    fn render_chromatic_aberration(&self, texture: &Texture2D) {
        let offset = self.chromatic_aberration * 5.0;

        // Red channel offset
        draw_texture_ex(
            texture,
            -offset,
            0.0,
            Color::new(1.0, 0.0, 0.0, 0.1),
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Blue channel offset
        draw_texture_ex(
            texture,
            offset,
            0.0,
            Color::new(0.0, 0.0, 1.0, 0.1),
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    fn render_vignette(&self) {
        let w = screen_width();
        let h = screen_height();
        let center = Vec2::new(w * 0.5, h * 0.5);
        let max_dist = center.length();

        // Draw gradient circles for vignette
        for i in 0..10 {
            let t = i as f32 / 10.0;
            let radius = max_dist * (0.7 + t * 0.3);
            let alpha = self.vignette_strength * t * 0.1;

            draw_circle_lines(
                center.x,
                center.y,
                radius,
                radius * 0.1,
                Color::new(0.0, 0.0, 0.0, alpha),
            );
        }
    }

    fn render_light(&self, light: &DynamicLight) {
        let mut intensity = light.intensity;

        if light.flicker {
            let time = get_time() as f32;
            intensity *= 0.8 + (time * 10.0).sin() * 0.2;
        }

        // Draw light gradient
        for i in 0..10 {
            let t = 1.0 - (i as f32 / 10.0);
            let radius = light.radius * t;
            let alpha = intensity * t * 0.1;

            draw_circle(
                light.position.x,
                light.position.y,
                radius,
                Color::new(
                    light.color.r,
                    light.color.g,
                    light.color.b,
                    alpha,
                ),
            );
        }

        // Draw light core
        draw_circle(
            light.position.x,
            light.position.y,
            light.radius * 0.1,
            Color::new(
                light.color.r,
                light.color.g,
                light.color.b,
                intensity,
            ),
        );
    }

    pub fn add_light(&mut self, position: Vec2, color: Color, intensity: f32, radius: f32) {
        self.dynamic_lights.push(DynamicLight {
            position,
            color,
            intensity,
            radius,
            flicker: false,
            cast_shadows: false,
        });
    }

    pub fn clear_lights(&mut self) {
        self.dynamic_lights.clear();
    }

    pub fn set_quality_preset(&mut self, preset: QualityPreset) {
        match preset {
            QualityPreset::Low => {
                self.bloom_enabled = false;
                self.chromatic_aberration = 0.0;
                self.particle_quality = ParticleQuality::Low;
                self.shadow_quality = ShadowQuality::None;
                self.motion_blur_enabled = false;
            }
            QualityPreset::Medium => {
                self.bloom_enabled = true;
                self.bloom_intensity = 0.3;
                self.chromatic_aberration = 0.05;
                self.particle_quality = ParticleQuality::Medium;
                self.shadow_quality = ShadowQuality::Low;
                self.motion_blur_enabled = false;
            }
            QualityPreset::High => {
                self.bloom_enabled = true;
                self.bloom_intensity = 0.5;
                self.chromatic_aberration = 0.1;
                self.particle_quality = ParticleQuality::High;
                self.shadow_quality = ShadowQuality::Medium;
                self.motion_blur_enabled = true;
            }
            QualityPreset::Ultra => {
                self.bloom_enabled = true;
                self.bloom_intensity = 0.7;
                self.chromatic_aberration = 0.15;
                self.particle_quality = ParticleQuality::Ultra;
                self.shadow_quality = ShadowQuality::High;
                self.motion_blur_enabled = true;
            }
        }
    }
}

pub enum QualityPreset {
    Low,
    Medium,
    High,
    Ultra,
}

/// Enhanced sprite with realistic rendering properties
#[derive(Clone)]
pub struct EnhancedSprite {
    pub texture: Texture2D,
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,

    // Material properties
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: Color,
    pub normal_intensity: f32,

    // Animation
    pub frame_index: u32,
    pub frame_count: u32,
    pub animation_speed: f32,

    // Effects
    pub glow: f32,
    pub outline_color: Option<Color>,
    pub distortion: f32,
}

impl EnhancedSprite {
    pub fn new(texture: Texture2D) -> Self {
        let width = texture.width();
        let height = texture.height();
        Self {
            texture,
            position: Vec2::ZERO,
            size: Vec2::new(width, height),
            rotation: 0.0,
            color: WHITE,
            metallic: 0.0,
            roughness: 0.5,
            emissive: BLACK,
            normal_intensity: 1.0,
            frame_index: 0,
            frame_count: 1,
            animation_speed: 1.0,
            glow: 0.0,
            outline_color: None,
            distortion: 0.0,
        }
    }

    pub fn draw(&self) {
        // Calculate frame rect for animation
        let frame_width = self.texture.width() / self.frame_count as f32;
        let source_rect = Rect::new(
            frame_width * self.frame_index as f32,
            0.0,
            frame_width,
            self.texture.height(),
        );

        // Draw outline if enabled
        if let Some(outline_color) = self.outline_color {
            let outline_offsets = [
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, -1.0),
                Vec2::new(0.0, 1.0),
            ];

            for offset in &outline_offsets {
                draw_texture_ex(
                    &self.texture,
                    self.position.x + offset.x,
                    self.position.y + offset.y,
                    outline_color,
                    DrawTextureParams {
                        dest_size: Some(self.size),
                        source: Some(source_rect),
                        rotation: self.rotation,
                        ..Default::default()
                    },
                );
            }
        }

        // Draw main sprite
        let mut final_color = self.color;

        // Apply emissive
        if self.emissive != BLACK {
            final_color.r = (final_color.r + self.emissive.r * self.glow).min(1.0);
            final_color.g = (final_color.g + self.emissive.g * self.glow).min(1.0);
            final_color.b = (final_color.b + self.emissive.b * self.glow).min(1.0);
        }

        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            final_color,
            DrawTextureParams {
                dest_size: Some(self.size),
                source: Some(source_rect),
                rotation: self.rotation,
                ..Default::default()
            },
        );

        // Apply glow effect
        if self.glow > 0.0 {
            for i in 1..4 {
                let glow_scale = 1.0 + (i as f32 * 0.1);
                let glow_alpha = self.glow * (1.0 / (i as f32 * 2.0));
                let glow_color = Color::new(
                    final_color.r,
                    final_color.g,
                    final_color.b,
                    glow_alpha,
                );

                draw_texture_ex(
                    &self.texture,
                    self.position.x - (self.size.x * (glow_scale - 1.0)) * 0.5,
                    self.position.y - (self.size.y * (glow_scale - 1.0)) * 0.5,
                    glow_color,
                    DrawTextureParams {
                        dest_size: Some(self.size * glow_scale),
                        source: Some(source_rect),
                        rotation: self.rotation,
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update animation
        if self.frame_count > 1 {
            let frame_duration = 1.0 / self.animation_speed;
            self.frame_index = ((get_time() as f32 / frame_duration) as u32) % self.frame_count;
        }
    }
}