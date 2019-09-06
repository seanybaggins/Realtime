use scheduler;
use std::thread;

pub const MAX_PRIORITY: i32 = 99;

#[derive(Debug)]
pub struct Task {
    period: u32,
    runtime: u32,
    thread_id: Option<thread::ThreadId>
}

impl Task {

    pub fn define_tasks() -> [Task; 3] {
        
        let task1 = Task {
            period: 2,
            runtime: 1,
            thread_id: None
        };
        let task2 = Task {
            period: 5,
            runtime: 2,
            thread_id: None
        };
        let task3 = Task {
            period: 10,
            runtime: 1,
            thread_id: None
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
