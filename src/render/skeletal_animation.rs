use macroquad::prelude::*;
use std::collections::HashMap;

/// Bone structure for skeletal animation
#[derive(Clone, Debug)]
pub struct Bone {
    pub id: u32,
    pub name: String,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
    pub rest_position: Vec2,
    pub rest_rotation: f32,
    pub rest_scale: Vec2,
    pub length: f32,

    // Current transform
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    // Animation data
    pub local_transform: Mat3,
    pub world_transform: Mat3,

    // IK properties
    pub is_ik_target: bool,
    pub ik_chain_length: u32,
    pub constraints: BoneConstraints,
}

#[derive(Clone, Debug)]
pub struct BoneConstraints {
    pub min_rotation: f32,
    pub max_rotation: f32,
    pub locked_position: bool,
    pub locked_rotation: bool,
    pub locked_scale: bool,
}

impl Default for BoneConstraints {
    fn default() -> Self {
        Self {
            min_rotation: -std::f32::consts::PI,
            max_rotation: std::f32::consts::PI,
            locked_position: false,
            locked_rotation: false,
            locked_scale: false,
        }
    }
}

/// Skeleton containing all bones
#[derive(Clone)]
pub struct Skeleton {
    pub bones: HashMap<u32, Bone>,
    pub root_bones: Vec<u32>,
    pub bone_order: Vec<u32>,

    // Mesh binding
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u32>,
    pub bone_weights: Vec<BoneWeight>,

    // Animation state
    pub current_pose: String,
    pub blend_poses: Vec<(String, f32)>,
}

#[derive(Clone, Debug)]
pub struct Vertex2D {
    pub position: Vec2,
    pub uv: Vec2,
    pub color: Color,
    pub normal: Vec2,
}

#[derive(Clone, Debug)]
pub struct BoneWeight {
    pub vertex_index: u32,
    pub bone_indices: [u32; 4],
    pub weights: [f32; 4],
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            bones: HashMap::new(),
            root_bones: Vec::new(),
            bone_order: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            bone_weights: Vec::new(),
            current_pose: String::new(),
            blend_poses: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Bone) -> u32 {
        let id = bone.id;

        if bone.parent.is_none() {
            self.root_bones.push(id);
        } else if let Some(parent_id) = bone.parent {
            if let Some(parent) = self.bones.get_mut(&parent_id) {
                parent.children.push(id);
            }
        }

        self.bones.insert(id, bone);
        self.update_bone_order();
        id
    }

    fn update_bone_order(&mut self) {
        self.bone_order.clear();
        let mut queue = self.root_bones.clone();

        while let Some(bone_id) = queue.pop() {
            self.bone_order.push(bone_id);
            if let Some(bone) = self.bones.get(&bone_id) {
                for child_id in &bone.children {
                    queue.push(*child_id);
                }
            }
        }
    }

    pub fn update_transforms(&mut self) {
        for &bone_id in &self.bone_order {
            if let Some(bone) = self.bones.get_mut(&bone_id) {
                // Calculate local transform
                let translation = Mat3::from_translation(bone.position);
                let rotation = Mat3::from_angle(bone.rotation);
                let scale = Mat3::from_scale(bone.scale);
                bone.local_transform = translation * rotation * scale;

                // Calculate world transform
                if let Some(parent_id) = bone.parent {
                    if let Some(parent) = self.bones.get(&parent_id) {
                        bone.world_transform = parent.world_transform * bone.local_transform;
                    }
                } else {
                    bone.world_transform = bone.local_transform;
                }
            }
        }
    }

    pub fn apply_skinning(&mut self) {
        let mut deformed_vertices = self.vertices.clone();

        for weight in &self.bone_weights {
            let vertex = &self.vertices[weight.vertex_index as usize];
            let mut final_position = Vec2::ZERO;

            for i in 0..4 {
                if weight.weights[i] > 0.0 {
                    if let Some(bone) = self.bones.get(&weight.bone_indices[i]) {
                        let transformed = transform_point(&bone.world_transform, vertex.position);
                        final_position += transformed * weight.weights[i];
                    }
                }
            }

            deformed_vertices[weight.vertex_index as usize].position = final_position;
        }

        self.vertices = deformed_vertices;
    }
}

/// Skeletal animation clip
#[derive(Clone)]
pub struct SkeletalAnimation {
    pub name: String,
    pub duration: f32,
    pub keyframes: HashMap<u32, Vec<BoneKeyframe>>,
    pub looping: bool,
    pub root_motion: bool,
}

