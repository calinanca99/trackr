use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize)]
pub struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Username {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Username(value))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize)]
pub struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Password {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err("password is too short".to_string())
        } else {
            Ok(Password(value))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize)]
pub struct Account {
    username: Username,
}

impl Account {
    pub fn new(username: Username) -> Self {
        Self { username }
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn errors_if_password_is_too_short() {
        // Arrange
        let password = "aaa".to_string();

        // Act
        let parsed_password = Password::try_from(password);

        // Assert
        assert!(parsed_password.is_err())
    }

    #[test]
    fn returns_parsed_password() {
        // Arrange
        let password = "somelongpassword".to_string();

        // Act
        let parsed_password = Password::try_from(password);

        // Assert
        assert!(parsed_password.is_ok());
        assert_eq!(parsed_password.unwrap().as_ref(), "somelongpassword");
    }
}
