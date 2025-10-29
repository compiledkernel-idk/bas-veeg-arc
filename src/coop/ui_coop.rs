use super::player_manager::{CoopPlayerManager, PlayerSlot, PLAYER_COLORS};
use super::shared_systems::{SharedComboSystem, ReviveSystem};
use macroquad::prelude::*;

/// Co-op specific UI elements
pub struct CoopUI {
    font_size: f32,
    show_player_indicators: bool,
    show_stats: bool,
}

impl CoopUI {
    pub fn new() -> Self {
        Self {
            font_size: 24.0,
            show_player_indicators: true,
            show_stats: true,
        }
    }

    /// Render player HUD elements for all active players
    pub fn render_player_huds(&self, player_manager: &CoopPlayerManager) {
        let active_players = player_manager.get_active_players();
        let player_count = active_players.len();

        for (i, player) in active_players.iter().enumerate() {
            // Calculate position for this player's HUD
            let x_offset = match player_count {
                1 => screen_width() * 0.5,
                2 => screen_width() * 0.25 + (i as f32 * screen_width() * 0.5),
                3 | 4 => {
                    let cols = 2;
                    let row = i / cols;
                    let col = i % cols;
                    screen_width() * 0.25 + (col as f32 * screen_width() * 0.5)
                }
                _ => screen_width() * 0.5,
            };

            let y_offset = match player_count {
                1 | 2 => 20.0,
                3 | 4 => {
                    let row = i / 2;
                    20.0 + (row as f32 * 60.0)
                }
                _ => 20.0,
            };

            self.render_player_hud(player, x_offset, y_offset);
        }
    }

    /// Render HUD for a single player
    fn render_player_hud(&self, player: &super::player_manager::CoopPlayer, x: f32, y: f32) {
        let color = player.color;

        // Player name/slot
        let text = format!("P{}", player.slot.to_index() + 1);
        draw_text(&text, x - 50.0, y, self.font_size, color);

        // Health bar would go here (requires entity health data)
        // Placeholder health bar
        let bar_width = 150.0;
        let bar_height = 10.0;

        // Background
        draw_rectangle(x - 40.0, y + 5.0, bar_width, bar_height, Color::new(0.2, 0.2, 0.2, 0.8));

        // Health (placeholder at 75%)
        let health_percent = 0.75; // Would get from entity
        draw_rectangle(
            x - 40.0,
            y + 5.0,
            bar_width * health_percent,
            bar_height,
            color,
        );

        // Border
        draw_rectangle_lines(x - 40.0, y + 5.0, bar_width, bar_height, 2.0, WHITE);

        // Downed indicator
        if player.is_downed {
            draw_text("DOWNED!", x - 20.0, y + 30.0, self.font_size * 0.7, RED);

            // Revive progress
            if player.revive_progress > 0.0 {
                let revive_bar_width = 100.0;
                draw_rectangle(
                    x - 20.0,
                    y + 35.0,
                    revive_bar_width,
                    5.0,
                    Color::new(0.2, 0.2, 0.2, 0.8),
                );
                draw_rectangle(
                    x - 20.0,
                    y + 35.0,
                    revive_bar_width * player.revive_progress,
                    5.0,
                    GREEN,
                );
            }
        }
    }

    /// Render shared combo counter
    pub fn render_combo_counter(&self, combo_system: &SharedComboSystem) {
        if !combo_system.is_combo_active() {
            return;
        }

        let combo_count = combo_system.get_combo_count();
        let combo_rank = combo_system.get_combo_rank();
        let combo_color = combo_rank.to_color();

        // Center of screen, near top
        let x = screen_width() * 0.5;
        let y = 100.0;

        // Combo count
        let combo_text = format!("{} HITS", combo_count);
        let text_width = measure_text(&combo_text, None, 60, 1.0).width;
        draw_text(&combo_text, x - text_width * 0.5, y, 60.0, combo_color);

        // Rank
        let rank_text = combo_rank.to_string();
        let rank_width = measure_text(rank_text, None, 80, 1.0).width;
        draw_text(rank_text, x - rank_width * 0.5, y + 60.0, 80.0, combo_color);

        // Combo timer bar
        let time_remaining = combo_system.get_time_remaining();
        let max_time = 2.0; // From SharedComboSystem::combo_decay_time
        let time_percent = time_remaining / max_time;

        let bar_width = 200.0;
        let bar_x = x - bar_width * 0.5;
        let bar_y = y + 80.0;

        draw_rectangle(bar_x, bar_y, bar_width, 5.0, Color::new(0.2, 0.2, 0.2, 0.8));
        draw_rectangle(bar_x, bar_y, bar_width * time_percent, 5.0, combo_color);
    }

    /// Render shared currency
    pub fn render_shared_currency(&self, player_manager: &CoopPlayerManager) {
        let currency = player_manager.get_currency();
        let text = format!("${:.0}", currency);

        // Top right corner
        let x = screen_width() - 150.0;
        let y = 30.0;

        draw_text(&text, x, y, self.font_size * 1.2, YELLOW);
    }

