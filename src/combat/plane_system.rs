use macroquad::prelude::*;

/// Plane system for Keizer Bom Taha character
#[derive(Clone, Debug)]
pub struct PlaneSystem {
    pub in_plane: bool,
    pub plane_health: f32,
    pub max_plane_health: f32,
    pub plane_cooldown: f32,
    pub plane_duration: f32,
    pub max_plane_duration: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub altitude: f32,
    pub target_altitude: f32,
    pub rotation: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub bombs_remaining: u32,
    pub max_bombs: u32,
    pub bomb_reload_timer: f32,
    pub bomb_reload_time: f32,
    pub active_bombs: Vec<Bomb>,
    pub entering_plane: bool,
    pub exiting_plane: bool,
    pub transition_timer: f32,
    pub transition_duration: f32,
}

/// Bomb projectile
#[derive(Clone, Debug)]
pub struct Bomb {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub lifetime: f32,
    pub damage: f32,
    pub explosion_radius: f32,
    pub armed: bool,
    pub exploded: bool,
}

/// Bomb drop pattern
#[derive(Clone, Copy, Debug)]
pub enum BombPattern {
    Single,       // Drop one bomb
    Line,         // Drop bombs in a line
    Circle,       // Drop bombs in a circle
    Cross,        // Drop bombs in a cross pattern
    CarpetBomb,   // Drop many bombs in sequence
}

impl PlaneSystem {
    pub fn new() -> Self {
        Self {
            in_plane: false,
            plane_health: 100.0,
            max_plane_health: 100.0,
            plane_cooldown: 0.0,
            plane_duration: 0.0,
            max_plane_duration: 15.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            altitude: 0.0,
            target_altitude: 0.0,
            rotation: 0.0,
            speed: 0.0,
            max_speed: 400.0,
            bombs_remaining: 10,
            max_bombs: 10,
            bomb_reload_timer: 0.0,
            bomb_reload_time: 1.0,
            active_bombs: Vec::new(),
            entering_plane: false,
            exiting_plane: false,
            transition_timer: 0.0,
            transition_duration: 1.5,
        }
    }

    /// Enter plane mode
    pub fn enter_plane(&mut self, position: Vec2) -> bool {
        if self.plane_cooldown > 0.0 {
            return false;
        }

        self.entering_plane = true;
        self.transition_timer = 0.0;
        self.position = position;
        self.velocity = Vec2::ZERO;
        self.target_altitude = 300.0;
        self.plane_health = self.max_plane_health;
        self.bombs_remaining = self.max_bombs;

        true
    }

    /// Exit plane mode
    pub fn exit_plane(&mut self) {
        self.exiting_plane = true;
        self.transition_timer = 0.0;
        self.target_altitude = 0.0;
    }

    /// Force exit (plane destroyed)
    pub fn force_exit(&mut self) {
        self.in_plane = false;
        self.entering_plane = false;
        self.exiting_plane = false;
        self.altitude = 0.0;
        self.target_altitude = 0.0;
        self.plane_cooldown = 30.0; // 30 second cooldown after destruction
        self.plane_health = 0.0;
    }

    /// Update plane system
    pub fn update(&mut self, dt: f32, input_direction: Vec2) {
        // Update cooldown
        if self.plane_cooldown > 0.0 {
            self.plane_cooldown -= dt;
        }

        // Handle transitions
        if self.entering_plane {
            self.transition_timer += dt;
            self.altitude = (self.transition_timer / self.transition_duration) * self.target_altitude;

            if self.transition_timer >= self.transition_duration {
                self.entering_plane = false;
                self.in_plane = true;
                self.plane_duration = 0.0;
            }
            return;
        }

        if self.exiting_plane {
            self.transition_timer += dt;
            self.altitude = self.target_altitude * (1.0 - (self.transition_timer / self.transition_duration));

            if self.transition_timer >= self.transition_duration {
                self.exiting_plane = false;
                self.in_plane = false;
                self.altitude = 0.0;
                self.plane_cooldown = 15.0; // 15 second cooldown after normal exit
            }
            return;
        }

        // Plane active logic
        if self.in_plane {
            self.plane_duration += dt;

            // Auto-exit after max duration
            if self.plane_duration >= self.max_plane_duration {
                self.exit_plane();
                return;
            }

            // Update altitude smoothly
            self.altitude += (self.target_altitude - self.altitude) * 0.05;

            // Handle movement
            if input_direction.length() > 0.0 {
                let target_velocity = input_direction.normalize() * self.max_speed;
                self.velocity += (target_velocity - self.velocity) * 0.1;
            } else {
                self.velocity *= 0.95; // Slow down when no input
            }

            // Update position
            self.position += self.velocity * dt;

            // Update rotation based on velocity
            if self.velocity.length() > 10.0 {
                self.rotation = self.velocity.y.atan2(self.velocity.x);
            }

            // Update speed
            self.speed = self.velocity.length();

            // Reload bombs
            if self.bombs_remaining < self.max_bombs {
                self.bomb_reload_timer += dt;
                if self.bomb_reload_timer >= self.bomb_reload_time {
                    self.bombs_remaining += 1;
                    self.bomb_reload_timer = 0.0;
                }
            }
        }

        // Update active bombs
        let mut bombs_to_remove = Vec::new();
        for (i, bomb) in self.active_bombs.iter_mut().enumerate() {
            bomb.update(dt);

            if bomb.exploded || bomb.lifetime > 10.0 {
                bombs_to_remove.push(i);
            }
        }

        // Remove exploded bombs
        for i in bombs_to_remove.iter().rev() {
            self.active_bombs.remove(*i);
        }
    }

