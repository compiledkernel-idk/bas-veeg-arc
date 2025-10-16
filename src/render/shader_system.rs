use macroquad::prelude::*;
use std::collections::HashMap;

/// Advanced shader system for realistic rendering
pub struct ShaderSystem {
    pub shaders: HashMap<String, ShaderPipeline>,
    pub active_shader: Option<String>,
    pub global_uniforms: HashMap<String, UniformValue>,
    pub render_targets: HashMap<String, RenderTarget>,
}

pub struct ShaderPipeline {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub material: Material,
    pub uniforms: HashMap<String, UniformValue>,
}

#[derive(Clone)]
pub enum UniformValue {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat3(Mat3),
    Mat4(Mat4),
    Texture(Texture2D),
    Color(Color),
}

impl ShaderSystem {
    pub fn new() -> Self {
        let mut system = Self {
            shaders: HashMap::new(),
            active_shader: None,
            global_uniforms: HashMap::new(),
            render_targets: HashMap::new(),
        };

        // Initialize default shaders
        system.create_default_shaders();
        system
    }

    fn create_default_shaders(&mut self) {
        // PBR-like shader for realistic materials
        self.add_shader(
            "pbr".to_string(),
            Self::pbr_vertex_shader(),
            Self::pbr_fragment_shader(),
        );

        // Lighting shader with normal mapping
        self.add_shader(
            "lighting".to_string(),
            Self::lighting_vertex_shader(),
            Self::lighting_fragment_shader(),
        );

        // Bloom effect shader
        self.add_shader(
            "bloom".to_string(),
            Self::bloom_vertex_shader(),
            Self::bloom_fragment_shader(),
        );

        // Shadow mapping shader
        self.add_shader(
            "shadow".to_string(),
            Self::shadow_vertex_shader(),
            Self::shadow_fragment_shader(),
        );

        // Water/liquid shader with refraction
        self.add_shader(
            "water".to_string(),
            Self::water_vertex_shader(),
            Self::water_fragment_shader(),
        );

        // Particle shader with soft blending
        self.add_shader(
            "particle".to_string(),
            Self::particle_vertex_shader(),
            Self::particle_fragment_shader(),
        );
    }

