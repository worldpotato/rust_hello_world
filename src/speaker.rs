use crate::message::MessageStruct;

mod screamer;
mod whisperer;

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
pub fn scream(message: &MessageStruct) -> i32 {
    trace!("scream in speaker");
    let ret_code = screamer::scream(message);
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
pub fn whisper(message: &MessageStruct) -> i32 {
    trace!("whisper in speaker");
    let ret_code = whisperer::whisper(message);
    return ret_code;
}
