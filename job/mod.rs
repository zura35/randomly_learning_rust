use std::sync::{Arc, RwLock};
use std::collections::LinkedList;

use std::time::Duration;
use std::thread;
use std::thread::sleep;

pub mod sample;

pub struct Enqueuer {
    queue_manager: Arc<QueueManager>,
}

pub struct Listener {
    queue_manager: Arc<QueueManager>,
}

pub trait JobInterface {
    fn run(&self);
}

pub struct QueueManager {
    queue: RwLock<LinkedList<Box<dyn JobInterface + Send + Sync>>>,
    is_closed: RwLock<bool>,
}

impl QueueManager {
    fn is_closed(&self) -> bool {
        *self.is_closed.read().unwrap()
    }
}

impl Enqueuer {
    pub fn enqueue(&mut self, job: Box<dyn JobInterface + Send + Sync>) {
        let queue_manager = &self.queue_manager;
        if queue_manager.is_closed() {
            println!("ERR: Job queue is closed");
            return;
        }

        queue_manager.queue.write().unwrap().push_back(job);
        println!("INFO: Job enqueued");
    }

    pub fn close(&mut self) {
        *self.queue_manager.is_closed.write().unwrap() = true;
    }
}

impl Listener {
    pub fn listen(&mut self) {
        let queue_manager = &self.queue_manager;
        loop {
            if queue_manager.is_closed() {
                println!("INFO: Job queue is closed, exiting");
                break;
            }

            if queue_manager.queue.read().unwrap().is_empty() {
                sleep(Duration::from_millis(1000));
                println!("INFO: Waiting for new job");
                continue;
            }

            if let Some(job) = queue_manager.queue.write().unwrap().pop_front() {
                thread::spawn(move || {
                    job.run();
                });
            }
        }
    }
}

pub struct Handler {
    // Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot
    // generally obtain a mutable reference to something inside an Arc. 
    // 
    // If you need to mutate through an Arc, use Mutex, RwLock, or one of the Atomic types.
    queue_manager: Arc<QueueManager>,
}

impl Handler {
    pub fn new() -> Handler {
        let queue_manager = QueueManager {
            queue: RwLock::new(LinkedList::new()),
            is_closed: RwLock::new(false),
        };

        Handler {
            queue_manager: Arc::new(queue_manager),
        }
    }

    pub fn enqueuer(&mut self) -> Enqueuer {
        let queue_manager = self.queue_manager.clone();
        Enqueuer {
            queue_manager: queue_manager,
        }
    }

    pub fn listener(&mut self) -> Listener {
        let queue_manager = self.queue_manager.clone();
        Listener {
            queue_manager: queue_manager,
        }
    }
}