mod state_machine;
use crate::state_machine::StateMachine;

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
