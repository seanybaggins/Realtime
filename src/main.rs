mod real_time_service;
mod fibonacci;

use real_time_service::Task;
use std::thread;
use std::time::{Duration, Instant};
use fibonacci::fibonacci;


fn main() {

    println!("Defining tasks");
    let tasks = Task::define_tasks();

    println!("Printing scheduler before changes");
    

    println!("Setting the scheduler policy for main thread/sequencer");

    println!("Printing scheduler after changes");
    
     /*
    println!("Spawning threads and setting priorities of tasks.");
    
    for (task_number, task) in tasks.iter().enumerate() {
        thread::spawn(move || {
            scheduler::set_self_priority(scheduler::Which::Process, priority: i32)
        });
    }
    
    let child = thread::spawn(move || {
        println!("Setting scheduler policy and priority for {:?}", thread::current().id());
        real_time_service::print_scheduler();
    });

    child.join()
        .expect("Deadlock possibly detected");
    */
}