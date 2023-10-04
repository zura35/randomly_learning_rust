mod state_machine;
use crate::state_machine::StateMachine;

mod job;
use crate::job::Handler;
use crate::job::sample::{GreeterJob, SumJob};

use std::time::Duration;
use std::thread;
use std::thread::sleep;

use std::sync::{Arc, Barrier};

fn main() {
    run_state_machine_example();
    run_job_handler_example();
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

fn run_job_handler_example() {
   let mut job_handler = Handler::new();
   
    // close the job queue after 5s
    let mut enqueuer = job_handler.enqueuer();
    thread::spawn(move || {
        println!("INFO: Closing job queue in 5s...");
        sleep(Duration::from_secs(5));
        println!("INFO: Closing job queue");
        enqueuer.close();
    });

    // Job Handler logic
    let mut job_listener = job_handler.listener();
    let handler = thread::spawn(move || {
        job_listener.listen();
    });

    // creates 5 greeter jobs
    let names = vec!["John", "Jane", "Jack", "Jill", "Joe"];
    for name in names {
        let job = Box::new(GreeterJob {
            name: String::from(name),
        });

        let mut enqueuer = job_handler.enqueuer();
        thread::spawn(move || {
            enqueuer.enqueue(job);
        });
    }

    // creates 5 sum jobs
    for i in 1..6 {
        let job = Box::new(SumJob {
            a: i,
            b: i + 1,
        });

        let mut enqueuer = job_handler.enqueuer();
        thread::spawn(move || {
            enqueuer.enqueue(job);
        });
    }

    handler.join().unwrap();
}