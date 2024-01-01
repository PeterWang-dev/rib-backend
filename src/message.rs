use std::fmt::Display;

pub struct Message {
    name: String,
    content: String,
}


impl Message {
    pub fn new(name: String, content: String) -> Message {
        Message {
            name,
            content,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", &self.name, &self.content)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_message() {
        let message = Message::new("Alice".to_string(), "Hello world!".to_string());
        assert_eq!(message.name, "Alice");
        assert_eq!(message.content, "Hello world!");
    }

    #[test]
    fn test_message_name() {
        let message = Message::new("Alice".to_string(), "Hello world!".to_string());
        assert_eq!(message.name(), "Alice");
    }

    #[test]
    fn test_message_content() {
        let message = Message::new("Alice".to_string(), "Hello world!".to_string());
        assert_eq!(message.content(), "Hello world!");
    }

    #[test]
    fn test_display_message() {
        let message = Message::new("Alice".to_string(), "Hello world!".to_string());
        assert_eq!(message.to_string(), "Alice: Hello world!");
    }
}