use crate::ecs::sys::{CombatSystem, System};
use crate::data::characters::{AbilityState, Character, CharacterId};
use crate::ecs::comp::*;
use crate::ecs::entity::EntityId;
use crate::ecs::world::World;
use crate::states::{State, StateType};
use macroquad::prelude::*;
use std::collections::HashMap;

struct DialogueLine {
    speaker: String,
    dutch: String,
    english: String,
    duration: f32,
}

pub struct EndlessState {
    world: World,
    combat_system: CombatSystem,
    player_entity: Option<EntityId>,
    enemy_entities: Vec<EntityId>,
    camera_position: Vec2,
    camera_target: Vec2,
    camera_zoom: f32,

    // Endless mode specific
    wave: u32,
    score: u32,
    enemies_per_wave: u32,
    enemies_spawned: u32,
    enemies_defeated: u32,
    wave_spawn_timer: f32,
    wave_spawn_interval: f32,
    difficulty_multiplier: f32,

    // UI
    current_dialogue: Option<DialogueLine>,
    dialogue_timer: f32,
    dialogue_queue: Vec<DialogueLine>,
    show_wave_banner: bool,
    wave_banner_timer: f32,

    // Player state
    selected_character: CharacterId,
    ability_state: AbilityState,
    player_health_multiplier: f32,
    player_attack_multiplier: f32,

    // Gameplay state
    game_over: bool,
    transition_to: Option<StateType>,
    burning_enemies: HashMap<EntityId, (f32, f32)>,

    // Auto-attack system
    auto_attack_timer: f32,
    auto_attack_delay: f32,
}

impl EndlessState {
    pub fn new() -> Self {
        let world = World::new();
        let combat_system = CombatSystem::new();
        let selected_character = crate::data::get_selected_character();

        Self {
            world,
            combat_system,
            player_entity: None,
            enemy_entities: Vec::new(),
            camera_position: Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
            camera_target: Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
            camera_zoom: 1.0,

            // Start with wave 1
            wave: 1,
            score: 0,
            enemies_per_wave: 3,
            enemies_spawned: 0,
            enemies_defeated: 0,
            wave_spawn_timer: 0.0,
            wave_spawn_interval: 2.0,
            difficulty_multiplier: 1.0,

            // UI
            current_dialogue: None,
            dialogue_timer: 0.0,
            dialogue_queue: Vec::new(),
            show_wave_banner: false,
            wave_banner_timer: 0.0,

            // Player state
            selected_character,
            ability_state: AbilityState::new(selected_character),
            player_health_multiplier: 1.0,
            player_attack_multiplier: 1.0,

            // Gameplay state
            game_over: false,
            transition_to: None,
            burning_enemies: HashMap::new(),

            // Auto-attack system
            auto_attack_timer: 0.0,
            auto_attack_delay: 0.25,
        }
    }

