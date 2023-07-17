// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // Sleeping by different amounts means my code below can finish the loop and then wait for the next handle.
            let sleep_duration = 1000 * i;
            // Reversing the sleep duration outputs all 10s, because first loop is blocked by first thread sleeping for 10s.
            // let sleep_duration = 1000 * (10 - i);
            thread::sleep(Duration::from_millis(sleep_duration));
            let mut num = status_shared.lock().unwrap();
            num.jobs_completed += 1
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
