#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credentials {
    pub secret_id: String,
    pub secret_key: String,
    pub token: Option<String>,
}

impl Credentials {
    pub fn new(secret_id: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
}
