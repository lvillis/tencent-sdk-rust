use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct Credentials {
    secret_id: String,
    secret_key: String,
    token: Option<String>,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("secret_id", &"[redacted]")
            .field("secret_key", &"[redacted]")
            .field("has_token", &self.token.is_some())
            .finish()
    }
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
        self.set_token(token);
        self
    }

    pub fn secret_id(&self) -> &str {
        &self.secret_id
    }

    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }
}
