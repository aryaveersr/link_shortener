const SLUG_MAX_LENGTH: usize = 128;

#[derive(Debug)]
pub struct Slug(String);

impl Slug {
    pub fn parse(input: String) -> Result<Self, String> {
        let is_too_long = input.len() > SLUG_MAX_LENGTH;

        let is_whitespace = input.trim().is_empty();

        let contains_invalid_characters = !input
            .chars()
            .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'));

        if is_too_long || is_whitespace || contains_invalid_characters {
            Err(format!(r#""{}" is not a valid slug."#, input))
        } else {
            Ok(Self(input))
        }
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_parses_valid_strings() {
        // # Arrange
        let test_cases = ["testing", "valid-slug", "ANOTHER_VALID_SLUG", "abc123"];

        for case in test_cases {
            // # Act
            let slug = Slug::parse(case.to_owned());

            // # Assert
            assert!(
                slug.is_ok(),
                r#"Failed to parse valid string: "{case}" with error: {}"#,
                slug.err().unwrap()
            );
        }
    }

    #[test]
    fn slug_does_not_parse_invalid_strings() {
        // # Arrange
        let test_cases = [
            "testing/slugs",
            "hello/there/how",
            "slug?hey=yes",
            "",
            "   ",
            "\t",
            "\n",
        ];

        for case in test_cases {
            // # Act
            let slug = Slug::parse(case.to_owned());

            // # Assert
            assert!(slug.is_err(), r#"Parsed invalid string: "{case}""#);
        }
    }

    #[test]
    fn slug_does_not_parse_string_too_long() {
        // # Arrange
        let string = "hello_world".repeat(20);
        let len = string.len();

        // # Act
        let slug = Slug::parse(string.clone());

        // # Assert
        assert!(len > SLUG_MAX_LENGTH);
        assert!(slug.is_err(), r#"Parsed string with length {len}"#,);
    }
}
