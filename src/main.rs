#[macro_use]
extern crate log;

use std::env;
use log::{debug, error, info, trace};

use crate::speaker::*;

pub mod speaker;

fn main() -> ! {
    env_logger::init();
    info!("info log");
    debug!("debug log");
    error!("error log");
    trace!("trace log");

    let rust_log = env::var("RUST_LOG");
    let mut env: String = "".to_string();
    if rust_log.is_ok() {
        env = rust_log.unwrap();
    }
    // let env = env::var("RUST_LOG").unwrap();
    println!("{env}");

    let message = String::from("hello my world");
    scream(&message);
    whisper(&message);
    std::process::exit(0)
}
