use macroquad::prelude::*;

pub struct PauseMenu {
    pub active: bool,
    pub selected_option: usize,
    pub options: Vec<String>,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            active: false,
            selected_option: 0,
            options: vec![
                "RESUME".to_string(),
                "RESTART".to_string(),
                "SETTINGS".to_string(),
                "QUIT TO MENU".to_string(),
            ],
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            self.selected_option = 0;
        }
    }

    pub fn navigate_up(&mut self) {
        if self.selected_option > 0 {
            self.selected_option -= 1;
        } else {
            self.selected_option = self.options.len() - 1;
        }
    }

    pub fn navigate_down(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }

    pub fn render(&self) {
        if !self.active {
            return;
        }

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        let box_width = 400.0;
        let box_height = 300.0;
        let box_x = screen_width() * 0.5 - box_width * 0.5;
        let box_y = screen_height() * 0.5 - box_height * 0.5;

        draw_rectangle(
            box_x,
            box_y,
            box_width,
            box_height,
            Color::new(0.1, 0.1, 0.15, 0.95),
        );
        draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, WHITE);

        let title = "PAUSED";
        let title_size = 50.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() * 0.5 - title_dims.width * 0.5,
            box_y + 60.0,
            title_size,
            WHITE,
        );

        for (i, option) in self.options.iter().enumerate() {
            let y = box_y + 120.0 + i as f32 * 40.0;
            let size = 30.0;
            let color = if i == self.selected_option {
                YELLOW
            } else {
                WHITE
            };

            let text_dims = measure_text(option, None, size as u16, 1.0);
            let x = screen_width() * 0.5 - text_dims.width * 0.5;

            if i == self.selected_option {
                draw_rectangle(
                    x - 20.0,
                    y - size * 0.8,
                    text_dims.width + 40.0,
                    size + 5.0,
                    Color::new(1.0, 1.0, 0.0, 0.1),
                );
            }

            draw_text(option, x, y, size, color);
        }
    }

    pub fn get_selected_action(&self) -> PauseAction {
        match self.selected_option {
            0 => PauseAction::Resume,
            1 => PauseAction::Restart,
            2 => PauseAction::Settings,
            3 => PauseAction::QuitToMenu,
            _ => PauseAction::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PauseAction {
    Resume,
    Restart,
    Settings,
    QuitToMenu,
    None,
}
