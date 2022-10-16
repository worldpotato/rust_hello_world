
pub fn my_scream(message: &String) -> i32 {
    /*! Print the given string as a speack bubble of a crab */
    trace!("my_scream");
    let upper = message.to_uppercase();

    let stdout = std::io::stdout();
    let width = message.chars().count();

    let mut writer = std::io::BufWriter::new(stdout.lock());
    ferris_says::say(upper.as_bytes(), width, &mut writer).unwrap();

    return 0;
}
