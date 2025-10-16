use macroquad::prelude::*;
use crate::render::{
    sprite_renderer::{Sprite, SpriteRenderer, Animation, AnimationFrame, Light2D},
    skeletal_animation::{Skeleton, SkeletalAnimationSystem, SkeletalAnimation},
    shader_system::{ShaderSystem, MaterialProperties},
};
use std::collections::HashMap;

/// Advanced character renderer with realistic features
pub struct CharacterRenderer {
    pub sprite_renderer: SpriteRenderer,
    pub skeletal_system: SkeletalAnimationSystem,
    pub shader_system: ShaderSystem,

    // Character models
    pub character_models: HashMap<String, CharacterModel>,

    // Render settings
    pub enable_shadows: bool,
    pub enable_reflections: bool,
    pub enable_subsurface_scattering: bool,
    pub enable_hair_physics: bool,
    pub enable_cloth_simulation: bool,

    // Lighting
    pub rim_light_intensity: f32,
    pub subsurface_color: Color,

    // LOD (Level of Detail)
    pub lod_distances: [f32; 3],
    pub current_lod: u32,
}

#[derive(Clone)]
pub struct CharacterModel {
    pub name: String,

    // Visual components
    pub base_sprite: Sprite,
    pub skeleton: Skeleton,
    pub animations: HashMap<String, SkeletalAnimation>,
    pub material: MaterialProperties,

    // Body parts for layered rendering
    pub body_parts: HashMap<String, BodyPart>,

    // Customization
    pub skin_color: Color,
    pub hair_color: Color,
    pub eye_color: Color,
    pub outfit_colors: Vec<Color>,

    // Attachments
    pub equipment: Vec<Equipment>,
    pub accessories: Vec<Accessory>,

    // Physics
    pub hair_strands: Vec<HairStrand>,
    pub cloth_pieces: Vec<ClothPiece>,

    // Effects
    pub aura_effect: Option<AuraEffect>,
    pub trail_effect: Option<TrailEffect>,
}

#[derive(Clone)]
pub struct BodyPart {
    pub name: String,
    pub sprite: Sprite,
    pub bone_attachment: u32,
    pub offset: Vec2,
    pub layer: i32,
    pub can_be_hidden: bool,
    pub material_override: Option<MaterialProperties>,
}

#[derive(Clone)]
pub struct Equipment {
    pub slot: EquipmentSlot,
    pub sprite: Sprite,
    pub material: MaterialProperties,
    pub stat_modifiers: HashMap<String, f32>,
    pub visual_effects: Vec<VisualEffect>,
}

#[derive(Clone, Debug)]
pub enum EquipmentSlot {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
    Back,
}

#[derive(Clone)]
pub struct Accessory {
    pub name: String,
    pub sprite: Sprite,
    pub attachment_bone: u32,
    pub offset: Vec2,
    pub rotation_offset: f32,
    pub follows_animation: bool,
}

#[derive(Clone)]
pub struct HairStrand {
    pub points: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub thickness: f32,
    pub color: Color,
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Clone)]
pub struct ClothPiece {
    pub vertices: Vec<Vec2>,
    pub uvs: Vec<Vec2>,
    pub constraints: Vec<ClothConstraint>,
    pub material: MaterialProperties,
    pub wind_affected: bool,
}

#[derive(Clone)]
pub struct ClothConstraint {
    pub point_a: usize,
    pub point_b: usize,
    pub rest_distance: f32,
    pub stiffness: f32,
}

#[derive(Clone)]
pub struct AuraEffect {
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub pulse_speed: f32,
    pub particle_density: f32,
}

#[derive(Clone)]
pub struct TrailEffect {
    pub positions: Vec<Vec2>,
    pub max_length: usize,
    pub width: f32,
    pub color_start: Color,
    pub color_end: Color,
    pub fade_speed: f32,
}

#[derive(Clone)]
pub enum VisualEffect {
    Glow(Color, f32),
    Sparkle(f32, f32),
    Fire(Color, f32),
    Lightning(Color, f32),
    Smoke(Color, f32),
}

