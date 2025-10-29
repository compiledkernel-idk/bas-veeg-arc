use macroquad::prelude::*;
use crate::ecs::CharacterType;
use crate::combat::{MoveId, StyleRank};
use std::collections::VecDeque;

/// Enhanced VFX system for combat polish and visual feedback
pub struct EnhancedVFXSystem {
    impact_effects: VecDeque<ImpactEffect>,
    character_auras: Vec<CharacterAura>,
    combo_text_effects: VecDeque<ComboTextEffect>,
    screen_distortions: VecDeque<ScreenDistortion>,
    freeze_frames: VecDeque<FreezeFrame>,
    trails: Vec<CharacterTrail>,
    dynamic_lights: VecDeque<DynamicLight>,
    impact_lines: VecDeque<ImpactLine>,
    speed_lines: Vec<SpeedLine>,
    damage_numbers: VecDeque<DamageNumber>,
}

/// Directional impact effect with physics
#[derive(Clone)]
pub struct ImpactEffect {
    pub position: Vec2,
    pub direction: Vec2,          // Impact direction
    pub intensity: f32,            // Impact strength
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub effect_type: ImpactType,
    pub sparks: Vec<Spark>,
}

#[derive(Clone, Copy, Debug)]
pub enum ImpactType {
    Light,      // Light punch/kick
    Medium,     // Medium attacks
    Heavy,      // Heavy attacks
    Counter,    // Counter hit
    Critical,   // Critical hit
    Guard,      // Blocked attack
    Parry,      // Perfect parry
}

#[derive(Clone)]
pub struct Spark {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color: Color,
}

/// Character-specific aura effects
#[derive(Clone)]
pub struct CharacterAura {
    pub character_type: CharacterType,
    pub position: Vec2,
    pub intensity: f32,
    pub aura_type: AuraType,
    pub particles: Vec<AuraParticle>,
    pub pulse_timer: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuraType {
    Idle,               // Subtle idle aura
    Charging,           // Charging meter
    PoweredUp,          // During ability
    Critical,           // Low health
    Victorious,         // After victory
    Defeated,           // Defeated state
    Plane(f32),         // Keizer plane mode (altitude)
    Authority(f32),     // Principal authority meter
    Cooking(u32),       // Petra food prep (food count)
}

#[derive(Clone)]
pub struct AuraParticle {
    pub offset: Vec2,
    pub angle: f32,
    pub distance: f32,
    pub speed: f32,
    pub color: Color,
    pub size: f32,
    pub lifetime: f32,
}

/// Stylish combo text effects
#[derive(Clone)]
pub struct ComboTextEffect {
    pub position: Vec2,
    pub text: String,
    pub style_rank: StyleRank,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale: f32,
    pub rotation: f32,
    pub velocity: Vec2,
    pub flash_intensity: f32,
}

/// Screen-space distortion effects
#[derive(Clone)]
pub struct ScreenDistortion {
    pub center: Vec2,
    pub radius: f32,
    pub intensity: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub distortion_type: DistortionType,
}

#[derive(Clone, Copy, Debug)]
pub enum DistortionType {
    Radial,         // Radial distortion from center
    Shockwave,      // Expanding shockwave distortion
    Vortex,         // Swirling vortex
    Crush,          // Crushing/compression effect
}

/// Freeze frame for impactful moments
#[derive(Clone)]
pub struct FreezeFrame {
    pub duration: f32,
    pub elapsed: f32,
    pub intensity: f32,  // How much to slow time (0.0 = full freeze, 0.5 = 50% speed)
    pub shake_intensity: f32,
}

/// Character trail for fast movement
#[derive(Clone)]
pub struct CharacterTrail {
    pub character_type: CharacterType,
    pub positions: VecDeque<TrailNode>,
    pub max_length: usize,
    pub color: Color,
    pub width: f32,
}

#[derive(Clone)]
pub struct TrailNode {
    pub position: Vec2,
    pub lifetime: f32,
}

/// Dynamic light source for dramatic lighting
#[derive(Clone)]
pub struct DynamicLight {
    pub position: Vec2,
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub flicker: bool,
    pub flicker_speed: f32,
}

/// Impact lines for powerful hits
#[derive(Clone)]
pub struct ImpactLine {
    pub position: Vec2,
    pub direction: Vec2,
    pub length: f32,
    pub thickness: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
}

/// Speed lines for rapid movement
#[derive(Clone)]
pub struct SpeedLine {
    pub start: Vec2,
    pub end: Vec2,
    pub speed: f32,
    pub offset: f32,
    pub lifetime: f32,
}

/// Floating damage numbers
#[derive(Clone)]
pub struct DamageNumber {
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub is_critical: bool,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale: f32,
    pub color: Color,
}

impl EnhancedVFXSystem {
    pub fn new() -> Self {
        Self {
            impact_effects: VecDeque::new(),
            character_auras: Vec::new(),
            combo_text_effects: VecDeque::new(),
            screen_distortions: VecDeque::new(),
            freeze_frames: VecDeque::new(),
            trails: Vec::new(),
            dynamic_lights: VecDeque::new(),
            impact_lines: VecDeque::new(),
            speed_lines: Vec::new(),
            damage_numbers: VecDeque::new(),
        }
    }

