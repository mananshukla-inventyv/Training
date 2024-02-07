use rand::Rng;
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};

use crate::modules::{
    task_scheduler::PENDING_QUERIES,
    type_collections::{Language, Request, SupportType},
};

pub fn random_data_gen() {
    loop {
        thread::sleep(Duration::from_secs(1));
        let skills = vec![
            "Customer Service".to_string(),
            "Problem-solving".to_string(),
            "Product Knowledge".to_string(),
            "Effective Communication".to_string(),
            "Time Management".to_string(),
            "Adaptability".to_string(),
            "Team Collaboration".to_string(),
            "Feedback Analysis".to_string(),
            "Proactive Engagement".to_string(),
            "Technical Proficiency".to_string(),
            "Cultural Sensitivity".to_string(),
            "Documentation".to_string(),
        ];

        let languages = vec!["English", "Spanish"];
        let support_type = vec!["OnCall", "OnChat"];
        let pending_queries = Arc::clone(&PENDING_QUERIES);
        // println!("generating Req");
        match pending_queries.write() {
            Ok(mut write_data) => {
                // println!("writing data");
                let idx = rand::thread_rng().gen_range(0..support_type.len());
                let s = support_type[idx].to_string();
                let task_type = if s == "OnChat" {
                    SupportType::OnChat
                } else {
                    SupportType::OnCall
                };

                let skill = skills[rand::thread_rng().gen_range(0..skills.len())].to_string();

                let language =
                    languages[rand::thread_rng().gen_range(0..languages.len())].to_string();
                let lang = if language == "English" {
                    Language::English
                } else {
                    Language::Spanish
                };

                write_data.push_back(Request {
                    task_type,
                    skills: skill,
                    language: lang,
                    timestamp: SystemTime::now(),
                });
            }
            Err(error) => {
                println!("Poison Error: {}", error);
            }
        }
        drop(pending_queries);
    }
}