impl CharacterRenderer {
    pub fn new() -> Self {
        Self {
            sprite_renderer: SpriteRenderer::new(),
            skeletal_system: SkeletalAnimationSystem::new(),
            shader_system: ShaderSystem::new(),
            character_models: HashMap::new(),
            enable_shadows: true,
            enable_reflections: true,
            enable_subsurface_scattering: true,
            enable_hair_physics: true,
            enable_cloth_simulation: true,
            rim_light_intensity: 0.3,
            subsurface_color: Color::new(1.0, 0.7, 0.7, 0.3),
            lod_distances: [10.0, 25.0, 50.0],
            current_lod: 0,
        }
    }

    pub fn create_character(&mut self, name: String) -> CharacterModel {
        let mut model = CharacterModel {
            name,
            base_sprite: Sprite::new(Texture2D::empty()),
            skeleton: Skeleton::new(),
            animations: HashMap::new(),
            material: MaterialProperties::default(),
            body_parts: HashMap::new(),
            skin_color: Color::new(1.0, 0.85, 0.75, 1.0),
            hair_color: Color::new(0.2, 0.15, 0.1, 1.0),
            eye_color: Color::new(0.3, 0.5, 0.8, 1.0),
            outfit_colors: vec![
                Color::new(0.2, 0.3, 0.8, 1.0),
                Color::new(0.8, 0.2, 0.2, 1.0),
            ],
            equipment: Vec::new(),
            accessories: Vec::new(),
            hair_strands: Vec::new(),
            cloth_pieces: Vec::new(),
            aura_effect: None,
            trail_effect: None,
        };

        // Create default body parts
        self.create_default_body_parts(&mut model);

        // Setup skeleton
        self.create_character_skeleton(&mut model);

        // Add default animations
        self.create_default_animations(&mut model);

        model
    }

    fn create_default_body_parts(&self, model: &mut CharacterModel) {
        let body_part_names = vec![
            ("head", 5),
            ("torso", 3),
            ("left_arm", 4),
            ("right_arm", 4),
            ("left_leg", 2),
            ("right_leg", 2),
            ("hair_front", 6),
            ("hair_back", 1),
        ];

        for (name, layer) in body_part_names {
            let part = BodyPart {
                name: name.to_string(),
                sprite: Sprite::new(Texture2D::empty()),
                bone_attachment: 0, // Will be set based on skeleton
                offset: Vec2::ZERO,
                layer,
                can_be_hidden: name.contains("hair"),
                material_override: None,
            };
            model.body_parts.insert(name.to_string(), part);
        }
    }

    fn create_character_skeleton(&self, model: &mut CharacterModel) {
        // This would create a full humanoid skeleton
        // For now, simplified version
        let mut skeleton = Skeleton::new();

        // Add bones (simplified)
        let bone_data = vec![
            ("root", None, Vec2::new(0.0, 0.0)),
            ("spine", Some("root"), Vec2::new(0.0, -10.0)),
            ("chest", Some("spine"), Vec2::new(0.0, -20.0)),
            ("neck", Some("chest"), Vec2::new(0.0, -10.0)),
            ("head", Some("neck"), Vec2::new(0.0, -15.0)),
            ("left_shoulder", Some("chest"), Vec2::new(-10.0, -5.0)),
            ("left_arm", Some("left_shoulder"), Vec2::new(-15.0, 0.0)),
            ("left_forearm", Some("left_arm"), Vec2::new(-15.0, 0.0)),
            ("left_hand", Some("left_forearm"), Vec2::new(-10.0, 0.0)),
            ("right_shoulder", Some("chest"), Vec2::new(10.0, -5.0)),
            ("right_arm", Some("right_shoulder"), Vec2::new(15.0, 0.0)),
            ("right_forearm", Some("right_arm"), Vec2::new(15.0, 0.0)),
            ("right_hand", Some("right_forearm"), Vec2::new(10.0, 0.0)),
            ("pelvis", Some("root"), Vec2::new(0.0, 10.0)),
            ("left_hip", Some("pelvis"), Vec2::new(-8.0, 5.0)),
            ("left_thigh", Some("left_hip"), Vec2::new(0.0, 20.0)),
            ("left_shin", Some("left_thigh"), Vec2::new(0.0, 20.0)),
            ("left_foot", Some("left_shin"), Vec2::new(0.0, 10.0)),
            ("right_hip", Some("pelvis"), Vec2::new(8.0, 5.0)),
            ("right_thigh", Some("right_hip"), Vec2::new(0.0, 20.0)),
            ("right_shin", Some("right_thigh"), Vec2::new(0.0, 20.0)),
            ("right_foot", Some("right_shin"), Vec2::new(0.0, 10.0)),
        ];

        model.skeleton = skeleton;
    }

