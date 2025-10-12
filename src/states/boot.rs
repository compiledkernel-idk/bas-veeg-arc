use crate::states::State;
use crate::updater::{UpdateStatus, Updater};
use macroquad::prelude::*;

pub struct BootState {
    timer: f32,
    logo_alpha: f32,
    transition_to_menu: bool,
    updater: Updater,
    update_check_started: bool,
    update_selected_option: usize, // 0 = Install Now, 1 = Skip
}

impl BootState {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            logo_alpha: 0.0,
            transition_to_menu: false,
            updater: Updater::new(),
            update_check_started: false,
            update_selected_option: 0,
        }
    }
}

impl State for BootState {
    fn enter(&mut self) {
        self.timer = 0.0;
        self.logo_alpha = 0.0;
        self.transition_to_menu = false;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.timer += dt;

        // Start update check after 0.5 seconds
        if self.timer > 0.5 && !self.update_check_started {
            self.update_check_started = true;
            self.updater.check_for_updates();
        }

        // Handle logo fade in/out
        if self.timer < 1.0 {
            self.logo_alpha = self.timer;
        } else if self.timer < 2.0 {
            self.logo_alpha = 1.0;
        } else if self.timer < 3.0 {
            self.logo_alpha = 3.0 - self.timer;
        } else {
            // After logo fade, check update status
            match self.updater.status {
                UpdateStatus::UpdateAvailable => {
                    // Show update dialog, don't transition yet
                }
                UpdateStatus::Checking => {
                    // Still checking, wait
                }
                UpdateStatus::Downloading | UpdateStatus::Installing => {
                    // Update in progress, wait
                }
                UpdateStatus::ReadyToInstall => {
                    // Update installed successfully, restart
                    self.updater.restart_game();
                }
                _ => {
                    // No update or error, proceed to menu
                    self.transition_to_menu = true;
                }
            }
        }
    }

    fn should_transition(&self) -> Option<crate::states::StateType> {
        if self.transition_to_menu {
            Some(crate::states::StateType::Menu)
        } else {
            None
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(BLACK);

        // Always show logo
        let text = "BAS VEEG ARC";
        let font_size = 80.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);

        draw_text(
            text,
            screen_width() * 0.5 - text_dims.width * 0.5,
            screen_height() * 0.5 - 100.0,
            font_size,
            Color::new(1.0, 1.0, 1.0, self.logo_alpha),
        );

        // Show version
        let version_text = format!("v{}", crate::updater::CURRENT_VERSION);
        let version_dims = measure_text(&version_text, None, 20, 1.0);
        draw_text(
            &version_text,
            screen_width() * 0.5 - version_dims.width * 0.5,
            screen_height() * 0.5 - 50.0,
            20.0,
            Color::new(0.7, 0.7, 0.7, self.logo_alpha * 0.7),
        );

        // Show update status
        match &self.updater.status {
            UpdateStatus::Checking => {
                let check_text = "Checking for updates...";
                let dims = measure_text(check_text, None, 24, 1.0);
                draw_text(
                    check_text,
                    screen_width() * 0.5 - dims.width * 0.5,
                    screen_height() * 0.7,
                    24.0,
                    Color::new(0.8, 0.8, 0.8, 1.0),
                );
            }
            UpdateStatus::UpdateAvailable => {
                self.render_update_dialog();
            }
            UpdateStatus::Downloading => {
                self.render_download_progress();
            }
            UpdateStatus::Installing => {
                let text = "Installing update...";
                let dims = measure_text(text, None, 24, 1.0);
                draw_text(
                    text,
                    screen_width() * 0.5 - dims.width * 0.5,
                    screen_height() * 0.7,
                    24.0,
                    Color::new(0.2, 1.0, 0.2, 1.0),
                );
            }
            UpdateStatus::Error => {
                if let Some(error) = &self.updater.error_message {
                    let dims = measure_text(error, None, 20, 1.0);
                    draw_text(
                        error,
                        screen_width() * 0.5 - dims.width * 0.5,
                        screen_height() * 0.7,
                        20.0,
                        Color::new(1.0, 0.3, 0.3, 1.0),
                    );
                }
            }
            UpdateStatus::UpToDate => {
                let text = "✓ Up to date";
                let dims = measure_text(text, None, 20, 1.0);
                draw_text(
                    text,
                    screen_width() * 0.5 - dims.width * 0.5,
                    screen_height() * 0.7,
                    20.0,
                    Color::new(0.2, 1.0, 0.2, 0.8),
                );
            }
            _ => {}
        }
    }

    fn handle_input(&mut self) {
        // Handle update dialog input
        if self.updater.status == UpdateStatus::UpdateAvailable {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if self.update_selected_option > 0 {
                    self.update_selected_option -= 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                if self.update_selected_option < 1 {
                    self.update_selected_option += 1;
                }
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::J) {
                if self.update_selected_option == 0 {
                    // Install update
                    self.updater.download_and_install();
                } else {
                    // Skip update
                    self.transition_to_menu = true;
                }
            }
        } else {
            // Skip boot screen with Space or Enter
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                self.timer = 3.0;
            }
        }
    }
}

