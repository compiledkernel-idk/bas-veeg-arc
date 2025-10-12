use crate::data::{Character, CharacterId, CHARACTERS};
use crate::states::{State, StateType};
use macroquad::prelude::*;

pub struct CharacterSelectState {
    selected_index: usize,
    selected_character: Option<CharacterId>,
    transition_to: Option<StateType>,
    show_details: bool,
    animation_time: f32,
    hover_pulse: f32,
}

impl CharacterSelectState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            selected_character: None,
            transition_to: None,
            show_details: true, // Always show details now
            animation_time: 0.0,
            hover_pulse: 0.0,
        }
    }

    pub fn get_selected_character(&self) -> Option<CharacterId> {
        self.selected_character
    }

    fn get_character_color(id: CharacterId) -> Color {
        match id {
            CharacterId::Berkay => Color::new(1.0, 0.5, 0.0, 1.0), // Orange
            CharacterId::Luca => Color::new(0.3, 0.6, 1.0, 1.0),   // Blue
            CharacterId::Gefferinho => Color::new(1.0, 0.2, 0.2, 1.0), // Red
            CharacterId::Bas => Color::new(0.0, 1.0, 0.5, 1.0),    // Green
            CharacterId::Hadi => Color::new(1.0, 0.8, 0.0, 1.0),   // Gold
            CharacterId::Nitin => Color::new(1.0, 0.3, 0.0, 1.0),  // Fire Orange
            CharacterId::PalaBaba => Color::new(0.8, 0.0, 0.2, 1.0), // Crimson Red (Turkish flag inspired)
        }
    }

    fn get_character_description(id: CharacterId) -> &'static str {
        match id {
            CharacterId::Berkay => "Unstoppable force with kebab power. Balanced fighter with solid offense and defense.",
            CharacterId::Luca => "The winter arc warrior. Pure offensive power with devastating damage output.",
            CharacterId::Gefferinho => "Enraged teacher's nightmare. Well-rounded fighter with speed and power.",
            CharacterId::Bas => "Master of the legendary Bas Veeg technique. AOE specialist who controls the battlefield.",
            CharacterId::Hadi => "Lightning fast from Dubai Emirates. Speed demon who strikes before you can react.",
            CharacterId::Nitin => "Sets the competition ablaze. DOT specialist with burning passion.",
            CharacterId::PalaBaba => "THE ULTIMATE FIGHTER. Turkish powerhouse with unmatched raw power.",
        }
    }

    fn draw_character_portrait(
        &self,
        x: f32,
        y: f32,
        size: f32,
        character: &Character,
        is_selected: bool,
    ) {
        let time = self.animation_time;
        let pulse = if is_selected { self.hover_pulse } else { 1.0 };
        let char_color = Self::get_character_color(character.id);

        // Draw character "figure" - stylized representation
        let figure_y = y + size * 0.3;

        // Glow effect for selected
        if is_selected {
            for i in 0..3 {
                let glow_size = size * 0.35 + i as f32 * 15.0 * pulse;
                let alpha = 0.3 - i as f32 * 0.1;
                draw_circle(
                    x,
                    figure_y,
                    glow_size,
                    Color::new(char_color.r, char_color.g, char_color.b, alpha * pulse),
                );
            }
        }

        // Character-specific attributes
        let (skin_color, head_size, body_width_mult, body_height_mult) = match character.id {
            CharacterId::Bas => (Color::new(0.85, 0.65, 0.45, 1.0), 0.20, 1.3, 1.2), // Bigger
            CharacterId::Berkay => (Color::new(1.0, 0.85, 0.70, 1.0), 0.18, 1.0, 1.0), // Normal
            CharacterId::Gefferinho => (Color::new(0.65, 0.45, 0.30, 1.0), 0.18, 1.0, 1.0), // Brown skin
            CharacterId::Hadi => (Color::new(0.65, 0.45, 0.30, 1.0), 0.18, 1.0, 1.0), // Brown skin
            CharacterId::Nitin => (Color::new(0.65, 0.45, 0.30, 1.0), 0.18, 1.0, 1.0), // Brown skin
            CharacterId::Luca => (Color::new(1.0, 0.85, 0.70, 1.0), 0.18, 1.0, 1.0),  // Normal
            CharacterId::PalaBaba => (Color::new(1.0, 0.90, 0.80, 1.0), 0.18, 1.0, 1.0), // White skin
        };

        // Head (skin color)
        let head_radius = size * head_size;
        draw_circle(x, figure_y, head_radius, skin_color);
        draw_circle_lines(
            x,
            figure_y,
            head_radius,
            2.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );

        // Draw hair based on character
        let brown_hair = Color::new(0.3, 0.2, 0.1, 1.0);
        let black_hair = Color::new(0.1, 0.1, 0.1, 1.0);

        match character.id {
            CharacterId::Bas => {
                // Big brown curly hair
                for i in 0..8 {
                    let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
                    let curl_x = x + angle.cos() * head_radius * 0.9;
                    let curl_y = figure_y - head_radius * 0.3 + angle.sin() * head_radius * 0.9;
                    draw_circle(curl_x, curl_y, head_radius * 0.35, brown_hair);
                }
            }
            CharacterId::Berkay => {
                // Medium fluffy brown hair
                for i in 0..6 {
                    let angle = (i as f32 / 6.0) * std::f32::consts::PI * 2.0;
                    let fluff_x = x + angle.cos() * head_radius * 0.7;
                    let fluff_y = figure_y - head_radius * 0.5 + angle.sin() * head_radius * 0.5;
                    draw_circle(fluff_x, fluff_y, head_radius * 0.30, brown_hair);
                }
            }
            CharacterId::Gefferinho => {
                // Brown skin with curly hair
                for i in 0..7 {
                    let angle = (i as f32 / 7.0) * std::f32::consts::PI * 2.0;
                    let curl_x = x + angle.cos() * head_radius * 0.8;
                    let curl_y = figure_y - head_radius * 0.4 + angle.sin() * head_radius * 0.8;
                    draw_circle(curl_x, curl_y, head_radius * 0.3, brown_hair);
                }
            }
            CharacterId::Hadi => {
                // Brown skin, short black buzz cut
                draw_circle(
                    x,
                    figure_y - head_radius * 0.3,
                    head_radius * 0.95,
                    black_hair,
                );
            }
            CharacterId::Nitin => {
                // Brown skin, medium slicked back hair
                draw_rectangle(
                    x - head_radius * 0.7,
                    figure_y - head_radius,
                    head_radius * 1.4,
                    head_radius * 0.8,
                    brown_hair,
                );
                // Slicked back effect
                for i in 0..4 {
                    let line_x = x - head_radius * 0.5 + i as f32 * head_radius * 0.35;
                    draw_line(
                        line_x,
                        figure_y - head_radius * 0.9,
                        line_x + head_radius * 0.2,
                        figure_y - head_radius * 0.2,
                        2.0,
                        Color::new(0.2, 0.15, 0.1, 1.0),
                    );
                }
            }
            CharacterId::Luca => {
                // Normal hair
                draw_circle(
                    x,
                    figure_y - head_radius * 0.4,
                    head_radius * 0.9,
                    brown_hair,
                );
            }
            CharacterId::PalaBaba => {
                // Medium brown hair
                for i in 0..5 {
                    let angle = (i as f32 / 5.0) * std::f32::consts::PI * 2.0;
                    let hair_x = x + angle.cos() * head_radius * 0.75;
                    let hair_y = figure_y - head_radius * 0.45 + angle.sin() * head_radius * 0.6;
                    draw_circle(hair_x, hair_y, head_radius * 0.28, brown_hair);
                }
            }
        }

        // Body (with outfit color)
        let body_width = size * 0.25 * body_width_mult;
        let body_height = size * 0.35 * body_height_mult;
        draw_rectangle(
            x - body_width * 0.5,
            figure_y + head_radius + 5.0,
            body_width,
            body_height,
            char_color,
        );
        draw_rectangle_lines(
            x - body_width * 0.5,
            figure_y + head_radius + 5.0,
            body_width,
            body_height,
            3.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );

        // Arms (animated slightly, skin colored)
        let arm_offset = (time * 2.0).sin() * 5.0;
        let arm_y_start = figure_y + head_radius + 10.0;
        draw_line(
            x - body_width * 0.5,
            arm_y_start,
            x - size * 0.35,
            arm_y_start + size * 0.15 + arm_offset,
            8.0,
            skin_color,
        );
        draw_line(
            x + body_width * 0.5,
            arm_y_start,
            x + size * 0.35,
            arm_y_start + size * 0.15 - arm_offset,
            8.0,
            skin_color,
        );

        // Ability icon effect around character
        if is_selected {
            for i in 0..character.effects.len() {
                let angle =
                    time + i as f32 * std::f32::consts::PI * 2.0 / character.effects.len() as f32;
                let orbit_x = x + angle.cos() * size * 0.4;
                let orbit_y = figure_y + angle.sin() * size * 0.4;
                draw_circle(orbit_x, orbit_y, 8.0, YELLOW);
                draw_circle(orbit_x, orbit_y, 5.0, WHITE);
            }
        }
    }
}

