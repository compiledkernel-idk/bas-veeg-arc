use crate::states::{State, StateType};
use crate::data::characters::{Character, CharacterId, CHARACTERS};
use crate::progression::SkillTreeManager;
use macroquad::prelude::*;

// Re-export types from progression module for easier access
use crate::progression::skill_tree::{SkillNode, SkillBranch};

pub struct SkillTreeUIState {
    skill_tree_manager: SkillTreeManager,
    selected_character: usize,
    selected_character_id: CharacterId,
    selected_branch: BranchSelection,
    selected_node: usize,
    transition_to: Option<StateType>,
    feedback_message: Option<(String, f32)>, // message and timer
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BranchSelection {
    Offense,
    Defense,
    Utility,
}

impl SkillTreeUIState {
    pub fn new() -> Self {
        Self {
            skill_tree_manager: SkillTreeManager::new(),
            selected_character: 0,
            selected_character_id: CHARACTERS[0].id,
            selected_branch: BranchSelection::Offense,
            selected_node: 0,
            transition_to: None,
            feedback_message: None,
        }
    }

    fn get_current_branch(&self) -> Option<&SkillBranch> {
        if let Some(tree) = self.skill_tree_manager.get_tree(self.selected_character_id) {
            Some(match self.selected_branch {
                BranchSelection::Offense => &tree.branch_offense,
                BranchSelection::Defense => &tree.branch_defense,
                BranchSelection::Utility => &tree.branch_utility,
            })
        } else {
            None
        }
    }

    fn get_current_branch_mut(&mut self) -> Option<&mut SkillBranch> {
        if let Some(tree) = self.skill_tree_manager.get_tree_mut(self.selected_character_id) {
            Some(match self.selected_branch {
                BranchSelection::Offense => &mut tree.branch_offense,
                BranchSelection::Defense => &mut tree.branch_defense,
                BranchSelection::Utility => &mut tree.branch_utility,
            })
        } else {
            None
        }
    }

    fn show_feedback(&mut self, message: String) {
        self.feedback_message = Some((message, 3.0));
    }
}

impl State for SkillTreeUIState {
    fn enter(&mut self) {
        self.transition_to = None;
        self.feedback_message = None;
    }

    fn exit(&mut self) {}

    fn update(&mut self, dt: f32) {
        // Update feedback message timer
        if let Some((_, ref mut timer)) = self.feedback_message {
            *timer -= dt;
            if *timer <= 0.0 {
                self.feedback_message = None;
            }
        }
    }

    fn fixed_update(&mut self, _dt: f64) {}

    fn render(&mut self, _interpolation: f32) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let sw = screen_width();
        let sh = screen_height();

        // Title
        let title = "SKILL TREE";
        let title_size = 50.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            sw * 0.5 - title_dims.width * 0.5,
            50.0,
            title_size,
            GOLD,
        );

        // Instructions
        let instructions = "A/D=Change Character | W/S=Select Node | Q/E=Switch Branch | ENTER=Upgrade | ESC=Back";
        let inst_size = 16.0;
        let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
        draw_text(
            instructions,
            sw * 0.5 - inst_dims.width * 0.5,
            sh - 30.0,
            inst_size,
            LIGHTGRAY,
        );

        // Character info section (left)
        let char_panel_x = 20.0;
        let char_panel_y = 100.0;
        let char_panel_width = 250.0;

        draw_rectangle_lines(char_panel_x, char_panel_y, char_panel_width, 400.0, 2.0, YELLOW);

        let character = Character::get_by_id(self.selected_character_id);
        draw_text("CHARACTER", char_panel_x + 10.0, char_panel_y + 30.0, 25.0, YELLOW);
        draw_text(character.name, char_panel_x + 10.0, char_panel_y + 60.0, 30.0, WHITE);

        // Character simple visual
        let char_x = char_panel_x + char_panel_width * 0.5;
        let char_y = char_panel_y + 150.0;
        draw_circle(char_x, char_y, 30.0, SKYBLUE);
        draw_rectangle(char_x - 20.0, char_y + 20.0, 40.0, 50.0, SKYBLUE);
        draw_rectangle(char_x - 35.0, char_y + 30.0, 15.0, 40.0, SKYBLUE);
        draw_rectangle(char_x + 20.0, char_y + 30.0, 15.0, 40.0, SKYBLUE);
        draw_rectangle(char_x - 15.0, char_y + 70.0, 12.0, 45.0, SKYBLUE);
        draw_rectangle(char_x + 3.0, char_y + 70.0, 12.0, 45.0, SKYBLUE);

        // Skill points available
        let available_points = self.skill_tree_manager.get_available_points(self.selected_character_id);
        draw_text(
            &format!("Skill Points: {}", available_points),
            char_panel_x + 10.0,
            char_panel_y + 270.0,
            22.0,
            if available_points > 0 { GREEN } else { GRAY },
        );

