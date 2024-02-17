use config::Config;
mod settings;
use lazy_static::lazy_static;
// use crate::error::{ConfigError, Result};
use serde::de::Deserialize;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(settings::get_config());
}

// pub fn get(property:String) -> String{
//     CONFIG.lock().unwrap().get(&property).unwrap()
// }

pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
    CONFIG.read().unwrap().get(key).unwrap()
}

pub fn get_res<'de, T: Deserialize<'de>>(key: &str) -> Result<T, config::ConfigError> {
    CONFIG.read().unwrap().get(key)
}

// pub async fn initialize_config() -> Result<(), Box<dyn Error>> {
//     let config_result = settings::get_config().await;
//     match config_result {
//         Ok(result) => {
//             let mut m_config = CONFIG.write().unwrap();
//             *m_config = result;
//             return Ok(());
//         }
//         Err(error) => {
//             return Err(error);
//         }
//     }
// }
