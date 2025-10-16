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
    Explosion,
    BloodSplatter,
    PowerUpSparkle,
    CriticalHit,
    ComboFlash,
    LightningStrike,
    FireBurst,
    IceShatter,
    PoisonCloud,
    ShockWave,
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
            VFXType::Explosion => {
                self.spawn_explosion_particles(position, 50);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.6,
                    color: Color::new(1.0, 0.5, 0.0, 1.0),
                    scale: 4.0,
                    rotation: 0.0,
                }
            }
            VFXType::BloodSplatter => {
                self.spawn_blood_particles(position, 30);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.8,
                    color: Color::new(0.8, 0.0, 0.0, 1.0),
                    scale: 2.0,
                    rotation: rand::gen_range(0.0, std::f32::consts::TAU),
                }
            }
            VFXType::PowerUpSparkle => {
                self.spawn_sparkle_particles(position, 15);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 1.0,
                    color: GOLD,
                    scale: 1.5,
                    rotation: 0.0,
                }
            }
            VFXType::CriticalHit => {
                self.spawn_critical_particles(position, 25);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.4,
                    color: Color::new(1.0, 1.0, 0.0, 1.0),
                    scale: 3.0,
                    rotation: 0.0,
                }
            }
            VFXType::ComboFlash => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.3,
                color: Color::new(1.0, 0.0, 1.0, 0.8),
                scale: 2.5,
                rotation: get_time() as f32 * 5.0,
            },
            VFXType::LightningStrike => {
                self.spawn_lightning_particles(position, 20);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.2,
                    color: Color::new(0.8, 0.8, 1.0, 1.0),
                    scale: 5.0,
                    rotation: 0.0,
                }
            }
            VFXType::FireBurst => {
                self.spawn_fire_particles(position, 40);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 1.0,
                    color: Color::new(1.0, 0.3, 0.0, 1.0),
                    scale: 2.5,
                    rotation: 0.0,
                }
            }
            VFXType::IceShatter => {
                self.spawn_ice_particles(position, 35);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 0.7,
                    color: Color::new(0.5, 0.8, 1.0, 1.0),
                    scale: 2.0,
                    rotation: rand::gen_range(0.0, std::f32::consts::TAU),
                }
            }
            VFXType::PoisonCloud => {
                self.spawn_poison_particles(position, 25);
                VisualEffect {
                    effect_type,
                    position,
                    lifetime: 0.0,
                    max_lifetime: 1.5,
                    color: Color::new(0.0, 0.8, 0.2, 0.7),
                    scale: 3.0,
                    rotation: 0.0,
                }
            }
            VFXType::ShockWave => VisualEffect {
                effect_type,
                position,
                lifetime: 0.0,
                max_lifetime: 0.5,
                color: Color::new(1.0, 1.0, 1.0, 0.6),
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

    fn spawn_explosion_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(200.0, 600.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, -100.0);
                particle.color = Color::new(
                    1.0,
                    rand::gen_range(0.3, 0.8),
                    rand::gen_range(0.0, 0.3),
                    1.0,
                );
                particle.size = rand::gen_range(4.0, 12.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.4, 0.8);
                particle.active = true;
            }
        }
    }

    fn spawn_blood_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(-std::f32::consts::PI * 0.75, -std::f32::consts::PI * 0.25);
                let speed = rand::gen_range(150.0, 400.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, 800.0);
                particle.color = Color::new(
                    rand::gen_range(0.6, 0.9),
                    0.0,
                    0.0,
                    rand::gen_range(0.8, 1.0),
                );
                particle.size = rand::gen_range(2.0, 6.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.6, 1.2);
                particle.active = true;
            }
        }
    }

    fn spawn_sparkle_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(50.0, 200.0);

                particle.position = position + Vec2::new(
                    rand::gen_range(-20.0, 20.0),
                    rand::gen_range(-20.0, 20.0)
                );
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, -50.0);
                particle.color = Color::new(1.0, 1.0, 0.0, 1.0);
                particle.size = rand::gen_range(1.0, 4.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.8, 1.5);
                particle.active = true;
            }
        }
    }

    fn spawn_critical_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(300.0, 500.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, 0.0);
                particle.color = Color::new(1.0, 1.0, 0.0, 1.0);
                particle.size = rand::gen_range(6.0, 10.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.2, 0.4);
                particle.active = true;
            }
        }
    }

    fn spawn_lightning_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                particle.position = position + Vec2::new(
                    rand::gen_range(-100.0, 100.0),
                    rand::gen_range(-200.0, 0.0)
                );
                particle.velocity = Vec2::new(
                    rand::gen_range(-50.0, 50.0),
                    rand::gen_range(100.0, 400.0)
                );
                particle.acceleration = Vec2::new(0.0, 0.0);
                particle.color = Color::new(0.8, 0.8, 1.0, 1.0);
                particle.size = rand::gen_range(2.0, 8.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.1, 0.3);
                particle.active = true;
            }
        }
    }

    fn spawn_fire_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(50.0, 150.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed - 100.0);
                particle.acceleration = Vec2::new(0.0, -200.0);
                particle.color = Color::new(
                    1.0,
                    rand::gen_range(0.2, 0.6),
                    0.0,
                    rand::gen_range(0.6, 1.0),
                );
                particle.size = rand::gen_range(4.0, 10.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.5, 1.0);
                particle.active = true;
            }
        }
    }

    fn spawn_ice_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(100.0, 400.0);

                particle.position = position;
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                particle.acceleration = Vec2::new(0.0, 300.0);
                particle.color = Color::new(
                    rand::gen_range(0.5, 0.7),
                    rand::gen_range(0.8, 1.0),
                    1.0,
                    rand::gen_range(0.8, 1.0),
                );
                particle.size = rand::gen_range(3.0, 8.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(0.4, 0.8);
                particle.active = true;
            }
        }
    }

    fn spawn_poison_particles(&mut self, position: Vec2, count: usize) {
        for particle in self.particle_pool.iter_mut().take(count) {
            if !particle.active {
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                let speed = rand::gen_range(20.0, 80.0);

                particle.position = position + Vec2::new(
                    rand::gen_range(-30.0, 30.0),
                    rand::gen_range(-30.0, 30.0)
                );
                particle.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed - 30.0);
                particle.acceleration = Vec2::new(0.0, -20.0);
                particle.color = Color::new(
                    0.0,
                    rand::gen_range(0.6, 0.9),
                    rand::gen_range(0.1, 0.3),
                    rand::gen_range(0.4, 0.7),
                );
                particle.size = rand::gen_range(6.0, 15.0);
                particle.lifetime = 0.0;
                particle.max_lifetime = rand::gen_range(1.0, 2.0);
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
                VFXType::Explosion => {
                    let size = effect.scale * 60.0 * (1.0 + effect.lifetime * 4.0);
                    let inner_size = size * 0.6;
                    draw_circle(effect.position.x, effect.position.y, size,
                        Color::new(1.0, 0.5, 0.0, alpha * 0.3));
                    draw_circle(effect.position.x, effect.position.y, inner_size,
                        Color::new(1.0, 1.0, 0.5, alpha * 0.5));
                    draw_circle_lines(effect.position.x, effect.position.y, size, 5.0,
                        Color::new(1.0, 0.3, 0.0, alpha));
                }
                VFXType::CriticalHit => {
                    let size = effect.scale * 40.0 * (1.0 + effect.lifetime * 6.0);
                    for i in 0..16 {
                        let angle = i as f32 * std::f32::consts::PI * 0.125;
                        let length = size * (1.0 - effect.lifetime / effect.max_lifetime * 0.5);
                        draw_line(
                            effect.position.x,
                            effect.position.y,
                            effect.position.x + angle.cos() * length,
                            effect.position.y + angle.sin() * length,
                            5.0,
                            color,
                        );
                    }
                    draw_circle_lines(effect.position.x, effect.position.y, size * 0.5, 3.0, color);
                }
                VFXType::ComboFlash => {
                    let size = effect.scale * 80.0 * (1.0 - effect.lifetime / effect.max_lifetime);
                    for i in 0..3 {
                        let offset_angle = effect.rotation + i as f32 * std::f32::consts::TAU / 3.0;
                        let ring_size = size * (1.0 + i as f32 * 0.3);
                        draw_circle_lines(
                            effect.position.x + offset_angle.cos() * 10.0,
                            effect.position.y + offset_angle.sin() * 10.0,
                            ring_size,
                            2.0,
                            color
                        );
                    }
                }
                VFXType::ShockWave => {
                    let size = effect.scale * 200.0 * effect.lifetime / effect.max_lifetime;
                    let thickness = 10.0 * (1.0 - effect.lifetime / effect.max_lifetime);
                    draw_circle_lines(effect.position.x, effect.position.y, size, thickness, color);
                    draw_circle_lines(effect.position.x, effect.position.y, size * 0.7, thickness * 0.7,
                        Color::new(color.r, color.g, color.b, color.a * 0.5));
                }
                VFXType::LightningStrike => {
                    let segments = 8;
                    let mut y = effect.position.y - 300.0;
                    let mut x = effect.position.x;

                    for _ in 0..segments {
                        let next_x = x + rand::gen_range(-30.0, 30.0);
                        let next_y = y + 300.0 / segments as f32;
                        draw_line(x, y, next_x, next_y, 3.0, color);
                        draw_line(x - 1.0, y, next_x - 1.0, next_y, 5.0,
                            Color::new(color.r, color.g, color.b, color.a * 0.5));
                        x = next_x;
                        y = next_y;
                    }
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