impl State for CharacterSelectState {
    fn enter(&mut self) {
        self.selected_index = 0;
        self.selected_character = None;
        self.transition_to = None;
        self.animation_time = 0.0;
        self.hover_pulse = 1.0;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.animation_time += dt;
        self.hover_pulse = (self.animation_time * 3.0).sin() * 0.15 + 0.85;
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        // Gradient background
        for i in 0..30 {
            let y = i as f32 * screen_height() / 30.0;
            let height = screen_height() / 30.0;
            let t = i as f32 / 30.0;
            let color = Color::new(0.05 * (1.0 - t), 0.05 * (1.0 - t), 0.1 + 0.05 * t, 1.0);
            draw_rectangle(0.0, y, screen_width(), height, color);
        }

        // Animated background particles
        for i in 0..20 {
            let offset = (self.animation_time * 0.5 + i as f32 * 0.3).sin() * 30.0;
            let x = (i as f32 * screen_width() / 20.0 + offset) % screen_width();
            let y = (self.animation_time * 20.0 + i as f32 * 40.0) % screen_height();
            let alpha = 0.3 + ((self.animation_time * 2.0 + i as f32).sin() * 0.2);
            draw_circle(x, y, 3.0, Color::new(1.0, 1.0, 1.0, alpha as f32));
        }

        // Title with glow
        let title = "SELECT YOUR FIGHTER";
        let title_size = 70.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        let title_x = screen_width() * 0.5 - title_dims.width * 0.5;

        // Title glow
        for offset in [-3.0, -2.0, -1.0, 1.0, 2.0, 3.0] {
            draw_text(
                title,
                title_x + offset,
                75.0,
                title_size,
                Color::new(1.0, 0.8, 0.0, 0.3),
            );
            draw_text(
                title,
                title_x,
                75.0 + offset,
                title_size,
                Color::new(1.0, 0.8, 0.0, 0.3),
            );
        }
        draw_text(title, title_x, 75.0, title_size, GOLD);

        // Character cards - 3 per row, 2 rows
        let chars_per_row = 3;
        let card_width = 320.0;
        let card_height = 340.0;
        let spacing = 40.0;
        let start_x =
            screen_width() * 0.5 - (chars_per_row as f32 * (card_width + spacing) - spacing) * 0.5;
        let start_y = 140.0;

        for (i, character) in CHARACTERS.iter().enumerate() {
            let row = i / chars_per_row;
            let col = i % chars_per_row;
            let x = start_x + col as f32 * (card_width + spacing);
            let y = start_y + row as f32 * (card_height + spacing);

            let is_selected = i == self.selected_index;
            let char_color = Self::get_character_color(character.id);

            // Card animation
            let card_offset = if is_selected {
                (self.animation_time * 4.0).sin() * 3.0
            } else {
                0.0
            };
            let card_y = y + card_offset;

            // Card shadow
            draw_rectangle(
                x + 5.0,
                card_y + 5.0,
                card_width,
                card_height,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );

            // Card background with gradient
            let bg_color = if is_selected {
                Color::new(0.15, 0.15, 0.25, 0.95)
            } else {
                Color::new(0.08, 0.08, 0.12, 0.9)
            };
            draw_rectangle(x, card_y, card_width, card_height, bg_color);

            // Colored top border
            let border_height = 6.0;
            draw_rectangle(x, card_y, card_width, border_height, char_color);

            // Selection glow
            if is_selected {
                let glow_width = 4.0 * self.hover_pulse;
                draw_rectangle_lines(x, card_y, card_width, card_height, glow_width, char_color);

                // Corner decorations
                let corner_size = 20.0;
                draw_line(x, card_y, x + corner_size, card_y, 3.0, GOLD);
                draw_line(x, card_y, x, card_y + corner_size, 3.0, GOLD);
                draw_line(
                    x + card_width,
                    card_y,
                    x + card_width - corner_size,
                    card_y,
                    3.0,
                    GOLD,
                );
                draw_line(
                    x + card_width,
                    card_y,
                    x + card_width,
                    card_y + corner_size,
                    3.0,
                    GOLD,
                );
                draw_line(
                    x,
                    card_y + card_height,
                    x + corner_size,
                    card_y + card_height,
                    3.0,
                    GOLD,
                );
                draw_line(
                    x,
                    card_y + card_height,
                    x,
                    card_y + card_height - corner_size,
                    3.0,
                    GOLD,
                );
                draw_line(
                    x + card_width,
                    card_y + card_height,
                    x + card_width - corner_size,
                    card_y + card_height,
                    3.0,
                    GOLD,
                );
                draw_line(
                    x + card_width,
                    card_y + card_height,
                    x + card_width,
                    card_y + card_height - corner_size,
                    3.0,
                    GOLD,
                );
            } else {
                draw_rectangle_lines(
                    x,
                    card_y,
                    card_width,
                    card_height,
                    2.0,
                    Color::new(0.3, 0.3, 0.4, 0.8),
                );
            }

            // Character portrait
            self.draw_character_portrait(
                x + card_width * 0.5,
                card_y + 50.0,
                100.0,
                character,
                is_selected,
            );

            // Character name
            let name_size = 36.0;
            let name_dims = measure_text(character.name, None, name_size as u16, 1.0);
            let name_y = card_y + 180.0;
            if is_selected {
                // Name glow for selected
                draw_text(
                    character.name,
                    x + card_width * 0.5 - name_dims.width * 0.5 + 2.0,
                    name_y + 2.0,
                    name_size,
                    Color::new(char_color.r, char_color.g, char_color.b, 0.5),
                );
            }
            draw_text(
                character.name,
                x + card_width * 0.5 - name_dims.width * 0.5,
                name_y,
                name_size,
                if is_selected { char_color } else { WHITE },
            );

            // Ability name in colored box
            let ability_size = 18.0;
            let ability_dims = measure_text(character.ability_name, None, ability_size as u16, 1.0);
            let ability_y = card_y + 215.0;
            draw_rectangle(
                x + card_width * 0.5 - ability_dims.width * 0.5 - 10.0,
                ability_y - 20.0,
                ability_dims.width + 20.0,
                28.0,
                Color::new(
                    char_color.r * 0.3,
                    char_color.g * 0.3,
                    char_color.b * 0.3,
                    0.8,
                ),
            );
            draw_text(
                character.ability_name,
                x + card_width * 0.5 - ability_dims.width * 0.5,
                ability_y,
                ability_size,
                char_color,
            );

            // Stats section
            let stats_y = card_y + 255.0;
            let stat_size = 16.0;

            draw_text(
                "STATS",
                x + 15.0,
                stats_y,
                stat_size,
                Color::new(0.7, 0.7, 0.7, 1.0),
            );
            draw_rectangle(
                x + 15.0,
                stats_y + 5.0,
                card_width - 30.0,
                1.0,
                Color::new(0.5, 0.5, 0.5, 0.5),
            );

            // Duration
            draw_text("Duration:", x + 15.0, stats_y + 25.0, stat_size, LIGHTGRAY);
            draw_text(
                &format!("{}s", character.duration),
                x + card_width - 50.0,
                stats_y + 25.0,
                stat_size,
                YELLOW,
            );

            // Cooldown
            draw_text("Cooldown:", x + 15.0, stats_y + 45.0, stat_size, LIGHTGRAY);
            draw_text(
                &format!("{}s", character.cooldown),
                x + card_width - 50.0,
                stats_y + 45.0,
                stat_size,
                YELLOW,
            );

            // Effects count
            draw_text("Effects:", x + 15.0, stats_y + 65.0, stat_size, LIGHTGRAY);
            draw_text(
                &format!("{}", character.effects.len()),
                x + card_width - 50.0,
                stats_y + 65.0,
                stat_size,
                YELLOW,
            );
        }

        // Details panel for selected character
        let character = &CHARACTERS[self.selected_index];
        let char_color = Self::get_character_color(character.id);
        let detail_y = start_y + 2.0 * (card_height + spacing) + 20.0;
        let detail_height = 140.0;

        // Detail panel background
        draw_rectangle(
            100.0,
            detail_y,
            screen_width() - 200.0,
            detail_height,
            Color::new(0.05, 0.05, 0.08, 0.95),
        );
        draw_rectangle_lines(
            100.0,
            detail_y,
            screen_width() - 200.0,
            detail_height,
            3.0,
            char_color,
        );

        // Character description
        let description = Self::get_character_description(character.id);
        draw_text(description, 120.0, detail_y + 25.0, 18.0, LIGHTGRAY);

        // Voice line section
        let voice_label = "VOICE LINE:";
        draw_text(voice_label, 120.0, detail_y + 55.0, 18.0, char_color);
        draw_text(
            &format!("\"{}\"", character.voice_line),
            120.0,
            detail_y + 78.0,
            20.0,
            GOLD,
        );

        // Effects section
        let effects_label = "ABILITY EFFECTS:";
        draw_text(effects_label, 120.0, detail_y + 108.0, 18.0, char_color);

        let mut effect_x = 120.0;
        for effect in character.effects {
            let effect_text = match effect {
                crate::data::characters::AbilityEffect::DamageBoost(mult) => {
                    format!("DMG ×{:.1}", mult)
                }
                crate::data::characters::AbilityEffect::HealthBoost(boost) => {
                    format!("HP +{:.0}", boost)
                }
                crate::data::characters::AbilityEffect::SpeedBoost(mult) => {
                    format!("SPD ×{:.1}", mult)
                }
                crate::data::characters::AbilityEffect::SplashDamage(dmg, radius) => {
                    format!("AOE {:.0}dmg", dmg)
                }
                crate::data::characters::AbilityEffect::FireDamage(dps, _) => {
                    format!("FIRE {:.0}dps", dps)
                }
            };

            let effect_dims = measure_text(&effect_text, None, 16, 1.0);
            draw_rectangle(
                effect_x,
                detail_y + 116.0,
                effect_dims.width + 16.0,
                24.0,
                Color::new(
                    char_color.r * 0.4,
                    char_color.g * 0.4,
                    char_color.b * 0.4,
                    0.8,
                ),
            );
            draw_rectangle_lines(
                effect_x,
                detail_y + 116.0,
                effect_dims.width + 16.0,
                24.0,
                2.0,
                char_color,
            );
            draw_text(&effect_text, effect_x + 8.0, detail_y + 133.0, 16.0, WHITE);
            effect_x += effect_dims.width + 26.0;
        }

        // Instructions at bottom
        let instructions_y = screen_height() - 50.0;
        draw_rectangle(
            0.0,
            instructions_y - 30.0,
            screen_width(),
            80.0,
            Color::new(0.0, 0.0, 0.0, 0.8),
        );

        let instructions = "ARROW KEYS: Navigate  |  ENTER: Select Character  |  ESC: Back to Menu";
        let inst_dims = measure_text(instructions, None, 22, 1.0);
        draw_text(
            instructions,
            screen_width() * 0.5 - inst_dims.width * 0.5,
            instructions_y,
            22.0,
            Color::new(1.0, 1.0, 1.0, 0.9),
        );

        // Keybind reminder
        let keybind_text = "Press E during gameplay to activate ability!";
        let keybind_dims = measure_text(keybind_text, None, 18, 1.0);
        draw_text(
            keybind_text,
            screen_width() * 0.5 - keybind_dims.width * 0.5,
            instructions_y + 25.0,
            18.0,
            YELLOW,
        );
    }

    fn handle_input(&mut self) {
        // Navigate left
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            if self.selected_index % 3 > 0 {
                self.selected_index -= 1;
            }
        }

        // Navigate right
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            if self.selected_index % 3 < 2 && self.selected_index < CHARACTERS.len() - 1 {
                self.selected_index += 1;
            }
        }

        // Navigate up
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_index >= 3 {
                self.selected_index -= 3;
            }
        }

        // Navigate down
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_index + 3 < CHARACTERS.len() {
                self.selected_index += 3;
            }
        }

        // Select character
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::J) {
            let character_id = CHARACTERS[self.selected_index].id;
            self.selected_character = Some(character_id);
            crate::data::set_selected_character(character_id);
            self.transition_to = Some(StateType::Gameplay);
        }

        // Go back
        if is_key_pressed(KeyCode::Escape) {
            self.transition_to = Some(StateType::Menu);
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
