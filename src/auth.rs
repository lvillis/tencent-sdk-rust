use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub enum Auth {
    Tc3(Tc3Auth),
    None,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Tc3Auth {
    secret_id: String,
    secret_key: String,
    token: Option<String>,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::Tc3(auth) => f.debug_tuple("Auth::Tc3").field(auth).finish(),
            Auth::None => f.write_str("Auth::None"),
        }
    }
}

impl fmt::Debug for Tc3Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tc3Auth")
            .field("secret_id", &"[redacted]")
            .field("secret_key", &"[redacted]")
            .field("has_token", &self.token.is_some())
            .finish()
    }
}

impl Auth {
    pub fn none() -> Self {
        Self::None
    }

    pub fn tc3(secret_id: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self::Tc3(Tc3Auth {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            token: None,
        })
    }
}

impl Tc3Auth {
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn secret_id(&self) -> &str {
        &self.secret_id
    }

    pub(crate) fn secret_key(&self) -> &str {
        &self.secret_key
    }

    pub(crate) fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
