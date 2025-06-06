use super::Content;

pub struct Paypal {
    username: String,
}

impl Paypal {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.into(),
        }
    }
}

impl Content for Paypal {
    fn generate(&self) -> String {
        format!("https://paypal.me/{}", self.username).clone()
    }
}
