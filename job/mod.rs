pub mod sample;
pub mod simple_handler;

pub trait JobInterface {
    fn run(&self);
}