    fn spawn_player(&mut self) {
        let player = self.world.create_entity();
        let character_type = self.selected_character.to_character_type();

        self.world.add_component(
            player,
            Transform {
                position: Vec2::new(screen_width() * 0.5, screen_height() - 100.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            },
        );

        self.world.add_component(
            player,
            Velocity {
                linear: Vec2::ZERO,
                angular: 0.0,
            },
        );

        self.world.add_component(
            player,
            Health {
                current: 100.0 * self.player_health_multiplier,
                maximum: 100.0 * self.player_health_multiplier,
                armor: 0.0,
            },
        );

        self.world.add_component(
            player,
            Fighter {
                character_type,
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

        self.world.add_component(
            player,
            CollisionBox {
                offset: Vec2::ZERO,
                size: Vec2::new(60.0, 80.0),
                active: true,
            },
        );

        self.player_entity = Some(player);
    }

    fn spawn_wave(&mut self) {
        self.wave_spawn_timer = 0.0;
        self.enemies_spawned = 0;
        self.enemies_defeated = 0;
        self.show_wave_banner = true;
        self.wave_banner_timer = 3.0;

        // Scale difficulty
        self.enemies_per_wave = 3 + (self.wave / 3);
        self.difficulty_multiplier = 1.0 + (self.wave as f32 * 0.2);

        // Show wave announcement
        self.dialogue_queue.push(DialogueLine {
            speaker: "System".to_string(),
            dutch: format!("Golf {}!", self.wave),
            english: format!("Wave {}!", self.wave),
            duration: 2.0,
        });
    }

    fn spawn_enemy(&mut self) {
        let enemy = self.world.create_entity();

        // Random enemy type based on wave
        let enemy_types = [
            CharacterType::Wolters,
            CharacterType::PrefectA,
            CharacterType::PrefectB,
            CharacterType::Chef,
            CharacterType::Librarian,
        ];

        let enemy_type = if self.wave < 5 {
            enemy_types[rand::gen_range(0, 2)].clone()
        } else if self.wave < 10 {
            enemy_types[rand::gen_range(0, 4)].clone()
        } else {
            enemy_types[rand::gen_range(0, enemy_types.len())].clone()
        };

        // Random spawn position at top of screen
        let spawn_x = rand::gen_range(100.0, screen_width() - 100.0);

        self.world.add_component(
            enemy,
            Transform {
                position: Vec2::new(spawn_x, 100.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            },
        );

        self.world.add_component(
            enemy,
            Velocity {
                linear: Vec2::ZERO,
                angular: 0.0,
            },
        );

        let health_base = match enemy_type {
            CharacterType::Wolters => 30.0,
            CharacterType::PrefectA | CharacterType::PrefectB => 40.0,
            CharacterType::Chef => 50.0,
            CharacterType::Librarian => 35.0,
            _ => 40.0,
        };

        self.world.add_component(
            enemy,
            Health {
                current: health_base * self.difficulty_multiplier,
                maximum: health_base * self.difficulty_multiplier,
                armor: 0.0,
            },
        );

        self.world.add_component(
            enemy,
            Fighter {
                character_type: enemy_type,
                state: FighterState::Idle,
                combo_counter: 0,
                meter: 0.0,
                max_meter: 100.0,
                hitstun: 0.0,
                blockstun: 0.0,
                invulnerable: false,
                facing: 1.0,
                attack_timer: 0.0,
                team: Team::Enemy,
                consecutive_hits_taken: 0,
                hit_decay_timer: 0.0,
            },
        );

        self.world.add_component(
            enemy,
            CollisionBox {
                offset: Vec2::ZERO,
                size: Vec2::new(60.0, 80.0),
                active: true,
            },
        );

        self.world.add_component(
            enemy,
            AIController {
                behavior: AIBehavior::Aggressive,
                target_entity: self.player_entity,
                state_timer: 0.0,
                reaction_delay: 0.2 + rand::gen_range(0.0, 0.3),
                difficulty: self.difficulty_multiplier,
            },
        );

        self.enemy_entities.push(enemy);
        self.enemies_spawned += 1;
    }

    fn update_wave_spawning(&mut self, dt: f32) {
        if self.enemies_spawned < self.enemies_per_wave {
            self.wave_spawn_timer += dt;
            if self.wave_spawn_timer >= self.wave_spawn_interval {
                self.spawn_enemy();
            }
        }
    }

    fn check_wave_complete(&mut self) {
        if self.enemies_defeated >= self.enemies_per_wave && self.enemy_entities.is_empty() {
            // Wave complete!
            self.score += 100 * self.wave;
            self.wave += 1;

            // Give player a small health bonus
            if let Some(player_entity) = self.player_entity {
                if let Some(health) = self.world.get_component_mut::<Health>(player_entity) {
                    let heal_amount = 20.0;
                    health.current = (health.current + heal_amount).min(health.maximum);
                }
            }

            // Start next wave
            self.spawn_wave();
        }
    }
}

impl State for EndlessState {
    fn enter(&mut self) {
        self.spawn_player();
        self.spawn_wave();
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        // Update dialogue
        if self.current_dialogue.is_none() && !self.dialogue_queue.is_empty() {
            self.current_dialogue = self.dialogue_queue.pop();
            self.dialogue_timer = 0.0;
        }

        if let Some(ref dialogue) = self.current_dialogue {
            self.dialogue_timer += dt;
            if self.dialogue_timer >= dialogue.duration {
                self.current_dialogue = None;
            } else {
                return; // Freeze game during dialogue
            }
        }

        if self.show_wave_banner {
            self.wave_banner_timer -= dt;
            if self.wave_banner_timer <= 0.0 {
                self.show_wave_banner = false;
            }
        }

        if self.game_over {
            return;
        }

        // Update wave spawning
        self.update_wave_spawning(dt);

        // Update ability state
        self.ability_state.update(dt);

        // Update auto-attack timer
        if self.auto_attack_timer > 0.0 {
            self.auto_attack_timer -= dt;
        }

        // Update burning enemies
        let mut enemies_to_remove = Vec::new();
        for (entity, (remaining_time, dps)) in self.burning_enemies.iter_mut() {
            *remaining_time -= dt;

            if let Some(health) = self.world.get_component_mut::<Health>(*entity) {
                health.current -= *dps * dt;
            }

            if *remaining_time <= 0.0 {
                enemies_to_remove.push(*entity);
            }
        }

        for entity in enemies_to_remove {
            self.burning_enemies.remove(&entity);
        }

        // Update combat
        let ability_damage_mult = self.ability_state.get_damage_multiplier();
        let total_damage_mult = self.player_attack_multiplier * ability_damage_mult;
        self.combat_system.set_player_attack_multiplier(total_damage_mult);
        self.combat_system.update(&mut self.world, dt);

        // Remove dead enemies
        let mut dead_enemies = Vec::new();
        for &entity in &self.enemy_entities {
            if let Some(health) = self.world.get_component::<Health>(entity) {
                if health.current <= 0.0 {
                    dead_enemies.push(entity);
                    self.enemies_defeated += 1;
                    self.score += 10 * self.wave;
                }
            }
        }

        for entity in dead_enemies {
            self.enemy_entities.retain(|&e| e != entity);
            self.world.destroy_entity(entity);
        }

        // Check if player died
        if let Some(player_entity) = self.player_entity {
            if let Some(health) = self.world.get_component::<Health>(player_entity) {
                if health.current <= 0.0 {
                    self.game_over = true;
                    self.dialogue_queue.push(DialogueLine {
                        speaker: "System".to_string(),
                        dutch: format!("Spel Voorbij! Score: {}", self.score),
                        english: format!("Game Over! Score: {}", self.score),
                        duration: 3.0,
                    });
                }
            }
        }

        // Check wave completion
        self.check_wave_complete();

        // Update camera
        if let Some(player_entity) = self.player_entity {
            if let Some(transform) = self.world.get_component::<Transform>(player_entity) {
                self.camera_target = transform.position;
            }
        }

        self.camera_position = self.camera_position.lerp(self.camera_target, 0.1);
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.1, 0.05, 0.15, 1.0));

        // Render player
        if let Some(player_entity) = self.player_entity {
            self.render_entity(player_entity);
        }

        // Render enemies
        for &enemy_entity in &self.enemy_entities {
            self.render_entity(enemy_entity);
        }

        // Render UI
        self.render_ui();

        // Render dialogue
        if let Some(ref dialogue) = self.current_dialogue {
            self.render_dialogue(dialogue);
        }

        // Render wave banner
        if self.show_wave_banner {
            let banner_text = format!("WAVE {}", self.wave);
            let font_size = 60.0;
            let text_dims = measure_text(&banner_text, None, font_size as u16, 1.0);

            draw_rectangle(
                0.0,
                screen_height() * 0.4,
                screen_width(),
                100.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );

            draw_text(
                &banner_text,
                screen_width() * 0.5 - text_dims.width * 0.5,
                screen_height() * 0.5,
                font_size,
                YELLOW,
            );
        }

        // Game over screen
        if self.game_over {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.7),
            );

            let game_over_text = "GAME OVER";
            let font_size = 80.0;
            let text_dims = measure_text(game_over_text, None, font_size as u16, 1.0);
            draw_text(
                game_over_text,
                screen_width() * 0.5 - text_dims.width * 0.5,
                screen_height() * 0.4,
                font_size,
                RED,
            );

            let score_text = format!("Final Score: {}", self.score);
            let font_size = 40.0;
            let text_dims = measure_text(&score_text, None, font_size as u16, 1.0);
            draw_text(
                &score_text,
                screen_width() * 0.5 - text_dims.width * 0.5,
                screen_height() * 0.5,
                font_size,
                WHITE,
            );

            let waves_text = format!("Waves Survived: {}", self.wave - 1);
            let text_dims = measure_text(&waves_text, None, font_size as u16, 1.0);
            draw_text(
                &waves_text,
                screen_width() * 0.5 - text_dims.width * 0.5,
                screen_height() * 0.55,
                font_size,
                WHITE,
            );

            draw_text(
                "Press ESC to return to menu",
                screen_width() * 0.5 - 150.0,
                screen_height() * 0.7,
                25.0,
                GRAY,
            );
        }
    }