    /// Spawn impact effect with direction
    pub fn spawn_impact(&mut self, position: Vec2, direction: Vec2, impact_type: ImpactType) {
        let (intensity, spark_count, lifetime, base_color) = match impact_type {
            ImpactType::Light => (1.0, 5, 0.15, YELLOW),
            ImpactType::Medium => (1.5, 10, 0.25, ORANGE),
            ImpactType::Heavy => (2.5, 20, 0.4, RED),
            ImpactType::Counter => (2.0, 15, 0.35, PURPLE),
            ImpactType::Critical => (3.0, 30, 0.5, Color::new(1.0, 0.2, 0.2, 1.0)),
            ImpactType::Guard => (0.8, 8, 0.2, SKYBLUE),
            ImpactType::Parry => (1.8, 18, 0.3, GOLD),
        };

        let mut sparks = Vec::new();
        for i in 0..spark_count {
            let angle_spread = 120.0_f32.to_radians();
            let base_angle = (-direction).angle();
            let angle = base_angle + rand::gen_range(-angle_spread / 2.0, angle_spread / 2.0);
            let speed = rand::gen_range(200.0, 600.0) * intensity;

            sparks.push(Spark {
                position,
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                lifetime: 0.0,
                max_lifetime: rand::gen_range(0.1, lifetime),
                size: rand::gen_range(0.5, 1.5) * intensity,
                color: base_color,
            });
        }

        self.impact_effects.push_back(ImpactEffect {
            position,
            direction,
            intensity,
            lifetime: 0.0,
            max_lifetime: lifetime,
            effect_type: impact_type,
            sparks,
        });

        // Add freeze frame for heavy hits
        if matches!(impact_type, ImpactType::Heavy | ImpactType::Critical) {
            self.add_freeze_frame(0.05 * intensity, 0.2, intensity * 5.0);
        }

        // Add impact lines
        self.add_impact_lines(position, direction, intensity);
    }

    /// Add freeze frame effect
    pub fn add_freeze_frame(&mut self, duration: f32, time_scale: f32, shake: f32) {
        self.freeze_frames.push_back(FreezeFrame {
            duration,
            elapsed: 0.0,
            intensity: time_scale,
            shake_intensity: shake,
        });
    }

    /// Add impact lines radiating from hit
    fn add_impact_lines(&mut self, position: Vec2, direction: Vec2, intensity: f32) {
        let line_count = (intensity * 8.0) as usize;
        for i in 0..line_count {
            let angle = (i as f32 / line_count as f32) * std::f32::consts::TAU;
            let dir = Vec2::new(angle.cos(), angle.sin());

            self.impact_lines.push_back(ImpactLine {
                position,
                direction: dir,
                length: rand::gen_range(8.0, 20.0) * intensity,
                thickness: rand::gen_range(0.5, 1.5) * intensity,
                lifetime: 0.0,
                max_lifetime: 0.15,
                color: Color::new(1.0, 1.0, 1.0, 0.6),
            });
        }
    }

