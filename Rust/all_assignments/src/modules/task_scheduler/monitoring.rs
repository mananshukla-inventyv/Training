use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::{Duration, SystemTime},
};

use super::data::QUEUES;

pub fn priority_checker() {
    loop {
        thread::sleep(Duration::from_secs(30));
        // println!("Monitoring");
        match QUEUES.write() {
            Ok(mut queues) => {

                let mut hmap = HashMap::new();
                for (each_queue_name, each_queue) in queues.iter_mut() {
                    let mut list_of_reqs = VecDeque::new();
                    let mut new_queue_name: Option<String> = None;
                    for mut each_req in each_queue.pop_front().into_iter() {
                        let original_queue_priority: Vec<&str> =
                            each_queue_name.split("_").collect();
                        let level_of_priority = *original_queue_priority.last().unwrap();
                        match each_req.timestamp.elapsed() {
                            Ok(time_elapsed) => {
                                if time_elapsed > Duration::from_secs(30) {
                                    each_req.timestamp = SystemTime::now();
                                    let new_priority_level = match level_of_priority {
                                        "L5" => "L4",
                                        "L4" => "L3",
                                        "L3" => "L2",
                                        "L2" | "L1" => "L1",
                                        _ => level_of_priority,
                                    };
                                    println!("----------------------");
                                    println!("Old Prior: {}", level_of_priority);
                                    println!("New Prior: {}", new_priority_level);
                                    println!("-------------------------------------------");
                                    let orig_queue: String = original_queue_priority.join("_");
                                    let level_of_priority = orig_queue
                                        .replace(level_of_priority, new_priority_level)
                                        .to_string();
                                    new_queue_name = Some([level_of_priority].join("_"));
                                    list_of_reqs.push_back(each_req);
                                } else {
                                    each_queue.push_back(each_req);
                                }
                            }

                            Err(err) => {
                                println!("Error in System Time : {}", err);
                            }
                        }
                    }
                    hmap.insert(new_queue_name, list_of_reqs);
                }

                for (each_name, mut each_req) in hmap {
                    if each_name.is_some() {
                        queues.entry(each_name.unwrap()).and_modify(|queue| {
                            queue.append(&mut each_req);
                        });
                    }
                }
            }
            Err(error) => {
                println!("Error is :{}", error);
            }
        }
        // println!("{:#?}",QUEUES.read().unwrap());
    }
}