// Helper methods for rendering
impl BootState {
    fn render_update_dialog(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        // Dialog box
        let box_width = 600.0;
        let box_height = 400.0;
        let box_x = screen_width() * 0.5 - box_width * 0.5;
        let box_y = screen_height() * 0.5 - box_height * 0.5;

        draw_rectangle(
            box_x,
            box_y,
            box_width,
            box_height,
            Color::new(0.15, 0.15, 0.2, 1.0),
        );
        draw_rectangle_lines(
            box_x,
            box_y,
            box_width,
            box_height,
            3.0,
            Color::new(0.3, 0.6, 1.0, 1.0),
        );

        // Title
        let title = "🎮 UPDATE AVAILABLE";
        let title_dims = measure_text(title, None, 36, 1.0);
        draw_text(
            title,
            box_x + box_width * 0.5 - title_dims.width * 0.5,
            box_y + 50.0,
            36.0,
            Color::new(0.4, 0.8, 1.0, 1.0),
        );

        // Version info
        if let Some(info) = &self.updater.info {
            let version_text = format!("New version {} is available!", info.latest_version);
            let version_dims = measure_text(&version_text, None, 24, 1.0);
            draw_text(
                &version_text,
                box_x + box_width * 0.5 - version_dims.width * 0.5,
                box_y + 100.0,
                24.0,
                WHITE,
            );

            // Changelog preview
            let changelog_title = "What's new:";
            draw_text(
                changelog_title,
                box_x + 40.0,
                box_y + 150.0,
                20.0,
                Color::new(0.8, 0.8, 0.8, 1.0),
            );

            // Show first 3 lines of changelog
            let lines: Vec<&str> = info.changelog.lines().take(3).collect();
            for (i, line) in lines.iter().enumerate() {
                let truncated = if line.len() > 60 {
                    format!("{}...", &line[..60])
                } else {
                    line.to_string()
                };
                draw_text(
                    &truncated,
                    box_x + 40.0,
                    box_y + 180.0 + i as f32 * 25.0,
                    18.0,
                    Color::new(0.7, 0.7, 0.7, 1.0),
                );
            }
        }

        // Options
        let options = ["Install Now & Restart", "Skip This Update"];
        for (i, option) in options.iter().enumerate() {
            let y = box_y + box_height - 100.0 + i as f32 * 50.0;
            let is_selected = i == self.update_selected_option;

            if is_selected {
                draw_rectangle(
                    box_x + 40.0,
                    y - 30.0,
                    box_width - 80.0,
                    40.0,
                    Color::new(0.2, 0.4, 0.8, 0.5),
                );
                draw_rectangle_lines(
                    box_x + 40.0,
                    y - 30.0,
                    box_width - 80.0,
                    40.0,
                    2.0,
                    Color::new(0.4, 0.6, 1.0, 1.0),
                );

                draw_text("▶", box_x + 50.0, y, 28.0, Color::new(1.0, 0.8, 0.0, 1.0));
            }

            let text_color = if is_selected {
                Color::new(1.0, 1.0, 0.8, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0)
            };

            draw_text(option, box_x + 90.0, y, 24.0, text_color);
        }

        // Instructions
        let instructions = "Use W/S or UP/DOWN to select, ENTER to confirm";
        let instr_dims = measure_text(instructions, None, 16, 1.0);
        draw_text(
            instructions,
            box_x + box_width * 0.5 - instr_dims.width * 0.5,
            box_y + box_height - 20.0,
            16.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );
    }

    fn render_download_progress(&self) {
        let text = "Downloading update...";
        let dims = measure_text(text, None, 24, 1.0);
        draw_text(
            text,
            screen_width() * 0.5 - dims.width * 0.5,
            screen_height() * 0.6,
            24.0,
            WHITE,
        );

        // Progress bar
        let bar_width = 400.0;
        let bar_height = 30.0;
        let bar_x = screen_width() * 0.5 - bar_width * 0.5;
        let bar_y = screen_height() * 0.65;

        draw_rectangle(
            bar_x,
            bar_y,
            bar_width,
            bar_height,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );

        let progress = self.updater.download_progress / 100.0;
        draw_rectangle(
            bar_x,
            bar_y,
            bar_width * progress,
            bar_height,
            Color::new(0.2, 0.8, 0.2, 1.0),
        );

        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);

        let percent = format!("{}%", self.updater.download_progress as u32);
        let percent_dims = measure_text(&percent, None, 20, 1.0);
        draw_text(
            &percent,
            screen_width() * 0.5 - percent_dims.width * 0.5,
            bar_y + 50.0,
            20.0,
            WHITE,
        );
    }
}
