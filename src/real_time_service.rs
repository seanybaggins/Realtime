
use std::thread;

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


