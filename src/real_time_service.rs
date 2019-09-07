pub extern crate scheduler;
pub use std::{thread, mem};
pub use std::time::{Duration, Instant};

pub const MAX_PRIORITY: i32 = 99;

#[derive(Debug)]
pub struct Task {
    period: u32,
    priority: i32,
    call_count: u32,
    instructed_to_run: bool,
}

pub struct Sequencer {
    task: Task,
    max_call_count: u32,
}

impl Task {

    pub fn define_tasks() -> [Task; 3] {

        let task1 = Task {
            period: 2,
            priority: MAX_PRIORITY - 1,
            call_count: 0,
            instructed_to_run: false,
        };
        let task2 = Task {
            period: 5,
            priority: MAX_PRIORITY - 2,
            call_count: 0,
            instructed_to_run: false,
        };
        let task3 = Task {
            period: 10,
            priority: MAX_PRIORITY - 3,
            call_count: 0,
            instructed_to_run: false,
        };

        return [task1, task2, task3]
    }
}

pub fn print_scheduler() {
    use scheduler::Policy;
    let process_schedule_policy = scheduler::get_self_policy()
        .expect("Error in getting the scheduler policy: {:?}");

    print!("\tSchedule Policy: ");

    match process_schedule_policy {
        Policy::Other => println!("Other"),
        Policy::Fifo => println!("First in first out"),
        Policy::RoundRobin => println!("Round Robin"),
        Policy::Batch => println!("Batch"),
        Policy::Idle => println!("Idle"),
        Policy::Deadline => println!("Deadline")
    }
}
