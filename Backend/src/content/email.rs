use super::Content;

pub struct Email {
    email: String,
    subject: String,
    message: String,
}

impl Email {
    pub fn new(email: &str, subject: &str, message: &str) -> Self {
        Self {
            email: email.into(),
            subject: subject.into(),
            message: message.into(),
        }
    }
}

impl Content for Email {
    fn generate(&self) -> String {
        format!(
            "MATMSG:TO:{};SUB:{};BODY:{};;",
            self.email, self.subject, self.message
        )
        .clone()
    }
}
