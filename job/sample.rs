use crate::job::JobInterface;

pub struct GreeterJob {
    pub name: String,
}

impl JobInterface for GreeterJob {
    fn run(&self) {
        println!("Hello, {}!", self.name);
    }
}

pub struct SumJob {
    pub a: i32,
    pub b: i32,
}

impl JobInterface for SumJob {
    fn run(&self) {
        println!("{} + {} = {}", self.a, self.b, self.a + self.b);
    }
}
