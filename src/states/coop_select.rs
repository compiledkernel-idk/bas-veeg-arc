use crate::states::{State, StateType};
use crate::data::characters::{Character, CharacterId, CHARACTERS};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputDeviceType {
    KeyboardMouse,
    Gamepad(u8),
}

/// Co-op character selection state - allows 2-4 players to join and pick characters
pub struct CoopSelectState {
    player_slots: [Option<PlayerSelection>; 4],
    active_slot: usize,
    ready_count: usize,
    countdown_timer: Option<f32>,
    transition_to: Option<StateType>,
}

#[derive(Clone)]
struct PlayerSelection {
    selected_character: usize,
    character_id: CharacterId,
    is_ready: bool,
    input_device: InputDeviceType,
    color: Color,
}

const PLAYER_COLORS: [Color; 4] = [
    Color::new(0.2, 0.5, 1.0, 1.0), // Blue (P1)
    Color::new(1.0, 0.2, 0.2, 1.0), // Red (P2)
    Color::new(0.2, 1.0, 0.2, 1.0), // Green (P3)
    Color::new(1.0, 1.0, 0.2, 1.0), // Yellow (P4)
];

impl CoopSelectState {
    pub fn new() -> Self {
        // Both players join automatically for local 2-player
        let mut slots = [None, None, None, None];
        slots[0] = Some(PlayerSelection {
            selected_character: 0,
            character_id: CHARACTERS[0].id,
            is_ready: false,
            input_device: InputDeviceType::KeyboardMouse, // WASD + J/K
            color: PLAYER_COLORS[0],
        });
        slots[1] = Some(PlayerSelection {
            selected_character: 1,
            character_id: CHARACTERS[1].id,
            is_ready: false,
            input_device: InputDeviceType::Gamepad(0), // Arrow keys + Enter/Backspace
            color: PLAYER_COLORS[1],
        });

        Self {
            player_slots: slots,
            active_slot: 0,
            ready_count: 0,
            countdown_timer: None,
            transition_to: None,
        }
    }

    fn add_player(&mut self, slot: usize) {
        if self.player_slots[slot].is_none() && slot < 4 {
            self.player_slots[slot] = Some(PlayerSelection {
                selected_character: 0,
                character_id: CHARACTERS[0].id,
                is_ready: false,
                input_device: InputDeviceType::KeyboardMouse,
                color: PLAYER_COLORS[slot],
            });
        }
    }

    fn remove_player(&mut self, slot: usize) {
        if let Some(player) = &self.player_slots[slot] {
            if !player.is_ready {
                self.player_slots[slot] = None;
            }
        }
    }

    pub fn get_player_configs(&self) -> Vec<(CharacterId, InputDeviceType)> {
        self.player_slots
            .iter()
            .filter_map(|slot| {
                slot.as_ref().map(|p| (p.character_id, p.input_device))
            })
            .collect()
    }
}

