use super::Content;

pub struct Url {
    url: String,
}

impl Url {
    pub fn new(url: &str) -> Self {
        Self { url: url.into() }
    }
}

impl Content for Url {
    fn generate(&self) -> String {
        self.url.clone()
    }
}