    /// Render player indicators above characters
    pub fn render_player_indicators(
        &self,
        player_manager: &CoopPlayerManager,
        player_positions: &[(PlayerSlot, Vec2)],
    ) {
        if !self.show_player_indicators {
            return;
        }

        for (slot, position) in player_positions {
            if let Some(player) = player_manager.get_player(*slot) {
                if !player.is_active {
                    continue;
                }

                let color = player.color;

                // Arrow above player
                let arrow_y = position.y - 80.0;
                let arrow_x = position.x;

                // Simple down arrow made of triangles
                let points = vec![
                    Vec2::new(arrow_x - 10.0, arrow_y - 10.0),
                    Vec2::new(arrow_x + 10.0, arrow_y - 10.0),
                    Vec2::new(arrow_x, arrow_y),
                ];

                // Draw filled triangle
                for i in 0..3 {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % 3];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, color);
                }

                // Player number
                let text = format!("P{}", slot.to_index() + 1);
                let text_width = measure_text(&text, None, 20, 1.0).width;
                draw_text(
                    &text,
                    arrow_x - text_width * 0.5,
                    arrow_y - 15.0,
                    20.0,
                    color,
                );

                // Downed indicator
                if player.is_downed {
                    draw_circle(arrow_x, arrow_y + 20.0, 15.0, Color::new(1.0, 0.0, 0.0, 0.5));
                    draw_text("!", arrow_x - 5.0, arrow_y + 25.0, 20.0, WHITE);
                }
            }
        }
    }

    /// Render player stats overlay (for pause/end screen)
    pub fn render_stats_overlay(&self, player_manager: &CoopPlayerManager) {
        if !self.show_stats {
            return;
        }

        let bg_color = Color::new(0.0, 0.0, 0.0, 0.8);
        let panel_width = 600.0;
        let panel_height = 400.0;
        let panel_x = (screen_width() - panel_width) * 0.5;
        let panel_y = (screen_height() - panel_height) * 0.5;

        // Background panel
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, bg_color);
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 3.0, WHITE);

        // Title
        let title = "PLAYER STATS";
        let title_width = measure_text(title, None, 40, 1.0).width;
        draw_text(
            title,
            panel_x + (panel_width - title_width) * 0.5,
            panel_y + 40.0,
            40.0,
            WHITE,
        );

        // Stats for each player
        let active_players = player_manager.get_active_players();
        let mut y_offset = panel_y + 80.0;

        for player in active_players {
            let color = player.color;

            // Player header
            let header = format!("Player {} - {}", player.slot.to_index() + 1, player.character_type.to_string());
            draw_text(&header, panel_x + 20.0, y_offset, self.font_size, color);
            y_offset += 30.0;

            // Stats
            let stats = [
                format!("Kills: {}", player.kills),
                format!("Deaths: {}", player.deaths),
                format!("K/D: {:.2}", player.get_kd_ratio()),
                format!("Damage Dealt: {:.0}", player.damage_dealt),
                format!("Damage Taken: {:.0}", player.damage_taken),
                format!("Combo Contribution: {}", player.combo_contribution),
            ];

            for stat in &stats {
                draw_text(stat, panel_x + 40.0, y_offset, self.font_size * 0.7, WHITE);
                y_offset += 25.0;
            }

            y_offset += 15.0; // Space between players
        }
    }

    /// Render revive prompt
    pub fn render_revive_prompt(&self, revive_system: &ReviveSystem, player_slot: PlayerSlot) {
        if let Some((reviver, progress)) = revive_system.get_active_revive(player_slot) {
            let text = format!("P{} reviving... {:.0}%", reviver.to_index() + 1, progress * 100.0);
            let x = screen_width() * 0.5 - 100.0;
            let y = screen_height() * 0.7;

            // Background
            draw_rectangle(x - 10.0, y - 30.0, 220.0, 40.0, Color::new(0.0, 0.0, 0.0, 0.7));

            draw_text(&text, x, y, self.font_size, GREEN);
        }
    }

    /// Render team super indicator
    pub fn render_team_super(
        &self,
        combo_meter: f32,
        super_cost: f32,
    ) {
        let bar_width = 200.0;
        let bar_height = 20.0;
        let x = (screen_width() - bar_width) * 0.5;
        let y = screen_height() - 60.0;

        // Title
        draw_text("TEAM SUPER", x + 40.0, y - 10.0, self.font_size * 0.8, GOLD);

        // Background
        draw_rectangle(x, y, bar_width, bar_height, Color::new(0.2, 0.2, 0.2, 0.8));

        // Fill
        let fill_percent = (combo_meter / super_cost).min(1.0);
        let fill_color = if fill_percent >= 1.0 {
            GOLD
        } else {
            Color::new(0.8, 0.5, 0.0, 1.0)
        };

        draw_rectangle(x, y, bar_width * fill_percent, bar_height, fill_color);

        // Border
        draw_rectangle_lines(x, y, bar_width, bar_height, 2.0, WHITE);

        // Ready indicator
        if fill_percent >= 1.0 {
            draw_text("READY!", x + bar_width + 10.0, y + 15.0, self.font_size, GOLD);
        }
    }

    /// Render drop-in prompt
    pub fn render_drop_in_prompt(&self, available_slot: PlayerSlot) {
        let text = format!(
            "Press START on controller {} to join!",
            available_slot.to_index() + 1
        );
        let x = screen_width() * 0.5 - 200.0;
        let y = screen_height() - 100.0;

        draw_rectangle(x - 10.0, y - 30.0, 420.0, 40.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text(&text, x, y, self.font_size * 0.8, YELLOW);
    }

    /// Toggle player indicators
    pub fn toggle_indicators(&mut self) {
        self.show_player_indicators = !self.show_player_indicators;
    }

    /// Toggle stats display
    pub fn toggle_stats(&mut self) {
        self.show_stats = !self.show_stats;
    }
}

// Helper extension trait for CharacterId
trait CharacterIdExt {
    fn to_string(&self) -> &str;
}

impl CharacterIdExt for crate::data::characters::CharacterId {
    fn to_string(&self) -> &str {
        // Would match on all character types
        "Character"
    }
}