#[derive(Clone, Debug)]
pub struct BoneKeyframe {
    pub time: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub interpolation: KeyframeInterpolation,
}

#[derive(Clone, Debug)]
pub enum KeyframeInterpolation {
    Linear,
    Step,
    Bezier(Vec2, Vec2),
    Catmull,
}

impl SkeletalAnimation {
    pub fn sample(&self, time: f32, bone_id: u32) -> (Vec2, f32, Vec2) {
        if let Some(keyframes) = self.keyframes.get(&bone_id) {
            if keyframes.is_empty() {
                return (Vec2::ZERO, 0.0, Vec2::ONE);
            }

            let t = if self.looping {
                time % self.duration
            } else {
                time.min(self.duration)
            };

            // Find surrounding keyframes
            let mut prev_frame = &keyframes[0];
            let mut next_frame = &keyframes[0];

            for i in 0..keyframes.len() - 1 {
                if keyframes[i].time <= t && keyframes[i + 1].time > t {
                    prev_frame = &keyframes[i];
                    next_frame = &keyframes[i + 1];
                    break;
                }
            }

            if prev_frame.time == next_frame.time {
                return (prev_frame.position, prev_frame.rotation, prev_frame.scale);
            }

            let alpha = (t - prev_frame.time) / (next_frame.time - prev_frame.time);

            match &prev_frame.interpolation {
                KeyframeInterpolation::Linear => {
                    let position = prev_frame.position.lerp(next_frame.position, alpha);
                    let rotation = lerp_angle(prev_frame.rotation, next_frame.rotation, alpha);
                    let scale = prev_frame.scale.lerp(next_frame.scale, alpha);
                    (position, rotation, scale)
                }
                KeyframeInterpolation::Step => {
                    (prev_frame.position, prev_frame.rotation, prev_frame.scale)
                }
                KeyframeInterpolation::Bezier(p1, p2) => {
                    let t = cubic_bezier(alpha, *p1, *p2);
                    let position = prev_frame.position.lerp(next_frame.position, t);
                    let rotation = lerp_angle(prev_frame.rotation, next_frame.rotation, t);
                    let scale = prev_frame.scale.lerp(next_frame.scale, t);
                    (position, rotation, scale)
                }
                KeyframeInterpolation::Catmull => {
                    let position = catmull_rom_spline(
                        get_prev_keyframe(keyframes, prev_frame).position,
                        prev_frame.position,
                        next_frame.position,
                        get_next_keyframe(keyframes, next_frame).position,
                        alpha,
                    );
                    let rotation = lerp_angle(prev_frame.rotation, next_frame.rotation, alpha);
                    let scale = catmull_rom_spline(
                        get_prev_keyframe(keyframes, prev_frame).scale,
                        prev_frame.scale,
                        next_frame.scale,
                        get_next_keyframe(keyframes, next_frame).scale,
                        alpha,
                    );
                    (position, rotation, scale)
                }
            }
        } else {
            (Vec2::ZERO, 0.0, Vec2::ONE)
        }
    }
}

/// Advanced skeletal animation system
pub struct SkeletalAnimationSystem {
    pub skeletons: HashMap<u32, Skeleton>,
    pub animations: HashMap<String, SkeletalAnimation>,
    pub animation_states: HashMap<u32, AnimationState>,

    // IK solver
    pub ik_chains: Vec<IKChain>,

    // Animation blending
    pub blend_trees: HashMap<String, BlendTree>,

    // Procedural animation
    pub procedural_bones: Vec<ProceduralBone>,
}

#[derive(Clone)]
pub struct AnimationState {
    pub skeleton_id: u32,
    pub current_animation: String,
    pub time: f32,
    pub speed: f32,
    pub weight: f32,
    pub blend_animations: Vec<(String, f32, f32)>, // (name, time, weight)
    pub transition_time: f32,
    pub transition_duration: f32,
}

#[derive(Clone)]
pub struct IKChain {
    pub skeleton_id: u32,
    pub target_bone: u32,
    pub chain_length: u32,
    pub target_position: Vec2,
    pub pole_target: Option<Vec2>,
    pub iterations: u32,
    pub tolerance: f32,
    pub weight: f32,
}

#[derive(Clone)]
pub struct BlendTree {
    pub name: String,
    pub nodes: Vec<BlendNode>,
    pub parameters: HashMap<String, f32>,
}