    /// Create or update character aura
    pub fn update_character_aura(&mut self, character_type: CharacterType, position: Vec2, aura_type: AuraType) {
        // Find existing aura or create new one
        if let Some(aura) = self.character_auras.iter_mut().find(|a| a.character_type == character_type) {
            aura.position = position;
            aura.aura_type = aura_type;
        } else {
            let particle_count = match aura_type {
                AuraType::Idle => 10,
                AuraType::Charging => 20,
                AuraType::PoweredUp => 30,
                AuraType::Critical => 25,
                AuraType::Victorious => 40,
                AuraType::Defeated => 5,
                AuraType::Plane(_) => 35,
                AuraType::Authority(_) => 30,
                AuraType::Cooking(_) => 15,
            };

            let mut particles = Vec::new();
            for i in 0..particle_count {
                particles.push(AuraParticle {
                    offset: Vec2::ZERO,
                    angle: (i as f32 / particle_count as f32) * std::f32::consts::TAU,
                    distance: rand::gen_range(40.0, 80.0),
                    speed: rand::gen_range(0.5, 2.0),
                    color: Self::aura_color(&aura_type),
                    size: rand::gen_range(2.0, 6.0),
                    lifetime: rand::gen_range(0.0, 2.0),
                });
            }

            self.character_auras.push(CharacterAura {
                character_type,
                position,
                intensity: 1.0,
                aura_type,
                particles,
                pulse_timer: 0.0,
            });
        }
    }

    fn aura_color(aura_type: &AuraType) -> Color {
        match aura_type {
            AuraType::Idle => Color::new(0.8, 0.8, 1.0, 0.3),
            AuraType::Charging => Color::new(0.0, 1.0, 1.0, 0.6),
            AuraType::PoweredUp => Color::new(1.0, 0.8, 0.0, 0.8),
            AuraType::Critical => Color::new(1.0, 0.0, 0.0, 0.7),
            AuraType::Victorious => Color::new(1.0, 1.0, 0.0, 0.9),
            AuraType::Defeated => Color::new(0.3, 0.3, 0.3, 0.4),
            AuraType::Plane(_) => Color::new(0.2, 0.6, 1.0, 0.7),
            AuraType::Authority(_) => Color::new(0.8, 0.0, 0.4, 0.7),
            AuraType::Cooking(_) => Color::new(1.0, 0.6, 0.0, 0.6),
        }
    }

    /// Show combo text
    pub fn show_combo_text(&mut self, position: Vec2, combo_count: u32, style_rank: StyleRank) {
        let text = match style_rank {
            StyleRank::D => format!("{} HIT", combo_count),
            StyleRank::C => format!("{} HIT COMBO!", combo_count),
            StyleRank::B => format!("{} HIT! GOOD!", combo_count),
            StyleRank::A => format!("{} HIT! GREAT!", combo_count),
            StyleRank::S => format!("{} HIT COMBO! STYLISH!", combo_count),
            StyleRank::SS => format!("{} HIT! SUPER STYLISH!!", combo_count),
            StyleRank::SSS => format!("{} HIT COMBO!!! SSS RANK!!!", combo_count),
        };

        self.combo_text_effects.push_back(ComboTextEffect {
            position: position + Vec2::new(0.0, -50.0),
            text,
            style_rank,
            lifetime: 0.0,
            max_lifetime: 1.5,
            scale: 1.0,
            rotation: 0.0,
            velocity: Vec2::new(0.0, -30.0),
            flash_intensity: 1.0,
        });
    }

    /// Show damage number
    pub fn show_damage_number(&mut self, position: Vec2, damage: f32, is_critical: bool) {
        let color = if is_critical {
            Color::new(1.0, 0.2, 0.2, 1.0)
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0)
        };

