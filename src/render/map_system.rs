use macroquad::prelude::*;

/// Enhanced map system with parallax, animated backgrounds, and weather
pub struct MapSystem {
    pub current_map: MapType,
    pub parallax_layers: Vec<ParallaxLayer>,
    pub animated_objects: Vec<AnimatedObject>,
    pub weather_system: WeatherSystem,
    pub lighting: LightingSystem,
    pub hazards: Vec<MapHazard>,
    pub interactive_objects: Vec<InteractiveObject>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapType {
    Classroom,
    Hallway,
    Cafeteria,
    Gym,
    Library,
    Rooftop,
    PrincipalOffice,
    JanitorCloset,
    Kitchen,
}

/// Parallax scrolling layer
pub struct ParallaxLayer {
    pub texture_name: String,
    pub position: Vec2,
    pub scroll_speed: f32,
    pub depth: f32,
    pub tint: Color,
    pub repeat_x: bool,
    pub repeat_y: bool,
}

/// Animated background object
pub struct AnimatedObject {
    pub object_type: BackgroundObjectType,
    pub position: Vec2,
    pub animation_frame: u32,
    pub animation_speed: f32,
    pub animation_timer: f32,
    pub max_frames: u32,
    pub size: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub enum BackgroundObjectType {
    SwayingLight,
    MovingStudent,
    FallingLeaves,
    FloatingDust,
    FlickeringScreen,
    BubblingBeaker,
    SteamVent,
    FlappingPoster,
    SpinningFan,
    DripWater,
}

/// Weather system for outdoor maps
pub struct WeatherSystem {
    pub weather_type: WeatherType,
    pub intensity: f32,
    pub particles: Vec<WeatherParticle>,
    pub wind_direction: Vec2,
    pub wind_speed: f32,
    pub ambient_light_tint: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Fog,
    Storm,
    Sunset,
}

pub struct WeatherParticle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
    pub alpha: f32,
    pub particle_type: WeatherParticleType,
}

#[derive(Clone, Copy, Debug)]
pub enum WeatherParticleType {
    RainDrop,
    Snowflake,
    FogWisp,
    Leaf,
    Dust,
}

/// Dynamic lighting system
pub struct LightingSystem {
    pub ambient_color: Color,
    pub dynamic_lights: Vec<DynamicLight>,
    pub shadows_enabled: bool,
    pub time_of_day: f32, // 0.0 = midnight, 0.5 = noon, 1.0 = midnight
}

pub struct DynamicLight {
    pub position: Vec2,
    pub color: Color,
    pub radius: f32,
    pub intensity: f32,
    pub flicker: bool,
    pub flicker_speed: f32,
    pub flicker_amount: f32,
}

/// Map hazard (affects gameplay)
pub struct MapHazard {
    pub hazard_type: HazardType,
    pub position: Vec2,
    pub size: Vec2,
    pub damage_per_second: f32,
    pub active: bool,
    pub duration: f32,
    pub lifetime: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum HazardType {
    Fire,
    Water,
    Electric,
    Poison,
    Spikes,
    SlipperyFloor,
    BrokenGlass,
}

/// Interactive background object
pub struct InteractiveObject {
    pub object_type: InteractiveType,
    pub position: Vec2,
    pub size: Vec2,
    pub health: f32,
    pub broken: bool,
    pub can_throw: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum InteractiveType {
    Desk,
    Chair,
    Locker,
    WindowGlass,
    PaintBucket,
    Bookshelf,
    VendingMachine,
    TrashCan,
    Whiteboard,
}

impl MapSystem {
    pub fn new(map_type: MapType) -> Self {
        let mut system = Self {
            current_map: map_type,
            parallax_layers: Vec::new(),
            animated_objects: Vec::new(),
            weather_system: WeatherSystem::new(WeatherType::Clear),
            lighting: LightingSystem::new(),
            hazards: Vec::new(),
            interactive_objects: Vec::new(),
        };

        system.initialize_map(map_type);
        system
    }

    fn initialize_map(&mut self, map_type: MapType) {
        match map_type {
            MapType::Classroom => self.setup_classroom(),
            MapType::Hallway => self.setup_hallway(),
            MapType::Cafeteria => self.setup_cafeteria(),
            MapType::Gym => self.setup_gym(),
            MapType::Library => self.setup_library(),
            MapType::Rooftop => self.setup_rooftop(),
            MapType::PrincipalOffice => self.setup_principal_office(),
            MapType::JanitorCloset => self.setup_janitor_closet(),
            MapType::Kitchen => self.setup_kitchen(),
        }
    }