#[derive(Clone)]
pub enum BlendNode {
    Animation(String),
    Blend2D(Box<BlendNode>, Box<BlendNode>, String), // parameter name
    BlendAdditive(Box<BlendNode>, Box<BlendNode>, f32),
    BlendOverride(Box<BlendNode>, Box<BlendNode>, Vec<u32>, f32), // bone mask
}

#[derive(Clone)]
pub struct ProceduralBone {
    pub skeleton_id: u32,
    pub bone_id: u32,
    pub animation_type: ProceduralAnimationType,
    pub parameters: HashMap<String, f32>,
}

#[derive(Clone)]
pub enum ProceduralAnimationType {
    LookAt(Vec2),
    Wiggle(f32, f32), // frequency, amplitude
    Spring(f32, f32, f32), // stiffness, damping, mass
    Pendulum(f32, f32), // gravity, length
    Noise(f32, f32), // frequency, amplitude
}

impl SkeletalAnimationSystem {
    pub fn new() -> Self {
        Self {
            skeletons: HashMap::new(),
            animations: HashMap::new(),
            animation_states: HashMap::new(),
            ik_chains: Vec::new(),
            blend_trees: HashMap::new(),
            procedural_bones: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update animation states
        for (skeleton_id, state) in &mut self.animation_states {
            state.time += dt * state.speed;

            // Apply animation to skeleton
            if let Some(skeleton) = self.skeletons.get_mut(skeleton_id) {
                self.apply_animation(skeleton, state);
            }
        }

        // Solve IK chains
        for chain in &self.ik_chains {
            if let Some(skeleton) = self.skeletons.get_mut(&chain.skeleton_id) {
                self.solve_ik(skeleton, chain);
            }
        }

        // Apply procedural animations
        for procedural in &self.procedural_bones {
            if let Some(skeleton) = self.skeletons.get_mut(&procedural.skeleton_id) {
                self.apply_procedural_animation(skeleton, procedural, dt);
            }
        }

        // Update skeleton transforms
        for skeleton in self.skeletons.values_mut() {
            skeleton.update_transforms();
            skeleton.apply_skinning();
        }
    }

    fn apply_animation(&self, skeleton: &mut Skeleton, state: &AnimationState) {
        if let Some(animation) = self.animations.get(&state.current_animation) {
            for (&bone_id, bone) in &mut skeleton.bones {
                let (position, rotation, scale) = animation.sample(state.time, bone_id);

                if state.blend_animations.is_empty() {
                    bone.position = position;
                    bone.rotation = rotation;
                    bone.scale = scale;
                } else {
                    // Blend with other animations
                    let mut final_position = position * state.weight;
                    let mut final_rotation = rotation * state.weight;
                    let mut final_scale = scale * state.weight;

                    for (blend_name, blend_time, blend_weight) in &state.blend_animations {
                        if let Some(blend_anim) = self.animations.get(blend_name) {
                            let (p, r, s) = blend_anim.sample(*blend_time, bone_id);
                            final_position += p * blend_weight;
                            final_rotation += r * blend_weight;
                            final_scale += s * blend_weight;
                        }
                    }

                    bone.position = final_position;
                    bone.rotation = final_rotation;
                    bone.scale = final_scale;
                }
            }
        }
    }

    fn solve_ik(&self, skeleton: &mut Skeleton, chain: &IKChain) {
        // FABRIK (Forward And Backward Reaching Inverse Kinematics) algorithm
        let mut bone_positions = Vec::new();
        let mut bone_ids = Vec::new();
        let mut current_bone = chain.target_bone;

        // Collect bones in chain
        for _ in 0..chain.chain_length {
            if let Some(bone) = skeleton.bones.get(&current_bone) {
                bone_positions.push(bone.position);
                bone_ids.push(current_bone);

                if let Some(parent) = bone.parent {
                    current_bone = parent;
                } else {
                    break;
                }
            }
        }

        if bone_positions.len() < 2 {
            return;
        }

        let root_pos = bone_positions.last().unwrap().clone();

        for _ in 0..chain.iterations {
            // Forward pass
            bone_positions[0] = chain.target_position;

            for i in 1..bone_positions.len() {
                let direction = (bone_positions[i] - bone_positions[i - 1]).normalize();
                if let Some(bone) = skeleton.bones.get(&bone_ids[i - 1]) {
                    bone_positions[i] = bone_positions[i - 1] + direction * bone.length;
                }
            }

            // Backward pass
            bone_positions[bone_positions.len() - 1] = root_pos;

            for i in (0..bone_positions.len() - 1).rev() {
                let direction = (bone_positions[i] - bone_positions[i + 1]).normalize();
                if let Some(bone) = skeleton.bones.get(&bone_ids[i]) {
                    bone_positions[i] = bone_positions[i + 1] + direction * bone.length;
                }
            }

            // Check tolerance
            let error = bone_positions[0].distance(chain.target_position);
            if error < chain.tolerance {
                break;
            }
        }

        // Apply positions back to bones
        for (i, &bone_id) in bone_ids.iter().enumerate() {
            if let Some(bone) = skeleton.bones.get_mut(&bone_id) {
                let blended_position = bone.position.lerp(bone_positions[i], chain.weight);
                bone.position = blended_position;

                // Calculate rotation to point to next bone
                if i > 0 {
                    let direction = (bone_positions[i - 1] - bone_positions[i]).normalize();
                    bone.rotation = direction.y.atan2(direction.x);
                }
            }
        }
    }

    fn apply_procedural_animation(&self, skeleton: &mut Skeleton, procedural: &ProceduralBone, dt: f32) {
        if let Some(bone) = skeleton.bones.get_mut(&procedural.bone_id) {
            match &procedural.animation_type {
                ProceduralAnimationType::LookAt(target) => {
                    let direction = (*target - bone.position).normalize();
                    bone.rotation = direction.y.atan2(direction.x);
                }
                ProceduralAnimationType::Wiggle(frequency, amplitude) => {
                    let time = get_time() as f32;
                    bone.rotation += (time * frequency).sin() * amplitude * dt;
                }
                ProceduralAnimationType::Spring(stiffness, damping, mass) => {
                    // Simple spring physics
                    let rest_rotation = bone.rest_rotation;
                    let displacement = bone.rotation - rest_rotation;
                    let spring_force = -stiffness * displacement;
                    let damping_force = -damping * 0.0; // Would need velocity tracking
                    let acceleration = (spring_force + damping_force) / mass;
                    bone.rotation += acceleration * dt * dt;
                }
                ProceduralAnimationType::Pendulum(gravity, length) => {
                    let angular_acceleration = -(gravity / length) * bone.rotation.sin();
                    bone.rotation += angular_acceleration * dt * dt;
                }
                ProceduralAnimationType::Noise(frequency, amplitude) => {
                    let time = get_time() as f32;
                    let noise = perlin_noise(time * frequency) * amplitude;
                    bone.rotation = bone.rest_rotation + noise;
                }
            }
        }
    }
}

// Utility functions
fn transform_point(mat: &Mat3, point: Vec2) -> Vec2 {
    let homogeneous = vec3(point.x, point.y, 1.0);
    let transformed = mat.mul_vec3(homogeneous);
    Vec2::new(transformed.x, transformed.y)
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff = b - a;
    let wrapped = ((diff + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)) - std::f32::consts::PI;
    a + wrapped * t
}

fn cubic_bezier(t: f32, p1: Vec2, p2: Vec2) -> f32 {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;

    mt2 * mt * 0.0 + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t2 * t * 1.0
}

fn catmull_rom_spline(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;

    Vec2::new(
        0.5 * ((2.0 * p1.x) + (-p0.x + p2.x) * t + (2.0 * p0.x - 5.0 * p1.x + 4.0 * p2.x - p3.x) * t2 + (-p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x) * t3),
        0.5 * ((2.0 * p1.y) + (-p0.y + p2.y) * t + (2.0 * p0.y - 5.0 * p1.y + 4.0 * p2.y - p3.y) * t2 + (-p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y) * t3),
    )
}

fn get_prev_keyframe<'a>(keyframes: &'a [BoneKeyframe], current: &BoneKeyframe) -> &'a BoneKeyframe {
    for i in 0..keyframes.len() {
        if keyframes[i].time == current.time && i > 0 {
            return &keyframes[i - 1];
        }
    }
    current
}

fn get_next_keyframe<'a>(keyframes: &'a [BoneKeyframe], current: &BoneKeyframe) -> &'a BoneKeyframe {
    for i in 0..keyframes.len() {
        if keyframes[i].time == current.time && i < keyframes.len() - 1 {
            return &keyframes[i + 1];
        }
    }
    current
}

fn perlin_noise(x: f32) -> f32 {
    // Simple noise function for procedural animation
    (x * 12.9898).sin() * 43758.5453 % 1.0
}