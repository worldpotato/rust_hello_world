#[derive(Debug)]
pub struct MessageStruct {
    pub greeting: String,
    pub message: String,
    pub closing: String,
}

impl MessageStruct {
    pub fn to_string(&self) -> String {
        let ret_str: String = format!("{},\n{}\n{}", self.greeting, self.message, self.closing);
        return ret_str;
    }
}