        self.damage_numbers.push_back(DamageNumber {
            position: position + Vec2::new(rand::gen_range(-20.0, 20.0), -30.0),
            velocity: Vec2::new(rand::gen_range(-20.0, 20.0), -80.0),
            damage,
            is_critical,
            lifetime: 0.0,
            max_lifetime: 1.0,
            scale: if is_critical { 1.5 } else { 1.0 },
            color,
        });
    }

    /// Add screen distortion
    pub fn add_screen_distortion(&mut self, center: Vec2, radius: f32, intensity: f32, distortion_type: DistortionType) {
        self.screen_distortions.push_back(ScreenDistortion {
            center,
            radius,
            intensity,
            lifetime: 0.0,
            max_lifetime: 0.5,
            distortion_type,
        });
    }

    /// Add character trail
    pub fn add_trail_node(&mut self, character_type: CharacterType, position: Vec2, color: Color) {
        if let Some(trail) = self.trails.iter_mut().find(|t| t.character_type == character_type) {
            trail.positions.push_front(TrailNode {
                position,
                lifetime: 0.0,
            });

            if trail.positions.len() > trail.max_length {
                trail.positions.pop_back();
            }
        } else {
            let mut trail = CharacterTrail {
                character_type,
                positions: VecDeque::new(),
                max_length: 10,
                color,
                width: 30.0,
            };
            trail.positions.push_front(TrailNode {
                position,
                lifetime: 0.0,
            });
            self.trails.push(trail);
        }
    }

    /// Spawn dynamic light
    pub fn spawn_dynamic_light(&mut self, position: Vec2, color: Color, intensity: f32, radius: f32, lifetime: f32, flicker: bool) {
        self.dynamic_lights.push_back(DynamicLight {
            position,
            color,
            intensity,
            radius,
            lifetime: 0.0,
            max_lifetime: lifetime,
            flicker,
            flicker_speed: rand::gen_range(5.0, 15.0),
        });
    }

    /// Add speed lines for fast movement
    pub fn add_speed_lines(&mut self, position: Vec2, direction: Vec2, intensity: f32) {
        let line_count = (intensity * 15.0) as usize;
        for i in 0..line_count {
            let offset = rand::gen_range(-100.0, 100.0);
            let perpendicular = Vec2::new(-direction.y, direction.x);
            let start = position + perpendicular * offset;
            let end = start - direction * rand::gen_range(50.0, 150.0);

            self.speed_lines.push(SpeedLine {
                start,
                end,
                speed: rand::gen_range(500.0, 1000.0),
                offset,
                lifetime: 0.0,
            });
        }
    }

