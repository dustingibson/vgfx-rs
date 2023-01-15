

use std::collections::HashMap;


extern crate nalgebra_glm as glm;

#[derive(Clone)]
pub struct State {
    name: String,
    transitions: HashMap<String, String>
}

impl State {

    pub fn new(name: String) -> Self {
        return  State {
            name: name,
            transitions: HashMap::new()
        }
    }

    pub fn add_transition(&mut self, transition_name: String, state_name: String) {
        self.transitions.insert(transition_name, state_name);
    }

    pub fn transition(&mut self, transition_name: String) -> String {
        return match self.transitions.get(&transition_name) {
            Some(s) => s.to_string(),
            None => panic!("transition doesn't exist for state")
        }
    }

}

#[derive(Clone)]
pub struct StateMachine {
    states: HashMap<String, State>,
    state_name: String
}

impl StateMachine {

    pub fn new(init_state_name: String) -> Self {
        return StateMachine {
            states: HashMap::new(),
            state_name: init_state_name
        }
    }

    pub fn get_state(&mut self) -> State {
        match self.states.get(&self.state_name) {
            Some(s) => s.clone(),
            None => panic!("state does not exist")
        }
    }

    pub fn add_state(&mut self, state_name: &String) {
        self.states.insert(state_name.to_string(), State::new(state_name.to_string()));
    }

    pub fn add_transition(&mut self, from_state_name: String, transition_name: String, to_state_name: String) {
        match self.states.get_mut(&from_state_name) {
            Some(s) => s.add_transition(transition_name, to_state_name),
            None => panic!("state does not exist")
        };
    }

    pub fn transition_state(&mut self, transition_name: String) {
        self.state_name = self.get_state().transition(transition_name);
    }
}

pub struct KeyState {
    pressed: StateMachine
}

pub struct SwitchState {
    switch: StateMachine
}

impl SwitchState {
    pub fn new() -> Self {
        let mut state_machine: StateMachine = StateMachine::new("off".to_string());
        state_machine.add_state(&"on".to_string());
        state_machine.add_state(&"off".to_string());
        state_machine.add_transition("on".to_string(), "switch".to_string(), "off".to_string());
        state_machine.add_transition("off".to_string(), "switch".to_string(), "on".to_string());
        return SwitchState {
            switch: state_machine
        }
    }

    pub fn flip(&mut self) {
        self.switch.transition_state("switch".to_string());
    }

    pub fn is_on(&mut self) -> bool {
        return self.switch.state_name == "on".to_string();
    }
}

impl KeyState {
    pub fn new() -> Self {
        let mut state_machine: StateMachine = StateMachine::new("not pressed".to_string());
        state_machine.add_state(&"not pressed".to_string());
        state_machine.add_state(&"press recorded".to_string());
        state_machine.add_state(&"press ignored".to_string());
        state_machine.add_transition("not pressed".to_string(), "press".to_string(), "press recorded".to_string());
        state_machine.add_transition("not pressed".to_string(), "release".to_string(), "not pressed".to_string());
        state_machine.add_transition("press recorded".to_string(), "press".to_string(), "press ignored".to_string());
        state_machine.add_transition("press recorded".to_string(), "release".to_string(), "not pressed".to_string());
        state_machine.add_transition("press ignored".to_string(), "press".to_string(), "press ignored".to_string());
        state_machine.add_transition("press ignored".to_string(), "release".to_string(), "not pressed".to_string());
        return KeyState {
            pressed: state_machine
        }
    }

    pub fn press(&mut self) {
        self.pressed.transition_state("press".to_string());
    }

    pub fn release(&mut self) {
        self.pressed.transition_state("release".to_string());
    }

    pub fn is_pressed(&mut self) -> bool {
        return self.pressed.state_name == "press recorded".to_string();
    }
}

pub struct DemoState {
    state: StateMachine
}

impl DemoState {
    pub fn new() -> Self {
        let mut state_machine: StateMachine = StateMachine::new("init".to_string());
        state_machine.add_state(&"init".to_string());
        state_machine.add_state(&"run".to_string());
        state_machine.add_transition("init".to_string(), "complete init".to_string(), "run".to_string());
        return DemoState {
            state: state_machine
        }
    }

    pub fn flip(&mut self) {
        self.state.transition_state("complete init".to_string());
    }

    pub fn is_initializing(&mut self) -> bool {
        return self.state.state_name == "init".to_string();
    }

    pub fn is_initialized(&mut self) -> bool {
        return self.state.state_name == "run".to_string();
    }
}