use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct DuplicateEmail;

impl std::error::Error for DuplicateEmail {}

impl Display for DuplicateEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email already exists")
    }
}
