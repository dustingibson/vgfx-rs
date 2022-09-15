

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