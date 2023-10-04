mod state_machine;
use crate::state_machine::StateMachine;

mod job;
use crate::job::simple_handler::SimpleHandler;
use crate::job::sample::{GreeterJob, SumJob};

use std::time::Duration;
use std::thread;
use std::thread::sleep;

use std::sync::Arc;

fn main() {
    run_state_machine_example();
    run_simpler_handler_example();
}

fn run_state_machine_example() {
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

fn run_simpler_handler_example() {
    // Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot
    // generally obtain a mutable reference to something inside an Arc. 
    // 
    // If you need to mutate through an Arc, use Mutex, RwLock, or one of the Atomic types.
    let simple_handler = Arc::new(SimpleHandler::new());

    // close the job queue after 3s
    let cloned = simple_handler.clone();
    thread::spawn(move || {
        let sec = 3;
        println!("INFO: Closing job queue in {}s...", sec);
        sleep(Duration::from_secs(sec));
        println!("INFO: Closing job queue");
        cloned.close();
    });

    // Job Handler logic
    let cloned = simple_handler.clone();
    let handler = thread::spawn(move || {
        cloned.listen();
    });

    // creates 5 greeter jobs
    let names = vec!["John", "Jane", "Jack", "Jill", "Joe"];
    for name in names {
        let job = Box::new(GreeterJob {
            name: String::from(name),
        });

        let cloned = simple_handler.clone();
        thread::spawn(move || {
            cloned.enqueue(job);
        });
    }

    // creates 5 sum jobs
    for i in 1..6 {
        let job = Box::new(SumJob {
            a: i,
            b: i + 1,
        });

        let cloned = simple_handler.clone();
        thread::spawn(move || {
            cloned.enqueue(job);
        });
    }

    handler.join().unwrap();
}