    /// Update all VFX
    pub fn update(&mut self, dt: f32) {
        // Update freeze frames
        self.freeze_frames.retain_mut(|freeze| {
            freeze.elapsed += dt;
            freeze.elapsed < freeze.duration
        });

        // Update impact effects
        self.impact_effects.retain_mut(|impact| {
            impact.lifetime += dt;
            for spark in &mut impact.sparks {
                spark.lifetime += dt;
                spark.position += spark.velocity * dt;
                spark.velocity.y += 500.0 * dt;  // Gravity
            }
            impact.lifetime < impact.max_lifetime
        });

        // Update character auras
        for aura in &mut self.character_auras {
            aura.pulse_timer += dt;
            for particle in &mut aura.particles {
                particle.lifetime += dt;
                particle.angle += particle.speed * dt;

                // Orbit motion
                particle.offset = Vec2::new(
                    particle.angle.cos() * particle.distance,
                    particle.angle.sin() * particle.distance + (particle.lifetime * 2.0).sin() * 10.0,
                );
            }
        }

        // Update combo text
        self.combo_text_effects.retain_mut(|text| {
            text.lifetime += dt;
            text.position += text.velocity * dt;
            text.flash_intensity = (text.lifetime * 10.0).sin().abs();
            text.scale = 1.0 + (1.0 - text.lifetime / text.max_lifetime) * 0.5;
            text.lifetime < text.max_lifetime
        });

        // Update screen distortions
        self.screen_distortions.retain_mut(|dist| {
            dist.lifetime += dt;
            dist.radius += 100.0 * dt;  // Expand over time
            dist.lifetime < dist.max_lifetime
        });

        // Update trails
        for trail in &mut self.trails {
            for node in &mut trail.positions {
                node.lifetime += dt;
            }
            trail.positions.retain(|node| node.lifetime < 0.5);
        }

        // Update dynamic lights
        self.dynamic_lights.retain_mut(|light| {
            light.lifetime += dt;
            if light.flicker {
                light.intensity = 1.0 + (get_time() as f32 * light.flicker_speed).sin() * 0.3;
            }
            light.lifetime < light.max_lifetime
        });

        // Update impact lines
        self.impact_lines.retain_mut(|line| {
            line.lifetime += dt;
            line.lifetime < line.max_lifetime
        });

        // Update speed lines
        self.speed_lines.retain_mut(|line| {
            line.lifetime += dt;
            line.start += Vec2::new(line.speed * dt, 0.0);
            line.end += Vec2::new(line.speed * dt, 0.0);
            line.lifetime < 0.3
        });

        // Update damage numbers
        self.damage_numbers.retain_mut(|num| {
            num.lifetime += dt;
            num.position += num.velocity * dt;
            num.velocity.y += 100.0 * dt;  // Gravity
            num.lifetime < num.max_lifetime
        });
    }

