use std::collections::HashMap;

pub struct StateMachine {
    state: String,
    start: String,
    end: String,
    registry: HashMap<String, Vec<String>>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: String::from("initial"),
            start: String::from("initial"),
            end: String::from("initial"),
            registry: HashMap::new(),
        }
    }

    pub fn set_start(&mut self, start: String) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: String) {
        self.end = end;
    }

    pub fn add_transition(&mut self, from: String, to: String) {
        if self.registry.contains_key(&from) {
            let to_states = self.registry.get(&from).unwrap();
            for state in to_states {
                if state == &to {
                    println!("DEBUG: Transition from {} to {} already exists", from, to);
                    return;
                }
            }

            let to_states = self.registry.get_mut(&from).unwrap();
            to_states.push(to);
        } else {
            let mut new_vector = Vec::new();
            new_vector.push(to);
            self.registry.insert(from, new_vector);
        }
        
        println!("registered transitions: {:?}", self.registry);
    }

    pub fn transit_to(&mut self, to: String) {
        if !self.registry.contains_key(&self.state) {
            println!("ERR: Invalid transition from {} to {}", self.state, to);
            return;
        }

        let to_states = self.registry.get(&self.state).unwrap();

        for state in to_states {
            if state == &to {
                self.state = to;
                return;
            }
        }

        println!("ERR: Invalid transition from {} to {}", self.state, to);
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }
}

fn main() {
    let mut m = StateMachine::new();
    m.set_end(String::from("completed"));
    m.add_transition(String::from("initial"), String::from("started"));
    m.add_transition(String::from("started"), String::from("completed"));
    m.add_transition(String::from("started"), String::from("failed"));
    println!("{}", m.state);

    m.transit_to(String::from("started"));
    println!("{}", m.state);

    m.transit_to(String::from("initial"));
    println!("{}", m.state);
}