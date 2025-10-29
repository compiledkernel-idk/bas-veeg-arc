use macroquad::prelude::*;

/// Motion trail effect for fast-moving entities
#[derive(Clone)]
pub struct MotionTrail {
    pub enabled: bool,
    pub trail_length: usize,
    pub trail_alpha: f32,
    pub trail_color: Color,
    pub positions: Vec<(Vec2, f32)>, // Position and timestamp
    pub trail_spacing: f32, // Min distance between trail points
    pub fade_speed: f32,
}

impl MotionTrail {
    pub fn new() -> Self {
        Self {
            enabled: false,
            trail_length: 8,
            trail_alpha: 0.6,
            trail_color: WHITE,
            positions: Vec::new(),
            trail_spacing: 15.0,
            fade_speed: 3.0,
        }
    }

    /// Update trail with new position
    pub fn update(&mut self, position: Vec2, dt: f32) {
        if !self.enabled {
            self.positions.clear();
            return;
        }

        // Check if we should add a new position
        let should_add = if let Some((last_pos, _)) = self.positions.last() {
            last_pos.distance(position) >= self.trail_spacing
        } else {
            true
        };

        if should_add {
            self.positions.push((position, 0.0));
        }

        // Update timestamps and remove old positions
        let mut to_remove = Vec::new();
        for (i, (_, timestamp)) in self.positions.iter_mut().enumerate() {
            *timestamp += dt * self.fade_speed;
            if *timestamp >= 1.0 {
                to_remove.push(i);
            }
        }

        // Remove from back to front to maintain indices
        for i in to_remove.iter().rev() {
            self.positions.remove(*i);
        }

        // Limit trail length
        while self.positions.len() > self.trail_length {
            self.positions.remove(0);
        }
    }

    /// Render the motion trail
    pub fn render(&self, texture: &Texture2D, size: Vec2, rotation: f32) {
        if !self.enabled || self.positions.is_empty() {
            return;
        }

        for (i, (pos, timestamp)) in self.positions.iter().enumerate() {
            // Calculate alpha based on position in trail and lifetime
            let position_alpha = i as f32 / self.positions.len() as f32;
            let lifetime_alpha = 1.0 - *timestamp;
            let final_alpha = position_alpha * lifetime_alpha * self.trail_alpha;

            // Calculate scale (trail gets smaller towards the end)
            let scale = 0.6 + (position_alpha * 0.4);

            let mut color = self.trail_color;
            color.a = final_alpha;

            draw_texture_ex(
                texture,
                pos.x - (size.x * scale * 0.5),
                pos.y - (size.y * scale * 0.5),
                color,
                DrawTextureParams {
                    dest_size: Some(size * scale),
                    rotation,
                    ..Default::default()
                },
            );
        }
    }

    /// Enable trail with color
    pub fn enable(&mut self, color: Color) {
        self.enabled = true;
        self.trail_color = color;
    }

    /// Disable trail
    pub fn disable(&mut self) {
        self.enabled = false;
        self.positions.clear();
    }
}

/// Attack trail effect - special trail for attacks
pub struct AttackTrail {
    pub active: bool,
    pub trail_points: Vec<TrailPoint>,
    pub max_points: usize,
    pub color: Color,
    pub width: f32,
}

#[derive(Clone)]
struct TrailPoint {
    position: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

impl AttackTrail {
    pub fn new() -> Self {
        Self {
            active: false,
            trail_points: Vec::new(),
            max_points: 15,
            color: Color::new(1.0, 0.3, 0.3, 0.8),
            width: 5.0,
        }
    }

    /// Start an attack trail from position
    pub fn start(&mut self, position: Vec2, color: Color) {
        self.active = true;
        self.color = color;
        self.trail_points.clear();
        self.add_point(position);
    }

    /// Add a point to the trail
    pub fn add_point(&mut self, position: Vec2) {
        if !self.active {
            return;
        }

        self.trail_points.push(TrailPoint {
            position,
            lifetime: 0.0,
            max_lifetime: 0.3,
        });

        if self.trail_points.len() > self.max_points {
            self.trail_points.remove(0);
        }
    }

    /// Update trail
    pub fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }

        let mut to_remove = Vec::new();
        for (i, point) in self.trail_points.iter_mut().enumerate() {
            point.lifetime += dt;
            if point.lifetime >= point.max_lifetime {
                to_remove.push(i);
            }
        }

        for i in to_remove.iter().rev() {
            self.trail_points.remove(*i);
        }

        if self.trail_points.is_empty() {
            self.active = false;
        }
    }

    /// Render the attack trail
    pub fn render(&self) {
        if !self.active || self.trail_points.len() < 2 {
            return;
        }

        for i in 0..self.trail_points.len() - 1 {
            let p1 = &self.trail_points[i];
            let p2 = &self.trail_points[i + 1];

            let alpha1 = (1.0 - p1.lifetime / p1.max_lifetime) * self.color.a;
            let alpha2 = (1.0 - p2.lifetime / p2.max_lifetime) * self.color.a;

            let width1 = self.width * (1.0 - p1.lifetime / p1.max_lifetime);
            let width2 = self.width * (1.0 - p2.lifetime / p2.max_lifetime);

            // Draw line segment with gradient
            let color1 = Color::new(self.color.r, self.color.g, self.color.b, alpha1);
            let color2 = Color::new(self.color.r, self.color.g, self.color.b, alpha2);

            // Draw multiple lines for thickness
            for offset in [-width1, 0.0, width1] {
                draw_line(
                    p1.position.x + offset,
                    p1.position.y,
                    p2.position.x + offset,
                    p2.position.y,
                    width1,
                    color1,
                );
            }
        }
    }

    /// End the trail
    pub fn end(&mut self) {
        self.active = false;
    }
}
