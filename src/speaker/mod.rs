pub mod screamer;
pub mod whisperer;

/// Print a crab to stdout that says something but in loud!
/// Returns `0` if the printing was ok is [`Ok`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let message = String::from("hello my world");
/// let status = speaker_scream(&message);
/// ```
pub fn scream(message: &String) -> i32 {
    trace!("scream in speaker");
    let ret_code = screamer::my_scream(message);
    return ret_code;
}

/// Print a crab to stdout that says something but silent!
/// Returns `0` if the printing was ok is [`Ok`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let message = String::from("hello my world");
/// let status = speaker_whisper(&message);
/// ```
pub fn whisper(message: &String) -> i32 {
    trace!("whisper in speaker");
    let ret_code = whisperer::my_whisper(message);
    return ret_code;
}