    fn create_default_animations(&self, model: &mut CharacterModel) {
        // Create basic animations
        let animation_names = vec![
            "idle",
            "walk",
            "run",
            "jump",
            "attack_light",
            "attack_heavy",
            "hurt",
            "death",
            "victory",
        ];

        for name in animation_names {
            let animation = SkeletalAnimation {
                name: name.to_string(),
                duration: 1.0,
                keyframes: HashMap::new(),
                looping: name == "idle" || name == "walk" || name == "run",
                root_motion: name == "walk" || name == "run",
            };
            model.animations.insert(name.to_string(), animation);
        }
    }

    pub fn render_character(
        &mut self,
        model: &CharacterModel,
        position: Vec2,
        animation_name: &str,
        animation_time: f32,
        camera_position: Vec2,
    ) {
        // Calculate LOD based on distance
        let distance = position.distance(camera_position);
        self.current_lod = self.calculate_lod(distance);

        // Apply skeletal animation
        if let Some(skeleton_id) = self.skeletal_system.skeletons.iter().find_map(|(id, skel)| {
            if skel.current_pose == animation_name {
                Some(*id)
            } else {
                None
            }
        }) {
            self.skeletal_system.update(0.016); // 60 FPS frame time
        }

        // Setup lighting for character
        self.setup_character_lighting(model, position);

        // Use appropriate shader based on LOD
        match self.current_lod {
            0 => {
                // High quality PBR rendering
                self.shader_system.use_shader("pbr");
                self.render_high_quality(model, position);
            }
            1 => {
                // Medium quality with simplified shading
                self.shader_system.use_shader("lighting");
                self.render_medium_quality(model, position);
            }
            _ => {
                // Low quality sprite rendering
                self.render_low_quality(model, position);
            }
        }

        // Render effects
        if self.current_lod < 2 {
            self.render_character_effects(model, position);
        }

        // Render physics-based elements
        if self.enable_hair_physics && self.current_lod == 0 {
            self.render_hair(model, position);
        }

        if self.enable_cloth_simulation && self.current_lod < 2 {
            self.render_cloth(model, position);
        }
    }

    fn calculate_lod(&self, distance: f32) -> u32 {
        if distance < self.lod_distances[0] {
            0
        } else if distance < self.lod_distances[1] {
            1
        } else if distance < self.lod_distances[2] {
            2
        } else {
            3
        }
    }

    fn setup_character_lighting(&mut self, model: &CharacterModel, position: Vec2) {
        self.sprite_renderer.clear_lights();

        // Key light
        self.sprite_renderer.add_light(Light2D {
            position: position + Vec2::new(50.0, -50.0),
            color: Color::new(1.0, 0.95, 0.8, 1.0),
            intensity: 1.0,
            radius: 200.0,
            falloff: 2.0,
            cast_shadows: self.enable_shadows,
        });

        // Fill light
        self.sprite_renderer.add_light(Light2D {
            position: position + Vec2::new(-30.0, -20.0),
            color: Color::new(0.6, 0.7, 0.9, 1.0),
            intensity: 0.5,
            radius: 150.0,
            falloff: 1.5,
            cast_shadows: false,
        });

        // Rim light
        self.sprite_renderer.add_light(Light2D {
            position: position + Vec2::new(0.0, -80.0),
            color: WHITE,
            intensity: self.rim_light_intensity,
            radius: 100.0,
            falloff: 3.0,
            cast_shadows: false,
        });

        // Aura light if present
        if let Some(aura) = &model.aura_effect {
            self.sprite_renderer.add_light(Light2D {
                position,
                color: aura.color,
                intensity: aura.intensity,
                radius: aura.radius,
                falloff: 1.0,
                cast_shadows: false,
            });
        }
    }