    /// Drop bomb with pattern
    pub fn drop_bomb(&mut self, pattern: BombPattern) -> Vec<Bomb> {
        if !self.in_plane || self.bombs_remaining == 0 {
            return Vec::new();
        }

        let bombs = match pattern {
            BombPattern::Single => {
                self.bombs_remaining -= 1;
                vec![self.create_bomb(Vec2::ZERO)]
            }
            BombPattern::Line => {
                let bombs_to_drop = 3.min(self.bombs_remaining);
                self.bombs_remaining -= bombs_to_drop;

                let mut bombs = Vec::new();
                for i in 0..bombs_to_drop {
                    let offset = Vec2::new(
                        (i as f32 - bombs_to_drop as f32 / 2.0) * 50.0,
                        0.0
                    );
                    bombs.push(self.create_bomb(offset));
                }
                bombs
            }
            BombPattern::Circle => {
                let bombs_to_drop = 6.min(self.bombs_remaining);
                self.bombs_remaining -= bombs_to_drop;

                let mut bombs = Vec::new();
                for i in 0..bombs_to_drop {
                    let angle = (i as f32 / bombs_to_drop as f32) * std::f32::consts::TAU;
                    let offset = Vec2::new(
                        angle.cos() * 80.0,
                        angle.sin() * 80.0
                    );
                    bombs.push(self.create_bomb(offset));
                }
                bombs
            }
            BombPattern::Cross => {
                let bombs_to_drop = 5.min(self.bombs_remaining);
                self.bombs_remaining -= bombs_to_drop;

                let mut bombs = Vec::new();
                // Center
                bombs.push(self.create_bomb(Vec2::ZERO));
                // Four directions
                if bombs_to_drop > 1 {
                    bombs.push(self.create_bomb(Vec2::new(60.0, 0.0)));
                }
                if bombs_to_drop > 2 {
                    bombs.push(self.create_bomb(Vec2::new(-60.0, 0.0)));
                }
                if bombs_to_drop > 3 {
                    bombs.push(self.create_bomb(Vec2::new(0.0, 60.0)));
                }
                if bombs_to_drop > 4 {
                    bombs.push(self.create_bomb(Vec2::new(0.0, -60.0)));
                }
                bombs
            }
            BombPattern::CarpetBomb => {
                let bombs_to_drop = self.bombs_remaining.min(10);
                self.bombs_remaining -= bombs_to_drop;

                let mut bombs = Vec::new();
                for i in 0..bombs_to_drop {
                    let offset = Vec2::new(
                        0.0,
                        (i as f32 * 30.0) - (bombs_to_drop as f32 * 15.0)
                    );
                    bombs.push(self.create_bomb(offset));
                }
                bombs
            }
        };

        // Add bombs to active list
        for bomb in &bombs {
            self.active_bombs.push(bomb.clone());
        }

        bombs
    }

    /// Create a single bomb
    fn create_bomb(&self, offset: Vec2) -> Bomb {
        let drop_position = self.position + offset;

        // Bombs inherit plane velocity and add gravity
        let initial_velocity = self.velocity * 0.5 + Vec2::new(0.0, 100.0);

        Bomb {
            position: drop_position,
            velocity: initial_velocity,
            rotation: 0.0,
            lifetime: 0.0,
            damage: 50.0,
            explosion_radius: 100.0,
            armed: false,
            exploded: false,
        }
    }

    /// Perform strafing run (machine gun)
    pub fn strafe(&mut self) -> Vec<Projectile> {
        if !self.in_plane {
            return Vec::new();
        }

        let mut projectiles = Vec::new();

        // Fire 3 bullets in a spread
        for i in 0..3 {
            let angle_offset = (i as f32 - 1.0) * 0.2;
            let angle = self.rotation + angle_offset;

            projectiles.push(Projectile {
                position: self.position,
                velocity: Vec2::new(angle.cos(), angle.sin()) * 600.0,
                damage: 15.0,
                lifetime: 0.0,
                max_lifetime: 2.0,
            });
        }

        projectiles
    }

    /// Take damage to plane
    pub fn take_damage(&mut self, damage: f32) {
        if !self.in_plane {
            return;
        }

        self.plane_health -= damage;

        if self.plane_health <= 0.0 {
            self.force_exit();
        }
    }

    /// Perform aerial slam (exit attack)
    pub fn aerial_slam(&mut self) -> AerialSlam {
        self.exit_plane();

        AerialSlam {
            impact_position: self.position,
            damage: 100.0,
            radius: 150.0,
            shockwave_speed: 300.0,
        }
    }

