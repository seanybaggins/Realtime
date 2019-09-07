pub extern crate scheduler;


pub use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

pub const MAX_PRIORITY: i32 = 99;
pub const ONE_CPU: usize = 1;

#[derive(Debug)]
pub struct Task {
    pub period: Arc<u32>,
    pub priority: Arc<i32>,
    pub call_count: Arc<Mutex<u32>>,
    pub instructed_to_run: Arc<Mutex<bool>>,
}

impl Task {

    pub fn define_tasks() -> [Task; 3] {
        let task1 = Task {
            period: Arc::new(2),
            priority: Arc::new(MAX_PRIORITY - 1),
            call_count: Arc::new(Mutex::new(0)),
            instructed_to_run: Arc::new(Mutex::new(false)),
        };
        let task2 = Task {
            period: Arc::new(5),
            priority: Arc::new(MAX_PRIORITY - 2),
            call_count: Arc::new(Mutex::new(0)),
            instructed_to_run: Arc::new(Mutex::new(false)),
        };
        let task3 = Task {
            period: Arc::new(10),
            priority: Arc::new(MAX_PRIORITY - 3),
            call_count: Arc::new(Mutex::new(0)),
            instructed_to_run: false,
        };
        return [task1, task2, task3]
    }
}

pub struct Sequencer {
    task: Task,
    max_sequences: u32,
}

impl Sequencer {
    pub fn define() -> Sequencer {
        Sequencer {
            task: Task {
                period: 1,
                priority: MAX_PRIORITY,
                call_count: 0,
                instructed_to_run: true,
            },
            max_sequences: 30
        }
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
