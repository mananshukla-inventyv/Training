// consumer.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_consumer(shared_data: Arc<Mutex<Vec<i32>>>) {
    thread::spawn(move || {
        loop {
            // Lock the mutex, pop the value from the vector if available, and unlock the mutex
            let mut data = shared_data.lock().unwrap();
            if let Some(value) = data.pop() {
                println!("Consumed: {}", value);
            }

            // Sleep for 2 seconds
            thread::sleep(Duration::from_secs(2));
            thread::yield_now();
        }
    }).join().unwrap();
}
