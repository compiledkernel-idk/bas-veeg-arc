pub mod boot;
pub mod cutscene;
pub mod gameplay;
pub mod menu;
pub mod results;
pub mod settings;
pub mod training;
pub mod versus;

use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum StateType {
    Boot,
    Menu,
    Gameplay,
    Cutscene,
    Training,
    Versus,
    Results,
    Settings,
}

pub trait State {
    fn enter(&mut self);
    fn exit(&mut self);
    fn update(&mut self, dt: f32);
    fn fixed_update(&mut self, dt: f64);
    fn render(&mut self, interpolation: f32);
    fn handle_input(&mut self);
    fn should_transition(&self) -> Option<StateType> {
        None
    }
}

pub struct StateManager {
    states: VecDeque<Box<dyn State>>,
    pending_transitions: Vec<StateTransition>,
    should_quit: bool,
}

pub enum StateTransition {
    Push(StateType),
    Pop,
    Replace(StateType),
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            states: VecDeque::new(),
            pending_transitions: Vec::new(),
            should_quit: false,
        }
    }

    pub fn push_state(&mut self, state_type: StateType) {
        let mut state = self.create_state(state_type);
        state.enter();
        self.states.push_back(state);
    }

    pub fn pop_state(&mut self) {
        if let Some(mut state) = self.states.pop_back() {
            state.exit();
        }

        if self.states.is_empty() {
            self.should_quit = true;
        }
    }

    pub fn replace_state(&mut self, state_type: StateType) {
        if let Some(mut old_state) = self.states.pop_back() {
            old_state.exit();
        }

        let mut new_state = self.create_state(state_type);
        new_state.enter();
        self.states.push_back(new_state);
    }

    pub fn update(&mut self, dt: f32) {
        self.process_transitions();

        if let Some(state) = self.states.back_mut() {
            state.handle_input();
            state.update(dt);

            if let Some(next_state) = state.should_transition() {
                self.pending_transitions
                    .push(StateTransition::Replace(next_state));
            }
        }
    }

    pub fn fixed_update(&mut self, dt: f64) {
        if let Some(state) = self.states.back_mut() {
            state.fixed_update(dt);
        }
    }

    pub fn render(&mut self, interpolation: f32) {
        if let Some(state) = self.states.back_mut() {
            state.render(interpolation);
        }
    }

    pub fn handle_escape(&mut self) {
        if self.states.len() > 1 {
            self.pending_transitions.push(StateTransition::Pop);
        } else {
            self.should_quit = true;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn process_transitions(&mut self) {
        let transitions = std::mem::take(&mut self.pending_transitions);

        for transition in transitions {
            match transition {
                StateTransition::Push(state_type) => self.push_state(state_type),
                StateTransition::Pop => self.pop_state(),
                StateTransition::Replace(state_type) => self.replace_state(state_type),
            }
        }
    }

    fn create_state(&self, state_type: StateType) -> Box<dyn State> {
        match state_type {
            StateType::Boot => Box::new(boot::BootState::new()),
            StateType::Menu => Box::new(menu::MenuState::new()),
            StateType::Gameplay => Box::new(gameplay::GameplayState::new()),
            StateType::Cutscene => Box::new(cutscene::CutsceneState::new()),
            StateType::Training => Box::new(training::TrainingState::new()),
            StateType::Versus => Box::new(versus::VersusState::new()),
            StateType::Results => Box::new(results::ResultsState::new()),
            StateType::Settings => Box::new(settings::SettingsState::new()),
        }
    }
}