    fn setup_classroom(&mut self) {
        // Parallax layers
        self.parallax_layers.push(ParallaxLayer {
            texture_name: "classroom_back_wall".to_string(),
            position: Vec2::ZERO,
            scroll_speed: 0.1,
            depth: 3.0,
            tint: WHITE,
            repeat_x: true,
            repeat_y: false,
        });

        self.parallax_layers.push(ParallaxLayer {
            texture_name: "classroom_desks".to_string(),
            position: Vec2::ZERO,
            scroll_speed: 0.5,
            depth: 2.0,
            tint: WHITE,
            repeat_x: true,
            repeat_y: false,
        });

        // Animated objects
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::SwayingLight,
            position: Vec2::new(300.0, 100.0),
            animation_frame: 0,
            animation_speed: 2.0,
            animation_timer: 0.0,
            max_frames: 8,
            size: Vec2::new(40.0, 80.0),
        });

        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::FlappingPoster,
            position: Vec2::new(800.0, 200.0),
            animation_frame: 0,
            animation_speed: 3.0,
            animation_timer: 0.0,
            max_frames: 4,
            size: Vec2::new(60.0, 80.0),
        });

        // Interactive objects
        for i in 0..5 {
            self.interactive_objects.push(InteractiveObject {
                object_type: InteractiveType::Desk,
                position: Vec2::new(200.0 + i as f32 * 150.0, 400.0),
                size: Vec2::new(80.0, 60.0),
                health: 50.0,
                broken: false,
                can_throw: true,
            });
        }

        // Lighting
        self.lighting.ambient_color = Color::new(0.9, 0.9, 0.85, 1.0);
        self.lighting.time_of_day = 0.4; // Morning

