#[macro_use]
extern crate log;

use log::{debug, error, info, trace};
use std::env;
use std::process::exit;

use crate::message::MessageStruct;
use crate::speaker::*;

// pub mod message;
pub mod message;
pub mod speaker;

fn main() -> ! {
    env_logger::init();
    info!("info log");
    debug!("debug log");
    error!("error log");
    trace!("trace log");

    let rust_log = env::var("RUST_LOG");
    let mut env: String = "".to_string();
    let rust_log_ok = rust_log.is_ok();
    if rust_log_ok {
        env = rust_log.unwrap();
    }
    // let env = env::var("RUST_LOG").unwrap();
    println!("{env}");

    let message = MessageStruct {
        greeting: String::from("Hello"),
        message: String::from("World"),
        closing: String::from("Cheers!"),
    };

    scream(&message);
    whisper(&message);
    exit(0)
}
