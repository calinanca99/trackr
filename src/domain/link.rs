use std::fmt::Display;

pub struct URL(String);

impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for URL {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(URL(value.to_string()))
    }
}

pub struct Name(String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Name {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Name(value.to_string()))
    }
}

pub struct Label(String);

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Label {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Label(value.to_string()))
    }
}