    /// Check if can enter plane
    pub fn can_enter_plane(&self) -> bool {
        !self.in_plane && !self.entering_plane && !self.exiting_plane && self.plane_cooldown <= 0.0
    }

    /// Get plane health percentage
    pub fn get_health_percent(&self) -> f32 {
        (self.plane_health / self.max_plane_health) * 100.0
    }

    /// Get duration remaining
    pub fn get_duration_remaining(&self) -> f32 {
        (self.max_plane_duration - self.plane_duration).max(0.0)
    }

    /// Check if vulnerable in plane
    pub fn is_vulnerable(&self) -> bool {
        self.in_plane && self.altitude < 200.0
    }
}

impl Bomb {
    /// Update bomb physics
    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;

        if self.exploded {
            return;
        }

        // Apply gravity
        self.velocity.y += 500.0 * dt;

        // Update position
        self.position += self.velocity * dt;

        // Rotation
        self.rotation += self.velocity.length() * 0.01 * dt;

        // Arm after 0.2 seconds
        if !self.armed && self.lifetime > 0.2 {
            self.armed = true;
        }

        // Explode on impact (ground level)
        if self.armed && self.position.y >= 540.0 { // Ground level
            self.explode();
        }
    }

    /// Explode bomb
    pub fn explode(&mut self) {
        self.exploded = true;
    }

    /// Check collision with entity
    pub fn check_collision(&self, entity_pos: Vec2, entity_radius: f32) -> bool {
        if !self.armed || self.exploded {
            return false;
        }

        self.position.distance(entity_pos) < (self.explosion_radius + entity_radius)
    }
}

/// Machine gun projectile
#[derive(Clone)]
pub struct Projectile {
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Projectile {
    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;
        self.position += self.velocity * dt;
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }

    pub fn check_collision(&self, entity_pos: Vec2, entity_radius: f32) -> bool {
        self.position.distance(entity_pos) < entity_radius + 5.0
    }
}

/// Aerial slam attack
pub struct AerialSlam {
    pub impact_position: Vec2,
    pub damage: f32,
    pub radius: f32,
    pub shockwave_speed: f32,
}

impl AerialSlam {
    pub fn check_hit(&self, entity_pos: Vec2) -> bool {
        self.impact_position.distance(entity_pos) <= self.radius
    }
}

/// Plane upgrade system
pub struct PlaneUpgrades {
    pub increased_duration: u32,      // Levels 0-5, +3s per level
    pub increased_bombs: u32,          // Levels 0-5, +2 bombs per level
    pub faster_reload: u32,            // Levels 0-5, -0.15s per level
    pub increased_damage: u32,         // Levels 0-5, +10 damage per level
    pub larger_explosions: u32,        // Levels 0-5, +20 radius per level
    pub armor_plating: u32,            // Levels 0-5, +20 health per level
    pub faster_plane: u32,             // Levels 0-5, +50 speed per level
}

impl PlaneUpgrades {
    pub fn new() -> Self {
        Self {
            increased_duration: 0,
            increased_bombs: 0,
            faster_reload: 0,
            increased_damage: 0,
            larger_explosions: 0,
            armor_plating: 0,
            faster_plane: 0,
        }
    }

    pub fn apply_to_system(&self, system: &mut PlaneSystem) {
        // Duration
        system.max_plane_duration = 15.0 + (self.increased_duration as f32 * 3.0);

        // Bombs
        system.max_bombs = 10 + (self.increased_bombs * 2);
        system.bombs_remaining = system.max_bombs;

        // Reload speed
        system.bomb_reload_time = 1.0 - (self.faster_reload as f32 * 0.15);

        // Plane health
        system.max_plane_health = 100.0 + (self.armor_plating as f32 * 20.0);

        // Speed
        system.max_speed = 400.0 + (self.faster_plane as f32 * 50.0);
    }

    pub fn get_bomb_damage(&self) -> f32 {
        50.0 + (self.increased_damage as f32 * 10.0)
    }

    pub fn get_explosion_radius(&self) -> f32 {
        100.0 + (self.larger_explosions as f32 * 20.0)
    }
}

/// Special plane abilities
pub enum PlaneAbility {
    Afterburner,      // Temporary speed boost
    ShieldBarrier,    // Temporary invulnerability
    HeavyBomb,        // Drop a massive bomb
    RapidFire,        // Machine gun fires faster
    Kamikaze,         // Sacrifice plane for massive damage
}

impl PlaneAbility {
    pub fn get_cooldown(&self) -> f32 {
        match self {
            PlaneAbility::Afterburner => 5.0,
            PlaneAbility::ShieldBarrier => 15.0,
            PlaneAbility::HeavyBomb => 10.0,
            PlaneAbility::RapidFire => 8.0,
            PlaneAbility::Kamikaze => 0.0, // One-time use
        }
    }

    pub fn get_duration(&self) -> f32 {
        match self {
            PlaneAbility::Afterburner => 3.0,
            PlaneAbility::ShieldBarrier => 5.0,
            PlaneAbility::HeavyBomb => 0.0,
            PlaneAbility::RapidFire => 4.0,
            PlaneAbility::Kamikaze => 0.0,
        }
    }
}
