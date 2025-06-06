use super::Content;

pub struct Whatsapp {
    number: String,
    message: String,
}

impl Whatsapp {
    pub fn new(number: &str, message: &str) -> Self {
        Self {
            number: number.into(),
            message: message.into(),
        }
    }
}

impl Content for Whatsapp {
    fn generate(&self) -> String {
        format!("https://wa.me/{}/?text={}", self.number, self.message).clone()
    }
}
