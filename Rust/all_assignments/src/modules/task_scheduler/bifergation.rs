use std::{collections::VecDeque, sync::Arc, thread, time::Duration};

use crate::modules::type_collections::{Language, SupportType};

use super::data::{LANGUAGES, PENDING_QUERIES, PRIORITY_LEVELS, QUEUES, SKILL, SUPPORT_TYPE};

pub fn priority_hmap_creator() {
    for each_task_type in SUPPORT_TYPE.iter() {
        for each_skill in SKILL.read().unwrap().iter() {
            for each_lang in LANGUAGES.iter() {
                for each_level in PRIORITY_LEVELS.iter() {
                    let queue_name = [
                        each_task_type.to_string(),
                        each_skill.to_string(),
                        each_lang.to_string(),
                        each_level.to_string(),
                    ]
                    .join("_");

                    match QUEUES.write() {
                        Ok(mut write_data) => {
                            write_data.insert(queue_name, VecDeque::new());
                        }
                        Err(err) => {
                            println!("Poison Error: {}", err);
                        }
                    }
                }
            }
        }
    }
    println!("Runtime Queues Created!");
}

pub fn bifergation() {
    priority_hmap_creator();
    loop {
        thread::sleep(Duration::from_secs(10));
        // println!("Going in to bifug");
        let queue_ref = Arc::clone(&QUEUES);
        match PENDING_QUERIES.write() {
            Ok(mut pending_queries) => {
                // println!("bifug took lock");
                for each_pending_query in pending_queries.pop_front().into_iter() {
                    let task_type = if each_pending_query.task_type == SupportType::OnCall {
                        "OnCall".to_string()
                    } else {
                        "OnChat".to_string()
                    };
                    let language = if each_pending_query.language == Language::English {
                        "English".to_string()
                    } else {
                        "Spanish".to_string()
                    };
                    let skill = each_pending_query.skills.to_string();
                    let initial_priority_level = "L5".to_string();
                    let bifurgated_queue =
                        [task_type, skill, language, initial_priority_level].join("_");
                    // println!("trying to take queue lock");
                    match queue_ref.write() {
                        Ok(mut write_data) => {
                            // println!("taken queue lock");
                            // println!("Task bifurgated to : {}", bifurgated_queue);
                            write_data.entry(bifurgated_queue).and_modify(|queue| {
                                // println!(
                                //     " and query where its bifugated is: {:?}",
                                //     each_pending_query
                                // );
                                queue.push_back(each_pending_query);
                            });
                        }
                        Err(error) => {
                            println!("Error is : {}", error);
                        }
                    }
                }
                // println!("end of bifug");
            }
            Err(error) => {
                println!("poison error is {}", error);
            }
        }
    }
}
