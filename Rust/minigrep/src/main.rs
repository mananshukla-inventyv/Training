use std::env::args;
use minigrep::Config;
// use minigrep::run;
fn main() {
// Building a mini version of cli searching tool

    let args:Vec<String>=args().collect();

    // println!("{:?}",args);
    let config=Config::new(args);
    minigrep::run(config);
}



