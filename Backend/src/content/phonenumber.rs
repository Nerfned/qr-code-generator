use super::Content;

pub struct Phonenumber {
    number: String,
}

impl Phonenumber {
    pub fn new(number: &str) -> Self {
        Self {
            number: number.into(),
        }
    }
}

impl Content for Phonenumber {
    fn generate(&self) -> String {
        format!("tel:{}", self.number)
    }
}
