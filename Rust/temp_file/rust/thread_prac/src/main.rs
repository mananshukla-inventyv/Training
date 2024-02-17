// use std::{thread, time::Duration};
// use std::sync::mpsc;

// // fn main() {
// //     let mut list=vec![1,2,3,4,5];
// //     println!("Before thread");
// //     list=thread::spawn(move || 
// //         {
// //             list.push(1);
// //             println!("{:?}",list);
// //             thread::sleep(Duration::from_secs(1));
// //             list
// //         }
// //     ).join().unwrap();
// //     list=thread::spawn(move || 
// //         {
// //             list.push(2);
// //             println!("{:?}",list);
// //             thread::sleep(Duration::from_secs(3));
// //             list
// //         }
// //     ).join().unwrap();
    
// //     list=thread::spawn(move || 
// //         {
// //             list.push(3);
// //             println!("{:?}",list);
// //             thread::sleep(Duration::from_secs(1));
// //             list
// //         }
// //     ).join().unwrap(); 

// //     // list.fill_with(|| 2);
// //     println!("{:?}",list);
// //     // list.sort_by_key(|k| k%2);
// //     // println!("{:?}",list);

// //     thread::scope(|s|{
// //         s.spawn(||{
// //             // println!("abc{:?}",&list);
// //             &mut list.push(22);
// //         });
// //     });
// //     println!("{:?}",list);

// //     let count = thread::available_parallelism().unwrap().get();
// //     assert!(count >= 1_usize);
// // } 


// fn main() {

//     let mut trial_list=vec![1,2,3,4];
//     trial_list=thread::spawn(move ||{
//         trial_list.push(111);
//         trial_list
//     }).join().unwrap();
//     println!("{:?}",trial_list);
//     // thread::spawn(move ||{
//     //     &mut trial_list.push(1);
//     // }).join().unwrap();
//     let (rx,tx)=mpsc::channel();
//     let mut list=vec![1,2,3,4,5];
//     // Unlike non-scoped threads, scoped threads can 
//     // borrow non-'static data, as the scope guarantees all threads will be joined at the end of the scope.
//     thread::scope(|s|{
//         // let mut mut_ref=&mut list;
//         s.spawn(||{

//                 println!("{:?}",&list);
//                 // &mut list.push(111);

//         });

//         s.spawn(||{
    
//             println!("{:?}",&list);
//             // &mut list.push(111);

//         });

//     });

//     println!("Before thread");
//     let rx1=rx.clone();
//     let rx2=rx.clone();
//     thread::spawn(move || 
//         {
//             list.push(11);
//             println!("{:?}",list);
//             thread::sleep(Duration::from_secs(1));
//             rx.send(list);
//         }
//     );
    
//     list=tx.recv().unwrap();
//     // let (rx,tx)=mpsc::channel();
//     thread::spawn(move || 
//         {
//             list.push(22);
//             println!("{:?}",list);
//             thread::sleep(Duration::from_secs(1));
//             rx1.send(list); 
//         }
//     );
//     list=tx.recv().unwrap();
//     // let (rx,tx)=mpsc::channel();
//     thread::spawn(move || 
//         {
//             list.push(33);
//             println!("{:?}",list);
//             thread::sleep(Duration::from_secs(1));
//             rx2.send(list);
//         }
//     ); 
//     list=tx.recv().unwrap();
//     // list.fill_with(|| 2);
//     println!("{:?}",list);
//     // list.sort_by_key(|k| k%2);
//     // println!("{:?}",list);

//     // thread::scope(|s|{
//     //     s.spawn(||{
//     //         println!("{:?}",&list);
//     //     });
//     // });
//     // println!("{:?}",list);

//     let count = thread::available_parallelism().unwrap().get();
//     assert!(count >= 1_usize);
    
// } 

// main.rs
mod producer;
mod consumer;

use std::sync::{Arc, Mutex};

fn main() {
    // Create a shared data structure (vector) wrapped in a mutex
    let shared_data = Arc::new(Mutex::new(Vec::new()));

    // Spawn the producer and consumer threads
    let p=producer::start_producer(Arc::clone(&shared_data));
    let q=consumer::start_consumer(Arc::clone(&shared_data));

    // Keep the main thread running
    loop {
        // Main thread can perform other tasks if needed
        // Sleeping for a longer duration here to keep the program running
        // p.join().unwrap();
        // q.join();
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
