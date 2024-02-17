#![allow(deprecated)]
use axum::async_trait;
use config::{AsyncSource, Config, ConfigError, Environment, File, Map};
use std::env;
use std::error::Error;
use std::io::{self, Read};

pub fn get_config() -> Config {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let env_settings = Environment::new();

    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name("resources/config/config"))
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        .add_source(
            File::with_name(&format!("resources/config/config-{}", run_mode)).required(false),
        )
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        .add_source(env_settings.prefix("app").separator("_"))
        // You may also programmatically change settings
        .build()
        .unwrap();

    // Now that we're done, let's access our configuration

    // You can deserialize (and thus freeze) the entire configuration as
    s
}

// pub async fn get_async_config() -> Result<Config, Box<dyn Error>> {
//     let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
//     let env_settings = Environment::new();
//     // uncomment UserConfig if you are running in local environment

//     let s: Config = Config::builder()
//         .add_source(File::with_name("resources/config/config"))
//         .add_source(File::with_name(&format!("resources/config/config-{}", run_mode)).required(false))
//         .add_async_source(ServiceConfig { env_key: run_mode.to_string() })
//         // This file shouldn't be checked in to git
//         .add_source(env_settings.prefix("app").separator("_"))
//         .build()
//         .await?;
//     Ok(s)
// }

#[derive(Debug)]
struct ServiceConfig {
    env_key: String,
}

#[async_trait]
#[cfg(not(feature = "Production"))]
#[cfg(not(feature = "Test"))]
impl AsyncSource for ServiceConfig {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        let local_config_file = std::fs::File::open("resources/development/global_config.json");
        match local_config_file {
            Ok(mut local_config_file) => {
                let mut content = String::new();
                let file_content: Result<usize, io::Error> =
                    local_config_file.read_to_string(&mut content);
                match file_content {
                    Ok(_) => {
                        let config_json: Map<String, config::Value> =
                            serde_json::from_str(content.as_str()).unwrap();
                        Ok(config_json)
                    }
                    Err(err) => Err(ConfigError::Message(err.to_string())),
                }
            }
            Err(err) => Err(ConfigError::Message(err.to_string())),
        }
    }
}

#[async_trait]
#[cfg(feature = "Production")]
impl AsyncSource for ServiceConfig {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        let _current_env = &self.env_key;

        // get session doc from tikv.
        let response = tikv_db::get_doc("ALL-CONFIG".to_string(), "test".to_string()).await;
        match response {
            Ok(result) => {
                let config_json: Map<String, config::Value> =
                    serde_json::from_str(result.as_str()).unwrap();
                Ok(config_json)
            }
            Err(error) => Err(ConfigError::NotFound(error)),
        }
    }
}

#[async_trait]
#[cfg(feature = "Test")]
impl AsyncSource for ServiceConfig {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        let local_config_file = std::fs::File::open("resources/test/global_config.json");
        match local_config_file {
            Ok(mut local_config_file) => {
                let mut content = String::new();
                let file_content: Result<usize, io::Error> =
                    local_config_file.read_to_string(&mut content);
                match file_content {
                    Ok(_) => {
                        let config_json: Map<String, config::Value> =
                            serde_json::from_str(content.as_str()).unwrap();
                        Ok(config_json)
                    }
                    Err(err) => Err(ConfigError::Message(err.to_string())),
                }
            }
            Err(err) => Err(ConfigError::Message(err.to_string())),
        }
    }
}
