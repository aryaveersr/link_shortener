pub struct Slug(String);

impl Slug {
    pub fn parse(input: String) -> Result<Self, String> {
        let is_too_long = input.len() > 128;

        let is_whitespace = input.trim().is_empty();

        let contains_invalid_characters = input.chars().any(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => false,
            _ => true,
        });

        match is_too_long || is_whitespace || contains_invalid_characters {
            true => Err(format!("{} is not a valid slug.", input)),
            false => Ok(Self(input)),
        }
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
