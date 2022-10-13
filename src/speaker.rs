use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn my_scream(message: &String) -> i32 {
    /*! Print the given string as a speack bubble of a crab */
    let upper = message.to_uppercase();

    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(upper.as_bytes(), width, &mut writer).unwrap();

    return 0;
}

pub fn my_whisper(message: &String) -> i32 {
    /*! Print the given string as a speack bubble of a crab */
    let mut lower = message.to_lowercase();

    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(lower.as_bytes(), width, &mut writer).unwrap();

    return 0;
}
