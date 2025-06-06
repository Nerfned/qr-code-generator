use super::Content;

pub struct Sms {
    number: String,
    message: String,
}

impl Sms {
    pub fn new(number: &str, message: &str) -> Self {
        Self {
            number: number.into(),
            message: message.into(),
        }
    }
}

impl Content for Sms {
    fn generate(&self) -> String {
        format!("SMSTO:{}:{}", self.number, self.message).clone()
    }
}
