use crate::states::{State, StateType};
use macroquad::prelude::*;

pub struct ControlsState {
    transition_to: Option<StateType>,
    scroll_offset: f32,
}

impl ControlsState {
    pub fn new() -> Self {
        Self {
            transition_to: None,
            scroll_offset: 0.0,
        }
    }
}

impl State for ControlsState {
    fn enter(&mut self) {
        self.transition_to = None;
        self.scroll_offset = 0.0;
    }

    fn exit(&mut self) {}

    fn update(&mut self, _dt: f32) {}

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

        // Title
        let title = "CONTROLS";
        let title_size = 70.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() * 0.5 - title_dims.width * 0.5,
            80.0,
            title_size,
            Color::new(1.0, 0.8, 0.0, 1.0),
        );

        let start_y = 180.0 - self.scroll_offset;
        let section_spacing = 60.0;
        let mut current_y = start_y;

        // Movement Section
        self.draw_section_header("MOVEMENT", current_y, Color::new(0.3, 0.8, 1.0, 1.0));
        current_y += 40.0;
        current_y = self.draw_keybind("W / Up Arrow", "Move Up", current_y);
        current_y = self.draw_keybind("S / Down Arrow", "Move Down", current_y);
        current_y = self.draw_keybind("A / Left Arrow", "Move Left", current_y);
        current_y = self.draw_keybind("D / Right Arrow", "Move Right", current_y);
        current_y += section_spacing;

        // Combat Section
        self.draw_section_header("COMBAT", current_y, Color::new(1.0, 0.3, 0.3, 1.0));
        current_y += 40.0;
        current_y = self.draw_keybind("J", "Light Attack", current_y);
        current_y = self.draw_keybind("K", "Heavy Attack", current_y);
        current_y = self.draw_keybind("L", "Special Attack", current_y);
        current_y = self.draw_keybind("E", "Activate Character Ability", current_y);
        current_y += section_spacing;

        // Shop Section
        self.draw_section_header("SHOP", current_y, Color::new(1.0, 0.9, 0.4, 1.0));
        current_y += 40.0;
        current_y = self.draw_keybind("B", "Open/Close Shop", current_y);
        current_y = self.draw_keybind("1-8", "Buy Upgrade", current_y);
        current_y += section_spacing;

        // Menu Section
        self.draw_section_header("MENU", current_y, Color::new(0.5, 1.0, 0.5, 1.0));
        current_y += 40.0;
        current_y = self.draw_keybind("Enter / J", "Select", current_y);
        current_y = self.draw_keybind("Space", "Toggle Details (Character Select)", current_y);
        current_y = self.draw_keybind("Escape", "Pause / Back", current_y);
        current_y += section_spacing;

        // Character Abilities Info Section
        self.draw_section_header(
            "CHARACTER ABILITIES",
            current_y,
            Color::new(1.0, 0.5, 1.0, 1.0),
        );
        current_y += 40.0;
        draw_text(
            "Each character has a unique ability:",
            200.0,
            current_y,
            20.0,
            LIGHTGRAY,
        );
        current_y += 35.0;
        current_y =
            self.draw_ability_info("Berkay", "Special Kebab - Damage & Health Boost", current_y);
        current_y = self.draw_ability_info("Luca", "Winter Arc - Massive Damage Boost", current_y);
        current_y = self.draw_ability_info(
            "Gefferinho",
            "Maar Mevrouw Rage - Speed, Damage & Health",
            current_y,
        );
        current_y = self.draw_ability_info("Bas", "Bas Veeg - AOE Splash Damage", current_y);
        current_y =
            self.draw_ability_info("Hadi", "Dubai Emirates - Massive Speed Boost", current_y);
        current_y = self.draw_ability_info(
            "Nitin",
            "Barra in je Kont - Sets Enemies on Fire",
            current_y,
        );
        current_y = self.draw_ability_info(
            "Yigit Baba",
            "Sivas Rage - ULTIMATE: Damage, Speed & Health (30s CD)",
            current_y,
        );

        // Footer
        let footer_y = screen_height() - 60.0;
        draw_rectangle(
            0.0,
            footer_y - 20.0,
            screen_width(),
            100.0,
            Color::new(0.0, 0.0, 0.0, 0.8),
        );
        let footer = "Press ESC to return to menu";
        let footer_dims = measure_text(footer, None, 24, 1.0);
        draw_text(
            footer,
            screen_width() * 0.5 - footer_dims.width * 0.5,
            footer_y + 10.0,
            24.0,
            WHITE,
        );
    }

    fn handle_input(&mut self) {
        // Scroll with arrow keys
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.scroll_offset -= 5.0;
            self.scroll_offset = self.scroll_offset.max(0.0);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.scroll_offset += 5.0;
            self.scroll_offset = self.scroll_offset.min(400.0);
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

impl ControlsState {
    fn draw_section_header(&self, text: &str, y: f32, color: Color) {
        if y < 120.0 || y > screen_height() - 50.0 {
            return;
        }

        draw_rectangle(180.0, y - 5.0, screen_width() - 360.0, 3.0, color);
        draw_text(text, 200.0, y + 20.0, 32.0, color);
    }

    fn draw_keybind(&self, key: &str, action: &str, y: f32) -> f32 {
        if y < 120.0 || y > screen_height() - 50.0 {
            return y + 35.0;
        }

        let x_key = 250.0;
        let x_action = 500.0;

        // Draw key background
        let key_dims = measure_text(key, None, 22, 1.0);
        draw_rectangle(
            x_key - 10.0,
            y - 22.0,
            key_dims.width + 20.0,
            30.0,
            Color::new(0.2, 0.2, 0.3, 0.8),
        );
        draw_rectangle_lines(
            x_key - 10.0,
            y - 22.0,
            key_dims.width + 20.0,
            30.0,
            2.0,
            Color::new(0.5, 0.5, 0.6, 1.0),
        );

        // Draw key text
        draw_text(key, x_key, y, 22.0, YELLOW);

        // Draw separator
        draw_text("→", x_action - 40.0, y, 22.0, GRAY);

        // Draw action text
        draw_text(action, x_action, y, 22.0, WHITE);

        y + 35.0
    }

    fn draw_ability_info(&self, character: &str, description: &str, y: f32) -> f32 {
        if y < 120.0 || y > screen_height() - 50.0 {
            return y + 30.0;
        }

        draw_text(
            &format!("• {}: {}", character, description),
            220.0,
            y,
            18.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
        );
        y + 30.0
    }
}