    fn handle_input(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Escape) {
                self.transition_to = Some(StateType::Menu);
            }
            return;
        }

        if let Some(player_entity) = self.player_entity {
            if let Some(fighter) = self.world.get_component::<Fighter>(player_entity) {
                if fighter.state != FighterState::Hitstun
                    && fighter.state != FighterState::Blockstun
                    && fighter.state != FighterState::KnockedDown
                {
                    let mut velocity = Vec2::ZERO;

                    // Movement
                    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                        velocity.x = -300.0;
                    }
                    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                        velocity.x = 300.0;
                    }
                    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                        velocity.y = -300.0;
                    }
                    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                        velocity.y = 300.0;
                    }

                    // Apply speed boost if ability is active
                    let speed_mult = self.ability_state.get_speed_multiplier();
                    velocity *= speed_mult;

                    if let Some(vel) = self.world.get_component_mut::<Velocity>(player_entity) {
                        vel.linear = velocity;
                    }

                    // Combat actions - auto-attack on hold
                    let mut new_state = None;

                    // Auto-attack system - hold button for continuous attacks
                    if is_key_down(KeyCode::J) || is_key_down(KeyCode::K) || is_key_down(KeyCode::L) {
                        // Check if it's time to attack again
                        if self.auto_attack_timer <= 0.0 {
                            if is_key_down(KeyCode::J) {
                                new_state = Some(FighterState::LightAttack);
                            } else if is_key_down(KeyCode::K) {
                                new_state = Some(FighterState::HeavyAttack);
                            } else if is_key_down(KeyCode::L) {
                                new_state = Some(FighterState::Special);
                            }

                            // Reset timer for next attack
                            if new_state.is_some() {
                                self.auto_attack_timer = self.auto_attack_delay;
                            }
                        }
                    } else {
                        // No attack button held - reset timer
                        self.auto_attack_timer = 0.0;
                    }

                    // Ability activation
                    if is_key_pressed(KeyCode::E) {
                        if self.ability_state.can_activate() {
                            let voice_line = self.ability_state.activate();
                            if !voice_line.is_empty() {
                                // Check if this is Jad's special KFC Rage ability
                                if self.selected_character == CharacterId::Jad {
                                    // Jad's special dialogue sequence
                                    self.dialogue_queue.push(DialogueLine {
                                        speaker: "Jad".to_string(),
                                        dutch: "KFC RAGE!".to_string(),
                                        english: "KFC RAGE!".to_string(),
                                        duration: 2.0,
                                    });
                                    self.dialogue_queue.push(DialogueLine {
                                        speaker: "Jad".to_string(),
                                        dutch: "nu ben ik boos".to_string(),
                                        english: "now I'm angry".to_string(),
                                        duration: 1.5,
                                    });
                                    self.dialogue_queue.push(DialogueLine {
                                        speaker: "Umut".to_string(),
                                        dutch: "typisch".to_string(),
                                        english: "typical".to_string(),
                                        duration: 1.5,
                                    });
                                    self.dialogue_queue.push(DialogueLine {
                                        speaker: "Jad".to_string(),
                                        dutch: "ik eet".to_string(),
                                        english: "I eat".to_string(),
                                        duration: 1.5,
                                    });
                                    self.dialogue_queue.reverse();
                                } else {
                                    // Regular ability activation dialogue
                                    let character = Character::get_by_id(self.selected_character);
                                    self.dialogue_queue.push(DialogueLine {
                                        speaker: character.name.to_string(),
                                        dutch: voice_line.to_string(),
                                        english: voice_line.to_string(),
                                        duration: 2.0,
                                    });
                                }

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
                                                        enemy_health.current -= damage;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Apply fire damage if applicable (Nitin's ability)
                                if let Some((dps, duration)) = self.ability_state.get_fire_damage() {
                                    // Apply burning to all enemies
                                    for &enemy_entity in &self.enemy_entities {
                                        self.burning_enemies.insert(enemy_entity, (duration, dps));
                                    }
                                }
                            }
                        }
                    }

                    if let Some(new_state) = new_state {
                        if let Some(fighter) = self.world.get_component_mut::<Fighter>(player_entity) {
                            fighter.state = new_state;
                            fighter.attack_timer = 0.3;
                        }
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.transition_to = Some(StateType::Menu);
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}

impl EndlessState {
    fn render_entity(&self, entity: EntityId) {
        let transform = self.world.get_component::<Transform>(entity);
        let fighter = self.world.get_component::<Fighter>(entity);
        let health = self.world.get_component::<Health>(entity);

        if let (Some(transform), Some(fighter)) = (transform, fighter) {
            let color = match fighter.team {
                Team::Player => BLUE,
                Team::Enemy => RED,
                Team::Ally => GREEN,
            };

            draw_rectangle(
                transform.position.x - 30.0,
                transform.position.y - 40.0,
                60.0,
                80.0,
                color,
            );

            // Draw health bar
            if let Some(health) = health {
                let health_percent = health.current / health.maximum;
                draw_rectangle(
                    transform.position.x - 30.0,
                    transform.position.y - 55.0,
                    60.0 * health_percent,
                    5.0,
                    GREEN,
                );
                draw_rectangle_lines(
                    transform.position.x - 30.0,
                    transform.position.y - 55.0,
                    60.0,
                    5.0,
                    1.0,
                    WHITE,
                );
            }
        }
    }

    fn render_ui(&self) {
        // Wave and score display
        let wave_text = format!("Wave: {}", self.wave);
        draw_text(&wave_text, 20.0, 40.0, 30.0, WHITE);

        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 20.0, 70.0, 30.0, WHITE);

        let enemies_text = format!("Enemies: {}/{}",
            self.enemies_per_wave - self.enemies_defeated,
            self.enemies_per_wave
        );
        draw_text(&enemies_text, 20.0, 100.0, 25.0, YELLOW);

        // Player health
        if let Some(player_entity) = self.player_entity {
            if let Some(health) = self.world.get_component::<Health>(player_entity) {
                let health_percent = health.current / health.maximum;
                let bar_width = 200.0;
                let bar_height = 20.0;
                let bar_x = 20.0;
                let bar_y = 130.0;

                draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::new(0.3, 0.0, 0.0, 1.0));
                draw_rectangle(bar_x, bar_y, bar_width * health_percent, bar_height, RED);
                draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);

                let health_text = format!("{:.0}/{:.0}", health.current, health.maximum);
                draw_text(&health_text, bar_x + 5.0, bar_y + 15.0, 15.0, WHITE);
            }
        }

        // Ability display
        let character = Character::get_by_id(self.selected_character);
        let ability_y = 160.0;

        draw_text("Ability:", 20.0, ability_y, 18.0, GRAY);

        let ability_text = if self.ability_state.active {
            format!("{} [{:.1}s]", character.ability_name, self.ability_state.active_time)
        } else if self.ability_state.cooldown_time > 0.0 {
            format!("{} [{:.1}s]", character.ability_name, self.ability_state.cooldown_time)
        } else {
            format!("{} [E]", character.ability_name)
        };

        draw_text(&ability_text, 80.0, ability_y, 18.0, WHITE);
    }

    fn render_dialogue(&self, dialogue: &DialogueLine) {
        let box_height = 120.0;
        let box_y = screen_height() - box_height - 20.0;

        // Background
        draw_rectangle(
            20.0,
            box_y,
            screen_width() - 40.0,
            box_height,
            Color::new(0.0, 0.0, 0.0, 0.9),
        );

        // Border
        draw_rectangle_lines(
            20.0,
            box_y,
            screen_width() - 40.0,
            box_height,
            3.0,
            WHITE,
        );

        // Speaker
        draw_text(
            &dialogue.speaker,
            40.0,
            box_y + 30.0,
            25.0,
            YELLOW,
        );

        // Text (Dutch)
        draw_text(
            &dialogue.dutch,
            40.0,
            box_y + 60.0,
            20.0,
            WHITE,
        );

        // Text (English) - smaller
        draw_text(
            &dialogue.english,
            40.0,
            box_y + 85.0,
            16.0,
            GRAY,
        );
    }
}