impl State for CoopSelectState {
    fn enter(&mut self) {
        self.ready_count = 0;
        self.countdown_timer = None;
        self.transition_to = None;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        // Update countdown if active
        if let Some(timer) = &mut self.countdown_timer {
            *timer -= dt;
            if *timer <= 0.0 {
                // Store co-op player configurations
                let player_chars: Vec<crate::data::CharacterId> = self.player_slots
                    .iter()
                    .filter_map(|slot| slot.as_ref().map(|p| p.character_id))
                    .collect();
                crate::data::set_coop_players(player_chars);

                self.transition_to = Some(StateType::CoopMode);
            }
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        let sw = screen_width();
        let sh = screen_height();

        // Title
        let title = "CO-OP MODE - CHARACTER SELECT";
        let title_size = 40.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            sw * 0.5 - title_dims.width * 0.5,
            60.0,
            title_size,
            YELLOW,
        );

        // Instructions
        let instructions = if self.countdown_timer.is_some() {
            "STARTING GAME..."
        } else {
            "P1: A/D=Select | J=Ready | K=Unready  |  P2: Arrows=Select | ENTER=Ready | BACKSPACE=Unready"
        };
        let inst_size = 20.0;
        let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
        draw_text(
            instructions,
            sw * 0.5 - inst_dims.width * 0.5,
            100.0,
            inst_size,
            WHITE,
        );

        // Player slots in a 2x2 grid
        let slot_width = 400.0;
        let slot_height = 500.0;
        let spacing_x = 50.0;
        let spacing_y = 30.0;
        let start_x = (sw - (slot_width * 2.0 + spacing_x)) * 0.5;
        let start_y = 140.0;

        for slot_idx in 0..4 {
            let col = slot_idx % 2;
            let row = slot_idx / 2;
            let x = start_x + col as f32 * (slot_width + spacing_x);
            let y = start_y + row as f32 * (slot_height + spacing_y);

            if let Some(player) = &self.player_slots[slot_idx] {
                // Active player slot
                let border_color = if player.is_ready {
                    GREEN
                } else if slot_idx == self.active_slot {
                    player.color
                } else {
                    GRAY
                };

                draw_rectangle_lines(x, y, slot_width, slot_height, 4.0, border_color);

                // Player number
                let p_text = format!("PLAYER {}", slot_idx + 1);
                draw_text(&p_text, x + 10.0, y + 30.0, 25.0, player.color);

                // Status
                let status = if player.is_ready {
                    "READY!"
                } else {
                    "Selecting..."
                };
                draw_text(status, x + 10.0, y + 55.0, 20.0, if player.is_ready { GREEN } else { WHITE });

                // Character preview
                let char_data = Character::get_by_id(player.character_id);
                let char_y = y + 120.0;

                // Draw simple character representation
                let center_x = x + slot_width * 0.5;

                // Head
                draw_circle(center_x, char_y, 40.0, player.color);

                // Body
                draw_rectangle(center_x - 30.0, char_y + 30.0, 60.0, 80.0, player.color);

                // Arms
                draw_rectangle(center_x - 60.0, char_y + 40.0, 25.0, 60.0, player.color);
                draw_rectangle(center_x + 35.0, char_y + 40.0, 25.0, 60.0, player.color);

                // Legs
                draw_rectangle(center_x - 25.0, char_y + 110.0, 20.0, 70.0, player.color);
                draw_rectangle(center_x + 5.0, char_y + 110.0, 20.0, 70.0, player.color);

                // Character info
                let info_y = y + 350.0;
                draw_text(char_data.name, x + 10.0, info_y, 30.0, WHITE);
                draw_text(&format!("Ability: {}", char_data.ability_name), x + 10.0, info_y + 30.0, 18.0, YELLOW);

                // Cooldown info
                draw_text(
                    &format!("Duration: {:.0}s | Cooldown: {:.0}s", char_data.duration, char_data.cooldown),
                    x + 10.0,
                    info_y + 50.0,
                    16.0,
                    GRAY,
                );

                // Effects
                let mut effect_y = info_y + 75.0;
                for effect in char_data.effects {
                    let effect_text = match effect {
                        crate::data::characters::AbilityEffect::DamageBoost(mult) => {
                            format!("DMG x{:.1}", mult)
                        }
                        crate::data::characters::AbilityEffect::HealthBoost(hp) => {
                            format!("+{:.0} HP", hp)
                        }
                        crate::data::characters::AbilityEffect::SpeedBoost(mult) => {
                            format!("SPD x{:.1}", mult)
                        }
                        crate::data::characters::AbilityEffect::SplashDamage(dmg, rad) => {
                            format!("AOE {:.0}dmg/{:.0}r", dmg, rad)
                        }
                        crate::data::characters::AbilityEffect::FireDamage(dps, dur) => {
                            format!("FIRE {:.0}dps {:.0}s", dps, dur)
                        }
                        _ => "Special".to_string(),
                    };
                    draw_text(&effect_text, x + 10.0, effect_y, 15.0, Color::new(0.8, 0.8, 0.8, 1.0));
                    effect_y += 18.0;
                }
            } else {
                // Empty slot
                draw_rectangle_lines(x, y, slot_width, slot_height, 2.0, DARKGRAY);

                let empty_text = format!("PLAYER {} - PRESS SPACE", slot_idx + 1);
                let text_dims = measure_text(&empty_text, None, 20, 1.0);
                draw_text(
                    &empty_text,
                    x + slot_width * 0.5 - text_dims.width * 0.5,
                    y + slot_height * 0.5,
                    20.0,
                    DARKGRAY,
                );
            }
        }

        // Ready status
        let active_players: usize = self.player_slots.iter().filter(|p| p.is_some()).count();
        let ready_text = format!("Ready: {}/{}", self.ready_count, active_players);
        draw_text(&ready_text, sw * 0.5 - 60.0, sh - 50.0, 25.0, if self.ready_count >= 2 && self.ready_count == active_players { GREEN } else { WHITE });

        // Countdown
        if let Some(timer) = self.countdown_timer {
            let countdown_text = format!("Starting in {:.0}...", timer.ceil());
            let cd_dims = measure_text(&countdown_text, None, 40, 1.0);
            draw_text(
                &countdown_text,
                sw * 0.5 - cd_dims.width * 0.5,
                sh * 0.5,
                40.0,
                YELLOW,
            );
        }
    }

