use url::Url;

const HREF_MAX_LENGTH: usize = 256;

#[derive(Debug)]
pub struct Href(Url);

impl Href {
    pub fn parse(input: &str) -> Result<Self, String> {
        // We try parsing the input as is, and if it fails, try again after
        // prepending 'https://' at the start.

        let url = Url::parse(input)
            .or_else(|_| Url::parse(&format!("https://{}", input)))
            .map_err(|_| format!(r#""{}" is not a valid URL."#, input))?;

        if url.scheme() == "javascript" {
            return Err(r#""javascript:" is not allowed as the href."#.into());
        }

        // The URL's .to_string() might be different because of an prepended `https://`, or a
        // trailing `/`.
        if url.to_string().len() > HREF_MAX_LENGTH {
            return Err("Input is too long.".into());
        }

        Ok(Self(url))
    }
}

impl AsRef<str> for Href {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn href_parses_valid_strings() {
        // # Arrange
        let test_cases = [
            "https://google.com",
            "github.com",
            "localhost:3000",
            "127.0.0.1:4004",
            "172.217.167.238",
            "data:,Hello%2C%20World%21",
            "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==",
            "data:text/html,%3Ch1%3EHello%2C%20World%21%3C%2Fh1%3E",
            "localhost",
        ];

        for case in test_cases {
            // # Act
            let href = Href::parse(case);

            // # Assert
            assert!(
                href.is_ok(),
                r#"Failed to parse valid string: "{}" with error: {}"#,
                case,
                href.unwrap_err()
            );
        }
    }

    #[test]
    fn href_does_not_parse_invalid_strings() {
        // # Arrange
        let test_cases = [
            "javascript:alert('Hello, world!')",
            "hello world",
            "",
            "    ",
            "\t",
        ];

        for case in test_cases {
            // # Act
            let href = Href::parse(case);

            // # Assert
            assert!(
                href.is_err(),
                r#"Parsed invalid string: "{}" as {}"#,
                case,
                href.unwrap().0
            );
        }
    }

    #[test]
    fn href_does_not_parse_string_too_long() {
        // # Arrange
        let string = "hello_world".repeat(40);

        // # Act
        let href = Href::parse(&string);

        // # Assert
        assert!(string.len() > HREF_MAX_LENGTH);
        assert!(
            href.is_err(),
            r#"Parsed string with length: {}"#,
            string.len()
        );
    }
}
