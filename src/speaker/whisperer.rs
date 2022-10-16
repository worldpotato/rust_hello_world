pub fn my_whisper(message: &String) -> i32 {

    /*! Print the given string as a speack bubble of a crab */
    let lower = message.to_lowercase();
    trace!("my_whisperer");

    let stdout = std::io::stdout();
    let width = message.chars().count();

    let mut writer = std::io::BufWriter::new(stdout.lock());
    ferris_says::say(lower.as_bytes(), width, &mut writer).unwrap();

    return 0;
}