    fn handle_input(&mut self) {
        if self.countdown_timer.is_some() {
            return; // Lock input during countdown
        }

        // Player 1 controls (WASD + J/K)
        if let Some(player1) = &mut self.player_slots[0] {
            if !player1.is_ready {
                // Character selection
                if is_key_pressed(KeyCode::A) {
                    if player1.selected_character > 0 {
                        player1.selected_character -= 1;
                    } else {
                        player1.selected_character = CHARACTERS.len() - 1;
                    }
                    player1.character_id = CHARACTERS[player1.selected_character].id;
                }

                if is_key_pressed(KeyCode::D) {
                    player1.selected_character = (player1.selected_character + 1) % CHARACTERS.len();
                    player1.character_id = CHARACTERS[player1.selected_character].id;
                }

                // Ready up
                if is_key_pressed(KeyCode::J) {
                    player1.is_ready = true;
                    self.ready_count += 1;
                    // Check countdown inline
                    let active_players = self.player_slots.iter().filter(|p| p.is_some()).count();
                    if self.ready_count >= 2 && self.ready_count == active_players {
                        self.countdown_timer = Some(3.0);
                    }
                }
            } else {
                // Unready
                if is_key_pressed(KeyCode::K) {
                    player1.is_ready = false;
                    self.ready_count = self.ready_count.saturating_sub(1);
                    self.countdown_timer = None;
                }
            }
        }

        // Player 2 controls (Arrow keys + Enter/Backspace)
        if let Some(player2) = &mut self.player_slots[1] {
            if !player2.is_ready {
                // Character selection
                if is_key_pressed(KeyCode::Left) {
                    if player2.selected_character > 0 {
                        player2.selected_character -= 1;
                    } else {
                        player2.selected_character = CHARACTERS.len() - 1;
                    }
                    player2.character_id = CHARACTERS[player2.selected_character].id;
                }

                if is_key_pressed(KeyCode::Right) {
                    player2.selected_character = (player2.selected_character + 1) % CHARACTERS.len();
                    player2.character_id = CHARACTERS[player2.selected_character].id;
                }

                // Ready up
                if is_key_pressed(KeyCode::Enter) {
                    player2.is_ready = true;
                    self.ready_count += 1;
                    // Check countdown inline
                    let active_players = self.player_slots.iter().filter(|p| p.is_some()).count();
                    if self.ready_count >= 2 && self.ready_count == active_players {
                        self.countdown_timer = Some(3.0);
                    }
                }
            } else {
                // Unready
                if is_key_pressed(KeyCode::Backspace) {
                    player2.is_ready = false;
                    self.ready_count = self.ready_count.saturating_sub(1);
                    self.countdown_timer = None;
                }
            }
        }

        // Cancel
        if is_key_pressed(KeyCode::Escape) {
            self.transition_to = Some(StateType::Menu);
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