    /// Render all enhanced VFX
    pub fn render(&self) {
        // Render impact lines
        for line in &self.impact_lines {
            let alpha = 1.0 - (line.lifetime / line.max_lifetime);
            let mut color = line.color;
            color.a *= alpha;

            let end_pos = line.position + line.direction * line.length;
            draw_line(
                line.position.x,
                line.position.y,
                end_pos.x,
                end_pos.y,
                line.thickness,
                color,
            );
        }

        // Render speed lines
        for line in &self.speed_lines {
            let alpha = 1.0 - (line.lifetime / 0.3);
            draw_line(
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
                2.0,
                Color::new(1.0, 1.0, 1.0, alpha * 0.5),
            );
        }

        // Render character trails
        for trail in &self.trails {
            if trail.positions.len() < 2 {
                continue;
            }

            for i in 0..trail.positions.len() - 1 {
                let alpha = 1.0 - (trail.positions[i].lifetime / 0.5);
                let width = trail.width * alpha;
                let mut color = trail.color;
                color.a *= alpha * 0.5;

                draw_line(
                    trail.positions[i].position.x,
                    trail.positions[i].position.y,
                    trail.positions[i + 1].position.x,
                    trail.positions[i + 1].position.y,
                    width,
                    color,
                );
            }
        }

        // Render character auras
        for aura in &self.character_auras {
            let pulse = (aura.pulse_timer * 2.0).sin() * 0.2 + 0.8;

            for particle in &aura.particles {
                let pos = aura.position + particle.offset;
                let mut color = particle.color;
                color.a *= pulse;

                draw_circle(pos.x, pos.y, particle.size, color);
            }
        }

        // Render impact effects
        for impact in &self.impact_effects {
            for spark in &impact.sparks {
                if spark.lifetime < spark.max_lifetime {
                    let alpha = 1.0 - (spark.lifetime / spark.max_lifetime);
                    let mut color = spark.color;
                    color.a *= alpha;
                    draw_circle(spark.position.x, spark.position.y, spark.size, color);
                }
            }

            // Draw impact flash
            let alpha = 1.0 - (impact.lifetime / impact.max_lifetime);
            let size = 6.0 * impact.intensity * (1.0 + impact.lifetime * 1.5);
            draw_circle(
                impact.position.x,
                impact.position.y,
                size,
                Color::new(1.0, 1.0, 1.0, alpha * 0.2),
            );
        }

        // Render dynamic lights
        for light in &self.dynamic_lights {
            let alpha = (1.0 - light.lifetime / light.max_lifetime) * light.intensity;
            let mut color = light.color;
            color.a *= alpha * 0.3;

            // Draw multiple circles for soft glow
            for i in 0..5 {
                let radius = light.radius * (1.0 - i as f32 * 0.15);
                color.a *= 0.5;
                draw_circle(light.position.x, light.position.y, radius, color);
            }
        }

        // Render combo text
        for text in &self.combo_text_effects {
            let alpha = 1.0 - (text.lifetime / text.max_lifetime);
            let color = match text.style_rank {
                StyleRank::D => Color::new(0.7, 0.7, 0.7, alpha),
                StyleRank::C => Color::new(0.0, 1.0, 0.5, alpha),
                StyleRank::B => Color::new(0.0, 0.5, 1.0, alpha),
                StyleRank::A => Color::new(1.0, 0.5, 0.0, alpha),
                StyleRank::S => Color::new(1.0, 1.0, 0.0, alpha),
                StyleRank::SS => Color::new(1.0, 0.0, 1.0, alpha),
                StyleRank::SSS => Color::new(1.0, 0.2, 0.2, alpha),
            };

            let font_size = 30.0 * text.scale;
            let text_size = measure_text(&text.text, None, font_size as u16, 1.0);

            // Draw text with outline
            for x_offset in [-2.0, 0.0, 2.0] {
                for y_offset in [-2.0, 0.0, 2.0] {
                    if x_offset != 0.0 || y_offset != 0.0 {
                        draw_text(
                            &text.text,
                            text.position.x - text_size.width * 0.5 + x_offset,
                            text.position.y + y_offset,
                            font_size,
                            BLACK,
                        );
                    }
                }
            }

            draw_text(
                &text.text,
                text.position.x - text_size.width * 0.5,
                text.position.y,
                font_size,
                color,
            );
        }

        // Render damage numbers
        for num in &self.damage_numbers {
            let alpha = 1.0 - (num.lifetime / num.max_lifetime);
            let mut color = num.color;
            color.a *= alpha;

            let text = if num.is_critical {
                format!("{:.0}!", num.damage)
            } else {
                format!("{:.0}", num.damage)
            };

            let font_size = 12.0 * num.scale;
            let text_size = measure_text(&text, None, font_size as u16, 1.0);

            // Outline
            for x_offset in [-1.0, 0.0, 1.0] {
                for y_offset in [-1.0, 0.0, 1.0] {
                    if x_offset != 0.0 || y_offset != 0.0 {
                        draw_text(
                            &text,
                            num.position.x - text_size.width * 0.5 + x_offset,
                            num.position.y + y_offset,
                            font_size,
                            BLACK,
                        );
                    }
                }
            }

            draw_text(
                &text,
                num.position.x - text_size.width * 0.5,
                num.position.y,
                font_size,
                color,
            );
        }
    }

    /// Get current time scale from active freeze frames
    pub fn get_time_scale(&self) -> f32 {
        if let Some(freeze) = self.freeze_frames.front() {
            freeze.intensity
        } else {
            1.0
        }
    }

    /// Get current shake intensity
    pub fn get_shake_intensity(&self) -> f32 {
        if let Some(freeze) = self.freeze_frames.front() {
            freeze.shake_intensity
        } else {
            0.0
        }
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.impact_effects.clear();
        self.character_auras.clear();
        self.combo_text_effects.clear();
        self.screen_distortions.clear();
        self.freeze_frames.clear();
        self.trails.clear();
        self.dynamic_lights.clear();
        self.impact_lines.clear();
        self.speed_lines.clear();
        self.damage_numbers.clear();
    }

    /// Remove character aura
    pub fn remove_character_aura(&mut self, character_type: CharacterType) {
        self.character_auras.retain(|a| a.character_type != character_type);
    }
}

impl Default for EnhancedVFXSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for Vec2 to calculate angle
trait Vec2Ext {
    fn angle(&self) -> f32;
}

impl Vec2Ext for Vec2 {
    fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}
