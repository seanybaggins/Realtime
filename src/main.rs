mod real_time_service;
mod fibonacci;

use real_time_service::{Task, scheduler, thread};
use fibonacci::fibonacci;
use libc;

fn main() {

    println!("Defining tasks");
    let tasks = Task::define_tasks();

    println!("Printing scheduler before changes");
    real_time_service::print_scheduler();

    println!("Setting the scheduler policy for main process/sequencer");
    scheduler::set_self_policy(scheduler::Policy::Fifo, real_time_service::MAX_PRIORITY)
        .expect("Unable to change scheduling priority. Did you run with sudo permissions?");

    println!("Printing scheduler after changes");
    real_time_service::print_scheduler();

    println!("Spawning threads and setting priorities of tasks.");
    /*
    for (task_number, task) in tasks.iter().enumerate() {
        thread::spawn(move || {
            scheduler::set_self_priority(scheduler::Which::Process, priority: i32)
        });
    }
    */
    let child = thread::spawn(move || {
        println!("Setting scheduler policy and priority for {:?}", thread::current().id());
        real_time_service::print_scheduler();
        scheduler::set_self_policy(scheduler::Policy::Fifo, real_time_service::MAX_PRIORITY - 1)
            .expect("failed to set scheduling priority");
        for _ in 1..10 {
            println!("Thread ran");
        } 
    });

    for _ in 1..10 {
            println!("Main ran");
    }

    child.join()
        .expect("Deadlock possibly detected");
}