    pub fn add_shader(&mut self, name: String, vertex: String, fragment: String) {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: &vertex,
                fragment: &fragment,
            },
            MaterialParams::default(),
        ).unwrap();

        self.shaders.insert(
            name,
            ShaderPipeline {
                vertex_shader: vertex,
                fragment_shader: fragment,
                material,
                uniforms: HashMap::new(),
            },
        );
    }

    pub fn set_uniform(&mut self, shader: &str, name: &str, value: UniformValue) {
        if let Some(pipeline) = self.shaders.get_mut(shader) {
            pipeline.uniforms.insert(name.to_string(), value.clone());

            // Apply uniform to material
            match value {
                UniformValue::Float(v) => {
                    pipeline.material.set_uniform(name, UniformType::Float1(v));
                }
                UniformValue::Vec2(v) => {
                    pipeline.material.set_uniform(name, UniformType::Float2(v.x, v.y));
                }
                UniformValue::Vec3(v) => {
                    pipeline.material.set_uniform(name, UniformType::Float3(v.x, v.y, v.z));
                }
                UniformValue::Vec4(v) => {
                    pipeline.material.set_uniform(name, UniformType::Float4(v.x, v.y, v.z, v.w));
                }
                UniformValue::Color(c) => {
                    pipeline.material.set_uniform(
                        name,
                        UniformType::Float4(c.r, c.g, c.b, c.a),
                    );
                }
                _ => {}
            }
        }
    }

    pub fn use_shader(&mut self, name: &str) {
        if let Some(pipeline) = self.shaders.get(name) {
            gl_use_material(&pipeline.material);
            self.active_shader = Some(name.to_string());
        }
    }

    pub fn reset_shader(&mut self) {
        gl_use_default_material();
        self.active_shader = None;
    }

    // PBR Shader Implementation
    fn pbr_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec3 normal;
        attribute vec3 tangent;

        uniform mat4 Model;
        uniform mat4 Projection;

        varying vec2 uv;
        varying vec3 world_position;
        varying vec3 world_normal;
        varying vec3 world_tangent;
        varying vec3 world_bitangent;

        void main() {
            vec4 world_pos = Model * vec4(position, 1.0);
            world_position = world_pos.xyz;
            world_normal = mat3(Model) * normal;
            world_tangent = mat3(Model) * tangent;
            world_bitangent = cross(world_normal, world_tangent);

            uv = texcoord;
            gl_Position = Projection * world_pos;
        }
        "#.to_string()
    }

    fn pbr_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec2 uv;
        varying vec3 world_position;
        varying vec3 world_normal;
        varying vec3 world_tangent;
        varying vec3 world_bitangent;

        uniform sampler2D albedo_texture;
        uniform sampler2D normal_texture;
        uniform sampler2D metallic_texture;
        uniform sampler2D roughness_texture;
        uniform sampler2D ao_texture;
        uniform sampler2D emission_texture;

        uniform vec3 camera_position;
        uniform vec3 light_positions[8];
        uniform vec3 light_colors[8];
        uniform float light_intensities[8];
        uniform int num_lights;

        uniform vec3 ambient_color;
        uniform float metallic;
        uniform float roughness;
        uniform float emission_strength;

        const float PI = 3.14159265359;

        // PBR Functions
        float DistributionGGX(vec3 N, vec3 H, float roughness) {
            float a = roughness * roughness;
            float a2 = a * a;
            float NdotH = max(dot(N, H), 0.0);
            float NdotH2 = NdotH * NdotH;

            float num = a2;
            float denom = NdotH2 * (a2 - 1.0) + 1.0;
            denom = PI * denom * denom;

            return num / max(denom, 0.0001);
        }

        float GeometrySchlickGGX(float NdotV, float roughness) {
            float r = roughness + 1.0;
            float k = (r * r) / 8.0;

            float num = NdotV;
            float denom = NdotV * (1.0 - k) + k;

            return num / max(denom, 0.0001);
        }

        float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
            float NdotV = max(dot(N, V), 0.0);
            float NdotL = max(dot(N, L), 0.0);
            float ggx2 = GeometrySchlickGGX(NdotV, roughness);
            float ggx1 = GeometrySchlickGGX(NdotL, roughness);

            return ggx1 * ggx2;
        }

        vec3 fresnelSchlick(float cosTheta, vec3 F0) {
            return F0 + (1.0 - F0) * pow(max(1.0 - cosTheta, 0.0), 5.0);
        }

        vec3 getNormalFromMap() {
            vec3 tangentNormal = texture2D(normal_texture, uv).xyz * 2.0 - 1.0;

            mat3 TBN = mat3(
                normalize(world_tangent),
                normalize(world_bitangent),
                normalize(world_normal)
            );

            return normalize(TBN * tangentNormal);
        }

        void main() {
            vec3 albedo = pow(texture2D(albedo_texture, uv).rgb, vec2(2.2));
            vec3 normal = getNormalFromMap();
            float metallic_value = texture2D(metallic_texture, uv).r * metallic;
            float roughness_value = texture2D(roughness_texture, uv).r * roughness;
            float ao = texture2D(ao_texture, uv).r;
            vec3 emission = texture2D(emission_texture, uv).rgb * emission_strength;

            vec3 V = normalize(camera_position - world_position);

            vec3 F0 = vec3(0.04);
            F0 = mix(F0, albedo, metallic_value);

            vec3 Lo = vec3(0.0);

            // Calculate lighting contribution from each light
            for(int i = 0; i < 8; i++) {
                if(i >= num_lights) break;

                vec3 L = normalize(light_positions[i] - world_position);
                vec3 H = normalize(V + L);
                float distance = length(light_positions[i] - world_position);
                float attenuation = 1.0 / (distance * distance);
                vec3 radiance = light_colors[i] * light_intensities[i] * attenuation;

                float NDF = DistributionGGX(normal, H, roughness_value);
                float G = GeometrySmith(normal, V, L, roughness_value);
                vec3 F = fresnelSchlick(max(dot(H, V), 0.0), F0);

                vec3 kS = F;
                vec3 kD = vec3(1.0) - kS;
                kD *= 1.0 - metallic_value;

                vec3 numerator = NDF * G * F;
                float denominator = 4.0 * max(dot(normal, V), 0.0) * max(dot(normal, L), 0.0);
                vec3 specular = numerator / max(denominator, 0.001);

                float NdotL = max(dot(normal, L), 0.0);
                Lo += (kD * albedo / PI + specular) * radiance * NdotL;
            }

            vec3 ambient = ambient_color * albedo * ao;
            vec3 color = ambient + Lo + emission;

            // Tone mapping
            color = color / (color + vec3(1.0));
            // Gamma correction
            color = pow(color, vec2(1.0/2.2));

            gl_FragColor = vec4(color, 1.0);
        }
        "#.to_string()
    }

    // Lighting shader with normal mapping
    fn lighting_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec3 normal;

        uniform mat4 Model;
        uniform mat4 Projection;

        varying vec2 uv;
        varying vec3 frag_position;
        varying vec3 frag_normal;

        void main() {
            vec4 world_pos = Model * vec4(position, 1.0);
            frag_position = world_pos.xyz;
            frag_normal = mat3(Model) * normal;
            uv = texcoord;
            gl_Position = Projection * world_pos;
        }
        "#.to_string()
    }

    fn lighting_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec2 uv;
        varying vec3 frag_position;
        varying vec3 frag_normal;

        uniform sampler2D diffuse_texture;
        uniform sampler2D normal_texture;
        uniform sampler2D specular_texture;

        uniform vec3 light_position;
        uniform vec3 light_color;
        uniform float light_intensity;
        uniform vec3 ambient_color;
        uniform vec3 view_position;

        uniform float specular_power;
        uniform float specular_intensity;

        void main() {
            vec3 color = texture2D(diffuse_texture, uv).rgb;
            vec3 normal = normalize(frag_normal);

            // Apply normal map if available
            vec3 normal_map = texture2D(normal_texture, uv).rgb;
            if(length(normal_map) > 0.0) {
                normal = normalize(normal_map * 2.0 - 1.0);
            }

            // Ambient
            vec3 ambient = ambient_color * color;

            // Diffuse
            vec3 light_dir = normalize(light_position - frag_position);
            float diff = max(dot(normal, light_dir), 0.0);
            vec3 diffuse = diff * light_color * color * light_intensity;

            // Specular
            vec3 view_dir = normalize(view_position - frag_position);
            vec3 reflect_dir = reflect(-light_dir, normal);
            float spec = pow(max(dot(view_dir, reflect_dir), 0.0), specular_power);
            vec3 specular_map = texture2D(specular_texture, uv).rgb;
            vec3 specular = spec * light_color * specular_map * specular_intensity;

            vec3 result = ambient + diffuse + specular;
            gl_FragColor = vec4(result, 1.0);
        }
        "#.to_string()
    }

    // Bloom effect shader
    fn bloom_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;

        varying vec2 uv;

        void main() {
            uv = texcoord;
            gl_Position = vec4(position, 1.0);
        }
        "#.to_string()
    }

    fn bloom_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec2 uv;

        uniform sampler2D scene_texture;
        uniform sampler2D bloom_texture;
        uniform float bloom_intensity;
        uniform float threshold;
        uniform int blur_passes;

        vec3 sampleBox(sampler2D tex, vec2 uv, float delta) {
            vec3 color = vec3(0.0);
            vec2 texelSize = vec2(1.0) / vec2(textureSize(tex, 0));

            for(int x = -1; x <= 1; x++) {
                for(int y = -1; y <= 1; y++) {
                    vec2 offset = vec2(float(x), float(y)) * texelSize * delta;
                    color += texture2D(tex, uv + offset).rgb;
                }
            }

            return color / 9.0;
        }

        void main() {
            vec3 scene = texture2D(scene_texture, uv).rgb;
            vec3 bloom = vec3(0.0);

            // Extract bright areas
            vec3 bright = max(scene - vec3(threshold), vec3(0.0));

            // Multi-pass blur for bloom
            for(int i = 0; i < 8; i++) {
                if(i >= blur_passes) break;
                bloom += sampleBox(bloom_texture, uv, float(i + 1));
            }

            bloom *= bloom_intensity / float(max(blur_passes, 1));

            vec3 result = scene + bloom;

            // Tone mapping
            result = result / (result + vec3(1.0));

            gl_FragColor = vec4(result, 1.0);
        }
        "#.to_string()
    }

    // Shadow mapping shader
    fn shadow_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;

        uniform mat4 Model;
        uniform mat4 LightSpaceMatrix;

        varying vec4 frag_pos_light_space;
        varying vec2 uv;

        void main() {
            vec4 world_pos = Model * vec4(position, 1.0);
            frag_pos_light_space = LightSpaceMatrix * world_pos;
            uv = texcoord;
            gl_Position = frag_pos_light_space;
        }
        "#.to_string()
    }

    fn shadow_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec4 frag_pos_light_space;
        varying vec2 uv;

        uniform sampler2D shadow_map;
        uniform sampler2D diffuse_texture;
        uniform float shadow_bias;
        uniform float shadow_strength;

        float calculateShadow() {
            vec3 proj_coords = frag_pos_light_space.xyz / frag_pos_light_space.w;
            proj_coords = proj_coords * 0.5 + 0.5;

            if(proj_coords.z > 1.0) return 0.0;

            float closest_depth = texture2D(shadow_map, proj_coords.xy).r;
            float current_depth = proj_coords.z;

            // PCF (Percentage Closer Filtering)
            float shadow = 0.0;
            vec2 texel_size = vec2(1.0) / vec2(textureSize(shadow_map, 0));
            for(int x = -1; x <= 1; x++) {
                for(int y = -1; y <= 1; y++) {
                    float pcf_depth = texture2D(
                        shadow_map,
                        proj_coords.xy + vec2(float(x), float(y)) * texel_size
                    ).r;
                    shadow += current_depth - shadow_bias > pcf_depth ? 1.0 : 0.0;
                }
            }
            shadow /= 9.0;

            return shadow * shadow_strength;
        }

        void main() {
            vec3 color = texture2D(diffuse_texture, uv).rgb;
            float shadow = calculateShadow();
            vec3 result = color * (1.0 - shadow);
            gl_FragColor = vec4(result, 1.0);
        }
        "#.to_string()
    }

    // Water shader with refraction and reflection
    fn water_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;

        uniform mat4 Model;
        uniform mat4 Projection;
        uniform float time;

        varying vec2 uv;
        varying vec3 world_pos;
        varying float wave_height;

        void main() {
            uv = texcoord;

            // Animated waves
            float wave1 = sin(position.x * 2.0 + time * 2.0) * 0.1;
            float wave2 = sin(position.z * 3.0 - time * 1.5) * 0.05;
            wave_height = wave1 + wave2;

            vec3 animated_pos = position + vec3(0.0, wave_height, 0.0);
            vec4 world = Model * vec4(animated_pos, 1.0);
            world_pos = world.xyz;

            gl_Position = Projection * world;
        }
        "#.to_string()
    }

    fn water_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec2 uv;
        varying vec3 world_pos;
        varying float wave_height;

        uniform sampler2D reflection_texture;
        uniform sampler2D refraction_texture;
        uniform sampler2D normal_map;
        uniform sampler2D depth_texture;

        uniform float time;
        uniform vec3 water_color;
        uniform vec3 view_position;
        uniform float water_clarity;
        uniform float wave_strength;

        void main() {
            // Animated normal for water surface
            vec2 normal_uv1 = uv * 4.0 + vec2(time * 0.02, time * 0.01);
            vec2 normal_uv2 = uv * 8.0 - vec2(time * 0.01, time * 0.02);

            vec3 normal1 = texture2D(normal_map, normal_uv1).rgb * 2.0 - 1.0;
            vec3 normal2 = texture2D(normal_map, normal_uv2).rgb * 2.0 - 1.0;
            vec3 normal = normalize(normal1 + normal2);

            // Refraction
            vec2 refract_coords = gl_FragCoord.xy / vec2(800.0, 600.0);
            refract_coords += normal.xy * wave_strength * 0.1;
            vec3 refraction = texture2D(refraction_texture, refract_coords).rgb;

            // Reflection
            vec2 reflect_coords = vec2(uv.x, 1.0 - uv.y);
            reflect_coords += normal.xy * wave_strength * 0.05;
            vec3 reflection = texture2D(reflection_texture, reflect_coords).rgb;

            // Fresnel effect
            vec3 view_dir = normalize(view_position - world_pos);
            float fresnel = pow(1.0 - max(dot(normal, view_dir), 0.0), 2.0);

            // Water depth for opacity
            float depth = texture2D(depth_texture, uv).r;
            float water_depth = smoothstep(0.0, 1.0, depth * water_clarity);

            // Combine
            vec3 color = mix(refraction, reflection, fresnel);
            color = mix(color, water_color, 0.3 * (1.0 - water_depth));

            // Add foam on wave peaks
            float foam = smoothstep(0.15, 0.2, abs(wave_height));
            color = mix(color, vec3(1.0), foam * 0.5);

            gl_FragColor = vec4(color, 0.8 + water_depth * 0.2);
        }
        "#.to_string()
    }

    // Particle shader with soft blending
    fn particle_vertex_shader() -> String {
        r#"
        #version 100
        precision highp float;

        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec4 color;

        uniform mat4 Model;
        uniform mat4 Projection;

        varying vec2 uv;
        varying vec4 particle_color;

        void main() {
            uv = texcoord;
            particle_color = color;
            gl_Position = Projection * Model * vec4(position, 1.0);
        }
        "#.to_string()
    }

    fn particle_fragment_shader() -> String {
        r#"
        #version 100
        precision highp float;

        varying vec2 uv;
        varying vec4 particle_color;

        uniform sampler2D particle_texture;
        uniform sampler2D scene_depth;
        uniform float soft_particles_distance;

        void main() {
            vec4 tex_color = texture2D(particle_texture, uv);

            // Soft particle blending with scene depth
            float scene_z = texture2D(scene_depth, gl_FragCoord.xy / vec2(800.0, 600.0)).r;
            float particle_z = gl_FragCoord.z;
            float fade = smoothstep(0.0, soft_particles_distance, scene_z - particle_z);

            vec4 color = tex_color * particle_color;
            color.a *= fade;

            // Additive blending for glow effect
            gl_FragColor = vec4(color.rgb * color.a, color.a);
        }
        "#.to_string()
    }
}