    fn render_high_quality(&mut self, model: &CharacterModel, position: Vec2) {
        // Render body parts in layer order
        let mut parts: Vec<_> = model.body_parts.values().collect();
        parts.sort_by_key(|p| p.layer);

        for part in parts {
            if part.can_be_hidden {
                continue; // Skip if hidden
            }

            let mut sprite = part.sprite.clone();
            sprite.position = position + part.offset;

            // Apply material properties
            if let Some(material) = &part.material_override {
                sprite.metallic = material.metallic;
                sprite.roughness = material.roughness;
                sprite.emission_strength = material.emission_strength;
            }

            // Apply skin color tinting
            if part.name.contains("arm") || part.name.contains("leg") || part.name == "head" {
                sprite.color = self.blend_colors(sprite.color, model.skin_color, 0.5);
            }

            // Apply subsurface scattering for skin
            if self.enable_subsurface_scattering && part.name.contains("head") {
                sprite.color = self.blend_colors(sprite.color, self.subsurface_color, 0.2);
            }

            self.sprite_renderer.draw_sprite(&sprite);
        }

        // Render equipment
        for equipment in &model.equipment {
            let mut sprite = equipment.sprite.clone();
            sprite.position = position;

            // Apply equipment visual effects
            for effect in &equipment.visual_effects {
                self.apply_visual_effect(&mut sprite, effect);
            }

            self.sprite_renderer.draw_sprite(&sprite);
        }
    }

    fn render_medium_quality(&mut self, model: &CharacterModel, position: Vec2) {
        // Simplified rendering with fewer layers
        let mut combined_sprite = model.base_sprite.clone();
        combined_sprite.position = position;
        self.sprite_renderer.draw_sprite(&combined_sprite);
    }

    fn render_low_quality(&mut self, model: &CharacterModel, position: Vec2) {
        // Simple sprite rendering
        draw_texture(
            &model.base_sprite.texture,
            position.x,
            position.y,
            model.base_sprite.color,
        );
    }

    fn render_character_effects(&mut self, model: &CharacterModel, position: Vec2) {
        // Render aura effect
        if let Some(aura) = &model.aura_effect {
            let time = get_time() as f32;
            let pulse = (time * aura.pulse_speed).sin() * 0.5 + 0.5;
            let radius = aura.radius * (1.0 + pulse * 0.2);

            draw_circle_lines(
                position.x,
                position.y,
                radius,
                2.0,
                Color::new(
                    aura.color.r,
                    aura.color.g,
                    aura.color.b,
                    aura.color.a * pulse,
                ),
            );
        }

        // Render trail effect
        if let Some(trail) = &model.trail_effect {
            for i in 1..trail.positions.len() {
                let alpha = (i as f32 / trail.positions.len() as f32) * trail.fade_speed;
                let color = self.blend_colors(trail.color_start, trail.color_end, alpha);

                draw_line(
                    trail.positions[i - 1].x,
                    trail.positions[i - 1].y,
                    trail.positions[i].x,
                    trail.positions[i].y,
                    trail.width * (1.0 - alpha * 0.5),
                    Color::new(color.r, color.g, color.b, color.a * alpha),
                );
            }
        }
    }

    fn render_hair(&self, model: &CharacterModel, position: Vec2) {
        for strand in &model.hair_strands {
            for i in 1..strand.points.len() {
                let thickness = strand.thickness * (1.0 - i as f32 / strand.points.len() as f32);
                draw_line(
                    position.x + strand.points[i - 1].x,
                    position.y + strand.points[i - 1].y,
                    position.x + strand.points[i].x,
                    position.y + strand.points[i].y,
                    thickness,
                    strand.color,
                );
            }
        }
    }

