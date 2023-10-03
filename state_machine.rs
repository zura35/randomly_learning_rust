use std::collections::HashMap;
use std::collections::HashSet;

pub struct StateMachine {
    pub state: String,
    start: String,
    end: String,
    registry: HashMap<String, HashSet<String>>,
}

const EMPTY_STATE: &str = "empty";

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: EMPTY_STATE.to_string(),
            start: EMPTY_STATE.to_string(),
            end: EMPTY_STATE.to_string(),
            registry: HashMap::new(),
        }
    }

    pub fn set_start(&mut self, start: &str) {
        self.start = start.to_string();
        if self.state == EMPTY_STATE {
            self.state = start.to_string();
        }
    }

    pub fn set_end(&mut self, end: &str) {
        self.end = end.to_string();
    }

    pub fn add_transition(&mut self, from: &str, to: &str) {
        let from = from.to_string();
        let to = to.to_string();

        if !self.registry.contains_key(&from) {
            let mut new_hash = HashSet::new();
            new_hash.insert(to);
            self.registry.insert(from, new_hash);
        } else {
            let to_states = self.registry.get_mut(&from).unwrap();
            to_states.insert(to);
        }

        println!("registered transitions: {:?}", self.registry);
    }

    pub fn transit_to(&mut self, to: &str) {
        let to = to.to_string();

        if !self.registry.contains_key(&self.state) {
            println!("ERR: Invalid transition from {} to {}", self.state, to);
            return;
        }

        let to_states = &self.registry[&self.state];
        if to_states.contains(&to) {
            self.state = to;
            return;
        }

        println!("ERR: Invalid transition from {} to {}", self.state, to);
    }

    pub fn print_state(&self) {
        println!("Current state: {}", self.state);
    }
}
