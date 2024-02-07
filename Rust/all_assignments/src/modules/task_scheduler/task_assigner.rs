use crate::modules::type_collections::Status;
use std::thread;
use std::time::Duration;

use super::data::{ASSIGNED, PRIORITY_LEVELS, QUEUES};
use super::CS_PROVIDERS;
pub fn task_assigner() {
    loop {
        thread::sleep(Duration::from_secs(1));
        match QUEUES.write() {
            Ok(mut queues) => {
                for each_level in PRIORITY_LEVELS.iter() {
                    for (each_queue_name, each_queue) in queues.iter_mut() {
                        if each_queue_name.contains(each_level) {
                            // println!("In each queue");
                            if !each_queue.is_empty() {
                                for each_req in each_queue.pop_front().into_iter() {
                                    let mut assigned = false;
                                    match CS_PROVIDERS.write() {
                                        Ok(mut cs_providers) => {
                                            for each_cs in cs_providers.iter_mut() {
                                                // let (ref req_skills,ref req_lang)=each_user;
                                                if each_cs.skills.contains(&each_req.skills)
                                                    && &each_cs.language == &each_req.language
                                                    && each_cs.status == Status::Online
                                                {
                                                    assigned = true;
                                                    each_cs.status = Status::Offline;
                                                    println!(
                                            "Query: {:#?}\n has been assigned to our support agent: {:?} after: {}s",
                                            each_req , 
                                            each_cs.id,
                                            each_req.timestamp.elapsed().unwrap().as_secs()
                                        );
                                                    break;
                                                }
                                            }
                                            if !assigned {
                                                // println!("All Support Providers are busy...Please Wait for some time...");
                                                each_queue.push_back(each_req);
                                            } else {
                                                match ASSIGNED.write() {
                                                    Ok(mut write_data) => {
                                                        write_data.push_back(each_req);
                                                    }
                                                    Err(err) => {
                                                        println!(
                                                "Error in writing to assigned queue : {}",
                                                err
                                            );
                                                    }
                                                }
                                            }
                                        }

                                        Err(error) => {
                                            println!("Error is: {}", error);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error is {}", error);
            }
        }
    }
}
