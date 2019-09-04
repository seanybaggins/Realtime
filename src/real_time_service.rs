use scheduler;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;    


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

        match FromPrimitive::from_i32(process_schedule_policy) {
            Some(SchedulePolicy::Other) => println!("Other"),
            Some(SchedulePolicy::FirstInFirstOut) => println!("First in first out"),
            Some(SchedulePolicy::RoundRobin) => println!("Round robin"),
            Some(SchedulePolicy::Batch) => println!("Batch"),
            Some(SchedulePolicy::Idle) => println!("Idle"),
            None => println!("Issue comparing primitive type on enum when printing process")
        }
}



#[derive(FromPrimitive)]
enum SchedulePolicy {
    Other = 0,
    FirstInFirstOut = 1,
    RoundRobin = 2,
    Batch = 3,
    Idle = 4
}