        // Total points info (just show it's a progression system)
        draw_text(
            "Upgrade skills to grow stronger!",
            char_panel_x + 10.0,
            char_panel_y + 300.0,
            16.0,
            LIGHTGRAY,
        );

        // Branch tabs (top center)
        let tab_width = 200.0;
        let tab_height = 50.0;
        let tab_spacing = 10.0;
        let tabs_start_x = sw * 0.5 - (tab_width * 1.5 + tab_spacing);
        let tabs_y = 100.0;

        let branches = [
            (BranchSelection::Offense, "OFFENSE", RED),
            (BranchSelection::Defense, "DEFENSE", BLUE),
            (BranchSelection::Utility, "UTILITY", GREEN),
        ];

        for (i, (branch_type, name, color)) in branches.iter().enumerate() {
            let tab_x = tabs_start_x + i as f32 * (tab_width + tab_spacing);
            let is_selected = *branch_type == self.selected_branch;

            let bg_color = if is_selected {
                Color::new(color.r, color.g, color.b, 0.3)
            } else {
                Color::new(0.1, 0.1, 0.1, 0.5)
            };

            draw_rectangle(tab_x, tabs_y, tab_width, tab_height, bg_color);
            draw_rectangle_lines(
                tab_x,
                tabs_y,
                tab_width,
                tab_height,
                if is_selected { 3.0 } else { 1.0 },
                *color,
            );

            let text_dims = measure_text(name, None, 20, 1.0);
            draw_text(
                name,
                tab_x + tab_width * 0.5 - text_dims.width * 0.5,
                tabs_y + 32.0,
                20.0,
                if is_selected { WHITE } else { GRAY },
            );
        }

        // Skill tree panel (center/right)
        let tree_panel_x = char_panel_x + char_panel_width + 20.0;
        let tree_panel_y = tabs_y + tab_height + 20.0;
        let tree_panel_width = sw - tree_panel_x - 20.0;
        let tree_panel_height = sh - tree_panel_y - 80.0;

        draw_rectangle_lines(tree_panel_x, tree_panel_y, tree_panel_width, tree_panel_height, 2.0, match self.selected_branch {
            BranchSelection::Offense => RED,
            BranchSelection::Defense => BLUE,
            BranchSelection::Utility => GREEN,
        });

        // Render skill nodes
        if let Some(branch) = self.get_current_branch() {
            // Branch name and description
            draw_text(&branch.name, tree_panel_x + 20.0, tree_panel_y + 30.0, 28.0, YELLOW);
            draw_text(&branch.description, tree_panel_x + 20.0, tree_panel_y + 55.0, 16.0, LIGHTGRAY);

            // Render nodes in a grid
            let node_size = 100.0;
            let node_spacing = 30.0;
            let nodes_start_x = tree_panel_x + 50.0;
            let nodes_start_y = tree_panel_y + 100.0;

            for (i, node) in branch.nodes.iter().enumerate() {
                let col = i % 4;
                let row = i / 4;
                let node_x = nodes_start_x + col as f32 * (node_size + node_spacing);
                let node_y = nodes_start_y + row as f32 * (node_size + node_spacing);

                // Node background
                let node_color = if i == self.selected_node {
                    Color::new(0.3, 0.3, 0.4, 1.0)
                } else if node.unlocked && node.current_level > 0 {
                    Color::new(0.2, 0.3, 0.2, 1.0)
                } else if node.unlocked {
                    Color::new(0.2, 0.2, 0.2, 1.0)
                } else {
                    Color::new(0.1, 0.1, 0.1, 0.8)
                };

                draw_rectangle(node_x, node_y, node_size, node_size, node_color);

                let border_color = if i == self.selected_node {
                    YELLOW
                } else if node.unlocked && node.current_level > 0 {
                    GREEN
                } else if node.unlocked {
                    WHITE
                } else {
                    DARKGRAY
                };

                draw_rectangle_lines(node_x, node_y, node_size, node_size, 2.0, border_color);

                // Node tier indicator
                draw_text(
                    &format!("T{}", node.tier),
                    node_x + 5.0,
                    node_y + 18.0,
                    14.0,
                    GRAY,
                );

                // Node name (truncate if too long)
                let name_display = if node.name.len() > 12 {
                    format!("{}...", &node.name[..9])
                } else {
                    node.name.clone()
                };
                draw_text(&name_display, node_x + 5.0, node_y + 42.0, 14.0, WHITE);

                // Level indicator
                let level_text = format!("{}/{}", node.current_level, node.max_level);
                draw_text(&level_text, node_x + 5.0, node_y + 62.0, 16.0, if node.current_level == node.max_level { GOLD } else { YELLOW });

                // Cost
                if node.current_level < node.max_level {
                    draw_text(
                        &format!("Cost: {}", node.skill_point_cost),
                        node_x + 5.0,
                        node_y + 82.0,
                        12.0,
                        if available_points >= node.skill_point_cost { GREEN } else { RED },
                    );
                } else {
                    draw_text("MAX", node_x + 30.0, node_y + 82.0, 14.0, GOLD);
                }
            }

            // Selected node details panel
            if let Some(node) = branch.nodes.get(self.selected_node) {
                let detail_panel_y = tree_panel_y + tree_panel_height - 180.0;
                draw_rectangle(
                    tree_panel_x + 20.0,
                    detail_panel_y,
                    tree_panel_width - 40.0,
                    160.0,
                    Color::new(0.1, 0.1, 0.15, 0.9),
                );
                draw_rectangle_lines(
                    tree_panel_x + 20.0,
                    detail_panel_y,
                    tree_panel_width - 40.0,
                    160.0,
                    2.0,
                    YELLOW,
                );

                let detail_x = tree_panel_x + 30.0;
                let mut detail_y = detail_panel_y + 25.0;

                draw_text(&node.name, detail_x, detail_y, 22.0, YELLOW);
                detail_y += 30.0;

                draw_text(&node.description, detail_x, detail_y, 16.0, LIGHTGRAY);
                detail_y += 25.0;

                // Status
                let status_text = if !node.unlocked {
                    "LOCKED - Requires prerequisites"
                } else if node.current_level >= node.max_level {
                    "MAXED"
                } else if available_points < node.skill_point_cost {
                    "Need more skill points"
                } else {
                    "Press ENTER to upgrade"
                };

                let status_color = if !node.unlocked {
                    RED
                } else if node.current_level >= node.max_level {
                    GOLD
                } else if available_points >= node.skill_point_cost {
                    GREEN
                } else {
                    ORANGE
                };

                draw_text(status_text, detail_x, detail_y, 18.0, status_color);
                detail_y += 25.0;

                // Prerequisites
                if !node.prerequisites.is_empty() && !node.unlocked {
                    let prereq_text = format!("Requires: {:?}", node.prerequisites);
                    draw_text(&prereq_text[..prereq_text.len().min(60)], detail_x, detail_y, 14.0, RED);
                }
            }
        }

