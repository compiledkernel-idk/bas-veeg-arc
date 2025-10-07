use macroquad::prelude::*;
use std::collections::VecDeque;

pub struct VFXManager {
    effects: VecDeque<VisualEffect>,
    particle_pool: Vec<Particle>,
}

#[derive(Clone)]
pub struct VisualEffect {
    pub effect_type: VFXType,
    pub position: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
    pub scale: f32,
    pub rotation: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VFXType {
    HitSpark,
    BlockFlash,
    DashTrail,
    SuperFlash,
    PaintSplash,
    DustCloud,
    EnergyBurst,
    ScreenFlash,
}

#[derive(Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub color: Color,
    pub size: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub active: bool,
}

impl VFXManager {
    pub fn new() -> Self {
        let mut particle_pool = Vec::with_capacity(1000);
        for _ in 0..1000 {
            particle_pool.push(Particle {
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
                acceleration: Vec2::ZERO,
                color: WHITE,
                size: 1.0,
                lifetime: 0.0,
                max_lifetime: 1.0,
                active: false,
            });
        }

        Self {
            effects: VecDeque::new(),
            particle_pool,
        }
    }

    pub fn spawn_effect(&mut self, effect_type: VFXType, position: Vec2) {
        let effect = match effect_type {
            VFXType::HitSpark => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.2,
                color: YELLOW,
                scale: 1.0,
                rotation: rand::gen_range(0.0, std::f32::consts::TAU),
            },
            VFXType::BlockFlash => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.15,
                color: Color::new(0.0, 0.5, 1.0, 0.8),
                scale: 1.5,
                rotation: 0.0,
            },
            VFXType::DashTrail => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.3,
                color: Color::new(1.0, 1.0, 1.0, 0.5),
                scale: 1.0,
                rotation: 0.0,
            },
            VFXType::SuperFlash => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.5,
                color: GOLD,
                scale: 3.0,
                rotation: 0.0,
            },
            VFXType::PaintSplash => {
                self.spawn_paint_particles(position, 20);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.4,
                    color: Color::new(
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.0, 0.5),
                        rand::gen_range(0.5, 1.0),
                        1.0,
                    ),
                    scale: 2.0,
                    rotation: rand::gen_range(0.0, std::f32::consts::TAU),
                }
            }
            VFXType::DustCloud => {
                self.spawn_dust_particles(position, 10);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.6,
                    color: Color::new(0.6, 0.5, 0.4, 0.7),
                    scale: 1.5,
                    rotation: 0.0,
                }
            }
            VFXType::EnergyBurst => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.4,
                color: Color::new(0.0, 1.0, 1.0, 1.0),
                scale: 2.5,
                rotation: 0.0,
            },
            VFXType::ScreenFlash => VisualEffect {
                effect_type,
                position: Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
                lifetime: 0.0,
                max_lifetime: 0.1,
                color: WHITE,
                scale: 1.0,
                rotation: 0.0,
            },
        };

        self.effects.push_back(effect);
    }

    fn spawn_paint_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(100.0, 400.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, 500.0);
                particle.color = Color::new(
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.0, 0.5),
                    rand::gen_range(0.5, 1.0),
                    1.0,
                );
                particle.size = rand::gen_range(2.0, 8.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.5, 1.0);
                particle.active = true;
            }
        }
    }

    fn spawn_dust_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::PI);
                let speed = rand::gen_range(50.0, 150.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, -angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, 100.0);
                particle.color = Color::new(0.6, 0.5, 0.4, 0.7);
                particle.size = rand::gen_range(3.0, 10.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.8, 1.5);
                particle.active = true;
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.effects.retain_mut(|effect| {
            effect.lifetime += dt;
            effect.lifetime < effect.max_lifetime
        });

        for particle in &mut self.particle_pool {
            if particle.active {
                particle.lifetime += dt;
                particle.velocity += particle.acceleration * dt;
                particle.position += particle.velocity * dt;

                if particle.lifetime >= particle.max_lifetime {
                    particle.active = false;
                }
            }
        }
    }

    pub fn render(&self) {
        for effect in &self.effects {
            let alpha = 1.0 - (effect.lifetime / effect.max_lifetime);
            let mut color = effect.color;
            color.a *= alpha;

            match effect.effect_type {
                VFXType::HitSpark => {
                    for i in 0..8 {
                        let angle = i as f32 * std::f32::consts::PI * 0.25 + effect.rotation;
                        let length =
                            effect.scale * 30.0 * (1.0 - effect.lifetime / effect.max_lifetime);
                        draw_line(
                            effect.position.x,
                            effect.position.y,
                            effect.position.x + angle.cos() * length,
                            effect.position.y + angle.sin() * length,
                            3.0,
                            color,
                        );
                    }
                }
                VFXType::BlockFlash => {
                    let size = effect.scale * 50.0 * (1.0 + effect.lifetime * 2.0);
                    draw_circle_lines(effect.position.x, effect.position.y, size, 4.0, color);
                }
                VFXType::SuperFlash => {
                    let size = effect.scale * 100.0 * (1.0 + effect.lifetime * 3.0);
                    draw_circle(effect.position.x, effect.position.y, size, color);
                }
                VFXType::ScreenFlash => {
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::new(1.0, 1.0, 1.0, alpha * 0.8),
                    );
                }
                _ => {}
            }
        }

        for particle in &self.particle_pool {
            if particle.active {
                let alpha = 1.0 - (particle.lifetime / particle.max_lifetime);
                let mut color = particle.color;
                color.a *= alpha;
                let size = particle.size * (1.0 - particle.lifetime / particle.max_lifetime * 0.5);
                draw_circle(particle.position.x, particle.position.y, size, color);
            }
        }
    }
}
