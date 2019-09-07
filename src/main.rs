mod real_time_service;
mod fibonacci;

use real_time_service::{Task, scheduler, MAX_PRIORITY};
use scheduler::{Policy, Which::Process};
use std::thread;
use std::sync::Arc;
//use real_time_service::crossbeam::thread;
use fibonacci::fibonacci;

fn main() {

    println!("Defining tasks");
    let tasks = Task::define_tasks();

    println!("Setting affinity to single core");
    let cpu = scheduler::get_self_affinity(real_time_service::ONE_CPU).unwrap();
    scheduler::set_self_affinity(cpu).expect("unable to set CPU affinity");

    println!("Setting the scheduler policy for main thread/sequencer");
    scheduler::set_self_policy(Policy::Fifo, MAX_PRIORITY)
        .expect("Unable to change scheduling priority. Did you run with sudo permissions?");

    println!("Printing scheduler after changes");
    real_time_service::print_scheduler();
    
    println!("Setting parameters for the sequencer");
    let sequencer = real_time_service::Sequencer::define();
    
    println!("Spawning threads and setting priorities of tasks.");
    let mut thread_handles = Vec::with_capacity(tasks.len());
    for index in 0..tasks.len() {
        let priority_p = tasks[index].priority.clone();
        let thread_handle = thread::spawn(move || {
            scheduler::set_self_priority(Process, *priority_p).unwrap();
        });
        &thread_handles.push(thread_handle);
    }
    
    println!("Joining threads and shutting down");
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
}