        // Add window lights
        self.lighting.dynamic_lights.push(DynamicLight {
            position: Vec2::new(400.0, 200.0),
            color: Color::new(1.0, 1.0, 0.8, 1.0),
            radius: 300.0,
            intensity: 0.8,
            flicker: false,
            flicker_speed: 0.0,
            flicker_amount: 0.0,
        });
    }

    fn setup_rooftop(&mut self) {
        // Sky parallax
        self.parallax_layers.push(ParallaxLayer {
            texture_name: "sky".to_string(),
            position: Vec2::ZERO,
            scroll_speed: 0.05,
            depth: 5.0,
            tint: Color::new(0.6, 0.7, 1.0, 1.0),
            repeat_x: true,
            repeat_y: false,
        });

        // Distant buildings
        self.parallax_layers.push(ParallaxLayer {
            texture_name: "city_background".to_string(),
            position: Vec2::ZERO,
            scroll_speed: 0.15,
            depth: 4.0,
            tint: Color::new(0.7, 0.7, 0.8, 1.0),
            repeat_x: true,
            repeat_y: false,
        });

        // Weather
        self.weather_system = WeatherSystem::new(WeatherType::Rain);
        self.weather_system.intensity = 0.6;
        self.weather_system.wind_direction = Vec2::new(0.3, 1.0);
        self.weather_system.wind_speed = 100.0;

        // Animated objects
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::FlappingPoster,
            position: Vec2::new(600.0, 250.0),
            animation_frame: 0,
            animation_speed: 5.0,
            animation_timer: 0.0,
            max_frames: 6,
            size: Vec2::new(40.0, 60.0),
        });

        // Lighting
        self.lighting.ambient_color = Color::new(0.7, 0.75, 0.8, 1.0);
        self.lighting.time_of_day = 0.6; // Afternoon
    }

    fn setup_cafeteria(&mut self) {
        // Tables and chairs
        for i in 0..4 {
            for j in 0..2 {
                self.interactive_objects.push(InteractiveObject {
                    object_type: InteractiveType::Chair,
                    position: Vec2::new(250.0 + i as f32 * 200.0, 350.0 + j as f32 * 100.0),
                    size: Vec2::new(50.0, 50.0),
                    health: 30.0,
                    broken: false,
                    can_throw: true,
                });
            }
        }

        // Vending machines
        self.interactive_objects.push(InteractiveObject {
            object_type: InteractiveType::VendingMachine,
            position: Vec2::new(100.0, 300.0),
            size: Vec2::new(80.0, 120.0),
            health: 100.0,
            broken: false,
            can_throw: false,
        });

        // Animated steam from food
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::SteamVent,
            position: Vec2::new(500.0, 250.0),
            animation_frame: 0,
            animation_speed: 4.0,
            animation_timer: 0.0,
            max_frames: 10,
            size: Vec2::new(60.0, 80.0),
        });

        // Lighting
        self.lighting.ambient_color = Color::new(1.0, 0.95, 0.85, 1.0);

        // Overhead lights
        for i in 0..6 {
            self.lighting.dynamic_lights.push(DynamicLight {
                position: Vec2::new(200.0 + i as f32 * 150.0, 150.0),
                color: Color::new(1.0, 1.0, 0.9, 1.0),
                radius: 200.0,
                intensity: 0.9,
                flicker: true,
                flicker_speed: 15.0,
                flicker_amount: 0.1,
            });
        }
    }

    fn setup_gym(&mut self) {
        // Parallax layers
        self.parallax_layers.push(ParallaxLayer {
            texture_name: "gym_bleachers".to_string(),
            position: Vec2::ZERO,
            scroll_speed: 0.2,
            depth: 3.0,
            tint: WHITE,
            repeat_x: true,
            repeat_y: false,
        });

        // Animated objects
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::SpinningFan,
            position: Vec2::new(400.0, 100.0),
            animation_frame: 0,
            animation_speed: 8.0,
            animation_timer: 0.0,
            max_frames: 12,
            size: Vec2::new(80.0, 80.0),
        });

        // Lighting - bright gym lights
        self.lighting.ambient_color = Color::new(1.0, 1.0, 1.0, 1.0);

        for i in 0..5 {
            self.lighting.dynamic_lights.push(DynamicLight {
                position: Vec2::new(200.0 + i as f32 * 200.0, 120.0),
                color: WHITE,
                radius: 250.0,
                intensity: 1.0,
                flicker: false,
                flicker_speed: 0.0,
                flicker_amount: 0.0,
            });
        }
    }

    fn setup_library(&mut self) {
        // Bookshelves
        for i in 0..5 {
            self.interactive_objects.push(InteractiveObject {
                object_type: InteractiveType::Bookshelf,
                position: Vec2::new(150.0 + i as f32 * 180.0, 250.0),
                size: Vec2::new(100.0, 180.0),
                health: 80.0,
                broken: false,
                can_throw: false,
            });
        }

        // Floating dust particles
        for i in 0..20 {
            self.animated_objects.push(AnimatedObject {
                object_type: BackgroundObjectType::FloatingDust,
                position: Vec2::new(
                    rand::gen_range(0.0, 1920.0),
                    rand::gen_range(0.0, 1080.0),
                ),
                animation_frame: 0,
                animation_speed: 1.0,
                animation_timer: rand::gen_range(0.0, 5.0),
                max_frames: 1,
                size: Vec2::new(3.0, 3.0),
            });
        }

        // Dim lighting
        self.lighting.ambient_color = Color::new(0.7, 0.7, 0.75, 1.0);

        // Reading lamps
        for i in 0..4 {
            self.lighting.dynamic_lights.push(DynamicLight {
                position: Vec2::new(300.0 + i as f32 * 250.0, 350.0),
                color: Color::new(1.0, 0.9, 0.7, 1.0),
                radius: 180.0,
                intensity: 0.7,
                flicker: false,
                flicker_speed: 0.0,
                flicker_amount: 0.0,
            });
        }
    }

    fn setup_hallway(&mut self) {
        // Lockers
        for i in 0..10 {
            self.interactive_objects.push(InteractiveObject {
                object_type: InteractiveType::Locker,
                position: Vec2::new(100.0 + i as f32 * 80.0, 280.0),
                size: Vec2::new(60.0, 120.0),
                health: 60.0,
                broken: false,
                can_throw: false,
            });
        }

        // Flickering lights
        for i in 0..8 {
            self.lighting.dynamic_lights.push(DynamicLight {
                position: Vec2::new(150.0 + i as f32 * 140.0, 100.0),
                color: Color::new(1.0, 1.0, 0.95, 1.0),
                radius: 200.0,
                intensity: 0.85,
                flicker: true,
                flicker_speed: 20.0,
                flicker_amount: 0.15,
            });
        }

        // Animated posters
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::FlappingPoster,
            position: Vec2::new(500.0, 220.0),
            animation_frame: 0,
            animation_speed: 2.5,
            animation_timer: 0.0,
            max_frames: 4,
            size: Vec2::new(50.0, 70.0),
        });

        self.lighting.ambient_color = Color::new(0.85, 0.85, 0.9, 1.0);
    }

    fn setup_principal_office(&mut self) {
        // Desk
        self.interactive_objects.push(InteractiveObject {
            object_type: InteractiveType::Desk,
            position: Vec2::new(700.0, 400.0),
            size: Vec2::new(150.0, 100.0),
            health: 100.0,
            broken: false,
            can_throw: false,
        });

        // Bookshelves
        self.interactive_objects.push(InteractiveObject {
            object_type: InteractiveType::Bookshelf,
            position: Vec2::new(200.0, 250.0),
            size: Vec2::new(120.0, 200.0),
            health: 80.0,
            broken: false,
            can_throw: false,
        });

        // Warm lighting
        self.lighting.ambient_color = Color::new(0.95, 0.9, 0.85, 1.0);

        self.lighting.dynamic_lights.push(DynamicLight {
            position: Vec2::new(700.0, 300.0),
            color: Color::new(1.0, 0.9, 0.7, 1.0),
            radius: 300.0,
            intensity: 0.9,
            flicker: false,
            flicker_speed: 0.0,
            flicker_amount: 0.0,
        });
    }

    fn setup_janitor_closet(&mut self) {
        // Paint buckets
        for i in 0..3 {
            self.interactive_objects.push(InteractiveObject {
                object_type: InteractiveType::PaintBucket,
                position: Vec2::new(300.0 + i as f32 * 100.0, 450.0),
                size: Vec2::new(40.0, 50.0),
                health: 20.0,
                broken: false,
                can_throw: true,
            });
        }

        // Water drips
        self.animated_objects.push(AnimatedObject {
            object_type: BackgroundObjectType::DripWater,
            position: Vec2::new(600.0, 150.0),
            animation_frame: 0,
            animation_speed: 3.0,
            animation_timer: 0.0,
            max_frames: 6,
            size: Vec2::new(10.0, 20.0),
        });

        // Slippery floor hazard
        self.hazards.push(MapHazard {
            hazard_type: HazardType::SlipperyFloor,
            position: Vec2::new(400.0, 450.0),
            size: Vec2::new(300.0, 200.0),
            damage_per_second: 0.0,
            active: true,
            duration: -1.0, // Permanent
            lifetime: 0.0,
        });

        // Dim lighting
        self.lighting.ambient_color = Color::new(0.6, 0.65, 0.7, 1.0);

        self.lighting.dynamic_lights.push(DynamicLight {
            position: Vec2::new(500.0, 200.0),
            color: Color::new(0.8, 0.9, 1.0, 1.0),
            radius: 250.0,
            intensity: 0.6,
            flicker: true,
            flicker_speed: 10.0,
            flicker_amount: 0.2,
        });
    }

    fn setup_kitchen(&mut self) {
        // Fire hazards
        self.hazards.push(MapHazard {
            hazard_type: HazardType::Fire,
            position: Vec2::new(300.0, 400.0),
            size: Vec2::new(100.0, 80.0),
            damage_per_second: 15.0,
            active: true,
            duration: -1.0,
            lifetime: 0.0,
        });

        // Steam vents
        for i in 0..3 {
            self.animated_objects.push(AnimatedObject {
                object_type: BackgroundObjectType::SteamVent,
                position: Vec2::new(400.0 + i as f32 * 200.0, 250.0),
                animation_frame: 0,
                animation_speed: 5.0,
                animation_timer: i as f32 * 0.5,
                max_frames: 8,
                size: Vec2::new(50.0, 70.0),
            });
        }

        // Trash cans
        for i in 0..2 {
            self.interactive_objects.push(InteractiveObject {
                object_type: InteractiveType::TrashCan,
                position: Vec2::new(200.0 + i as f32 * 400.0, 450.0),
                size: Vec2::new(50.0, 60.0),
                health: 40.0,
                broken: false,
                can_throw: true,
            });
        }

        // Warm kitchen lighting
        self.lighting.ambient_color = Color::new(1.0, 0.9, 0.8, 1.0);

        // Overhead lights
        for i in 0..5 {
            self.lighting.dynamic_lights.push(DynamicLight {
                position: Vec2::new(250.0 + i as f32 * 180.0, 120.0),
                color: Color::new(1.0, 0.95, 0.85, 1.0),
                radius: 180.0,
                intensity: 0.9,
                flicker: false,
                flicker_speed: 0.0,
                flicker_amount: 0.0,
            });
        }

        // Fire light
        self.lighting.dynamic_lights.push(DynamicLight {
            position: Vec2::new(300.0, 400.0),
            color: Color::new(1.0, 0.5, 0.2, 1.0),
            radius: 150.0,
            intensity: 1.0,
            flicker: true,
            flicker_speed: 25.0,
            flicker_amount: 0.3,
        });
    }

    pub fn update(&mut self, dt: f32, camera_position: Vec2) {
        // Update parallax layers
        for layer in &mut self.parallax_layers {
            layer.position.x = camera_position.x * layer.scroll_speed;
        }

        // Update animated objects
        for obj in &mut self.animated_objects {
            obj.animation_timer += dt * obj.animation_speed;
            obj.animation_frame = (obj.animation_timer as u32) % obj.max_frames;
        }

        // Update weather
        self.weather_system.update(dt);

        // Update lighting
        self.lighting.update(dt);

        // Update hazards
        for hazard in &mut self.hazards {
            if hazard.duration > 0.0 {
                hazard.lifetime += dt;
                if hazard.lifetime >= hazard.duration {
                    hazard.active = false;
                }
            }
        }
    }

    pub fn add_hazard(&mut self, hazard: MapHazard) {
        self.hazards.push(hazard);
    }

    pub fn break_object(&mut self, position: Vec2) -> Option<Vec2> {
        for obj in &mut self.interactive_objects {
            if !obj.broken && position.distance(obj.position) < 50.0 {
                obj.health -= 50.0;
                if obj.health <= 0.0 {
                    obj.broken = true;
                    return Some(obj.position);
                }
            }
        }
        None
    }
}

