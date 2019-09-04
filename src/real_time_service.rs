use scheduler;    

const SCHED_OTHER: i32 = 0;
const SCHED_FIFO: i32 = 1;
const SCHED_RR: i32 = 2;
const SCHED_BATCH: i32 = 3;
const SCHED_IDLE: i32 = 5;


#[derive(Debug)]
pub struct Task {
    period: u32,
    runtime: u32,
}

impl Task {
    /*
    pub fn new(period: u32, runtime: u32) -> Task {
        Task{
            period: period,
            runtime: runtime,
        }
    }
    */

    pub fn define_tasks() -> [Task; 3] {
        
        let task1 = Task {
            period: 2,
            runtime: 1,
        };
        let task2 = Task {
            period: 5,
            runtime: 2,
        };
        let task3 = Task {
            period: 10,
            runtime: 1,
        };

        return [task1, task2, task3]
    }
}

pub fn print_scheduler() {
        let process_schedule_policy = scheduler::get_self_priority(scheduler::Which::Process)
            .expect("Error in getting the scheduler policy: {:?}");

        print!("\tSchedule Policy: ");

        match process_schedule_policy {
            SCHED_OTHER => println!("Other"),
            SCHED_FIFO => print!("First in first out"),
            SCHED_RR => println!("Round Robin"),
            SCHED_BATCH => println!("Batch"),
            SCHED_IDLE => println!("Idle"),
            _ => println!("Policy unknown")
        }
}
