use super::player_manager::PlayerSlot;
use macroquad::prelude::*;

/// Input device types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputDevice {
    Keyboard,
    Gamepad(u8),
}

/// Handles input for multiple players with different input devices
pub struct CoopInputHandler {
    keyboard_bindings: KeyboardBindings,
    gamepad_bindings: GamepadBindings,
    last_gamepad_count: u8,
}

/// Keyboard bindings for player 1
#[derive(Clone)]
pub struct KeyboardBindings {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub light_attack: KeyCode,
    pub heavy_attack: KeyCode,
    pub special_attack: KeyCode,
    pub ability: KeyCode,
    pub dodge: KeyCode,
    pub block: KeyCode,
    pub interact: KeyCode, // For reviving
    pub pause: KeyCode,
}

/// Gamepad bindings (standard layout)
#[derive(Clone)]
pub struct GamepadBindings {
    pub light_attack: GamepadButton,
    pub heavy_attack: GamepadButton,
    pub special_attack: GamepadButton,
    pub ability: GamepadButton,
    pub dodge: GamepadButton,
    pub block: GamepadButton,
    pub interact: GamepadButton,
    pub pause: GamepadButton,
}

/// Gamepad button identifiers (Xbox/PlayStation style)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamepadButton {
    A,      // Xbox A / PS Cross
    B,      // Xbox B / PS Circle
    X,      // Xbox X / PS Square
    Y,      // Xbox Y / PS Triangle
    LB,     // Left Bumper / L1
    RB,     // Right Bumper / R1
    LT,     // Left Trigger / L2
    RT,     // Right Trigger / R2
    Start,  // Start / Options
    Select, // Select / Share
}

/// Player input state
#[derive(Default, Clone)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub special_attack: bool,
    pub ability: bool,
    pub dodge: bool,
    pub block: bool,
    pub interact: bool,
    pub pause: bool,

    // Pressed this frame
    pub light_attack_pressed: bool,
    pub heavy_attack_pressed: bool,
    pub special_attack_pressed: bool,
    pub ability_pressed: bool,
    pub dodge_pressed: bool,
    pub block_pressed: bool,
    pub interact_pressed: bool,
    pub pause_pressed: bool,
}

impl CoopInputHandler {
    pub fn new() -> Self {
        Self {
            keyboard_bindings: KeyboardBindings::default(),
            gamepad_bindings: GamepadBindings::default(),
            last_gamepad_count: 0,
        }
    }

    /// Get input for a specific player
    pub fn get_player_input(&mut self, slot: PlayerSlot, device: InputDevice) -> PlayerInput {
        match device {
            InputDevice::Keyboard => self.get_keyboard_input(),
            InputDevice::Gamepad(id) => self.get_gamepad_input(id),
        }
    }

    /// Get keyboard input (Player 1 only)
    fn get_keyboard_input(&self) -> PlayerInput {
        let bindings = &self.keyboard_bindings;

        let mut movement = Vec2::ZERO;
        if is_key_down(bindings.move_up) {
            movement.y -= 1.0;
        }
        if is_key_down(bindings.move_down) {
            movement.y += 1.0;
        }
        if is_key_down(bindings.move_left) {
            movement.x -= 1.0;
        }
        if is_key_down(bindings.move_right) {
            movement.x += 1.0;
        }

        // Normalize movement
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        PlayerInput {
            movement,
            light_attack: is_key_down(bindings.light_attack),
            heavy_attack: is_key_down(bindings.heavy_attack),
            special_attack: is_key_down(bindings.special_attack),
            ability: is_key_down(bindings.ability),
            dodge: is_key_down(bindings.dodge),
            block: is_key_down(bindings.block),
            interact: is_key_down(bindings.interact),
            pause: is_key_down(bindings.pause),

            light_attack_pressed: is_key_pressed(bindings.light_attack),
            heavy_attack_pressed: is_key_pressed(bindings.heavy_attack),
            special_attack_pressed: is_key_pressed(bindings.special_attack),
            ability_pressed: is_key_pressed(bindings.ability),
            dodge_pressed: is_key_pressed(bindings.dodge),
            block_pressed: is_key_pressed(bindings.block),
            interact_pressed: is_key_pressed(bindings.interact),
            pause_pressed: is_key_pressed(bindings.pause),
        }
    }

