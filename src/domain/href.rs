use std::fmt::Display;
use url::Url;

pub struct Href(Url);

impl Href {
    pub fn parse(input: &str) -> Result<Self, String> {
        // We try parsing the input as is, and if it fails, try again after
        // appending 'https://' at the start.
        let url = {
            let parse_as_is = Url::parse(input);

            match parse_as_is {
                Ok(url) => Ok(url),
                Err(_) => Url::parse(&format!("https://{}", input)),
            }
        }
        .map_err(|_| format!("{} is not a valid URL.", input))?;

        if url.scheme() == "javascript" {
            return Err(format!("{} is not an allowed href.", input));
        }

        Ok(Self(url))
    }
}

impl Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn href_parses_valid_strings() {
        // Arrange
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
            // Act
            let href = Href::parse(case);

            // Assert
            assert!(
                href.is_ok(),
                r#"Failed to parse valid string: "{}" with error: {}"#,
                case,
                href.err().unwrap()
            );
        }
    }

    #[test]
    fn href_does_not_parse_invalid_strings() {
        // Arrange
        let test_cases = [
            "javascript:alert('Hello, world!')",
            "hello world",
            "",
            "    ",
            "\t",
        ];

        for case in test_cases {
            // Act
            let href = Href::parse(case);

            // Assert
            assert!(
                href.is_err(),
                r#"Parsed invalid string: "{}" as {}"#,
                case,
                href.unwrap().0
            );
        }
    }
}
