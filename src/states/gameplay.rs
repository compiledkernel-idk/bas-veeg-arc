use std::collections::HashMap;

use crate::combat::hitbox::{Hitbox, SpecialType};
use crate::combat::hurtbox::Hurtbox;
use crate::combat::inputs::InputManager;
use crate::data::{AbilityState, CharacterId, ShopManager, UpgradeId};
use crate::ecs::System as EcsSystem;
use crate::states::StateType;
use crate::ecs::{
    AIBehavior, AIController, Bomb, BossPhase, CharacterType, EntityId, Fighter, FighterState, Health,
    HitboxComponent, HurtboxComponent, Particle, ParticleType, Team, Transform, Velocity, World,
};
use crate::ecs::{
    AISystem, AnimationSystem, CombatSystem, MovementSystem, ParticleSystem, PhysicsSystem,
};
use crate::states::State;
use macroquad::prelude::*;

pub struct GameplayState {
    world: World,
    player_entity: Option<EntityId>,
    ally_entities: Vec<EntityId>,
    ally_roster: Vec<CharacterType>,
    enemy_entities: Vec<EntityId>,
    camera_pos: Vec2,
    movement_system: MovementSystem,
    physics_system: PhysicsSystem,
    animation_system: AnimationSystem,
    combat_system: CombatSystem,
    particle_system: ParticleSystem,
    ai_system: AISystem,
    input_manager: InputManager,
    current_map: MapType,
    current_wave: usize,
    waves_completed: usize,
    enemies_to_spawn: usize,
    spawn_timer: f32,
    paused: bool,
    dialogue_queue: Vec<DialogueLine>,
    current_dialogue: Option<DialogueLine>,
    dialogue_timer: f32,
    show_controls: bool,
    control_fade: f32,
    previous_fighter_states: HashMap<EntityId, FighterState>,
    shop_manager: ShopManager,
    shop_open: bool,
    shop_feedback_timer: f32,
    shop_feedback_message: Option<String>,
    player_move_speed: f32,
    player_max_health: f32,
    player_attack_multiplier: f32,
    game_over: bool,
    selected_character: CharacterId,
    ability_state: AbilityState,
    ability_voice_line: Option<String>,
    ability_voice_timer: f32,
    burning_enemies: HashMap<EntityId, (f32, f32)>, // entity -> (remaining_time, dps)
    transition_to: Option<StateType>,
    bomb_entities: Vec<EntityId>,
    bomb_spawn_timer: f32,
    boss_battle_won: bool,
    dialogue_choice_active: bool,
    dialogue_choice_selected: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapType {
    Classroom,
    Hallway,
    Cafeteria,
    Gym,
    Library,
    Rooftop,
}

struct DialogueLine {
    speaker: String,
    dutch: String,
    english: String,
    duration: f32,
}

#[derive(Clone, Copy)]
struct ShopOption {
    id: UpgradeId,
    title: &'static str,
    description: &'static str,
    cost: u32,
}

const SHOP_OPTIONS: [ShopOption; 3] = [
    ShopOption {
        id: UpgradeId::AttackBoost,
        title: "Painted Fury",
        description: "Boost your attacks for bigger damage bursts.",
        cost: 150,
    },
    ShopOption {
        id: UpgradeId::HealthBoost,
        title: "Protective Apron",
        description: "Raise your max health to stay in the fight longer.",
        cost: 120,
    },
    ShopOption {
        id: UpgradeId::SpeedBoost,
        title: "Turbo Mop Shoes",
        description: "Move faster to outmaneuver classmates.",
        cost: 100,
    },
];

impl GameplayState {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            player_entity: None,
            ally_entities: Vec::new(),
            ally_roster: vec![
                CharacterType::Luca,
                CharacterType::Hadi,
                CharacterType::Berkay,
                CharacterType::Nitin,
            ],
            enemy_entities: Vec::new(),
            camera_pos: Vec2::ZERO,
            movement_system: MovementSystem,
            physics_system: PhysicsSystem::new(),
            animation_system: AnimationSystem,
            combat_system: CombatSystem::new(),
            particle_system: ParticleSystem,
            ai_system: AISystem,
            input_manager: InputManager::new(),
            current_map: MapType::Classroom,
            current_wave: 0,
            waves_completed: 0,
            enemies_to_spawn: 0,
            spawn_timer: 0.0,
            paused: false,
            dialogue_queue: Vec::new(),
            current_dialogue: None,
            dialogue_timer: 0.0,
            show_controls: true,
            control_fade: 1.0,
            previous_fighter_states: HashMap::new(),
            shop_manager: ShopManager::load(),
            shop_open: false,
            shop_feedback_timer: 0.0,
            shop_feedback_message: None,
            player_move_speed: 260.0,
            player_max_health: 100.0,
            player_attack_multiplier: 1.0,
            game_over: false,
            selected_character: crate::data::get_selected_character(),
            ability_state: AbilityState::new(crate::data::get_selected_character()),
            ability_voice_line: None,
            ability_voice_timer: 0.0,
            burning_enemies: HashMap::new(),
            transition_to: None,
            bomb_entities: Vec::new(),
            bomb_spawn_timer: 0.0,
            boss_battle_won: false,
            dialogue_choice_active: false,
            dialogue_choice_selected: 0,
        }
    }

    fn spawn_player(&mut self) {
        let entity = self.world.create_entity();

        self.world.add_component(
            entity,
            Transform {
                position: Vec2::new(200.0, 500.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            },
        );

        self.world.add_component(
            entity,
            Velocity {
                linear: Vec2::ZERO,
                angular: 0.0,
            },
        );

        self.world.add_component(
            entity,
            Health {
                current: self.player_max_health,
                maximum: self.player_max_health,
                armor: 0.0,
            },
        );

        self.world.add_component(
            entity,
            HurtboxComponent {
                hurtbox: Hurtbox::new_standing(),
                active: true,
            },
        );

        self.world.add_component(
            entity,
            HitboxComponent {
                hitbox: Hitbox::new_light(),
                active: false,
                hits_registered: Vec::new(),
            },
        );

        self.world.add_component(
            entity,
            Fighter {
                character_type: CharacterType::Bas,
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: 1.0,
                attack_timer: 0.0,
                team: Team::Player,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        self.player_entity = Some(entity);
    }

    fn spawn_big_boss(&mut self) {
        // Spawn GIANT Bastiaan boss - 2x size, 500 HP, 30 damage
        let boss_entity = self.world.create_entity();

        self.world.add_component(
            boss_entity,
            Transform {
                position: Vec2::new(900.0, 500.0),
                rotation: 0.0,
                scale: Vec2::new(2.0, 2.0), // GIANT size!
            },
        );

        self.world.add_component(boss_entity, Velocity { linear: Vec2::ZERO, angular: 0.0 });

        self.world.add_component(
            boss_entity,
            Health {
                current: 500.0,
                maximum: 500.0,
                armor: 0.0,
            },
        );

        self.world.add_component(boss_entity, HurtboxComponent { hurtbox: Hurtbox::new_standing(), active: true });
        self.world.add_component(boss_entity, HitboxComponent { hitbox: Hitbox::new_light(), active: false, hits_registered: Vec::new() });

        self.world.add_component(
            boss_entity,
            Fighter {
                character_type: CharacterType::Bastiaan,
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: -1.0,
                attack_timer: 0.0,
                team: Team::Enemy,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        self.world.add_component(
            boss_entity,
            AIController {
                behavior: AIBehavior::Boss(BossPhase::Phase1),
                target_entity: None,
                state_timer: 0.0,
                reaction_delay: 0.15, // Very fast attacks
                difficulty: 1.0, // Maximum difficulty
            },
        );

        self.enemy_entities.push(boss_entity);

        // Spawn Keizer Bom Taha - bomb thrower in the sky
        let keizer_entity = self.world.create_entity();

        self.world.add_component(
            keizer_entity,
            Transform {
                position: Vec2::new(screen_width() * 0.7, 200.0), // High in the sky
                rotation: 0.0,
                scale: Vec2::new(1.5, 1.5), // Bigger than normal
            },
        );

        self.world.add_component(keizer_entity, Velocity { linear: Vec2::ZERO, angular: 0.0 });

        self.world.add_component(
            keizer_entity,
            Health {
                current: 250.0,
                maximum: 250.0,
                armor: 0.0,
            },
        );

        self.world.add_component(keizer_entity, HurtboxComponent { hurtbox: Hurtbox::new_standing(), active: true });
        self.world.add_component(keizer_entity, HitboxComponent { hitbox: Hitbox::new_light(), active: false, hits_registered: Vec::new() });

        self.world.add_component(
            keizer_entity,
            Fighter {
                character_type: CharacterType::KeizerBomTaha,
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: -1.0,
                attack_timer: 0.0,
                team: Team::Enemy,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        self.world.add_component(
            keizer_entity,
            AIController {
                behavior: AIBehavior::Support,
                target_entity: None,
                state_timer: 0.0,
                reaction_delay: 1.5, // Slow bomb attacks
                difficulty: 0.8,
            },
        );

        self.enemy_entities.push(keizer_entity);

        // Show boss intro dialogue
        self.show_dialogue("Bastiaan", "IK BEN DE EINDBAAS!", "I AM THE FINAL BOSS!");
        self.show_dialogue("Keizer Bom Taha", "Ik gooi bommen!", "I throw bombs!");
    }

    fn spawn_bomb(&mut self, pos: Vec2) {
        let bomb_entity = self.world.create_entity();

        self.world.add_component(
            bomb_entity,
            Transform {
                position: pos,
                rotation: 0.0,
                scale: Vec2::new(1.0, 1.0),
            },
        );

        self.world.add_component(
            bomb_entity,
            Velocity {
                linear: Vec2::new(0.0, 0.0), // Will fall due to gravity
                angular: 0.0,
            },
        );

        self.world.add_component(
            bomb_entity,
            Bomb {
                damage: 15.0,
                owner_id: 0, // Keizer's bomb
            },
        );

        self.bomb_entities.push(bomb_entity);
    }

    fn spawn_enemy(&mut self, pos: Vec2, character: CharacterType) {
        let entity = self.world.create_entity();

        self.world.add_component(
            entity,
            Transform {
                position: pos,
                rotation: 0.0,
                scale: Vec2::ONE,
            },
        );

        self.world.add_component(
            entity,
            Velocity {
                linear: Vec2::ZERO,
                angular: 0.0,
            },
        );

        self.world.add_component(
            entity,
            Health {
                current: 100.0,
                maximum: 100.0,
                armor: 0.0,
            },
        );

        self.world.add_component(
            entity,
            HurtboxComponent {
                hurtbox: Hurtbox::new_standing(),
                active: true,
            },
        );

        self.world.add_component(
            entity,
            HitboxComponent {
                hitbox: Hitbox::new_light(),
                active: false,
                hits_registered: Vec::new(),
            },
        );

        self.world.add_component(
            entity,
            Fighter {
                character_type: character.clone(),
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: -1.0,
                attack_timer: 0.0,
                team: Team::Enemy,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        let (behavior, difficulty) = match character {
            CharacterType::Wolters => (AIBehavior::Aggressive, 0.4),
            CharacterType::PrefectA | CharacterType::PrefectB => (AIBehavior::Defensive, 0.35),
            CharacterType::Chef => (AIBehavior::Balanced, 0.55),
            CharacterType::Librarian => (AIBehavior::Balanced, 0.5),
            CharacterType::Coach => (AIBehavior::Aggressive, 0.65),
            CharacterType::Bastiaan => (AIBehavior::Boss(BossPhase::Phase1), 0.7),
            CharacterType::KeizerBomTaha => (AIBehavior::Support, 0.8),
            _ => (AIBehavior::Balanced, 0.4),
        };

        self.world.add_component(
            entity,
            AIController {
                behavior,
                target_entity: None, // Let AI system find nearest target (player or ally)
                state_timer: rand::gen_range(0.0, 0.2),
                reaction_delay: (0.35_f32 - difficulty * 0.15_f32).max(0.18_f32),
                difficulty,
            },
        );

        self.enemy_entities.push(entity);
    }

    fn spawn_ally(&mut self, pos: Vec2, character: CharacterType) {
        let entity = self.world.create_entity();

        self.world.add_component(
            entity,
            Transform {
                position: pos,
                rotation: 0.0,
                scale: Vec2::ONE,
            },
        );

        self.world.add_component(
            entity,
            Velocity {
                linear: Vec2::ZERO,
                angular: 0.0,
            },
        );

        self.world.add_component(
            entity,
            Health {
                current: 150.0,
                maximum: 150.0,
                armor: 0.0,
            },
        );

        self.world.add_component(
            entity,
            HurtboxComponent {
                hurtbox: Hurtbox::new_standing(),
                active: true,
            },
        );

        self.world.add_component(
            entity,
            HitboxComponent {
                hitbox: Hitbox::new_light(),
                active: false,
                hits_registered: Vec::new(),
            },
        );

        self.world.add_component(
            entity,
            Fighter {
                character_type: character,
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: 1.0,
                attack_timer: 0.0,
                team: Team::Ally,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        self.world.add_component(
            entity,
            AIController {
                behavior: AIBehavior::Support,
                target_entity: None,
                state_timer: rand::gen_range(0.0, 0.2),
                reaction_delay: 0.32,
                difficulty: 0.45,
            },
        );

        self.ally_entities.push(entity);
    }

    fn refresh_allies_for_wave(&mut self) {
        for entity in self.ally_entities.drain(..) {
            self.world.destroy_entity(entity);
        }

        let spawn_positions = [Vec2::new(160.0, 520.0), Vec2::new(260.0, 540.0)];
        let mut roster = self.ally_roster.clone();

        for pos in spawn_positions {
            if roster.is_empty() {
                break;
            }
            let idx = rand::gen_range(0, roster.len());
            let character = roster.remove(idx);
            self.spawn_ally(pos, character);
        }
    }
}

impl State for GameplayState {
    fn enter(&mut self) {
        self.apply_initial_upgrades();
        self.spawn_player();
        self.combat_system
            .set_player_attack_multiplier(self.player_attack_multiplier);

        self.dialogue_queue.push(DialogueLine {
            speaker: "Meneer Wolters".to_string(),
            dutch: "Bas, vegen!".to_string(),
            english: "Bas, sweep!".to_string(),
            duration: 2.0,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Meneer Wolters".to_string(),
            dutch: "Bas! Vegen, nu meteen!".to_string(),
            english: "Bas! Sweep right now!".to_string(),
            duration: 2.4,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Berkay".to_string(),
            dutch: "Nee bro, ze gaan zien.".to_string(),
            english: "No bro, they'll see.".to_string(),
            duration: 2.2,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Luca".to_string(),
            dutch: "Wacht maar… ik heb een winter arc plan.".to_string(),
            english: "Just wait... I've got a winter arc plan.".to_string(),
            duration: 2.5,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Nitin".to_string(),
            dutch: "Ik ga m’n barras in hun stoppen.".to_string(),
            english: "I'm putting my barras in them.".to_string(),
            duration: 2.5,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Hadi".to_string(),
            dutch: "Aina broeg… ze gaan zien.".to_string(),
            english: "Always, bro... they'll see.".to_string(),
            duration: 2.3,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Bas".to_string(),
            dutch: "Kom dan! Ik veeg niks, bro!".to_string(),
            english: "Come on then! I'm not sweeping anything, bro!".to_string(),
            duration: 2.4,
        });

        self.dialogue_queue.push(DialogueLine {
            speaker: "Bastiaan".to_string(),
            dutch: "Je hebt alles verpest, Bas! Mijn kunst was perfect!".to_string(),
            english: "You ruined everything, Bas! My artwork was perfect!".to_string(),
            duration: 2.6,
        });

        self.dialogue_queue.reverse();

        self.start_wave();
    }

    fn exit(&mut self) {
        let _ = self.shop_manager.save();
    }

    fn update(&mut self, dt: f32) {
        if self.shop_feedback_timer > 0.0 {
            self.shop_feedback_timer -= dt;
            if self.shop_feedback_timer <= 0.0 {
                self.shop_feedback_timer = 0.0;
                self.shop_feedback_message = None;
            }
        }

        if self.paused {
            return;
        }

        self.check_game_over();
        if self.game_over {
            return;
        }

        // Update ability state
        self.ability_state.update(dt);

        // Update ability voice line timer
        if self.ability_voice_timer > 0.0 {
            self.ability_voice_timer -= dt;
            if self.ability_voice_timer <= 0.0 {
                self.ability_voice_line = None;
            }
        }

        // Update combat system with ability damage multiplier
        let ability_damage_mult = self.ability_state.get_damage_multiplier();
        let total_damage_mult = self.player_attack_multiplier * ability_damage_mult;
        self.combat_system.set_player_attack_multiplier(total_damage_mult);

        // Update burning enemies
        let mut enemies_to_remove = Vec::new();
        for (entity, (remaining_time, dps)) in self.burning_enemies.iter_mut() {
            *remaining_time -= dt;

            // Apply fire damage
            if let Some(enemy_health) = self.world.get_component_mut::<Health>(*entity) {
                enemy_health.current = (enemy_health.current - *dps * dt).max(0.0);
            }

            // Mark for removal if time expired
            if *remaining_time <= 0.0 {
                enemies_to_remove.push(*entity);
            }
        }

        // Remove expired burning effects
        for entity in enemies_to_remove {
            self.burning_enemies.remove(&entity);
        }

        if self.current_dialogue.is_none() && !self.dialogue_queue.is_empty() {
            self.current_dialogue = self.dialogue_queue.pop();
            self.dialogue_timer = 0.0;
        }

        if let Some(ref dialogue) = self.current_dialogue {
            self.dialogue_timer += dt;
            if self.dialogue_timer >= dialogue.duration {
                self.current_dialogue = None;
            } else {
                self.shop_open = false;
                return;
            }
        }

        if self.show_controls {
            self.control_fade -= dt * 0.1;
            if self.control_fade <= 0.0 {
                self.show_controls = false;
            }
        }

        if self.shop_open {
            return;
        }

        // Handle dialogue choice after boss battle
        if self.dialogue_choice_active {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.dialogue_choice_selected > 0 {
                    self.dialogue_choice_selected -= 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                if self.dialogue_choice_selected < 2 {
                    self.dialogue_choice_selected += 1;
                }
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::J) {
                // Player made a choice, continue with game
                self.dialogue_choice_active = false;
                self.boss_battle_won = true;
                // Show response based on choice
                match self.dialogue_choice_selected {
                    0 => self.show_dialogue("Bas", "Laten we een feestje bouwen!", "Let's build a party!"),
                    1 => self.show_dialogue("Bas", "We moeten de school repareren.", "We must repair the school."),
                    2 => self.show_dialogue("Bas", "Tijd om naar huis te gaan.", "Time to go home."),
                    _ => {}
                }
            }
            return;
        }

        // Bomb spawning logic - check if Keizer is alive
        let keizer_alive = self.enemy_entities.iter().any(|&entity| {
            if let Some(fighter) = self.world.get_component::<Fighter>(entity) {
                fighter.character_type == CharacterType::KeizerBomTaha
            } else {
                false
            }
        });

        if keizer_alive && !self.boss_battle_won {
            self.bomb_spawn_timer -= dt;
            if self.bomb_spawn_timer <= 0.0 {
                // Find Keizer's position
                if let Some(&keizer_entity) = self.enemy_entities.iter().find(|&&entity| {
                    if let Some(fighter) = self.world.get_component::<Fighter>(entity) {
                        fighter.character_type == CharacterType::KeizerBomTaha
                    } else {
                        false
                    }
                }) {
                    if let Some(keizer_transform) = self.world.get_component::<Transform>(keizer_entity) {
                        self.spawn_bomb(keizer_transform.position);
                    }
                }
                self.bomb_spawn_timer = 2.5; // Spawn bomb every 2.5 seconds
            }
        }

        // Update bombs - make them fall and check collisions
        let mut bombs_to_remove = Vec::new();
        let mut explosion_positions = Vec::new();
        let mut entities_to_damage = Vec::new();

        for &bomb_entity in &self.bomb_entities {
            if let Some(velocity) = self.world.get_component_mut::<Velocity>(bomb_entity) {
                velocity.linear.y += 400.0 * dt; // Gravity
            }
        }

        for &bomb_entity in &self.bomb_entities {
            if let Some(transform) = self.world.get_component::<Transform>(bomb_entity) {
                let bomb_pos = transform.position;

                // Check if bomb hit the ground
                if bomb_pos.y > 600.0 {
                    bombs_to_remove.push(bomb_entity);
                    explosion_positions.push(bomb_pos);
                } else {
                    // Check collision with player
                    if let Some(player_entity) = self.player_entity {
                        if let Some(player_transform) = self.world.get_component::<Transform>(player_entity) {
                            let dist = (bomb_pos - player_transform.position).length();
                            if dist < 40.0 {
                                bombs_to_remove.push(bomb_entity);
                                explosion_positions.push(bomb_pos);
                                entities_to_damage.push(player_entity);
                            }
                        }
                    }

                    // Check collision with allies
                    for &ally_entity in &self.ally_entities {
                        if let Some(ally_transform) = self.world.get_component::<Transform>(ally_entity) {
                            let dist = (bomb_pos - ally_transform.position).length();
                            if dist < 40.0 && !bombs_to_remove.contains(&bomb_entity) {
                                bombs_to_remove.push(bomb_entity);
                                explosion_positions.push(bomb_pos);
                                entities_to_damage.push(ally_entity);
                            }
                        }
                    }
                }
            }
        }

        // Apply damage to hit entities
        for entity in entities_to_damage {
            if let Some(health) = self.world.get_component_mut::<Health>(entity) {
                health.current = (health.current - 15.0).max(0.0);
            }
        }

        // Create explosion particles
        for explosion_pos in explosion_positions {
            let particle_count = 20;
            for _ in 0..particle_count {
                let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
                let speed = rand::gen_range(50.0, 200.0);
                let particle_entity = self.world.create_entity();
                self.world.add_component(
                    particle_entity,
                    Transform {
                        position: explosion_pos,
                        rotation: 0.0,
                        scale: Vec2::ONE,
                    },
                );
                self.world.add_component(
                    particle_entity,
                    Velocity {
                        linear: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                        angular: 0.0,
                    },
                );
                self.world.add_component(
                    particle_entity,
                    Particle {
                        particle_type: ParticleType::Smoke,
                        lifetime: 0.0,
                        max_lifetime: 1.0,
                        velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                        acceleration: Vec2::new(0.0, 50.0),
                        color_start: Color::new(1.0, 0.5, 0.0, 1.0),
                        color_end: Color::new(0.3, 0.3, 0.3, 0.0),
                        size_start: 15.0,
                        size_end: 3.0,
                    },
                );
            }
        }

        // Remove exploded bombs
        for bomb_entity in bombs_to_remove {
            self.world.destroy_entity(bomb_entity);
            self.bomb_entities.retain(|&e| e != bomb_entity);
        }

        if self.enemies_to_spawn > 0 {
            self.spawn_timer -= dt;
            if self.spawn_timer <= 0.0 {
                self.spawn_wave_enemy();
                self.enemies_to_spawn -= 1;
                self.spawn_timer = 1.0;
            }
        }

        self.enemy_entities.retain(|&entity| {
            if let Some(health) = self.world.get_component::<Health>(entity) {
                if health.current > 0.0 {
                    true
                } else {
                    self.world.destroy_entity(entity);
                    false
                }
            } else {
                false
            }
        });

        // Remove dead allies
        self.ally_entities.retain(|&entity| {
            if let Some(health) = self.world.get_component::<Health>(entity) {
                if health.current > 0.0 {
                    true
                } else {
                    self.world.destroy_entity(entity);
                    false
                }
            } else {
                false
            }
        });

        // Check if both bosses are defeated on Rooftop
        if self.current_map == MapType::Rooftop
            && self.enemy_entities.is_empty()
            && !self.boss_battle_won
            && !self.dialogue_choice_active {
            // Both bosses are dead! Show dialogue choice
            self.dialogue_choice_active = true;
            self.dialogue_choice_selected = 0;
        }

        if self.enemy_entities.is_empty() && self.enemies_to_spawn == 0 && !self.dialogue_choice_active {
            self.complete_wave();
        }

        self.input_manager.update();

        self.movement_system.update(&mut self.world, dt);
        self.physics_system.update(&mut self.world, dt);
        self.animation_system.update(&mut self.world, dt);
        self.combat_system.update(&mut self.world, dt);
        self.particle_system.update(&mut self.world, dt);
        self.ai_system.update(&mut self.world, dt);
        self.process_fighter_states(dt);
        self.clamp_fighter_positions();

        self.check_game_over();
        self.camera_pos = Vec2::ZERO;
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        // Gradient backgrounds for each map
        match self.current_map {
            MapType::Classroom => {
                // Warm classroom gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.9 - t * 0.15, 0.8 - t * 0.1, 0.6 - t * 0.05, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_classroom();
            }
            MapType::Hallway => {
                // Cool hallway gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.85 - t * 0.2, 0.85 - t * 0.15, 0.9 - t * 0.1, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_hallway();
            }
            MapType::Cafeteria => {
                // Bright cafeteria gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.95 - t * 0.1, 0.9 - t * 0.15, 0.75 - t * 0.1, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_cafeteria();
            }
            MapType::Gym => {
                // Athletic gym gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.65 - t * 0.15, 0.75 - t * 0.2, 0.85 - t * 0.15, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_gym();
            }
            MapType::Library => {
                // Scholarly library gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.65 - t * 0.2, 0.55 - t * 0.15, 0.5 - t * 0.15, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_library();
            }
            MapType::Rooftop => {
                // Dramatic rooftop gradient
                for i in 0..40 {
                    let y = i as f32 * screen_height() / 40.0;
                    let height = screen_height() / 40.0;
                    let t = i as f32 / 40.0;
                    let color = Color::new(0.5 - t * 0.2, 0.7 - t * 0.3, 0.95 - t * 0.2, 1.0);
                    draw_rectangle(0.0, y, screen_width(), height, color);
                }
                self.render_rooftop();
            }
        }

        // Add atmospheric particles
        let time = get_time();
        for i in 0..15 {
            let x = ((time * 15.0 + i as f64 * 50.0).sin() * screen_width() as f64 * 0.5 + screen_width() as f64 * 0.5) as f32;
            let y = ((time * 20.0 + i as f64 * 30.0) % screen_height() as f64) as f32;
            let size = 2.0 + ((time * 2.0 + i as f64).sin() * 1.5) as f32;
            let alpha = 0.15 + ((time * 3.0 + i as f64).sin() * 0.1) as f32;
            draw_circle(x, y, size, Color::new(1.0, 1.0, 1.0, alpha));
        }

        let mut draw_order: Vec<_> = self
            .world
            .query::<Transform>()
            .map(|(entity, transform)| (entity, transform.position))
            .collect();

        draw_order.sort_by(|a, b| {
            a.1.y
                .partial_cmp(&b.1.y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (entity, _) in draw_order {
            if let Some(transform) = self.world.get_component::<Transform>(entity) {
                let pos = transform.position - self.camera_pos;

                if let Some(fighter) = self.world.get_component::<Fighter>(entity) {
                    // Enhanced shadow with gradient effect
                    let shadow_color = Color::new(0.0, 0.0, 0.0, 0.4);
                    draw_ellipse(pos.x, pos.y + 70.0, 48.0, 16.0, 0.0, shadow_color);
                    draw_ellipse(pos.x, pos.y + 70.0, 36.0, 12.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.2));

                    let is_player = self.player_entity.map(|id| id == entity).unwrap_or(false);
                    let is_ally = self.ally_entities.contains(&entity);

                    // Character outline glow
                    let glow_color = if is_player {
                        Color::new(1.0, 0.9, 0.0, 0.5)
                    } else if is_ally {
                        Color::new(0.3, 1.0, 0.5, 0.4)
                    } else {
                        Color::new(1.0, 0.3, 0.3, 0.4)
                    };

                    for offset in [3.0, 2.0, 1.0] {
                        draw_circle(pos.x, pos.y, 32.0 + offset, Color::new(glow_color.r, glow_color.g, glow_color.b, glow_color.a * 0.3));
                    }

                    // Draw ability aura for player
                    if is_player && self.ability_state.active {
                        let time = get_time();
                        let pulse = (time * 3.0).sin() * 0.3 + 0.7;
                        let aura_color = match self.selected_character {
                            CharacterId::Berkay => Color::new(1.0, 0.5, 0.0, 0.4 * pulse as f32),
                            CharacterId::Luca => Color::new(0.3, 0.6, 1.0, 0.4 * pulse as f32),
                            CharacterId::Gefferinho => Color::new(1.0, 0.2, 0.2, 0.4 * pulse as f32),
                            CharacterId::Bas => Color::new(0.0, 1.0, 0.5, 0.4 * pulse as f32),
                            CharacterId::Hadi => Color::new(1.0, 0.8, 0.0, 0.4 * pulse as f32),
                            CharacterId::Nitin => Color::new(1.0, 0.3, 0.0, 0.4 * pulse as f32),
                        };
                        // Draw multiple pulsing rings for aura effect
                        for i in 0..4 {
                            let radius = 55.0 + i as f32 * 18.0 + (time * 2.0 + i as f64).sin() as f32 * 5.0;
                            draw_circle_lines(pos.x, pos.y, radius, 4.0, aura_color);
                        }

                        // Add particle sparkles around player
                        for i in 0..8 {
                            let angle = time * 2.0 + i as f64 * std::f64::consts::PI * 0.25;
                            let sparkle_x = pos.x + (angle.cos() as f32) * 65.0;
                            let sparkle_y = pos.y + (angle.sin() as f32) * 65.0;
                            let sparkle_pulse = ((time * 5.0 + i as f64 * 0.3).sin() * 0.5 + 0.5) as f32;
                            draw_circle(sparkle_x, sparkle_y, 3.0 * sparkle_pulse, aura_color);
                        }
                    }

                    self.render_character(pos, fighter);

                    // Render fire effect for burning enemies
                    if self.burning_enemies.contains_key(&entity) {
                        let time = get_time();
                        for i in 0..3 {
                            let offset_y = -30.0 - i as f32 * 10.0 + (time * 5.0 + i as f64 * 0.5).sin() as f32 * 5.0;
                            let offset_x = ((time * 3.0 + i as f64 * 0.7).sin() * 10.0) as f32;
                            let size = 8.0 + i as f32 * 2.0;
                            let alpha = 0.7 - i as f32 * 0.2;
                            draw_circle(
                                pos.x + offset_x,
                                pos.y + offset_y,
                                size,
                                Color::new(1.0, 0.5, 0.0, alpha),
                            );
                            draw_circle(
                                pos.x + offset_x,
                                pos.y + offset_y,
                                size * 0.6,
                                Color::new(1.0, 0.8, 0.0, alpha),
                            );
                        }
                    }

                    let is_ally = self.ally_entities.contains(&entity);
                    let name = self.character_display_name(&fighter.character_type, is_player);

                    let tag_color = if is_player {
                        YELLOW
                    } else if is_ally {
                        Color::new(0.5, 1.0, 0.6, 1.0)
                    } else {
                        Color::new(1.0, 0.5, 0.5, 1.0)
                    };

                    self.render_nametag(pos, &name, tag_color);
                }

                if let Some(health) = self.world.get_component::<Health>(entity) {
                    let health_pct = health.current / health.maximum;

                    // Enhanced health bar with gradient and glow
                    let bar_width = 70.0;
                    let bar_height = 8.0;
                    let bar_x = pos.x - bar_width * 0.5;
                    let bar_y = pos.y - 85.0;

                    // Background shadow
                    draw_rectangle(bar_x + 2.0, bar_y + 2.0, bar_width, bar_height, Color::new(0.0, 0.0, 0.0, 0.5));

                    // Dark background
                    draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::new(0.15, 0.15, 0.15, 0.9));

                    // Health bar with gradient (green to yellow to red based on health)
                    let health_color = if health_pct > 0.6 {
                        Color::new(0.2 + health_pct * 0.3, 1.0, 0.3, 1.0)
                    } else if health_pct > 0.3 {
                        Color::new(1.0, 0.8, 0.2, 1.0)
                    } else {
                        Color::new(1.0, 0.3, 0.2, 1.0)
                    };

                    draw_rectangle(bar_x, bar_y, bar_width * health_pct, bar_height, health_color);

                    // Add glow effect on health bar
                    if health_pct > 0.0 {
                        draw_rectangle(bar_x, bar_y, bar_width * health_pct, bar_height * 0.4,
                            Color::new(health_color.r + 0.2, health_color.g + 0.2, health_color.b + 0.2, 0.6));
                    }

                    // Border
                    draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, Color::new(0.8, 0.8, 0.8, 0.9));

                    // Health text
                    let health_text = format!("{:.0}/{:.0}", health.current, health.maximum);
                    let text_size = 14.0;
                    let text_dims = measure_text(&health_text, None, text_size as u16, 1.0);
                    draw_text(&health_text, bar_x + bar_width * 0.5 - text_dims.width * 0.5, bar_y - 3.0, text_size, WHITE);
                }

                // Render bombs
                if let Some(_bomb) = self.world.get_component::<Bomb>(entity) {
                    let time = get_time() as f32;

                    // Shadow
                    draw_ellipse(pos.x, pos.y + 20.0, 18.0, 8.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.4));

                    // Main bomb body - black sphere
                    draw_circle(pos.x, pos.y, 18.0, Color::new(0.1, 0.1, 0.1, 1.0));
                    draw_circle(pos.x - 4.0, pos.y - 4.0, 15.0, Color::new(0.2, 0.2, 0.2, 1.0));

                    // Highlight to make it look 3D
                    draw_circle(pos.x - 6.0, pos.y - 6.0, 6.0, Color::new(0.4, 0.4, 0.4, 0.8));

                    // Fuse on top
                    draw_rectangle(pos.x - 2.0, pos.y - 18.0, 4.0, 10.0, Color::new(0.3, 0.2, 0.1, 1.0));

                    // Animated sparking fuse tip
                    let spark_size = ((time * 10.0).sin().abs() * 3.0 + 2.0) as f32;
                    draw_circle(pos.x, pos.y - 18.0, spark_size, Color::new(1.0, 0.3, 0.0, 1.0));
                    draw_circle(pos.x, pos.y - 18.0, spark_size * 0.6, Color::new(1.0, 0.8, 0.0, 1.0));

                    // Danger symbol
                    draw_text("!", pos.x - 3.0, pos.y + 5.0, 20.0, Color::new(1.0, 0.0, 0.0, 0.9));
                }
            }
        }

        self.render_hud();
        self.render_controls();
        self.render_dialogue();

        if self.shop_open {
            self.render_shop();
        }

        self.render_shop_feedback();

        // Render dialogue choice if active
        if self.dialogue_choice_active {
            self.render_dialogue_choice();
        }

        if self.game_over {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.7),
            );
            let title = "GAME OVER";
            let title_size = 72.0;
            let title_dims = measure_text(title, None, title_size as u16, 1.0);
            draw_text(
                title,
                screen_width() * 0.5 - title_dims.width * 0.5,
                screen_height() * 0.4,
                title_size,
                Color::new(1.0, 0.2, 0.2, 1.0),
            );

            let prompt = "Bas fainted! Press ESC or ENTER to return to menu.";
            let prompt_dims = measure_text(prompt, None, 28, 1.0);
            draw_text(
                prompt,
                screen_width() * 0.5 - prompt_dims.width * 0.5,
                screen_height() * 0.4 + 60.0,
                28.0,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
        } else if self.paused {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
            let text = "PAUSED";
            let size = 60.0;
            let dims = measure_text(text, None, size as u16, 1.0);
            draw_text(
                text,
                screen_width() * 0.5 - dims.width * 0.5,
                screen_height() * 0.5,
                size,
                WHITE,
            );
        }
    }

    fn handle_input(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
                self.transition_to = Some(StateType::Menu);
            }
            return;
        }

        if let Some(_) = self.current_dialogue {
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                self.current_dialogue = None;
                self.dialogue_timer = 0.0;
            } else {
                return;
            }
        }

        if is_key_pressed(KeyCode::B) {
            self.shop_open = !self.shop_open;
            if self.shop_open {
                self.set_shop_feedback("Shop opened — press 1-3 to buy upgrades");
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.paused = !self.paused;
        }

        if self.paused {
            return;
        }

        if self.shop_open {
            self.handle_shop_controls();
            return;
        }

        if let Some(player_entity) = self.player_entity {
            let mut move_input = 0.0;
            let mut move_depth = 0.0;
            let mut new_state = None;

            if let Some(fighter) = self.world.get_component::<Fighter>(player_entity) {
                if fighter.hitstun <= 0.0 && fighter.blockstun <= 0.0 {
                    if is_key_down(KeyCode::A) {
                        move_input -= 1.0;
                    }
                    if is_key_down(KeyCode::D) {
                        move_input += 1.0;
                    }
                    if is_key_down(KeyCode::W) {
                        move_depth -= 1.0;
                    }
                    if is_key_down(KeyCode::S) {
                        move_depth += 1.0;
                    }

                    if is_key_pressed(KeyCode::J) {
                        new_state = Some(FighterState::LightAttack);
                    }
                    if is_key_pressed(KeyCode::K) {
                        new_state = Some(FighterState::HeavyAttack);
                    }
                    if is_key_pressed(KeyCode::L) {
                        new_state = Some(FighterState::Special);
                    }

                    // Ability activation
                    if is_key_pressed(KeyCode::E) {
                        if self.ability_state.can_activate() {
                            let voice_line = self.ability_state.activate();
                            if !voice_line.is_empty() {
                                self.ability_voice_line = Some(voice_line.to_string());
                                self.ability_voice_timer = 2.0;

                                // Apply health boost if applicable
                                let health_boost = self.ability_state.get_health_boost();
                                if health_boost > 0.0 {
                                    if let Some(health) = self.world.get_component_mut::<Health>(player_entity) {
                                        health.current = (health.current + health_boost).min(health.maximum);
                                    }
                                }

                                // Trigger splash damage if applicable (Bas's ability)
                                if let Some((damage, radius)) = self.ability_state.get_splash_damage() {
                                    if let Some(player_transform) = self.world.get_component::<Transform>(player_entity) {
                                        let player_pos = player_transform.position;

                                        // Damage all nearby enemies
                                        for &enemy_entity in &self.enemy_entities.clone() {
                                            if let Some(enemy_transform) = self.world.get_component::<Transform>(enemy_entity) {
                                                let distance = player_pos.distance(enemy_transform.position);
                                                if distance <= radius {
                                                    if let Some(enemy_health) = self.world.get_component_mut::<Health>(enemy_entity) {
                                                        enemy_health.current = (enemy_health.current - damage).max(0.0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Trigger fire damage if applicable (Nitin's ability)
                                if let Some((dps, duration)) = self.ability_state.get_fire_damage() {
                                    // Set all enemies on fire
                                    for &enemy_entity in &self.enemy_entities {
                                        self.burning_enemies.insert(enemy_entity, (duration, dps));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(transform) = self.world.get_component_mut::<Transform>(player_entity) {
                let dt = get_frame_time();
                let speed_multiplier = self.ability_state.get_speed_multiplier();
                let effective_move_speed = self.player_move_speed * speed_multiplier;
                let depth_speed = effective_move_speed * 0.65;
                transform.position.x += move_input * effective_move_speed * dt;
                transform.position.y += move_depth * depth_speed * dt;

                let min_depth = 340.0;
                let max_depth = 660.0;
                transform.position.y = transform.position.y.clamp(min_depth, max_depth);
                transform.position.x = transform.position.x.clamp(60.0, screen_width() - 60.0);
            }

            if let Some(velocity) = self.world.get_component_mut::<Velocity>(player_entity) {
                velocity.linear = Vec2::ZERO;
            }

            if let Some(fighter) = self.world.get_component_mut::<Fighter>(player_entity) {
                if move_input.abs() > 0.01
                    && matches!(fighter.state, FighterState::Idle | FighterState::Walking)
                {
                    fighter.state = FighterState::Walking;
                } else if move_input.abs() <= 0.01 && fighter.state == FighterState::Walking {
                    fighter.state = FighterState::Idle;
                }

                if move_input > 0.1 {
                    fighter.facing = 1.0;
                } else if move_input < -0.1 {
                    fighter.facing = -1.0;
                }
            }

            if let Some(state) = new_state {
                if let Some(fighter) = self.world.get_component_mut::<Fighter>(player_entity) {
                    fighter.state = state;
                }
            }
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}

impl GameplayState {
    fn start_wave(&mut self) {
        self.current_wave += 1;
        self.refresh_allies_for_wave();

        // Special boss battle on rooftop - spawn immediately
        if self.current_map == MapType::Rooftop && self.current_wave == 1 {
            self.spawn_big_boss();
            self.enemies_to_spawn = 0;
            return;
        }

        let enemy_count = match self.current_map {
            MapType::Classroom => 3 + self.current_wave,
            MapType::Hallway => 5 + self.current_wave,
            MapType::Cafeteria => 6 + self.current_wave,
            MapType::Gym => 8 + self.current_wave,
            MapType::Library => 7 + self.current_wave,
            MapType::Rooftop => 0, // Bosses already spawned
        };
        self.enemies_to_spawn = enemy_count;
        self.spawn_timer = 0.5;
    }

    fn spawn_wave_enemy(&mut self) {
        let spawn_x = 800.0 + (get_time() * 1000.0) as f32 % 200.0;
        let spawn_y = (450.0 + (get_time() * 777.0) as f32 % 200.0).clamp(340.0, 660.0);

        let enemy_type = match self.current_map {
            MapType::Classroom => CharacterType::Wolters,
            MapType::Hallway | MapType::Cafeteria => {
                if (get_time() * 100.0) as i32 % 2 == 0 {
                    CharacterType::PrefectA
                } else {
                    CharacterType::PrefectB
                }
            }
            MapType::Gym => CharacterType::Coach,
            MapType::Library => CharacterType::Librarian,
            MapType::Rooftop => CharacterType::Bastiaan,
        };

        self.spawn_enemy(Vec2::new(spawn_x, spawn_y), enemy_type);
    }

    fn complete_wave(&mut self) {
        self.waves_completed += 1;

        let reward = 40 + (self.current_wave as u32 * 5);
        self.grant_currency(reward, "Wave cleared");

        if self.current_wave >= 3 {
            self.transition_to_next_map();
        } else {
            self.start_wave();
        }
    }

    fn transition_to_next_map(&mut self) {
        self.current_wave = 0;

        self.current_map = match self.current_map {
            MapType::Classroom => {
                self.show_dialogue("Luca", "Naar de gang!", "To the hallway!");
                MapType::Hallway
            }
            MapType::Hallway => {
                self.show_dialogue(
                    "Berkay",
                    "Laten we naar de kantine gaan!",
                    "Let's go to the cafeteria!",
                );
                MapType::Cafeteria
            }
            MapType::Cafeteria => {
                self.show_dialogue("Hadi", "De gymzaal is volgende!", "The gym is next!");
                MapType::Gym
            }
            MapType::Gym => {
                self.show_dialogue("Nitin", "Naar de bibliotheek!", "To the library!");
                MapType::Library
            }
            MapType::Library => {
                self.show_dialogue(
                    "You",
                    "Naar het dak voor de laatste strijd!",
                    "To the roof for the final battle!",
                );
                MapType::Rooftop
            }
            MapType::Rooftop => MapType::Rooftop,
        };

        self.start_wave();
    }

    fn show_dialogue(&mut self, speaker: &str, dutch: &str, english: &str) {
        self.dialogue_queue.push(DialogueLine {
            speaker: speaker.to_string(),
            dutch: dutch.to_string(),
            english: english.to_string(),
            duration: 2.5,
        });
    }

    fn process_fighter_states(&mut self, dt: f32) {
        let fighter_states: Vec<_> = self
            .world
            .query::<Fighter>()
            .map(|(entity, fighter)| (entity, fighter.state))
            .collect();

        let mut state_changes = Vec::new();

        for (entity, state) in &fighter_states {
            let previous = self.previous_fighter_states.get(entity).copied();
            if previous != Some(*state) {
                state_changes.push((*entity, previous, *state));
            }
        }

        let mut deactivate_hitboxes = Vec::new();

        for (entity, _) in &fighter_states {
            if let Some(fighter) = self.world.get_component_mut::<Fighter>(*entity) {
                if fighter.attack_timer > 0.0 {
                    fighter.attack_timer -= dt;
                    if fighter.attack_timer <= 0.0 {
                        fighter.attack_timer = 0.0;
                        deactivate_hitboxes.push(*entity);

                        if matches!(
                            fighter.state,
                            FighterState::LightAttack
                                | FighterState::HeavyAttack
                                | FighterState::Special
                                | FighterState::Super
                        ) {
                            fighter.state = FighterState::Idle;
                        }
                    }
                }

                if fighter.hitstun > 0.0 {
                    fighter.hitstun = (fighter.hitstun - dt).max(0.0);
                    if fighter.hitstun <= 0.0 && fighter.state == FighterState::Hitstun {
                        fighter.state = FighterState::Idle;
                    }
                }

                if fighter.blockstun > 0.0 {
                    fighter.blockstun = (fighter.blockstun - dt).max(0.0);
                    if fighter.blockstun <= 0.0 && fighter.state == FighterState::Blockstun {
                        fighter.state = FighterState::Idle;
                    }
                }
            }
        }

        for entity in deactivate_hitboxes {
            if let Some(hitbox) = self.world.get_component_mut::<HitboxComponent>(entity) {
                hitbox.active = false;
                hitbox.hits_registered.clear();
            }
        }

        for (entity, previous, current) in state_changes {
            self.on_fighter_state_changed(entity, previous, current);
        }

        self.previous_fighter_states.clear();
        for (entity, state) in fighter_states {
            self.previous_fighter_states.insert(entity, state);
        }
    }

    fn clamp_fighter_positions(&mut self) {
        let fighters: Vec<_> = self
            .world
            .query::<Fighter>()
            .map(|(entity, _)| entity)
            .collect();

        for entity in fighters {
            if let Some(transform) = self.world.get_component_mut::<Transform>(entity) {
                transform.position.x = transform.position.x.clamp(60.0, screen_width() - 60.0);
                transform.position.y = transform.position.y.clamp(340.0, 660.0);
            }
        }
    }

    fn check_game_over(&mut self) {
        if self.game_over {
            return;
        }

        if let Some(player_entity) = self.player_entity {
            let dead = match self.world.get_component::<Health>(player_entity) {
                Some(health) => health.current <= 0.0,
                None => true,
            };

            if dead {
                self.trigger_game_over();
            }
        }
    }

    fn trigger_game_over(&mut self) {
        self.game_over = true;
        self.shop_open = false;
        self.dialogue_queue.clear();
        self.current_dialogue = None;
    }

    fn on_fighter_state_changed(
        &mut self,
        entity: EntityId,
        _previous: Option<FighterState>,
        current: FighterState,
    ) {
        if matches!(
            current,
            FighterState::LightAttack
                | FighterState::HeavyAttack
                | FighterState::Special
                | FighterState::Super
        ) {
            self.activate_attack_hitbox(entity, current);

            if let Some(origin) = self
                .world
                .get_component::<Transform>(entity)
                .map(|t| t.position)
            {
                self.spawn_attack_particles(origin, current);
            }
        }
    }

    fn activate_attack_hitbox(&mut self, entity: EntityId, state: FighterState) {
        let (duration, hitbox) = match state {
            FighterState::LightAttack => (0.25, Hitbox::new_light()),
            FighterState::HeavyAttack => (0.4, Hitbox::new_heavy()),
            FighterState::Special => (0.55, Hitbox::new_special(SpecialType::Paintbrush)),
            FighterState::Super => (0.8, Hitbox::new_super()),
            _ => (0.0, Hitbox::new_light()),
        };

        let mut new_facing = None;
        if let Some(velocity) = self.world.get_component::<Velocity>(entity) {
            if velocity.linear.x > 1.0 {
                new_facing = Some(1.0);
            } else if velocity.linear.x < -1.0 {
                new_facing = Some(-1.0);
            }
        }

        if let Some(fighter) = self.world.get_component_mut::<Fighter>(entity) {
            fighter.attack_timer = duration;
            if let Some(facing) = new_facing {
                fighter.facing = facing;
            }
        }

        if let Some(hitbox_comp) = self.world.get_component_mut::<HitboxComponent>(entity) {
            hitbox_comp.hitbox = hitbox;
            hitbox_comp.active = true;
            hitbox_comp.hits_registered.clear();
        }
    }

    fn spawn_attack_particles(&mut self, center: Vec2, state: FighterState) {
        let (count, base_color, lifetime, size_start) = match state {
            FighterState::LightAttack => (6, Color::new(1.0, 0.85, 0.4, 0.9), 0.25, 12.0),
            FighterState::HeavyAttack => (10, Color::new(1.0, 0.55, 0.35, 0.95), 0.3, 14.0),
            FighterState::Special => (14, Color::new(0.45, 0.8, 1.0, 0.95), 0.35, 16.0),
            FighterState::Super => (20, Color::new(0.85, 0.4, 1.0, 1.0), 0.4, 18.0),
            _ => (6, Color::new(1.0, 0.85, 0.4, 0.9), 0.25, 12.0),
        };

        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let dir = Vec2::new(angle.cos(), angle.sin());
            let speed = rand::gen_range(140.0, 280.0);
            let velocity = dir * speed;
            let acceleration = dir * -220.0;
            let offset = dir * rand::gen_range(6.0, 22.0);
            let particle_lifetime = rand::gen_range(lifetime * 0.6, lifetime * 1.1);
            let size_end = size_start * 0.25;

            let particle_entity = self.world.create_entity();

            self.world.add_component(
                particle_entity,
                Transform {
                    position: center + offset,
                    rotation: 0.0,
                    scale: Vec2::splat(size_start),
                },
            );

            self.world.add_component(
                particle_entity,
                Particle {
                    particle_type: ParticleType::Energy,
                    lifetime: 0.0,
                    max_lifetime: particle_lifetime,
                    velocity,
                    acceleration,
                    color_start: base_color,
                    color_end: Color::new(base_color.r, base_color.g, base_color.b, 0.0),
                    size_start,
                    size_end,
                },
            );
        }
    }

    fn set_shop_feedback<S: Into<String>>(&mut self, message: S) {
        self.shop_feedback_message = Some(message.into());
        self.shop_feedback_timer = 2.5;
    }

    fn grant_currency(&mut self, amount: u32, reason: &str) {
        if amount == 0 {
            return;
        }

        self.shop_manager.add_currency(amount);
        match self.shop_manager.save() {
            Ok(_) => self.set_shop_feedback(format!("{} (+{} Arc Tokens)", reason, amount)),
            Err(err) => self.set_shop_feedback(format!("Currency save failed: {}", err)),
        }
    }

    fn apply_initial_upgrades(&mut self) {
        self.player_move_speed = 260.0;
        self.player_max_health = 100.0;
        self.player_attack_multiplier = 1.0;

        if self.shop_manager.has_upgrade(UpgradeId::SpeedBoost) {
            self.player_move_speed = 320.0;
        }
        if self.shop_manager.has_upgrade(UpgradeId::HealthBoost) {
            self.player_max_health = 140.0;
        }
        if self.shop_manager.has_upgrade(UpgradeId::AttackBoost) {
            self.player_attack_multiplier = 1.4;
        }
    }

    fn apply_upgrade_effect(&mut self, upgrade: UpgradeId) {
        match upgrade {
            UpgradeId::AttackBoost => {
                self.player_attack_multiplier = 1.4;
                self.combat_system
                    .set_player_attack_multiplier(self.player_attack_multiplier);
                self.set_shop_feedback("Attack power increased!");
            }
            UpgradeId::HealthBoost => {
                self.player_max_health = 140.0;
                if let Some(player_entity) = self.player_entity {
                    if let Some(health) = self.world.get_component_mut::<Health>(player_entity) {
                        let missing = self.player_max_health - health.maximum;
                        health.maximum = self.player_max_health;
                        health.current = (health.current + missing).min(health.maximum);
                    }
                }
                self.set_shop_feedback("Health increased!");
            }
            UpgradeId::SpeedBoost => {
                self.player_move_speed = 320.0;
                self.set_shop_feedback("Movement speed increased!");
            }
        }
    }

    fn handle_shop_controls(&mut self) {
        for (index, option) in SHOP_OPTIONS.iter().enumerate() {
            let key = match index {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                _ => continue,
            };

            if is_key_pressed(key) {
                if self.shop_manager.has_upgrade(option.id) {
                    self.set_shop_feedback(format!("{} is already owned", option.title));
                    continue;
                }

                if self.shop_manager.try_purchase(option.id, option.cost) {
                    self.apply_upgrade_effect(option.id);
                    if let Err(err) = self.shop_manager.save() {
                        self.set_shop_feedback(format!("Purchase save failed: {}", err));
                    }
                } else {
                    self.set_shop_feedback("Not enough Arc Tokens");
                }
            }
        }
    }

    fn render_shop(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay);

        let title = "Arc Supply Shop";
        let title_size = 48.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() * 0.5 - title_dims.width * 0.5,
            140.0,
            title_size,
            YELLOW,
        );

        let currency_text = format!("Arc Tokens: {}", self.shop_manager.currency());
        draw_rectangle(
            screen_width() * 0.5 - 180.0,
            160.0,
            360.0,
            38.0,
            Color::new(0.15, 0.15, 0.2, 0.9),
        );
        draw_text(
            &currency_text,
            screen_width() * 0.5 - 160.0,
            188.0,
            26.0,
            Color::new(1.0, 0.9, 0.3, 1.0),
        );

        let base_x = screen_width() * 0.5 - 340.0;
        let base_y = 220.0;
        let width = 680.0;
        let height = 100.0;

        for (index, option) in SHOP_OPTIONS.iter().enumerate() {
            let y = base_y + index as f32 * (height + 20.0);
            let owned = self.shop_manager.has_upgrade(option.id);
            let background = if owned {
                Color::new(0.15, 0.35, 0.18, 0.9)
            } else {
                Color::new(0.18, 0.18, 0.26, 0.9)
            };

            draw_rectangle(base_x, y, width, height, background);
            draw_rectangle_lines(
                base_x,
                y,
                width,
                height,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );

            let key_label = format!("{}.", index + 1);
            draw_text(&key_label, base_x + 16.0, y + 42.0, 26.0, WHITE);

            draw_text(
                option.title,
                base_x + 60.0,
                y + 38.0,
                28.0,
                Color::new(1.0, 0.9, 0.5, 1.0),
            );
            draw_text(
                option.description,
                base_x + 60.0,
                y + 68.0,
                20.0,
                Color::new(0.85, 0.85, 0.95, 1.0),
            );

            let cost_text = if owned {
                "Owned".to_string()
            } else {
                format!("Cost: {}", option.cost)
            };

            let cost_color = if owned {
                Color::new(0.6, 1.0, 0.6, 1.0)
            } else if self.shop_manager.currency() >= option.cost {
                Color::new(0.7, 0.9, 1.0, 1.0)
            } else {
                Color::new(1.0, 0.5, 0.5, 1.0)
            };

            draw_text(
                &cost_text,
                base_x + width - 180.0,
                y + 50.0,
                24.0,
                cost_color,
            );
        }

        let footer = "Press 1, 2, 3 to buy upgrades • B to close";
        let footer_dims = measure_text(footer, None, 22, 1.0);
        draw_text(
            footer,
            screen_width() * 0.5 - footer_dims.width * 0.5,
            screen_height() - 60.0,
            22.0,
            WHITE,
        );
    }

    fn render_shop_feedback(&self) {
        if let Some(ref message) = self.shop_feedback_message {
            let alpha = self.shop_feedback_timer.min(1.5) / 1.5;
            let bg = Color::new(0.05, 0.05, 0.1, alpha * 0.85);
            let text_color = Color::new(1.0, 0.95, 0.7, alpha);
            let dims = measure_text(message, None, 24, 1.0);
            let x = screen_width() * 0.5 - dims.width * 0.5 - 14.0;
            let y = screen_height() - 100.0;
            draw_rectangle(x, y - 28.0, dims.width + 28.0, 46.0, bg);
            draw_rectangle_lines(
                x,
                y - 28.0,
                dims.width + 28.0,
                46.0,
                1.5,
                Color::new(1.0, 1.0, 1.0, alpha * 0.6),
            );
            draw_text(
                message,
                screen_width() * 0.5 - dims.width * 0.5,
                y,
                24.0,
                text_color,
            );
        }
    }

    fn render_hud(&self) {
        if let Some(player_entity) = self.player_entity {
            if let Some(health) = self.world.get_component::<Health>(player_entity) {
                draw_rectangle(50.0, 50.0, 300.0, 30.0, Color::new(0.2, 0.0, 0.0, 0.8));
                draw_rectangle(
                    50.0,
                    50.0,
                    300.0 * (health.current / health.maximum),
                    30.0,
                    Color::new(0.8, 0.0, 0.0, 1.0),
                );
                draw_rectangle_lines(50.0, 50.0, 300.0, 30.0, 2.0, WHITE);

                let hp_text = format!("{:.0}/{:.0}", health.current, health.maximum);
                draw_text(&hp_text, 60.0, 70.0, 20.0, WHITE);
            }

            if let Some(fighter) = self.world.get_component::<Fighter>(player_entity) {
                draw_rectangle(50.0, 90.0, 200.0, 20.0, Color::new(0.0, 0.0, 0.2, 0.8));
                draw_rectangle(
                    50.0,
                    90.0,
                    200.0 * (fighter.meter / fighter.max_meter),
                    20.0,
                    Color::new(0.0, 0.4, 0.8, 1.0),
                );
                draw_rectangle_lines(50.0, 90.0, 200.0, 20.0, 2.0, WHITE);

                if fighter.combo_counter > 0 {
                    let combo_text = format!("COMBO x{}", fighter.combo_counter);
                    draw_text(&combo_text, screen_width() - 200.0, 100.0, 40.0, YELLOW);
                }
            }
        }

        let map_text = format!("{:?}", self.current_map);
        draw_text(&map_text, screen_width() - 200.0, 40.0, 30.0, WHITE);

        let wave_text = format!("Wave {}", self.current_wave);
        draw_text(&wave_text, screen_width() - 200.0, 70.0, 25.0, YELLOW);

        let enemies_text = format!(
            "Enemies: {} + {}",
            self.enemy_entities.len(),
            self.enemies_to_spawn
        );
        draw_text(
            &enemies_text,
            50.0,
            130.0,
            20.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );

        let currency_text = format!("Arc Tokens: {}", self.shop_manager.currency());
        draw_text(
            &currency_text,
            screen_width() - 200.0,
            130.0,
            22.0,
            Color::new(1.0, 0.9, 0.4, 1.0),
        );

        for (i, ally_entity) in self.ally_entities.iter().enumerate() {
            if let Some(health) = self.world.get_component::<Health>(*ally_entity) {
                if let Some(fighter) = self.world.get_component::<Fighter>(*ally_entity) {
                    let y = 160.0 + i as f32 * 25.0;
                    let name = self.character_display_name(&fighter.character_type, false);

                    draw_text(&name, 50.0, y, 18.0, Color::new(0.8, 0.8, 0.8, 1.0));
                    draw_rectangle(120.0, y - 12.0, 100.0, 10.0, Color::new(0.2, 0.0, 0.0, 0.6));
                    draw_rectangle(
                        120.0,
                        y - 12.0,
                        100.0 * (health.current / health.maximum),
                        10.0,
                        Color::new(0.0, 0.8, 0.0, 1.0),
                    );
                }
            }
        }

        // Render ability UI
        use crate::data::Character;
        let character = Character::get_by_id(self.selected_character);
        let ability_y = 120.0;

        // Ability icon/button
        let ability_color = if self.ability_state.can_activate() {
            Color::new(0.0, 0.8, 0.0, 0.8)
        } else if self.ability_state.active {
            Color::new(1.0, 0.8, 0.0, 0.8)
        } else {
            Color::new(0.3, 0.3, 0.3, 0.8)
        };

        draw_rectangle(50.0, ability_y, 200.0, 40.0, Color::new(0.0, 0.0, 0.0, 0.6));

        if self.ability_state.active {
            let remaining_pct = self.ability_state.active_time / character.duration;
            draw_rectangle(50.0, ability_y, 200.0 * remaining_pct, 40.0, ability_color);
        } else if self.ability_state.cooldown_time > 0.0 {
            let cooldown_pct = 1.0 - (self.ability_state.cooldown_time / character.cooldown);
            draw_rectangle(50.0, ability_y, 200.0 * cooldown_pct, 40.0, ability_color);
        } else {
            draw_rectangle(50.0, ability_y, 200.0, 40.0, ability_color);
        }

        draw_rectangle_lines(50.0, ability_y, 200.0, 40.0, 2.0, WHITE);

        let ability_text = if self.ability_state.active {
            format!("{} [{:.1}s]", character.ability_name, self.ability_state.active_time)
        } else if self.ability_state.cooldown_time > 0.0 {
            format!("{} [{:.1}s]", character.ability_name, self.ability_state.cooldown_time)
        } else {
            format!("{} [E]", character.ability_name)
        };

        draw_text(&ability_text, 60.0, ability_y + 25.0, 18.0, WHITE);

        // Show voice line if active
        if let Some(ref voice_line) = self.ability_voice_line {
            let voice_y = screen_height() * 0.3;
            let voice_size = 40.0;
            let voice_dims = measure_text(voice_line, None, voice_size as u16, 1.0);

            // Background
            draw_rectangle(
                screen_width() * 0.5 - voice_dims.width * 0.5 - 20.0,
                voice_y - 40.0,
                voice_dims.width + 40.0,
                60.0,
                Color::new(0.0, 0.0, 0.0, 0.7),
            );

            // Text with glow effect
            let glow_color = Color::new(1.0, 1.0, 0.0, 0.5);
            for dx in [-2.0, 0.0, 2.0] {
                for dy in [-2.0, 0.0, 2.0] {
                    if dx != 0.0 || dy != 0.0 {
                        draw_text(
                            voice_line,
                            screen_width() * 0.5 - voice_dims.width * 0.5 + dx,
                            voice_y + dy,
                            voice_size,
                            glow_color,
                        );
                    }
                }
            }
            draw_text(
                voice_line,
                screen_width() * 0.5 - voice_dims.width * 0.5,
                voice_y,
                voice_size,
                YELLOW,
            );
        }
    }

    fn character_display_name(&self, character: &CharacterType, is_player: bool) -> String {
        if is_player {
            return "You".to_string();
        }

        match character {
            CharacterType::Bas => "Bas".to_string(),
            CharacterType::Berkay => "Berkay".to_string(),
            CharacterType::Hadi => "Hadi".to_string(),
            CharacterType::Luca => "Luca".to_string(),
            CharacterType::Nitin => "Nitin".to_string(),
            CharacterType::Wolters => "Wolters".to_string(),
            CharacterType::PrefectA => "Prefect A".to_string(),
            CharacterType::PrefectB => "Prefect B".to_string(),
            CharacterType::Chef => "Chef".to_string(),
            CharacterType::Librarian => "Librarian".to_string(),
            CharacterType::Coach => "Coach".to_string(),
            CharacterType::Bastiaan => "BOSS BASTIAAN".to_string(),
            CharacterType::KeizerBomTaha => "Keizer Bom Taha".to_string(),
        }
    }

    fn render_nametag(&self, pos: Vec2, name: &str, color: Color) {
        if name.is_empty() {
            return;
        }

        let font_size = 20.0;
        let metrics = measure_text(name, None, font_size as u16, 1.0);
        let width = metrics.width + 24.0;
        let height = metrics.height + 14.0;
        let x = pos.x - width * 0.5;
        let y = pos.y - 115.0;

        // Shadow
        draw_rectangle(x + 2.0, y + 2.0, width, height, Color::new(0.0, 0.0, 0.0, 0.6));

        // Background with gradient effect
        draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.85));

        // Top highlight
        draw_rectangle(x, y, width, height * 0.3, Color::new(color.r * 0.5, color.g * 0.5, color.b * 0.5, 0.6));

        // Colored border with glow
        draw_rectangle_lines(x, y, width, height, 2.5, color);
        draw_rectangle_lines(x - 1.0, y - 1.0, width + 2.0, height + 2.0, 1.0,
            Color::new(color.r, color.g, color.b, 0.4));

        // Text with subtle shadow
        draw_text(
            name,
            pos.x - metrics.width * 0.5 + 1.0,
            y + height - 4.0,
            font_size,
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        // Main text
        draw_text(
            name,
            pos.x - metrics.width * 0.5,
            y + height - 5.0,
            font_size,
            color,
        );
    }

    fn render_classroom(&self) {
        let time = get_time() as f32;

        // Enhanced floor with tiles
        let floor_y = 600.0;
        for i in 0..20 {
            for j in 0..4 {
                let x = i as f32 * 80.0;
                let y = floor_y + j as f32 * 40.0;
                let tile_shade = if (i + j) % 2 == 0 { 0.45 } else { 0.42 };
                draw_rectangle(x, y, 80.0, 40.0, Color::new(tile_shade, tile_shade - 0.05, tile_shade - 0.1, 1.0));
                draw_rectangle_lines(x, y, 80.0, 40.0, 1.0, Color::new(0.3, 0.25, 0.2, 0.3));
            }
        }

        // Ceiling beam with depth
        draw_rectangle(0.0, 95.0, screen_width(), 30.0, Color::new(0.25, 0.2, 0.15, 1.0));
        draw_rectangle(0.0, 100.0, screen_width(), 20.0, Color::new(0.35, 0.28, 0.22, 1.0));
        draw_rectangle(0.0, 118.0, screen_width(), 2.0, Color::new(0.15, 0.12, 0.1, 1.0));

        // Enhanced blackboard with frame
        draw_rectangle(45.0, 115.0, 310.0, 210.0, Color::new(0.15, 0.12, 0.1, 1.0)); // Shadow
        draw_rectangle(40.0, 110.0, 310.0, 210.0, Color::new(0.5, 0.42, 0.35, 1.0)); // Frame
        draw_rectangle(50.0, 120.0, 290.0, 190.0, Color::new(0.08, 0.25, 0.08, 1.0)); // Board

        // Frame details
        draw_rectangle(45.0, 115.0, 300.0, 5.0, Color::new(0.6, 0.52, 0.45, 1.0));
        draw_rectangle(45.0, 115.0, 5.0, 200.0, Color::new(0.6, 0.52, 0.45, 1.0));

        // Chalk text with glow
        let text = "BAS VEGEN";
        draw_text(text, 82.0, 202.0, 42.0, Color::new(0.7, 0.7, 0.7, 0.3));
        draw_text(text, 80.0, 200.0, 42.0, WHITE);
        draw_text("A + B = C", 82.0, 252.0, 32.0, Color::new(0.7, 0.7, 0.7, 0.3));
        draw_text("A + B = C", 80.0, 250.0, 32.0, WHITE);

        // Chalk ledge
        draw_rectangle(50.0, 308.0, 290.0, 8.0, Color::new(0.4, 0.34, 0.28, 1.0));

        // Enhanced desks with shadows and depth
        for i in 0..4 {
            let desk_x = 450.0 + (i as f32 * 150.0);

            // Desk shadow
            draw_rectangle(desk_x + 3.0, 453.0, 120.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.3));

            // Desk top with highlight
            draw_rectangle(desk_x, 450.0, 120.0, 80.0, Color::new(0.55, 0.45, 0.35, 1.0));
            draw_rectangle(desk_x, 450.0, 120.0, 15.0, Color::new(0.65, 0.55, 0.45, 1.0)); // Highlight
            draw_rectangle_lines(desk_x, 450.0, 120.0, 80.0, 2.0, Color::new(0.4, 0.32, 0.25, 1.0));

            // Chair with backrest
            draw_rectangle(desk_x + 40.0, 383.0, 40.0, 70.0, Color::new(0.25, 0.25, 0.3, 1.0));
            draw_rectangle(desk_x + 42.0, 380.0, 36.0, 68.0, Color::new(0.35, 0.35, 0.4, 1.0));
            draw_rectangle(desk_x + 35.0, 360.0, 50.0, 25.0, Color::new(0.35, 0.35, 0.4, 1.0));
        }

        // Enhanced teacher's desk
        let teacher_x = screen_width() - 160.0;
        draw_rectangle(teacher_x + 5.0, 203.0, 110.0, 155.0, Color::new(0.0, 0.0, 0.0, 0.3));
        draw_rectangle(teacher_x, 198.0, 110.0, 155.0, Color::new(0.65, 0.55, 0.45, 1.0));
        draw_rectangle(teacher_x, 198.0, 110.0, 20.0, Color::new(0.75, 0.65, 0.55, 1.0));
        draw_rectangle_lines(teacher_x, 198.0, 110.0, 155.0, 3.0, Color::new(0.5, 0.4, 0.32, 1.0));

        // Computer/papers on desk
        draw_rectangle(teacher_x + 10.0, 208.0, 45.0, 35.0, Color::new(0.85, 0.85, 0.85, 1.0));
        draw_rectangle(teacher_x + 12.0, 210.0, 41.0, 31.0, Color::new(0.2, 0.3, 0.4, 1.0));
        draw_rectangle(teacher_x + 60.0, 215.0, 35.0, 25.0, Color::new(0.95, 0.95, 0.85, 1.0));

        // Enhanced plant
        draw_rectangle(screen_width() - 255.0, 403.0, 60.0, 105.0, Color::new(0.0, 0.0, 0.0, 0.3));
        draw_rectangle(screen_width() - 260.0, 400.0, 60.0, 105.0, Color::new(0.45, 0.3, 0.2, 1.0));
        draw_circle(screen_width() - 230.0, 380.0, 50.0, Color::new(0.25, 0.55, 0.25, 1.0));
        draw_circle(screen_width() - 240.0, 390.0, 35.0, Color::new(0.3, 0.6, 0.3, 1.0));
        draw_circle(screen_width() - 220.0, 395.0, 30.0, Color::new(0.35, 0.65, 0.35, 1.0));

        // Enhanced windows with light rays
        for i in 0..7 {
            let window_x = 380.0 + (i as f32 * 130.0);
            if window_x < screen_width() - 220.0 {
                // Window shadow
                draw_rectangle(window_x + 3.0, 153.0, 85.0, 125.0, Color::new(0.0, 0.0, 0.0, 0.2));

                // Window frame
                draw_rectangle(window_x - 5.0, 145.0, 95.0, 135.0, Color::new(0.45, 0.38, 0.3, 1.0));

                // Glass with subtle animation
                let light_variation = ((time + i as f32 * 0.5).sin() * 0.05 + 0.75) as f32;
                draw_rectangle(window_x, 150.0, 85.0, 125.0, Color::new(0.65 * light_variation, 0.75 * light_variation, 0.95, 0.4));

                // Light rays through window
                for ray in 0..3 {
                    let ray_x = window_x + 15.0 + ray as f32 * 25.0;
                    let ray_alpha = 0.1 + ((time * 0.5 + ray as f32 * 0.3).sin() * 0.05) as f32;
                    for y_step in 0..10 {
                        let y_pos = 275.0 + y_step as f32 * 35.0;
                        draw_line(ray_x, 275.0, ray_x + 15.0, y_pos, 25.0, Color::new(1.0, 1.0, 0.9, ray_alpha));
                    }
                }

                // Window dividers
                draw_rectangle_lines(window_x, 150.0, 85.0, 125.0, 4.0, Color::new(0.4, 0.33, 0.26, 1.0));
                draw_line(window_x + 42.5, 150.0, window_x + 42.5, 275.0, 3.0, Color::new(0.4, 0.33, 0.26, 1.0));
                draw_line(window_x, 212.5, window_x + 85.0, 212.5, 3.0, Color::new(0.4, 0.33, 0.26, 1.0));
            }
        }
    }

    fn render_hallway(&self) {
        let time = get_time() as f32;

        // Tiled floor
        let floor_y = 600.0;
        for i in 0..25 {
            for j in 0..3 {
                let x = i as f32 * 70.0;
                let y = floor_y + j as f32 * 50.0;
                let shade = if (i + j) % 2 == 0 { 0.65 } else { 0.6 };
                draw_rectangle(x, y, 70.0, 50.0, Color::new(shade, shade - 0.08, shade - 0.15, 1.0));
            }
        }

        // Enhanced lockers with depth
        for i in 0..20 {
            let locker_x = i as f32 * 80.0;
            draw_rectangle(locker_x + 3.0, 153.0, 70.0, 350.0, Color::new(0.0, 0.0, 0.0, 0.3));
            draw_rectangle(locker_x, 150.0, 70.0, 350.0, Color::new(0.45, 0.45, 0.65, 1.0));
            draw_rectangle(locker_x, 150.0, 70.0, 30.0, Color::new(0.55, 0.55, 0.75, 1.0));
            draw_rectangle_lines(locker_x, 150.0, 70.0, 350.0, 3.0, Color::new(0.35, 0.35, 0.55, 1.0));
            draw_circle(locker_x + 35.0, 325.0, 7.0, Color::new(0.2, 0.2, 0.2, 1.0));
            draw_circle(locker_x + 35.0, 325.0, 5.0, Color::new(0.8, 0.8, 0.8, 1.0));
            draw_line(locker_x, 320.0, locker_x + 70.0, 320.0, 2.0, Color::new(0.35, 0.35, 0.55, 1.0));
        }

        // Ceiling with lights
        draw_rectangle(0.0, 0.0, screen_width(), 150.0, Color::new(0.92, 0.92, 0.87, 1.0));
        for i in 0..10 {
            let light_x = i as f32 * 200.0 + 100.0;
            let glow = ((time * 2.0 + i as f32).sin() * 0.1 + 0.9) as f32;
            draw_circle(light_x, 75.0, 35.0, Color::new(1.0 * glow, 0.95 * glow, 0.8 * glow, 0.6));
            draw_circle(light_x, 75.0, 28.0, Color::new(1.0, 0.98, 0.85, 1.0));
        }
    }

    fn render_cafeteria(&self) {
        let floor_y = 600.0;
        draw_rectangle(
            0.0,
            floor_y,
            screen_width(),
            screen_height() - floor_y,
            Color::new(0.8, 0.8, 0.7, 1.0),
        );

        for i in 0..6 {
            let table_x = 150.0 + i as f32 * 250.0;
            draw_rectangle(table_x, 400.0, 200.0, 100.0, Color::new(0.6, 0.5, 0.4, 1.0));
            draw_rectangle(
                table_x + 10.0,
                500.0,
                20.0,
                80.0,
                Color::new(0.4, 0.3, 0.2, 1.0),
            );
            draw_rectangle(
                table_x + 170.0,
                500.0,
                20.0,
                80.0,
                Color::new(0.4, 0.3, 0.2, 1.0),
            );
            draw_rectangle(
                table_x + 20.0,
                480.0,
                60.0,
                20.0,
                Color::new(0.5, 0.4, 0.3, 1.0),
            );
            draw_rectangle(
                table_x + 120.0,
                480.0,
                60.0,
                20.0,
                Color::new(0.5, 0.4, 0.3, 1.0),
            );
        }

        draw_rectangle(
            0.0,
            100.0,
            screen_width(),
            150.0,
            Color::new(0.7, 0.7, 0.7, 1.0),
        );
        draw_rectangle(50.0, 120.0, 200.0, 100.0, Color::new(0.9, 0.9, 0.9, 1.0));
        draw_text("MENU", 100.0, 180.0, 30.0, Color::new(0.2, 0.2, 0.2, 1.0));
    }

    fn render_gym(&self) {
        let floor_y = 600.0;
        draw_rectangle(
            0.0,
            floor_y,
            screen_width(),
            screen_height() - floor_y,
            Color::new(0.9, 0.7, 0.5, 1.0),
        );

        for i in 0..20 {
            let line_x = i as f32 * 100.0;
            draw_line(
                line_x,
                floor_y,
                line_x,
                screen_height(),
                3.0,
                Color::new(0.8, 0.6, 0.4, 1.0),
            );
        }

        draw_circle(
            screen_width() * 0.5,
            450.0,
            100.0,
            Color::new(1.0, 1.0, 1.0, 0.3),
        );
        draw_circle_lines(screen_width() * 0.5, 450.0, 100.0, 5.0, WHITE);

        draw_rectangle(100.0, 200.0, 10.0, 200.0, Color::new(0.8, 0.8, 0.8, 1.0));
        draw_circle(100.0, 180.0, 50.0, Color::new(1.0, 0.6, 0.2, 1.0));
        draw_rectangle(95.0, 180.0, 10.0, 300.0, Color::new(1.0, 1.0, 1.0, 1.0));

        draw_rectangle(
            screen_width() - 110.0,
            200.0,
            10.0,
            200.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );
        draw_circle(
            screen_width() - 100.0,
            180.0,
            50.0,
            Color::new(1.0, 0.6, 0.2, 1.0),
        );
    }

    fn render_library(&self) {
        let floor_y = 600.0;
        draw_rectangle(
            0.0,
            floor_y,
            screen_width(),
            screen_height() - floor_y,
            Color::new(0.5, 0.4, 0.3, 1.0),
        );

        for i in 0..8 {
            let shelf_x = 100.0 + i as f32 * 200.0;
            draw_rectangle(shelf_x, 150.0, 150.0, 400.0, Color::new(0.4, 0.3, 0.2, 1.0));
            for j in 0..5 {
                let shelf_y = 170.0 + j as f32 * 75.0;
                draw_rectangle(
                    shelf_x + 5.0,
                    shelf_y,
                    140.0,
                    65.0,
                    Color::new(0.3, 0.2, 0.1, 1.0),
                );
                for k in 0..10 {
                    let book_x = shelf_x + 10.0 + k as f32 * 13.0;
                    let book_color = if k % 3 == 0 {
                        Color::new(0.7, 0.2, 0.2, 1.0)
                    } else if k % 3 == 1 {
                        Color::new(0.2, 0.4, 0.7, 1.0)
                    } else {
                        Color::new(0.2, 0.6, 0.2, 1.0)
                    };
                    draw_rectangle(book_x, shelf_y + 5.0, 10.0, 55.0, book_color);
                }
            }
        }

        draw_rectangle(
            screen_width() * 0.5 - 100.0,
            450.0,
            200.0,
            80.0,
            Color::new(0.6, 0.5, 0.4, 1.0),
        );
        draw_rectangle(
            screen_width() * 0.5 - 80.0,
            390.0,
            60.0,
            60.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );
        draw_rectangle(
            screen_width() * 0.5 + 20.0,
            390.0,
            60.0,
            60.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    fn render_rooftop(&self) {
        let floor_y = 600.0;
        draw_rectangle(
            0.0,
            floor_y,
            screen_width(),
            screen_height() - floor_y,
            Color::new(0.4, 0.4, 0.4, 1.0),
        );

        for i in 0..3 {
            draw_circle(
                200.0 + i as f32 * 500.0,
                100.0,
                80.0,
                Color::new(1.0, 1.0, 1.0, 0.8),
            );
            draw_circle(
                250.0 + i as f32 * 500.0,
                120.0,
                60.0,
                Color::new(1.0, 1.0, 1.0, 0.7),
            );
        }

        draw_rectangle(
            0.0,
            580.0,
            screen_width(),
            20.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
        draw_rectangle(
            0.0,
            590.0,
            screen_width(),
            5.0,
            Color::new(0.8, 0.8, 0.0, 1.0),
        );

        for i in 0..10 {
            let fence_x = i as f32 * 150.0;
            draw_rectangle(fence_x, 450.0, 5.0, 130.0, Color::new(0.3, 0.3, 0.3, 1.0));
        }
        draw_rectangle(
            0.0,
            450.0,
            screen_width(),
            8.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );
        draw_rectangle(
            0.0,
            515.0,
            screen_width(),
            8.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );

        draw_rectangle(100.0, 350.0, 80.0, 100.0, Color::new(0.7, 0.7, 0.7, 1.0));
        draw_rectangle(110.0, 360.0, 60.0, 30.0, Color::new(0.2, 0.2, 0.2, 1.0));

        draw_rectangle(
            screen_width() - 200.0,
            300.0,
            150.0,
            150.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );
        draw_circle(
            screen_width() - 125.0,
            375.0,
            40.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
    }

    fn render_character(&self, base_pos: Vec2, fighter: &Fighter) {
        let state = fighter.state;
        let time = get_time() as f32;
        let attack_phase = Self::attack_phase(state, fighter.attack_timer);
        let bob = if matches!(state, FighterState::Walking) {
            (time * 8.0).sin() * 2.0
        } else {
            0.0
        };
        let sway = match state {
            FighterState::LightAttack
            | FighterState::HeavyAttack
            | FighterState::Special
            | FighterState::Super => fighter.facing * (6.0 + 8.0 * attack_phase),
            FighterState::Hitstun => -fighter.facing * 4.0,
            FighterState::Dodging => fighter.facing * (time * 14.0).cos() * 3.0,
            _ => 0.0,
        };

        let pos = base_pos + Vec2::new(sway, bob);

        match &fighter.character_type {
            CharacterType::Bas => {
                // HEAD with depth and shading
                draw_circle(pos.x + 1.5, pos.y - 43.0, 26.0, Color::new(0.7, 0.6, 0.5, 0.5)); // Shadow
                draw_circle(pos.x, pos.y - 45.0, 26.0, Color::new(0.92, 0.82, 0.72, 1.0)); // Base
                draw_circle(pos.x - 6.0, pos.y - 48.0, 10.0, Color::new(1.0, 0.9, 0.8, 0.4)); // Highlight

                // NECK
                draw_rectangle(pos.x - 8.0, pos.y - 22.0, 16.0, 10.0, Color::new(0.9, 0.8, 0.7, 1.0));

                // TORSO with blue shirt and depth
                draw_rectangle(pos.x - 22.0, pos.y - 18.0, 44.0, 52.0, Color::new(0.15, 0.2, 0.6, 1.0)); // Shadow layer
                draw_rectangle(pos.x - 20.0, pos.y - 20.0, 40.0, 50.0, Color::new(0.2, 0.35, 0.85, 1.0)); // Main shirt
                draw_rectangle(pos.x - 20.0, pos.y - 20.0, 40.0, 8.0, Color::new(0.3, 0.45, 0.95, 1.0)); // Shoulder highlight

                // Collar
                draw_line(pos.x - 10.0, pos.y - 20.0, pos.x, pos.y - 10.0, 3.0, Color::new(0.15, 0.25, 0.7, 1.0));
                draw_line(pos.x + 10.0, pos.y - 20.0, pos.x, pos.y - 10.0, 3.0, Color::new(0.15, 0.25, 0.7, 1.0));

                // Shirt buttons
                for i in 0..3 {
                    let button_y = pos.y - 8.0 + i as f32 * 12.0;
                    draw_circle(pos.x, button_y, 2.5, Color::new(0.9, 0.9, 0.9, 1.0));
                    draw_circle(pos.x, button_y, 1.5, Color::new(0.15, 0.2, 0.6, 1.0));
                }

                // PANTS with shading
                draw_rectangle(pos.x - 16.0, pos.y + 32.0, 32.0, 42.0, Color::new(0.25, 0.25, 0.35, 1.0)); // Shadow
                draw_rectangle(pos.x - 15.0, pos.y + 30.0, 30.0, 40.0, Color::new(0.35, 0.35, 0.45, 1.0)); // Main pants
                draw_rectangle(pos.x - 15.0, pos.y + 30.0, 30.0, 6.0, Color::new(0.45, 0.45, 0.55, 1.0)); // Waist highlight

                // Belt
                draw_rectangle(pos.x - 16.0, pos.y + 28.0, 32.0, 5.0, Color::new(0.2, 0.15, 0.1, 1.0));
                draw_rectangle(pos.x - 4.0, pos.y + 27.0, 8.0, 7.0, Color::new(0.6, 0.5, 0.3, 1.0)); // Buckle
                draw_rectangle(pos.x - 2.0, pos.y + 28.0, 4.0, 5.0, Color::new(0.4, 0.3, 0.2, 1.0));

                // LEGS with depth
                draw_rectangle(pos.x - 11.0, pos.y + 72.0, 10.0, 22.0, Color::new(0.25, 0.25, 0.35, 1.0));
                draw_rectangle(pos.x + 1.0, pos.y + 72.0, 10.0, 22.0, Color::new(0.25, 0.25, 0.35, 1.0));
                draw_rectangle(pos.x - 10.0, pos.y + 70.0, 8.0, 20.0, Color::new(0.35, 0.35, 0.45, 1.0));
                draw_rectangle(pos.x + 2.0, pos.y + 70.0, 8.0, 20.0, Color::new(0.35, 0.35, 0.45, 1.0));

                // SHOES with detail
                draw_ellipse(pos.x - 6.0, pos.y + 92.0, 10.0, 6.0, 0.0, Color::new(0.15, 0.15, 0.15, 1.0));
                draw_ellipse(pos.x + 6.0, pos.y + 92.0, 10.0, 6.0, 0.0, Color::new(0.15, 0.15, 0.15, 1.0));
                draw_ellipse(pos.x - 6.0, pos.y + 91.0, 9.0, 5.0, 0.0, Color::new(0.25, 0.25, 0.25, 1.0));
                draw_ellipse(pos.x + 6.0, pos.y + 91.0, 9.0, 5.0, 0.0, Color::new(0.25, 0.25, 0.25, 1.0));

                // ARMS with muscles and skin tone
                let arm_idle = Vec2::new(15.0, (time * 12.0).cos() * 1.5);
                let arm_offset = match state {
                    FighterState::LightAttack | FighterState::HeavyAttack | FighterState::Special | FighterState::Super => {
                        Vec2::new(28.0 + 40.0 * attack_phase, -12.0 - 24.0 * attack_phase)
                    }
                    _ => arm_idle,
                };

                // Left arm (back)
                draw_rectangle(pos.x - 26.0, pos.y - 9.0, 10.0, 32.0, Color::new(0.82, 0.72, 0.62, 1.0));
                draw_rectangle(pos.x - 25.0, pos.y - 10.0, 8.0, 30.0, Color::new(0.9, 0.8, 0.7, 1.0));
                draw_rectangle(pos.x - 25.0, pos.y - 10.0, 8.0, 8.0, Color::new(1.0, 0.9, 0.8, 0.6)); // Highlight

                // Right arm (front, animated)
                draw_rectangle(pos.x + 18.0, pos.y - 9.0 + arm_offset.y, 10.0, 32.0, Color::new(0.82, 0.72, 0.62, 1.0));
                draw_rectangle(pos.x + 17.0, pos.y - 10.0 + arm_offset.y, 8.0, 30.0, Color::new(0.9, 0.8, 0.7, 1.0));
                draw_rectangle(pos.x + 17.0, pos.y - 10.0 + arm_offset.y, 8.0, 8.0, Color::new(1.0, 0.9, 0.8, 0.6));

                // HANDS/FISTS
                if matches!(state, FighterState::LightAttack | FighterState::HeavyAttack | FighterState::Special | FighterState::Super) {
                    draw_circle(pos.x + 26.0 + arm_offset.x, pos.y + arm_offset.y + 2.0, 10.0, Color::new(0.82, 0.72, 0.62, 1.0));
                    draw_circle(pos.x + 25.0 + arm_offset.x, pos.y + arm_offset.y, 9.0, Color::new(0.9, 0.8, 0.7, 1.0));
                    self.render_attack_slash(pos + Vec2::new(25.0 + arm_offset.x, arm_offset.y), fighter, attack_phase);
                } else {
                    draw_circle(pos.x + 17.0, pos.y + 22.0, 6.0, Color::new(0.9, 0.8, 0.7, 1.0));
                    draw_circle(pos.x - 25.0, pos.y + 22.0, 6.0, Color::new(0.9, 0.8, 0.7, 1.0));
                }

                // FACE DETAILS
                // Eyes
                draw_circle(pos.x - 9.0, pos.y - 47.0, 5.0, Color::new(1.0, 1.0, 1.0, 1.0));
                draw_circle(pos.x + 9.0, pos.y - 47.0, 5.0, Color::new(1.0, 1.0, 1.0, 1.0));
                draw_circle(pos.x - 8.0, pos.y - 46.0, 3.5, Color::new(0.15, 0.3, 0.5, 1.0));
                draw_circle(pos.x + 8.0, pos.y - 46.0, 3.5, Color::new(0.15, 0.3, 0.5, 1.0));
                draw_circle(pos.x - 7.0, pos.y - 47.0, 1.5, Color::new(1.0, 1.0, 1.0, 0.8)); // Glint
                draw_circle(pos.x + 9.0, pos.y - 47.0, 1.5, Color::new(1.0, 1.0, 1.0, 0.8));

                // Eyebrows
                draw_line(pos.x - 12.0, pos.y - 52.0, pos.x - 5.0, pos.y - 53.0, 2.0, Color::new(0.3, 0.25, 0.2, 1.0));
                draw_line(pos.x + 12.0, pos.y - 52.0, pos.x + 5.0, pos.y - 53.0, 2.0, Color::new(0.3, 0.25, 0.2, 1.0));

                // Nose
                draw_line(pos.x, pos.y - 44.0, pos.x + 2.0, pos.y - 40.0, 2.0, Color::new(0.82, 0.72, 0.62, 1.0));

                // Mouth
                let mouth_curve_y = if matches!(state, FighterState::Hitstun) { -36.0 } else { -38.0 };
                draw_line(pos.x - 6.0, pos.y + mouth_curve_y, pos.x + 6.0, pos.y + mouth_curve_y, 2.0, Color::new(0.6, 0.3, 0.3, 1.0));

                // HAIR with volume
                draw_circle(pos.x - 10.0, pos.y - 60.0, 12.0, Color::new(0.35, 0.28, 0.22, 1.0));
                draw_circle(pos.x + 8.0, pos.y - 62.0, 11.0, Color::new(0.35, 0.28, 0.22, 1.0));
                draw_circle(pos.x, pos.y - 65.0, 13.0, Color::new(0.35, 0.28, 0.22, 1.0));
                draw_circle(pos.x - 15.0, pos.y - 55.0, 10.0, Color::new(0.35, 0.28, 0.22, 1.0));
                draw_circle(pos.x + 13.0, pos.y - 57.0, 9.0, Color::new(0.35, 0.28, 0.22, 1.0));

                // Hair highlights
                draw_circle(pos.x - 5.0, pos.y - 63.0, 5.0, Color::new(0.45, 0.38, 0.32, 0.6));
            }
            CharacterType::Wolters => {
                draw_circle(pos.x, pos.y - 45.0, 28.0, Color::new(0.9, 0.7, 0.6, 1.0));

                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 20.0,
                    50.0,
                    55.0,
                    Color::new(0.4, 0.4, 0.4, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y - 15.0,
                    30.0,
                    8.0,
                    Color::new(0.8, 0.2, 0.2, 1.0),
                );

                draw_rectangle(
                    pos.x - 18.0,
                    pos.y + 35.0,
                    36.0,
                    40.0,
                    Color::new(0.2, 0.2, 0.3, 1.0),
                );

                draw_rectangle(
                    pos.x - 12.0,
                    pos.y + 75.0,
                    10.0,
                    25.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 75.0,
                    10.0,
                    25.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );

                draw_rectangle(
                    pos.x - 30.0,
                    pos.y - 10.0,
                    10.0,
                    35.0,
                    Color::new(0.9, 0.7, 0.6, 1.0),
                );
                draw_rectangle(
                    pos.x + 20.0,
                    pos.y - 10.0,
                    10.0,
                    35.0,
                    Color::new(0.9, 0.7, 0.6, 1.0),
                );

                draw_circle(
                    pos.x - 10.0,
                    pos.y - 48.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 10.0,
                    pos.y - 48.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );

                draw_rectangle(
                    pos.x - 10.0,
                    pos.y - 35.0,
                    20.0,
                    3.0,
                    Color::new(0.5, 0.4, 0.3, 1.0),
                );

                draw_circle(
                    pos.x - 30.0,
                    pos.y - 45.0,
                    8.0,
                    Color::new(0.7, 0.5, 0.4, 1.0),
                );
                draw_circle(
                    pos.x + 30.0,
                    pos.y - 45.0,
                    8.0,
                    Color::new(0.7, 0.5, 0.4, 1.0),
                );

                draw_rectangle(
                    pos.x - 10.0,
                    pos.y - 30.0,
                    20.0,
                    2.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
            }
            CharacterType::Luca => {
                draw_circle(pos.x, pos.y - 45.0, 24.0, Color::new(0.95, 0.85, 0.75, 1.0));
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y - 20.0,
                    40.0,
                    50.0,
                    Color::new(0.1, 0.6, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y + 30.0,
                    30.0,
                    40.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 10.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.95, 0.85, 0.75, 1.0),
                );
                draw_rectangle(
                    pos.x + 17.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.95, 0.85, 0.75, 1.0),
                );
                draw_circle(
                    pos.x - 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.1, 0.3, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.1, 0.3, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 18.0,
                    pos.y - 62.0,
                    36.0,
                    4.0,
                    Color::new(0.6, 0.5, 0.4, 1.0),
                );
            }
            CharacterType::Hadi => {
                draw_circle(pos.x, pos.y - 45.0, 23.0, Color::new(0.85, 0.75, 0.65, 1.0));
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y - 20.0,
                    40.0,
                    50.0,
                    Color::new(0.8, 0.4, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y + 30.0,
                    30.0,
                    40.0,
                    Color::new(0.4, 0.4, 0.5, 1.0),
                );
                draw_rectangle(
                    pos.x - 10.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.85, 0.75, 0.65, 1.0),
                );
                draw_rectangle(
                    pos.x + 17.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.85, 0.75, 0.65, 1.0),
                );
                draw_circle(
                    pos.x - 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.3, 0.2, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.3, 0.2, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y - 60.0,
                    30.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
            }
            CharacterType::Berkay => {
                draw_circle(pos.x, pos.y - 45.0, 25.0, Color::new(0.9, 0.8, 0.7, 1.0));
                draw_rectangle(
                    pos.x - 22.0,
                    pos.y - 20.0,
                    44.0,
                    52.0,
                    Color::new(0.5, 0.5, 0.5, 1.0),
                );
                draw_rectangle(
                    pos.x - 16.0,
                    pos.y + 32.0,
                    32.0,
                    38.0,
                    Color::new(0.1, 0.1, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x - 10.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 27.0,
                    pos.y - 10.0,
                    9.0,
                    32.0,
                    Color::new(0.9, 0.8, 0.7, 1.0),
                );
                draw_rectangle(
                    pos.x + 18.0,
                    pos.y - 10.0,
                    9.0,
                    32.0,
                    Color::new(0.9, 0.8, 0.7, 1.0),
                );
                draw_circle(
                    pos.x - 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_circle(
                    pos.x + 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 22.0,
                    pos.y - 63.0,
                    44.0,
                    5.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
            }
            CharacterType::Nitin => {
                draw_circle(pos.x, pos.y - 45.0, 24.0, Color::new(0.75, 0.6, 0.5, 1.0));
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y - 20.0,
                    40.0,
                    50.0,
                    Color::new(0.3, 0.3, 0.6, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y + 30.0,
                    30.0,
                    40.0,
                    Color::new(0.5, 0.3, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 10.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    8.0,
                    20.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.75, 0.6, 0.5, 1.0),
                );
                draw_rectangle(
                    pos.x + 17.0,
                    pos.y - 10.0,
                    8.0,
                    30.0,
                    Color::new(0.75, 0.6, 0.5, 1.0),
                );
                draw_circle(
                    pos.x - 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 8.0,
                    pos.y - 45.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y - 62.0,
                    40.0,
                    4.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
            }
            CharacterType::PrefectA | CharacterType::PrefectB => {
                draw_circle(pos.x, pos.y - 45.0, 26.0, Color::new(0.9, 0.8, 0.7, 1.0));
                draw_rectangle(
                    pos.x - 23.0,
                    pos.y - 20.0,
                    46.0,
                    52.0,
                    Color::new(0.2, 0.2, 0.5, 1.0),
                );
                draw_rectangle(
                    pos.x - 10.0,
                    pos.y - 15.0,
                    20.0,
                    5.0,
                    Color::new(0.9, 0.9, 0.0, 1.0),
                );
                draw_rectangle(
                    pos.x - 17.0,
                    pos.y + 32.0,
                    34.0,
                    38.0,
                    Color::new(0.2, 0.2, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x - 11.0,
                    pos.y + 70.0,
                    9.0,
                    22.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    9.0,
                    22.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 28.0,
                    pos.y - 10.0,
                    9.0,
                    32.0,
                    Color::new(0.9, 0.8, 0.7, 1.0),
                );
                draw_rectangle(
                    pos.x + 19.0,
                    pos.y - 10.0,
                    9.0,
                    32.0,
                    Color::new(0.9, 0.8, 0.7, 1.0),
                );
                draw_circle(
                    pos.x - 9.0,
                    pos.y - 47.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 9.0,
                    pos.y - 47.0,
                    3.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 5.0,
                    pos.y - 35.0,
                    10.0,
                    2.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
            }
            CharacterType::Chef => {
                draw_circle(pos.x, pos.y - 45.0, 30.0, Color::new(0.95, 0.85, 0.8, 1.0));
                draw_rectangle(
                    pos.x - 30.0,
                    pos.y - 20.0,
                    60.0,
                    60.0,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                );
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y + 40.0,
                    40.0,
                    35.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x - 14.0,
                    pos.y + 75.0,
                    11.0,
                    20.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x + 3.0,
                    pos.y + 75.0,
                    11.0,
                    20.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 35.0,
                    pos.y - 10.0,
                    12.0,
                    35.0,
                    Color::new(0.95, 0.85, 0.8, 1.0),
                );
                draw_rectangle(
                    pos.x + 23.0,
                    pos.y - 10.0,
                    12.0,
                    35.0,
                    Color::new(0.95, 0.85, 0.8, 1.0),
                );
                draw_circle(pos.x + 35.0, pos.y, 10.0, Color::new(0.7, 0.7, 0.7, 1.0));
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 75.0,
                    50.0,
                    25.0,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                );
            }
            CharacterType::Librarian => {
                draw_circle(pos.x, pos.y - 45.0, 25.0, Color::new(0.95, 0.9, 0.85, 1.0));
                draw_rectangle(
                    pos.x - 22.0,
                    pos.y - 20.0,
                    44.0,
                    55.0,
                    Color::new(0.5, 0.3, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 17.0,
                    pos.y + 35.0,
                    34.0,
                    35.0,
                    Color::new(0.3, 0.2, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 11.0,
                    pos.y + 70.0,
                    9.0,
                    20.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 70.0,
                    9.0,
                    20.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 27.0,
                    pos.y - 10.0,
                    9.0,
                    30.0,
                    Color::new(0.95, 0.9, 0.85, 1.0),
                );
                draw_rectangle(
                    pos.x + 18.0,
                    pos.y - 10.0,
                    9.0,
                    30.0,
                    Color::new(0.95, 0.9, 0.85, 1.0),
                );
                draw_rectangle(
                    pos.x + 27.0,
                    pos.y - 5.0,
                    15.0,
                    20.0,
                    Color::new(0.2, 0.4, 0.6, 1.0),
                );
                draw_rectangle(
                    pos.x - 15.0,
                    pos.y - 50.0,
                    8.0,
                    8.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x + 7.0,
                    pos.y - 50.0,
                    8.0,
                    8.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                draw_rectangle(
                    pos.x - 17.0,
                    pos.y - 65.0,
                    34.0,
                    8.0,
                    Color::new(0.7, 0.7, 0.7, 1.0),
                );
            }
            CharacterType::Coach => {
                draw_circle(pos.x, pos.y - 45.0, 28.0, Color::new(0.9, 0.75, 0.65, 1.0));
                draw_rectangle(
                    pos.x - 28.0,
                    pos.y - 20.0,
                    56.0,
                    58.0,
                    Color::new(0.2, 0.5, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 12.0,
                    pos.y - 10.0,
                    24.0,
                    4.0,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                );
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y + 38.0,
                    40.0,
                    37.0,
                    Color::new(0.1, 0.3, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 13.0,
                    pos.y + 75.0,
                    11.0,
                    22.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 75.0,
                    11.0,
                    22.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 33.0,
                    pos.y - 10.0,
                    11.0,
                    35.0,
                    Color::new(0.9, 0.75, 0.65, 1.0),
                );
                draw_rectangle(
                    pos.x + 22.0,
                    pos.y - 10.0,
                    11.0,
                    35.0,
                    Color::new(0.9, 0.75, 0.65, 1.0),
                );
                draw_circle(
                    pos.x + 33.0,
                    pos.y - 5.0,
                    8.0,
                    Color::new(0.8, 0.8, 0.8, 1.0),
                );
                draw_rectangle(
                    pos.x - 20.0,
                    pos.y - 68.0,
                    40.0,
                    15.0,
                    Color::new(0.8, 0.1, 0.1, 1.0),
                );
            }
            CharacterType::Bastiaan => {
                draw_circle(pos.x, pos.y - 45.0, 27.0, Color::new(0.95, 0.85, 0.75, 1.0));
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 20.0,
                    50.0,
                    55.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 8.0,
                    pos.y - 15.0,
                    16.0,
                    20.0,
                    Color::new(0.9, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 18.0,
                    pos.y + 35.0,
                    36.0,
                    40.0,
                    Color::new(0.2, 0.2, 0.2, 1.0),
                );
                draw_rectangle(
                    pos.x - 12.0,
                    pos.y + 75.0,
                    10.0,
                    25.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x + 2.0,
                    pos.y + 75.0,
                    10.0,
                    25.0,
                    Color::new(0.1, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 30.0,
                    pos.y - 10.0,
                    10.0,
                    35.0,
                    Color::new(0.95, 0.85, 0.75, 1.0),
                );
                draw_rectangle(
                    pos.x + 20.0,
                    pos.y - 10.0,
                    10.0,
                    35.0,
                    Color::new(0.95, 0.85, 0.75, 1.0),
                );
                draw_rectangle(
                    pos.x + 30.0,
                    pos.y - 8.0,
                    20.0,
                    10.0,
                    Color::new(0.6, 0.3, 0.1, 1.0),
                );
                draw_circle(
                    pos.x - 10.0,
                    pos.y - 48.0,
                    4.0,
                    Color::new(0.8, 0.1, 0.1, 1.0),
                );
                draw_circle(
                    pos.x + 10.0,
                    pos.y - 48.0,
                    4.0,
                    Color::new(0.8, 0.1, 0.1, 1.0),
                );
                draw_rectangle(
                    pos.x - 25.0,
                    pos.y - 70.0,
                    50.0,
                    15.0,
                    Color::new(0.3, 0.3, 0.8, 1.0),
                );
                draw_circle(pos.x, pos.y - 62.0, 10.0, Color::new(0.1, 0.1, 0.1, 1.0));
            }
            CharacterType::KeizerBomTaha => {
                // Plane body - metallic silver
                draw_rectangle(pos.x - 50.0, pos.y - 20.0, 100.0, 35.0, Color::new(0.7, 0.7, 0.75, 1.0));
                draw_rectangle(pos.x - 48.0, pos.y - 18.0, 96.0, 31.0, Color::new(0.8, 0.8, 0.85, 1.0));

                // Cockpit window
                draw_rectangle(pos.x - 30.0, pos.y - 15.0, 25.0, 15.0, Color::new(0.3, 0.5, 0.7, 0.7));

                // Wings
                draw_rectangle(pos.x - 80.0, pos.y, 160.0, 12.0, Color::new(0.65, 0.65, 0.7, 1.0));

                // Keizer head in cockpit
                draw_circle(pos.x - 17.0, pos.y - 8.0, 10.0, Color::new(0.9, 0.8, 0.7, 1.0));
                draw_circle(pos.x - 20.0, pos.y - 10.0, 3.0, Color::new(0.2, 0.2, 0.2, 1.0));
                draw_circle(pos.x - 14.0, pos.y - 10.0, 3.0, Color::new(0.2, 0.2, 0.2, 1.0));

                // Propeller (animated)
                let prop_angle = time * 20.0;
                for i in 0..3 {
                    let angle = prop_angle + (i as f32 * std::f32::consts::PI * 2.0 / 3.0);
                    let px = pos.x + 50.0 + angle.cos() * 25.0;
                    let py = pos.y + angle.sin() * 25.0;
                    draw_line(pos.x + 50.0, pos.y, px, py, 3.0, Color::new(0.5, 0.5, 0.5, 0.8));
                }

                // Bombs hanging underneath
                let bomb_offset = (time * 3.0).sin() * 3.0;
                for i in 0..2 {
                    let bomb_x = pos.x - 20.0 + i as f32 * 40.0;
                    let bomb_y = pos.y + 20.0 + bomb_offset;
                    draw_ellipse(bomb_x, bomb_y, 8.0, 12.0, 0.0, Color::new(0.2, 0.2, 0.2, 1.0));
                    draw_ellipse(bomb_x, bomb_y - 10.0, 3.0, 5.0, 0.0, Color::new(0.8, 0.2, 0.2, 1.0)); // Fuse
                }
            }
        }

        if attack_phase > 0.0 {
            self.render_attack_slash(base_pos, fighter, attack_phase);
        }

        if matches!(state, FighterState::Hitstun) {
            draw_circle_lines(
                pos.x,
                pos.y - 46.0,
                28.0,
                3.0,
                Color::new(1.0, 0.3, 0.3, 0.5),
            );
        }

        if matches!(state, FighterState::Special | FighterState::Super) {
            let aura_alpha = 0.25 + 0.35 * (attack_phase.max(0.1));
            draw_circle_lines(
                pos.x,
                pos.y - 40.0,
                36.0 + attack_phase * 14.0,
                2.0,
                Color::new(0.4, 0.8, 1.0, aura_alpha),
            );
        }
    }

    fn render_attack_slash(&self, pos: Vec2, fighter: &Fighter, phase: f32) {
        if let Some(total) = Self::attack_total_duration(fighter.state) {
            if total <= 0.0 || fighter.attack_timer <= 0.0 {
                return;
            }

            let dir = if fighter.facing >= 0.0 { 1.0 } else { -1.0 };
            let swing = 40.0 + 55.0 * phase;
            let height = 20.0 + 18.0 * phase;
            let alpha = 0.2 + 0.5 * (1.0 - phase);
            let core = Color::new(1.0, 0.85, 0.3, alpha);
            let outline = Color::new(1.0, 0.95, 0.6, alpha + 0.15);

            let base_y = pos.y - 28.0;
            let tip = Vec2::new(pos.x + dir * (swing + 12.0), base_y - height * 0.2);
            let base1 = Vec2::new(pos.x + dir * (swing - 15.0), base_y + height);
            let base2 = Vec2::new(pos.x + dir * (swing - 15.0), base_y - height);

            draw_triangle(base1, base2, tip, core);
            draw_triangle_lines(base1, base2, tip, 2.0, outline);
            draw_circle(tip.x, tip.y, 6.0, outline);
        }
    }

    fn attack_total_duration(state: FighterState) -> Option<f32> {
        match state {
            FighterState::LightAttack => Some(0.25),
            FighterState::HeavyAttack => Some(0.4),
            FighterState::Special => Some(0.55),
            FighterState::Super => Some(0.8),
            _ => None,
        }
    }

    fn attack_phase(state: FighterState, attack_timer: f32) -> f32 {
        match Self::attack_total_duration(state) {
            Some(total) if total > 0.0 && attack_timer > 0.0 => {
                1.0 - (attack_timer / total).clamp(0.0, 1.0)
            }
            _ => 0.0,
        }
    }

    fn render_controls(&self) {
        if self.show_controls {
            let alpha = self.control_fade.max(0.0);
            let header_color = Color::new(1.0, 1.0, 1.0, alpha);
            let detail_color = Color::new(0.85, 0.85, 0.9, alpha);
            let bg_color = Color::new(0.05, 0.05, 0.08, alpha * 0.85);

            let controls = vec![
                ("Strafe", "A / D"),
                ("Depth", "W / S"),
                ("Light", "J"),
                ("Heavy", "K"),
                ("Special", "L"),
                ("Shop", "B"),
                ("Pause", "Esc"),
            ];

            let row_height = 30.0;
            let block_height = controls.len() as f32 * row_height + 24.0;
            let block_width = 280.0;
            let origin_x = screen_width() - block_width - 20.0;
            let origin_y = screen_height() - block_height - 20.0;

            draw_rectangle(origin_x, origin_y, block_width, block_height, bg_color);
            draw_rectangle_lines(
                origin_x,
                origin_y,
                block_width,
                block_height,
                1.5,
                Color::new(1.0, 1.0, 1.0, alpha * 0.4),
            );

            for (index, (label, keys)) in controls.iter().enumerate() {
                let y = origin_y + 20.0 + index as f32 * row_height;
                draw_text(label, origin_x + 12.0, y, 20.0, header_color);
                draw_text(keys, origin_x + 120.0, y, 20.0, detail_color);
            }
        }
    }

    fn render_dialogue(&self) {
        if let Some(ref dialogue) = self.current_dialogue {
            let box_height = 150.0;
            let box_y = screen_height() - box_height - 20.0;

            draw_rectangle(
                20.0,
                box_y,
                screen_width() - 40.0,
                box_height,
                Color::new(0.0, 0.0, 0.0, 0.9),
            );
            draw_rectangle_lines(20.0, box_y, screen_width() - 40.0, box_height, 3.0, WHITE);

            draw_text(&dialogue.speaker, 40.0, box_y + 30.0, 28.0, YELLOW);

            draw_text(&dialogue.dutch, 40.0, box_y + 65.0, 24.0, WHITE);

            draw_text(
                &dialogue.english,
                40.0,
                box_y + 100.0,
                20.0,
                Color::new(0.7, 0.7, 0.7, 1.0),
            );
        }
    }

    fn render_dialogue_choice(&self) {
        // Overlay to darken the screen
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        // Title
        let title = "BOSSES DEFEATED!";
        let title_size = 52.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() * 0.5 - title_dims.width * 0.5,
            screen_height() * 0.25,
            title_size,
            Color::new(1.0, 0.8, 0.0, 1.0),
        );

        // Question
        let question = "What should we do now?";
        let question_dims = measure_text(question, None, 32, 1.0);
        draw_text(
            question,
            screen_width() * 0.5 - question_dims.width * 0.5,
            screen_height() * 0.35,
            32.0,
            WHITE,
        );

        // Choice options
        let choices = [
            "Build a party to celebrate!",
            "Repair the school together.",
            "Go home and rest.",
        ];

        let choice_y_start = screen_height() * 0.45;
        let choice_spacing = 70.0;

        for (i, choice) in choices.iter().enumerate() {
            let y = choice_y_start + i as f32 * choice_spacing;
            let is_selected = i == self.dialogue_choice_selected;

            // Draw selection box
            if is_selected {
                draw_rectangle(
                    screen_width() * 0.25 - 20.0,
                    y - 35.0,
                    screen_width() * 0.5 + 40.0,
                    60.0,
                    Color::new(0.2, 0.4, 0.8, 0.5),
                );
                draw_rectangle_lines(
                    screen_width() * 0.25 - 20.0,
                    y - 35.0,
                    screen_width() * 0.5 + 40.0,
                    60.0,
                    3.0,
                    Color::new(0.4, 0.6, 1.0, 1.0),
                );

                // Arrow indicator
                draw_text(
                    "▶",
                    screen_width() * 0.25 - 50.0,
                    y + 5.0,
                    36.0,
                    Color::new(1.0, 0.8, 0.0, 1.0),
                );
            }

            // Draw choice text
            let text_color = if is_selected {
                Color::new(1.0, 1.0, 0.8, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0)
            };

            draw_text(
                choice,
                screen_width() * 0.25,
                y + 5.0,
                28.0,
                text_color,
            );
        }

        // Instructions
        let instruction = "Use W/S or UP/DOWN to select, ENTER to confirm";
        let instruction_dims = measure_text(instruction, None, 20, 1.0);
        draw_text(
            instruction,
            screen_width() * 0.5 - instruction_dims.width * 0.5,
            screen_height() * 0.85,
            20.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
    }
}
