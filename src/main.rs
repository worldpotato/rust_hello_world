// use ferris_says::say;
// use std::io::{stdout, BufWriter};

mod speaker;

fn main() -> ! {
    let message = String::from("hello my world");
    // let _message_2 = message;
    speaker::my_scream(&message);
    speaker::my_whisper(&message);
    std::process::exit(0)
}

