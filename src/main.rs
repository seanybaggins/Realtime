mod real_time_service;
use real_time_service::Task;

fn main() {
    println!("Defining tasks");
    let tasks = Task::define_tasks();

    println!("Printing scheduler before changes");
    real_time_service::print_scheduler();

    println!("Setting the scheduler policy for main process");
    

    println!("Tasks = {:?}", tasks);
}