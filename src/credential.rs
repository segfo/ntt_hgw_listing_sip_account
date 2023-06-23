#[derive(Debug, Clone)]
pub enum CredentialKind {
    IdPassword(String, String),
}
#[derive(Debug, Clone)]
pub struct Credential {
    credential: CredentialKind,
}
impl Credential {
    pub fn id_password(id: impl Into<String>, password: impl Into<String>) -> Self {
        Credential {
            credential: CredentialKind::IdPassword(id.into(), password.into()),
        }
    }
}

impl std::fmt::Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.credential {
            CredentialKind::IdPassword(id, pass) => write!(f, "id: {} / password: {}", id, pass),
        }
    }
}