impl WeatherSystem {
    pub fn new(weather_type: WeatherType) -> Self {
        let mut system = Self {
            weather_type,
            intensity: 0.5,
            particles: Vec::new(),
            wind_direction: Vec2::new(0.0, 1.0),
            wind_speed: 50.0,
            ambient_light_tint: WHITE,
        };

        system.set_weather(weather_type);
        system
    }

    pub fn set_weather(&mut self, weather_type: WeatherType) {
        self.weather_type = weather_type;
        self.particles.clear();

        match weather_type {
            WeatherType::Clear => {
                self.ambient_light_tint = WHITE;
            }
            WeatherType::Rain => {
                self.ambient_light_tint = Color::new(0.7, 0.7, 0.8, 1.0);
                self.spawn_weather_particles(100);
            }
            WeatherType::Snow => {
                self.ambient_light_tint = Color::new(0.9, 0.9, 1.0, 1.0);
                self.spawn_weather_particles(80);
            }
            WeatherType::Fog => {
                self.ambient_light_tint = Color::new(0.8, 0.8, 0.85, 1.0);
                self.spawn_weather_particles(50);
            }
            WeatherType::Storm => {
                self.ambient_light_tint = Color::new(0.5, 0.5, 0.6, 1.0);
                self.spawn_weather_particles(150);
            }
            WeatherType::Sunset => {
                self.ambient_light_tint = Color::new(1.0, 0.7, 0.5, 1.0);
            }
        }
    }