        // Feedback message
        if let Some((ref message, timer)) = self.feedback_message {
            let alpha = (timer / 3.0).min(1.0);
            let msg_size = 25.0;
            let msg_dims = measure_text(message, None, msg_size as u16, 1.0);
            draw_text(
                message,
                sw * 0.5 - msg_dims.width * 0.5,
                sh * 0.5 - 100.0,
                msg_size,
                Color::new(1.0, 1.0, 0.0, alpha),
            );
        }
    }

    fn handle_input(&mut self) {
        // Character selection
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            if self.selected_character > 0 {
                self.selected_character -= 1;
            } else {
                self.selected_character = CHARACTERS.len() - 1;
            }
            self.selected_character_id = CHARACTERS[self.selected_character].id;
            self.selected_node = 0;
        }

        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            self.selected_character = (self.selected_character + 1) % CHARACTERS.len();
            self.selected_character_id = CHARACTERS[self.selected_character].id;
            self.selected_node = 0;
        }

        // Branch selection
        if is_key_pressed(KeyCode::Q) {
            self.selected_branch = match self.selected_branch {
                BranchSelection::Offense => BranchSelection::Utility,
                BranchSelection::Defense => BranchSelection::Offense,
                BranchSelection::Utility => BranchSelection::Defense,
            };
            self.selected_node = 0;
        }

        if is_key_pressed(KeyCode::E) {
            self.selected_branch = match self.selected_branch {
                BranchSelection::Offense => BranchSelection::Defense,
                BranchSelection::Defense => BranchSelection::Utility,
                BranchSelection::Utility => BranchSelection::Offense,
            };
            self.selected_node = 0;
        }

        // Node selection - get branch length first to avoid borrow issues
        let branch_len = self.get_current_branch().map(|b| b.nodes.len()).unwrap_or(0);

        if branch_len > 0 {
            if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                if self.selected_node >= 4 {
                    self.selected_node -= 4;
                }
            }

            if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                if self.selected_node + 4 < branch_len {
                    self.selected_node += 4;
                }
            }

            // Upgrade node - get skill ID first to avoid borrow issues
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::J) {
                let skill_id = self.get_current_branch()
                    .and_then(|b| b.nodes.get(self.selected_node))
                    .map(|n| n.id.clone());

                if let Some(id) = skill_id {
                    if self.skill_tree_manager.upgrade_skill(self.selected_character_id, &id) {
                        self.show_feedback("Skill upgraded!".to_string());
                    } else {
                        self.show_feedback("Cannot upgrade skill".to_string());
                    }
                }
            }
        }

        // Exit
        if is_key_pressed(KeyCode::Escape) {
            self.transition_to = Some(StateType::Menu);
        }

        // Give test skill points (for testing)
        if is_key_pressed(KeyCode::P) {
            self.skill_tree_manager.add_skill_points(self.selected_character_id, 5);
            self.show_feedback("Added 5 skill points for testing!".to_string());
        }
    }

    fn should_transition(&self) -> Option<StateType> {
        self.transition_to
    }
}
