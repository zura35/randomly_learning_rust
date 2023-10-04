use std::sync::{RwLock};
use std::collections::LinkedList;
use std::time::Duration;
// use std::thread;
use std::thread::sleep;

use crate::job::JobInterface;

pub struct SimpleHandler {
    queue: RwLock<LinkedList<Box<dyn JobInterface + Send + Sync>>>,
    is_closed: RwLock<bool>,
}

impl SimpleHandler {
    pub fn new() -> SimpleHandler {
        SimpleHandler {
            queue: RwLock::new(LinkedList::new()),
            is_closed: RwLock::new(false),
        }
    }

    pub fn enqueue(&self, job: Box<dyn JobInterface + Send + Sync>) {
        if self.is_closed() {
            println!("ERR: Job queue is closed");
            return;
        }

        self.queue.write().unwrap().push_back(job);
        println!("INFO: Job enqueued");
    }

    pub fn close(&self) {
        *self.is_closed.write().unwrap() = true;
    }

    pub fn listen(&self) {
        loop {
            if self.is_closed() {
                println!("INFO: Job queue is closed, exiting");
                break;
            }

            if self.queue.read().unwrap().is_empty() {
                println!("INFO: Job queue is empty, sleeping for 1s");
                sleep(Duration::from_secs(1));
                continue;
            }

            let job = self.queue.write().unwrap().pop_front().unwrap();
            job.run();
        }

        println!("INFO: Exit handler");
    }

    fn is_closed(&self) -> bool {
        *self.is_closed.read().unwrap()
    }
}