/// Material properties for realistic rendering
#[derive(Clone)]
pub struct MaterialProperties {
    pub albedo: Color,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,
    pub emission: Color,
    pub emission_strength: f32,
    pub normal_strength: f32,
    pub specular_intensity: f32,

    // Textures
    pub albedo_texture: Option<Texture2D>,
    pub normal_texture: Option<Texture2D>,
    pub metallic_texture: Option<Texture2D>,
    pub roughness_texture: Option<Texture2D>,
    pub ao_texture: Option<Texture2D>,
    pub emission_texture: Option<Texture2D>,
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            albedo: WHITE,
            metallic: 0.0,
            roughness: 0.5,
            ao: 1.0,
            emission: BLACK,
            emission_strength: 0.0,
            normal_strength: 1.0,
            specular_intensity: 1.0,
            albedo_texture: None,
            normal_texture: None,
            metallic_texture: None,
            roughness_texture: None,
            ao_texture: None,
            emission_texture: None,
        }
    }
}

impl MaterialProperties {
    pub fn metal() -> Self {
        Self {
            metallic: 1.0,
            roughness: 0.1,
            ..Default::default()
        }
    }

    pub fn plastic() -> Self {
        Self {
            metallic: 0.0,
            roughness: 0.3,
            ..Default::default()
        }
    }

    pub fn wood() -> Self {
        Self {
            albedo: Color::new(0.4, 0.2, 0.1, 1.0),
            metallic: 0.0,
            roughness: 0.8,
            ..Default::default()
        }
    }

    pub fn glass() -> Self {
        Self {
            albedo: Color::new(0.9, 0.9, 0.95, 0.2),
            metallic: 0.0,
            roughness: 0.0,
            ..Default::default()
        }
    }

    pub fn emissive(color: Color, strength: f32) -> Self {
        Self {
            emission: color,
            emission_strength: strength,
            ..Default::default()
        }
    }
}