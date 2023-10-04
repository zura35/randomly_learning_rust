use std::sync::{Arc, RwLock};
use std::thread::{self, sleep};
use std::collections::LinkedList;
use std::time::Duration;

pub trait JobInterface {
    fn run(&self);
}

pub struct GreeterJob {
    name: String,
}

impl JobInterface for GreeterJob {
    fn run(&self) {
        println!("Hello, {}!", self.name);
    }
}

pub struct SumJob {
    a: i32,
    b: i32,
}

impl JobInterface for SumJob {
    fn run(&self) {
        println!("{} + {} = {}", self.a, self.b, self.a + self.b);
    }
}

pub struct JobQueue {
    queue: LinkedList<Box<dyn JobInterface + Send + Sync>>,
}

impl JobQueue {
    pub fn new() -> JobQueue {
        JobQueue {
            queue: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, job: Box<dyn JobInterface + Send + Sync> ) {
        self.queue.push_back(job);

        println!("Job enqueued");
    }

    pub fn dequeue_and_run(&mut self) {
        if let Some(job) = self.queue.pop_front() {
            thread::spawn(move || {
                job.run();
            });
        }
    }
}

pub fn run() {
    // create a job_queue shared between threads
    let job_queue = Arc::new(RwLock::new(JobQueue::new()));
    
    let executor = job_queue.clone();

    // Job Handler logic
    let handler = thread::spawn(move || {
        println!("Handler started");

        let mut count = 0;
        loop {
            if executor.read().unwrap().queue.is_empty() {
                sleep(Duration::from_millis(100));
                count += 1;
                println!("Handler waiting");
            } else {
                let mut executor = executor.write().unwrap();
                executor.dequeue_and_run();
            }

            if count >= 10 {
                println!("Handler exiting");
                break;
            }
        }
    });

    // creates 5 greeter jobs
    let names = vec!["John", "Jane", "Jack", "Jill", "Joe"];
    for name in names {
        let job = Box::new(GreeterJob {
            name: String::from(name),
        });

        let enqueuer = job_queue.clone();
        thread::spawn(move || {
            let mut job_queue = enqueuer.write().unwrap();

            job_queue.enqueue(job);
        });
    }

    // creates 5 sum jobs
    for i in 1..6 {
        let job = Box::new(SumJob {
            a: i,
            b: i + 1,
        });

        let enqueuer = job_queue.clone();
        thread::spawn(move || {
            let mut job_queue = enqueuer.write().unwrap();

            job_queue.enqueue(job);
        });
    }

    handler.join().unwrap();
}