    fn spawn_weather_particles(&mut self, count: usize) {
        let particle_type = match self.weather_type {
            WeatherType::Rain | WeatherType::Storm => WeatherParticleType::RainDrop,
            WeatherType::Snow => WeatherParticleType::Snowflake,
            WeatherType::Fog => WeatherParticleType::FogWisp,
            _ => return,
        };

        for _ in 0..count {
            self.particles.push(WeatherParticle {
                position: Vec2::new(
                    rand::gen_range(0.0, 1920.0),
                    rand::gen_range(-100.0, 1080.0),
                ),
                velocity: Vec2::new(0.0, 200.0),
                size: rand::gen_range(2.0, 5.0),
                alpha: rand::gen_range(0.3, 0.8),
                particle_type,
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.position += (particle.velocity + self.wind_direction * self.wind_speed) * dt;

            // Wrap particles
            if particle.position.y > 1080.0 {
                particle.position.y = -50.0;
                particle.position.x = rand::gen_range(0.0, 1920.0);
            }
            if particle.position.x > 1920.0 {
                particle.position.x = 0.0;
            } else if particle.position.x < 0.0 {
                particle.position.x = 1920.0;
            }
        }
    }
}

impl LightingSystem {
    pub fn new() -> Self {
        Self {
            ambient_color: WHITE,
            dynamic_lights: Vec::new(),
            shadows_enabled: false,
            time_of_day: 0.5,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for light in &mut self.dynamic_lights {
            if light.flicker {
                let flicker = (get_time() as f32 * light.flicker_speed).sin() * light.flicker_amount;
                light.intensity = 1.0 - light.flicker_amount + flicker.abs();
            }
        }
    }
}
