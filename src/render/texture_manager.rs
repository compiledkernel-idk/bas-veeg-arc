use macroquad::prelude::*;
use std::collections::HashMap;
use crate::render::graphics_enhancement::EnhancedSprite;

/// Manages procedurally generated textures for the game
pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
    sprites: HashMap<String, EnhancedSprite>,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut manager = Self {
            textures: HashMap::new(),
            sprites: HashMap::new(),
        };
        manager.generate_all_textures();
        manager
    }

    fn generate_all_textures(&mut self) {
        // Generate character textures
        self.generate_character_texture("bas", Color::new(0.85, 0.65, 0.45, 1.0));
        self.generate_character_texture("berkay", Color::new(1.0, 0.85, 0.70, 1.0));
        self.generate_character_texture("gefferinho", Color::new(0.65, 0.45, 0.30, 1.0));
        self.generate_character_texture("hadi", Color::new(0.65, 0.45, 0.30, 1.0));
        self.generate_character_texture("nitin", Color::new(0.65, 0.45, 0.30, 1.0));
        self.generate_character_texture("luca", Color::new(1.0, 0.85, 0.70, 1.0));
        self.generate_character_texture("palababa", Color::new(1.0, 0.90, 0.80, 1.0));

        // Generate effect textures
        self.generate_explosion_texture();
        self.generate_lightning_texture();
        self.generate_fire_texture();
        self.generate_ice_texture();
        self.generate_impact_texture();
        self.generate_aura_texture();
    }

    fn generate_character_texture(&mut self, name: &str, skin_color: Color) {
        let size = 128;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        // Draw character with realistic shading
        for y in 0..size {
            for x in 0..size {
                let cx = size as f32 / 2.0;
                let cy = size as f32 / 2.0;

                // Head region (upper third)
                if y < size / 3 {
                    let head_radius = size as f32 * 0.2;
                    let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy * 0.5).powi(2)).sqrt();

                    if dist < head_radius {
                        // Apply lighting gradient for 3D effect
                        let light_factor = 1.0 - (dist / head_radius) * 0.3;
                        let shaded_color = Color::new(
                            skin_color.r * light_factor,
                            skin_color.g * light_factor,
                            skin_color.b * light_factor,
                            1.0
                        );
                        image.set_pixel(x as u32, y as u32, shaded_color);

                        // Add highlight for realism
                        let highlight_dist = ((x as f32 - (cx - head_radius * 0.3)).powi(2) +
                                            (y as f32 - (cy * 0.5 - head_radius * 0.3)).powi(2)).sqrt();
                        if highlight_dist < head_radius * 0.2 {
                            let highlight = Color::new(
                                (skin_color.r + 0.2).min(1.0),
                                (skin_color.g + 0.2).min(1.0),
                                (skin_color.b + 0.2).min(1.0),
                                0.3
                            );
                            let existing = image.get_pixel(x as u32, y as u32);
                            image.set_pixel(x as u32, y as u32, Color::new(
                                existing.r * 0.7 + highlight.r * 0.3,
                                existing.g * 0.7 + highlight.g * 0.3,
                                existing.b * 0.7 + highlight.b * 0.3,
                                1.0
                            ));
                        }
                    }
                }

                // Body region (middle third)
                else if y < (size * 2) / 3 {
                    let body_width = size as f32 * 0.35;
                    let body_height = size as f32 * 0.3;
                    let body_y = cy;

                    if (x as f32 - cx).abs() < body_width / 2.0 &&
                       (y as f32 - body_y).abs() < body_height / 2.0 {
                        // Shirt color with shading
                        let shirt_base = match name {
                            "bas" => Color::new(0.8, 0.2, 0.2, 1.0), // Red
                            "berkay" => Color::new(0.2, 0.2, 0.8, 1.0), // Blue
                            "gefferinho" => Color::new(0.2, 0.8, 0.2, 1.0), // Green
                            "hadi" => Color::new(0.8, 0.8, 0.2, 1.0), // Yellow
                            "nitin" => Color::new(0.8, 0.2, 0.8, 1.0), // Purple
                            "luca" => Color::new(0.2, 0.8, 0.8, 1.0), // Cyan
                            _ => Color::new(0.5, 0.5, 0.5, 1.0), // Gray
                        };

                        // Add muscle definition with shadows
                        let muscle_shadow = ((x as f32 - cx).abs() / (body_width / 2.0)) * 0.2;
                        let shaded_shirt = Color::new(
                            shirt_base.r * (1.0 - muscle_shadow),
                            shirt_base.g * (1.0 - muscle_shadow),
                            shirt_base.b * (1.0 - muscle_shadow),
                            1.0
                        );
                        image.set_pixel(x as u32, y as u32, shaded_shirt);
                    }
                }

                // Legs region (lower third)
                else {
                    let leg_separation = size as f32 * 0.08;
                    let leg_width = size as f32 * 0.12;
                    let leg_height = size as f32 * 0.3;
                    let legs_y = cy * 1.5;

                    // Left leg
                    if (x as f32 - (cx - leg_separation)).abs() < leg_width / 2.0 &&
                       y as f32 > legs_y && (y as f32) < legs_y + leg_height {
                        let pants_color = Color::new(0.2, 0.2, 0.3, 1.0);
                        let shadow = ((y as f32 - legs_y) / leg_height) * 0.3;
                        image.set_pixel(x as u32, y as u32, Color::new(
                            pants_color.r * (1.0 - shadow),
                            pants_color.g * (1.0 - shadow),
                            pants_color.b * (1.0 - shadow),
                            1.0
                        ));
                    }

                    // Right leg
                    if (x as f32 - (cx + leg_separation)).abs() < leg_width / 2.0 &&
                       y as f32 > legs_y && (y as f32) < legs_y + leg_height {
                        let pants_color = Color::new(0.2, 0.2, 0.3, 1.0);
                        let shadow = ((y as f32 - legs_y) / leg_height) * 0.3;
                        image.set_pixel(x as u32, y as u32, Color::new(
                            pants_color.r * (1.0 - shadow),
                            pants_color.g * (1.0 - shadow),
                            pants_color.b * (1.0 - shadow),
                            1.0
                        ));
                    }
                }
            }
        }

        // Add hair
        for y in 0..size/4 {
            for x in 0..size {
                let cx = size as f32 / 2.0;
                let cy = size as f32 * 0.25;
                let hair_radius = size as f32 * 0.22;
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();

                if dist < hair_radius && y < size / 6 {
                    let hair_color = if name == "palababa" {
                        Color::new(0.9, 0.9, 0.9, 1.0) // White hair
                    } else {
                        Color::new(0.1, 0.1, 0.1, 1.0) // Black hair
                    };

                    // Add hair texture
                    let noise = ((x as f32 * 0.1).sin() * (y as f32 * 0.1).cos()).abs();
                    let textured_hair = Color::new(
                        hair_color.r * (0.8 + noise * 0.2),
                        hair_color.g * (0.8 + noise * 0.2),
                        hair_color.b * (0.8 + noise * 0.2),
                        1.0
                    );
                    image.set_pixel(x as u32, y as u32, textured_hair);
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.metallic = 0.1;
        sprite.roughness = 0.8;
        sprite.emissive = Color::new(0.0, 0.0, 0.0, 0.0);
        sprite.glow = 0.0;

        self.textures.insert(format!("character_{}", name), texture);
        self.sprites.insert(format!("character_{}", name), sprite);
    }

    fn generate_explosion_texture(&mut self) {
        let size = 256;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        let cx = size as f32 / 2.0;
        let cy = size as f32 / 2.0;

        for y in 0..size {
            for x in 0..size {
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let max_dist = size as f32 / 2.0;

                if dist < max_dist {
                    let t = dist / max_dist;
                    let noise = ((x as f32 * 0.05).sin() * (y as f32 * 0.05).cos()).abs();

                    // Create explosion gradient
                    let r = (1.0 - t).powf(0.5);
                    let g = (1.0 - t).powf(1.5) * 0.8;
                    let b = (1.0 - t).powf(3.0) * 0.3;
                    let a = (1.0 - t).powf(0.8) * (1.0 - noise * 0.3);

                    image.set_pixel(x as u32, y as u32, Color::new(r, g, b, a));
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.emissive = Color::new(1.0, 0.5, 0.0, 1.0);
        sprite.glow = 1.0;

        self.textures.insert("explosion".to_string(), texture);
        self.sprites.insert("explosion".to_string(), sprite);
    }

    fn generate_lightning_texture(&mut self) {
        let size = 256;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        // Generate lightning bolt pattern
        let mut y = 0;
        let mut x = size / 2;

        while y < size {
            let thickness = 3 + (rand::gen_range(0, 3) as u16);

            for dy in 0..5 {
                for dx in 0..thickness {
                    let px = (x + dx - thickness/2) as u32;
                    let py = (y + dy) as u32;

                    if px < size as u32 && py < size as u32 {
                        let intensity = 1.0 - (dx as f32 / thickness as f32).abs();
                        image.set_pixel(px, py, Color::new(0.8, 0.8, 1.0, intensity));

                        // Add glow
                        for gx in -2i32..=2 {
                            for gy in -2i32..=2 {
                                let gpx = (px as i32 + gx) as u32;
                                let gpy = (py as i32 + gy) as u32;
                                if gpx < size as u32 && gpy < size as u32 {
                                    let existing = image.get_pixel(gpx, gpy);
                                    let glow_intensity = 0.3 / (gx.abs() + gy.abs() + 1) as f32;
                                    image.set_pixel(gpx, gpy, Color::new(
                                        (existing.r + 0.6 * glow_intensity).min(1.0),
                                        (existing.g + 0.6 * glow_intensity).min(1.0),
                                        (existing.b + 1.0 * glow_intensity).min(1.0),
                                        (existing.a + glow_intensity).min(1.0)
                                    ));
                                }
                            }
                        }
                    }
                }

                y += 1;
            }

            x = (x as i32 + rand::gen_range(-20, 20)).max(10).min(size as i32 - 10) as u16;
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.emissive = Color::new(0.5, 0.5, 1.0, 1.0);
        sprite.glow = 1.0;

        self.textures.insert("lightning".to_string(), texture);
        self.sprites.insert("lightning".to_string(), sprite);
    }

    fn generate_fire_texture(&mut self) {
        let size = 128;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        for y in 0..size {
            for x in 0..size {
                let cx = size as f32 / 2.0;
                let dist_x = ((x as f32 - cx) / (size as f32 * 0.3)).abs();
                let height_factor = 1.0 - (y as f32 / size as f32);

                if dist_x < 1.0 && height_factor > 0.0 {
                    let flame_shape = (1.0 - dist_x) * height_factor.powf(0.5);
                    let noise = ((x as f32 * 0.1).sin() * (y as f32 * 0.05).cos()).abs();

                    if flame_shape > 0.1 {
                        let intensity = flame_shape * (1.0 + noise * 0.3);
                        let r = intensity.min(1.0);
                        let g = (intensity * 0.5).min(1.0);
                        let b = (intensity * 0.1).min(1.0);
                        let a = (flame_shape * 0.8).min(1.0);

                        image.set_pixel(x as u32, y as u32, Color::new(r, g, b, a));
                    }
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.emissive = Color::new(1.0, 0.3, 0.0, 1.0);
        sprite.glow = 0.8;
        sprite.frame_count = 1;

        self.textures.insert("fire".to_string(), texture);
        self.sprites.insert("fire".to_string(), sprite);
    }

    fn generate_ice_texture(&mut self) {
        let size = 128;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        for y in 0..size {
            for x in 0..size {
                let cx = size as f32 / 2.0;
                let cy = size as f32 / 2.0;

                // Create crystalline pattern
                let angle = ((y as f32 - cy).atan2(x as f32 - cx) * 3.0).sin().abs();
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let max_dist = size as f32 / 2.0;

                if dist < max_dist {
                    let crystal = angle * (1.0 - dist / max_dist);
                    let noise = ((x as f32 * 0.08).sin() * (y as f32 * 0.08).cos()).abs();

                    let intensity = crystal * (1.0 + noise * 0.5);
                    let r = (0.5 + intensity * 0.3).min(1.0);
                    let g = (0.7 + intensity * 0.2).min(1.0);
                    let b = (0.9 + intensity * 0.1).min(1.0);
                    let a = (0.3 + intensity * 0.7).min(1.0);

                    image.set_pixel(x as u32, y as u32, Color::new(r, g, b, a));
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.metallic = 0.8;
        sprite.roughness = 0.1;
        sprite.emissive = Color::new(0.0, 0.2, 0.5, 0.5);

        self.textures.insert("ice".to_string(), texture);
        self.sprites.insert("ice".to_string(), sprite);
    }

    fn generate_impact_texture(&mut self) {
        let size = 128;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        let cx = size as f32 / 2.0;
        let cy = size as f32 / 2.0;

        for y in 0..size {
            for x in 0..size {
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let max_dist = size as f32 / 2.0;

                // Create impact ring
                let ring_thickness = max_dist * 0.3;
                let ring_center = max_dist * 0.6;
                let in_ring = (dist - ring_center).abs() < ring_thickness;

                if in_ring {
                    let ring_t = 1.0 - (dist - ring_center).abs() / ring_thickness;
                    let angle_noise = ((x as f32).atan2(y as f32) * 8.0).sin().abs() * 0.3;
                    let intensity = ring_t * (1.0 - angle_noise);

                    image.set_pixel(x as u32, y as u32, Color::new(
                        1.0,
                        0.9,
                        0.7,
                        intensity
                    ));
                }

                // Add center burst
                if dist < max_dist * 0.3 {
                    let burst_t = 1.0 - dist / (max_dist * 0.3);
                    let existing = image.get_pixel(x as u32, y as u32);
                    image.set_pixel(x as u32, y as u32, Color::new(
                        (existing.r + burst_t * 0.8).min(1.0),
                        (existing.g + burst_t * 0.6).min(1.0),
                        (existing.b + burst_t * 0.2).min(1.0),
                        (existing.a + burst_t * 0.5).min(1.0)
                    ));
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.glow = 0.6;

        self.textures.insert("impact".to_string(), texture);
        self.sprites.insert("impact".to_string(), sprite);
    }

    fn generate_aura_texture(&mut self) {
        let size = 256;
        let mut image = Image::gen_image_color(size, size, Color::new(0.0, 0.0, 0.0, 0.0));

        let cx = size as f32 / 2.0;
        let cy = size as f32 / 2.0;

        for y in 0..size {
            for x in 0..size {
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let max_dist = size as f32 / 2.0;

                if dist < max_dist {
                    let t = dist / max_dist;
                    let angle = (y as f32 - cy).atan2(x as f32 - cx);
                    let wave = ((angle * 4.0 + dist * 0.1).sin() * 0.5 + 0.5) * 0.3;

                    let intensity = (1.0 - t).powf(1.5) * (0.7 + wave);

                    image.set_pixel(x as u32, y as u32, Color::new(
                        0.4 * intensity,
                        0.6 * intensity,
                        1.0 * intensity,
                        intensity * 0.6
                    ));
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        let mut sprite = EnhancedSprite::new(texture.clone());
        sprite.emissive = Color::new(0.2, 0.4, 1.0, 0.8);
        sprite.glow = 1.0;

        self.textures.insert("aura".to_string(), texture);
        self.sprites.insert("aura".to_string(), sprite);
    }

    pub fn get_sprite(&self, name: &str) -> Option<&EnhancedSprite> {
        self.sprites.get(name)
    }

    pub fn get_sprite_mut(&mut self, name: &str) -> Option<&mut EnhancedSprite> {
        self.sprites.get_mut(name)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn create_enhanced_sprite(&self, texture_name: &str) -> Option<EnhancedSprite> {
        self.textures.get(texture_name).map(|tex| {
            let mut sprite = EnhancedSprite::new(tex.clone());

            // Set default properties based on texture type
            if texture_name.contains("explosion") || texture_name.contains("fire") {
                sprite.emissive = Color::new(1.0, 0.5, 0.0, 1.0);
                sprite.glow = 1.0;
            } else if texture_name.contains("lightning") {
                sprite.emissive = Color::new(0.5, 0.5, 1.0, 1.0);
                sprite.glow = 1.0;
            } else if texture_name.contains("ice") {
                sprite.metallic = 0.8;
                sprite.roughness = 0.1;
            }

            sprite
        })
    }
}