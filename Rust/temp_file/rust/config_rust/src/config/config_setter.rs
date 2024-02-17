
use std::env;

use config::{Config, File};

pub fn config_setter(){
    let run_mode=env::var("RUN_MODE").unwrap_or("dev".to_string());

    let conf=Config::builder()
        .add_source(File::with_name(
            "./src/config/config-dev.json"
        ))
        .add_source(File::with_name(&format!("./src/config/config-{}",run_mode)).required(false)).build().unwrap();

    println!("{:?}",conf.get::<String>("id").unwrap())
    
}