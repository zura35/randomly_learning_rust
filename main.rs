use std::collections::HashMap;
use std::collections::HashSet;


pub struct StateMachine {
    state: String,
    start: String,
    end: String,
    registry: HashMap<String, HashSet<String>>,
}

impl StateMachine {
    pub fn new() -> Self {
        let empty_state: String = String::from("EMPTY");

        StateMachine {
            state: empty_state.clone(),
            start: empty_state.clone(),
            end: empty_state.clone(),
            registry: HashMap::new(),
        }
    }

    pub fn set_start(&mut self, start: &str) {
        self.start = start.to_string();
        if self.state == "EMPTY" {
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

fn main() {
    let mut m = StateMachine::new();

    let initial_str = "initial".to_string();
    let started_str = "started".to_string();
    let completed_str = "completed".to_string();
    let failed_str = "failed".to_string();

    m.set_start(&initial_str);
    m.set_end(&completed_str);
    m.add_transition(&initial_str, &started_str);
    m.add_transition(&started_str, &completed_str);
    m.add_transition(&started_str, &failed_str);
    m.print_state();
    assert_eq!(m.state, initial_str);

    m.transit_to(&started_str);
    m.print_state();
    assert_eq!(m.state, started_str);

    m.transit_to(&initial_str);
    m.print_state();
    assert_eq!(m.state, started_str);
}
