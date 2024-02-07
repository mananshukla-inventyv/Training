use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, RwLock}};

use lazy_static::lazy_static;

use crate::modules::type_collections::{Request, User};

lazy_static!(
    
    pub static ref PENDING_QUERIES:Arc<RwLock<VecDeque<Request>>>=Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CS_PROVIDERS:Arc<RwLock<VecDeque<User>>>=Arc::new(RwLock::new(serde_json::from_str(&fs::read_to_string("./data/Master_Data.json").unwrap()).unwrap()));
    pub static ref ASSIGNED:Arc<RwLock<VecDeque<Request>>>=Arc::new(RwLock::new(VecDeque::new()));
    pub static ref QUEUES:Arc<RwLock<HashMap<String,VecDeque<Request>>>>=Arc::new(RwLock::new(HashMap::new()));
    pub static ref SKILL:Arc<RwLock<Vec<String>>>=Arc::new(RwLock::new(

        vec![
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
            "Documentation".to_string()
            ]
              ));
    pub static ref LANGUAGES:Vec<String>=vec!["English".to_string(),"Spanish".to_string()];
    pub static ref SUPPORT_TYPE:Vec<String>=vec!["OnCall".to_string(),"OnChat".to_string()];
    pub static ref PRIORITY_LEVELS:Vec<String>=vec!["L1".to_string(),"L2".to_string(),"L3".to_string(),"L4".to_string(),"L5".to_string()];
);
