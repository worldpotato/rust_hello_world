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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_string() {
        let message = MessageStruct {
            greeting: String::from("Hello"),
            message: String::from("World"),
            closing: String::from("Cheers!"),
        };
        assert_eq!(message.greeting, "Hello");
        let ret_str: String = format!("{},\n{}\n{}", "Hello", "World", "Cheers!");
        assert_eq!(message.to_string(), ret_str)
    }

    #[test]
    fn test_constructor() {
        let message = MessageStruct {
            greeting: String::from("Hello"),
            message: String::from("World"),
            closing: String::from("Cheers!"),
        };
        assert_eq!(message.greeting, "Hello");
        assert_eq!(message.message, "World");
        assert_eq!(message.closing, "Cheers!");
    }
}
