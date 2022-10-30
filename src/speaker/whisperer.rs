use crate::message::MessageStruct;

pub fn whisper(message: &MessageStruct) -> i32 {

    /*! Print the given string as a speack bubble of a crab */
    let temp_message = message.to_string();
    let lower = temp_message.to_lowercase();
    trace!("my_whisperer");

    let stdout = std::io::stdout();
    let width = temp_message.chars().count();

    let mut writer = std::io::BufWriter::new(stdout.lock());
    ferris_says::say(lower.as_bytes(), width, &mut writer).unwrap();

    return 0;
}