    /// Get gamepad input
    fn get_gamepad_input(&self, gamepad_id: u8) -> PlayerInput {
        // Check if gamepad is connected
        // Note: Macroquad has limited gamepad support, this is a simplified implementation
        // In a full implementation, you'd use a proper gamepad library

        let mut movement = Vec2::ZERO;

        // Simulated gamepad input - in reality you'd query actual gamepad state
        // This is a placeholder for demonstration
        let bindings = &self.gamepad_bindings;

        PlayerInput {
            movement,
            light_attack: false,
            heavy_attack: false,
            special_attack: false,
            ability: false,
            dodge: false,
            block: false,
            interact: false,
            pause: false,

            light_attack_pressed: false,
            heavy_attack_pressed: false,
            special_attack_pressed: false,
            ability_pressed: false,
            dodge_pressed: false,
            block_pressed: false,
            interact_pressed: false,
            pause_pressed: false,
        }
    }

    /// Check for new gamepad connections (drop-in)
    pub fn check_for_new_gamepads(&mut self) -> Vec<u8> {
        // Simplified - in real implementation would detect actual gamepad connections
        Vec::new()
    }

    /// Check for gamepad disconnections (drop-out)
    pub fn check_for_disconnected_gamepads(&mut self) -> Vec<u8> {
        // Simplified - in real implementation would detect gamepad disconnections
        Vec::new()
    }

    /// Get movement stick input for gamepad
    fn get_gamepad_stick(&self, gamepad_id: u8, stick: GamepadStick) -> Vec2 {
        // Placeholder - would query actual gamepad state
        Vec2::ZERO
    }

    /// Check if gamepad button is pressed
    fn is_gamepad_button_down(&self, gamepad_id: u8, button: GamepadButton) -> bool {
        // Placeholder - would query actual gamepad state
        false
    }

    /// Check if gamepad button was just pressed
    fn is_gamepad_button_pressed(&self, gamepad_id: u8, button: GamepadButton) -> bool {
        // Placeholder - would query actual gamepad state
        false
    }

    /// Remap keyboard bindings
    pub fn remap_keyboard(&mut self, bindings: KeyboardBindings) {
        self.keyboard_bindings = bindings;
    }

    /// Remap gamepad bindings
    pub fn remap_gamepad(&mut self, bindings: GamepadBindings) {
        self.gamepad_bindings = bindings;
    }
}

/// Gamepad stick identifiers
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamepadStick {
    LeftStick,
    RightStick,
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        Self {
            move_up: KeyCode::W,
            move_down: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            light_attack: KeyCode::J,
            heavy_attack: KeyCode::K,
            special_attack: KeyCode::L,
            ability: KeyCode::E,
            dodge: KeyCode::Space,
            block: KeyCode::LeftShift,
            interact: KeyCode::F,
            pause: KeyCode::Escape,
        }
    }
}

impl Default for GamepadBindings {
    fn default() -> Self {
        Self {
            light_attack: GamepadButton::X,     // Xbox X / PS Square
            heavy_attack: GamepadButton::Y,     // Xbox Y / PS Triangle
            special_attack: GamepadButton::B,   // Xbox B / PS Circle
            ability: GamepadButton::A,          // Xbox A / PS Cross
            dodge: GamepadButton::RB,           // Right Bumper
            block: GamepadButton::LB,           // Left Bumper
            interact: GamepadButton::A,         // Xbox A / PS Cross
            pause: GamepadButton::Start,        // Start button
        }
    }
}

impl KeyboardBindings {
    /// Alternative WASD + Arrow keys layout for 2-player keyboard
    pub fn arrows_layout() -> Self {
        Self {
            move_up: KeyCode::Up,
            move_down: KeyCode::Down,
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
            light_attack: KeyCode::KpMultiply,  // Numpad *
            heavy_attack: KeyCode::Minus,       // Regular minus key
            special_attack: KeyCode::Equal,     // Regular plus/equal key
            ability: KeyCode::RightShift,
            dodge: KeyCode::KpDecimal,          // Numpad .
            block: KeyCode::KpEnter,
            interact: KeyCode::Kp0,
            pause: KeyCode::Escape,
        }
    }
}
