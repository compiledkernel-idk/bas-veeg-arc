use macroquad::prelude::*;

pub struct MenuUI {
    pub selected_index: usize,
    pub menu_items: Vec<MenuItem>,
    pub transition_timer: f32,
    pub background_scroll: f32,
}

#[derive(Clone)]
pub struct MenuItem {
    pub text: String,
    pub enabled: bool,
    pub action: MenuAction,
    pub position: Vec2,
    pub target_position: Vec2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    StartGame,
    Settings,
    Training,
    Versus,
    Credits,
    Exit,
    Back,
    None,
}

impl MenuUI {
    pub fn new(items: Vec<String>) -> Self {
        let mut menu_items = Vec::new();
        let start_y = 300.0;
        let spacing = 60.0;

        for (i, text) in items.into_iter().enumerate() {
            let action = match text.as_str() {
                "START GAME" | "STORY" => MenuAction::StartGame,
                "SETTINGS" => MenuAction::Settings,
                "TRAINING" => MenuAction::Training,
                "VERSUS" => MenuAction::Versus,
                "CREDITS" => MenuAction::Credits,
                "EXIT" | "QUIT" => MenuAction::Exit,
                "BACK" => MenuAction::Back,
                _ => MenuAction::None,
            };

            let y = start_y + i as f32 * spacing;
            menu_items.push(MenuItem {
                text,
                enabled: true,
                action,
                position: Vec2::new(screen_width() * 0.5, y),
                target_position: Vec2::new(screen_width() * 0.5, y),
            });
        }

        Self {
            selected_index: 0,
            menu_items,
            transition_timer: 0.0,
            background_scroll: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.transition_timer += dt;
        self.background_scroll += dt * 20.0;

        for item in &mut self.menu_items {
            item.position = item.position.lerp(item.target_position, 0.1);
        }
    }

    pub fn navigate_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.menu_items.len() - 1;
        }
    }

    pub fn navigate_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.menu_items.len();
    }

    pub fn get_selected_action(&self) -> MenuAction {
        if self.selected_index < self.menu_items.len() {
            self.menu_items[self.selected_index].action.clone()
        } else {
            MenuAction::None
        }
    }

    pub fn render(&self) {
        self.render_background();

        for (i, item) in self.menu_items.iter().enumerate() {
            let is_selected = i == self.selected_index;
            let color = if !item.enabled {
                GRAY
            } else if is_selected {
                YELLOW
            } else {
                WHITE
            };

            let size = if is_selected { 45.0 } else { 35.0 };
            let text_dims = measure_text(&item.text, None, size as u16, 1.0);
            let x = item.position.x - text_dims.width * 0.5;

            if is_selected {
                let pulse = (self.transition_timer * 3.0).sin() * 0.5 + 0.5;
                let glow_color = Color::new(1.0, 1.0, 0.0, 0.2 * pulse);
                draw_rectangle(
                    x - 30.0,
                    item.position.y - size * 0.8,
                    text_dims.width + 60.0,
                    size + 10.0,
                    glow_color,
                );

                draw_circle(x - 50.0, item.position.y - size * 0.3, 5.0, YELLOW);
                draw_circle(
                    x + text_dims.width + 50.0,
                    item.position.y - size * 0.3,
                    5.0,
                    YELLOW,
                );
            }

            draw_text(&item.text, x, item.position.y, size, color);
        }
    }

    fn render_background(&self) {
        for i in 0..30 {
            for j in 0..20 {
                let x = (i as f32 * 80.0 - self.background_scroll) % screen_width();
                let y = j as f32 * 80.0;
                let alpha = ((i + j) % 2) as f32 * 0.05 + 0.02;
                draw_rectangle(x, y, 40.0, 40.0, Color::new(0.2, 0.1, 0.3, alpha));
            }
        }
    }
}
