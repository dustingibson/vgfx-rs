use gl;
use gl::types::*;
use std::collections::HashMap;


extern crate nalgebra_glm as glm;

pub struct State {
    states: HashMap<String, State>,
    name: String
}


impl State {

    pub fn new(name: String) -> State {
        return State {
            states: HashMap::new(),
            name: name
        }
    }

    pub fn addNewState(&mut self, transition: String, stateName: String) {
        self.states.insert(transition, State::new(stateName));
    }

    pub fn addState(&mut self, transition: String, curState: State) {
        self.states.insert(transition, curState);
    }

    fn test() -> String {
        return "blah".to_string();
    }
}