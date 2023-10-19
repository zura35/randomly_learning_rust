mod event_sourcing;
use crate::event_sourcing::order_aggregate;
use uuid::Uuid;
use chrono::Utc;

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
    run_order_event_sourcing_example();
    run_state_machine_example();
    run_simpler_handler_example();
}

fn run_order_event_sourcing_example() {
    let projector = order_aggregate::OrderAggregateProjector::new();
    let order_id = Uuid::new_v4();
    let created_event = order_aggregate::OrderEvent {
        id: Uuid::new_v4(),
        order_id: order_id,
        version: 1,
        created_at: Utc::now(),
        event_type: order_aggregate::OrderEventType::Created {
            user_id: Uuid::new_v4(),
            amount: 100.0,
        },
    };

    let updated_event = order_aggregate::OrderEvent {
        id: Uuid::new_v4(),
        order_id: order_id,
        version: 2,
        created_at: Utc::now(),
        event_type: order_aggregate::OrderEventType::Updated {
            amount: 200.0,
        },
    };

    let paid_event = order_aggregate::OrderEvent {
        id: Uuid::new_v4(),
        order_id: order_id,
        version: 3,
        created_at: Utc::now(),
        event_type: order_aggregate::OrderEventType::Paid,
    };

    let cancelled_event = order_aggregate::OrderEvent {
        id: Uuid::new_v4(),
        order_id: order_id,
        version: 4,
        created_at: Utc::now(),
        event_type: order_aggregate::OrderEventType::Cancelled,
    };

    let events = vec![
        created_event,
        updated_event,
        paid_event,
        cancelled_event,
    ];

    let order = projector.replay(events, 0);
    println!("INFO: Order: {:?}", order);
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