    fn render_cloth(&self, model: &CharacterModel, position: Vec2) {
        for cloth in &model.cloth_pieces {
            // Render cloth mesh
            // This would use the vertices and constraints to render deformable cloth
            // For now, simplified rendering
            if cloth.vertices.len() >= 3 {
                for i in 2..cloth.vertices.len() {
                    draw_triangle(
                        Vec2::new(
                            position.x + cloth.vertices[0].x,
                            position.y + cloth.vertices[0].y,
                        ),
                        Vec2::new(
                            position.x + cloth.vertices[i - 1].x,
                            position.y + cloth.vertices[i - 1].y,
                        ),
                        Vec2::new(
                            position.x + cloth.vertices[i].x,
                            position.y + cloth.vertices[i].y,
                        ),
                        Color::new(0.5, 0.5, 0.5, 0.8),
                    );
                }
            }
        }
    }

    fn apply_visual_effect(&self, sprite: &mut Sprite, effect: &VisualEffect) {
        match effect {
            VisualEffect::Glow(color, intensity) => {
                sprite.emission_strength = *intensity;
                sprite.color = self.blend_colors(sprite.color, *color, 0.5);
            }
            VisualEffect::Sparkle(frequency, amplitude) => {
                let time = get_time() as f32;
                let sparkle = (time * frequency).sin() * amplitude;
                sprite.emission_strength += sparkle;
            }
            VisualEffect::Fire(color, intensity) => {
                sprite.color = self.blend_colors(sprite.color, *color, *intensity);
                sprite.emission_strength = *intensity;
            }
            VisualEffect::Lightning(color, intensity) => {
                let time = get_time() as f32;
                let flicker = if (time * 10.0).sin() > 0.8 { 1.0 } else { 0.0 };
                sprite.color = self.blend_colors(sprite.color, *color, flicker * *intensity);
            }
            VisualEffect::Smoke(color, density) => {
                sprite.color.a *= 1.0 - density;
                sprite.color = self.blend_colors(sprite.color, *color, *density);
            }
        }
    }

    fn blend_colors(&self, a: Color, b: Color, t: f32) -> Color {
        Color::new(
            a.r * (1.0 - t) + b.r * t,
            a.g * (1.0 - t) + b.g * t,
            a.b * (1.0 - t) + b.b * t,
            a.a * (1.0 - t) + b.a * t,
        )
    }

    pub fn update_physics(&mut self, dt: f32) {
        // Update hair physics
        for model in self.character_models.values_mut() {
            for strand in &mut model.hair_strands {
                for i in 1..strand.points.len() {
                    // Simple spring physics
                    let displacement = strand.points[i] - strand.points[i - 1];
                    let spring_force = displacement * -strand.stiffness;
                    let damping_force = strand.velocities[i] * -strand.damping;
                    let gravity = Vec2::new(0.0, 98.0);

                    let acceleration = (spring_force + damping_force + gravity) * dt;
                    strand.velocities[i] += acceleration;
                    strand.points[i] += strand.velocities[i] * dt;
                }
            }

            // Update cloth physics
            for cloth in &mut model.cloth_pieces {
                // Apply constraints
                for constraint in &cloth.constraints {
                    let p1 = cloth.vertices[constraint.point_a];
                    let p2 = cloth.vertices[constraint.point_b];
                    let distance = p1.distance(p2);
                    let difference = constraint.rest_distance - distance;

                    if difference.abs() > 0.001 {
                        let correction = (p2 - p1).normalize() * (difference * 0.5);
                        cloth.vertices[constraint.point_a] -= correction * constraint.stiffness;
                        cloth.vertices[constraint.point_b] += correction * constraint.stiffness;
                    }
                }

                // Apply wind if enabled
                if cloth.wind_affected {
                    let wind = Vec2::new((get_time() as f32).sin() * 5.0, 0.0);
                    for vertex in &mut cloth.vertices {
                        *vertex += wind * dt;
                    }
                }
            }
        }
    }
}