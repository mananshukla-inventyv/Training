// producer.rs
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn start_producer(shared_data: Arc<Mutex<Vec<i32>>>){
    thread::spawn(move || {
        loop {
            // Produce a random integer (for demonstration purposes)
            let value = rand::random::<i32>();

            // Lock the mutex, push the value into the vector, and unlock the mutex
            let mut data = shared_data.lock().unwrap();
            data.push(value);
            println!("Produced: {}", value);

            // Sleep for 2 seconds
            thread::sleep(Duration::from_secs(2));
            thread::yield_now();
        }
    }).join().unwrap();
}
