use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub enum InputAction {
    Left,
    Right,
    Up,
    Down,
    LightAttack,
    HeavyAttack,
    Special,
    Super,
    Parry,
    Dodge,
    Jump,
    Crouch,
}

#[derive(Clone, Debug)]
pub struct InputEvent {
    pub action: InputAction,
    pub timestamp: f64,
    pub pressed: bool,
}

pub struct InputManager {
    buffer: VecDeque<InputEvent>,
    buffer_window: f64,
    current_state: InputState,
}

#[derive(Clone, Debug)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub special: bool,
    pub super_move: bool,
    pub parry: bool,
    pub dodge: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            buffer_window: 0.2,
            current_state: InputState {
                left: false,
                right: false,
                up: false,
                down: false,
                light_attack: false,
                heavy_attack: false,
                special: false,
                super_move: false,
                parry: false,
                dodge: false,
            },
        }
    }

    pub fn update(&mut self) {
        let current_time = get_time();

        self.current_state.left = is_key_down(KeyCode::A);
        self.current_state.right = is_key_down(KeyCode::D);
        self.current_state.up = is_key_down(KeyCode::W);
        self.current_state.down = is_key_down(KeyCode::S);

        if is_key_pressed(KeyCode::A) {
            self.add_input(InputAction::Left, current_time, true);
        }
        if is_key_released(KeyCode::A) {
            self.add_input(InputAction::Left, current_time, false);
        }

        if is_key_pressed(KeyCode::D) {
            self.add_input(InputAction::Right, current_time, true);
        }
        if is_key_released(KeyCode::D) {
            self.add_input(InputAction::Right, current_time, false);
        }

        if is_key_pressed(KeyCode::W) {
            self.add_input(InputAction::Up, current_time, true);
            self.add_input(InputAction::Jump, current_time, true);
        }

        if is_key_pressed(KeyCode::S) {
            self.add_input(InputAction::Down, current_time, true);
            self.add_input(InputAction::Crouch, current_time, true);
        }

        if is_key_pressed(KeyCode::J) {
            self.current_state.light_attack = true;
            self.add_input(InputAction::LightAttack, current_time, true);
        }

        if is_key_pressed(KeyCode::K) {
            self.current_state.heavy_attack = true;
            self.add_input(InputAction::HeavyAttack, current_time, true);
        }

        if is_key_pressed(KeyCode::L) {
            self.current_state.special = true;
            self.add_input(InputAction::Special, current_time, true);
        }

        if is_key_pressed(KeyCode::U) {
            self.current_state.super_move = true;
            self.add_input(InputAction::Super, current_time, true);
        }

        if is_key_pressed(KeyCode::I) {
            self.current_state.parry = true;
            self.add_input(InputAction::Parry, current_time, true);
        }

        if is_key_pressed(KeyCode::O) {
            self.current_state.dodge = true;
            self.add_input(InputAction::Dodge, current_time, true);
        }

        self.clean_buffer(current_time);
    }

    fn add_input(&mut self, action: InputAction, timestamp: f64, pressed: bool) {
        self.buffer.push_back(InputEvent {
            action,
            timestamp,
            pressed,
        });

        if self.buffer.len() > 20 {
            self.buffer.pop_front();
        }
    }

    fn clean_buffer(&mut self, current_time: f64) {
        while let Some(event) = self.buffer.front() {
            if current_time - event.timestamp > self.buffer_window {
                self.buffer.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn check_sequence(&self, sequence: &[InputAction], window: f64) -> bool {
        if sequence.is_empty() || self.buffer.len() < sequence.len() {
            return false;
        }

        let current_time = get_time();
        let mut matched = 0;
        let mut last_time = 0.0;

        for event in self.buffer.iter().rev() {
            if current_time - event.timestamp > window {
                break;
            }

            if event.pressed && event.action == sequence[matched] {
                if matched == 0 || event.timestamp > last_time {
                    last_time = event.timestamp;
                    matched += 1;

                    if matched == sequence.len() {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_state(&self) -> &InputState {
        &self.current_state
    }

    pub fn consume_input(&mut self, action: InputAction) {
        self.buffer.retain(|event| event.action != action);

        match action {
            InputAction::LightAttack => self.current_state.light_attack = false,
            InputAction::HeavyAttack => self.current_state.heavy_attack = false,
            InputAction::Special => self.current_state.special = false,
            InputAction::Super => self.current_state.super_move = false,
            InputAction::Parry => self.current_state.parry = false,
            InputAction::Dodge => self.current_state.dodge = false,
            _ => {}
        }
    }
}
