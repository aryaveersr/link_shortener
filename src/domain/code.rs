const CODE_MAX_VALUE: u32 = 9999_9999;

#[derive(Debug)]
pub struct Code(u32);

impl Code {
    pub fn parse(input: u32) -> Result<Self, String> {
        let has_more_than_8_digits = input > CODE_MAX_VALUE;

        if has_more_than_8_digits {
            return Err(format!(r#""{}" is not a valid code."#, input));
        }

        Ok(Code(input))
    }

    pub fn generate() -> Self {
        Self(fastrand::u32(0..=CODE_MAX_VALUE))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_parses_valid_numbers() {
        // # Arrange
        let test_cases = [0, 1, CODE_MAX_VALUE, 737];

        for case in test_cases {
            // # Act
            let code = Code::parse(case);

            // # Assert
            assert!(
                code.is_ok(),
                "Failed to parse valid number: {case} with error: {}",
                code.unwrap_err()
            );
        }
    }

    #[test]
    fn code_does_not_parse_invalid_numbers() {
        // # Arrange
        let test_cases = [CODE_MAX_VALUE * 10, CODE_MAX_VALUE + 1];

        for case in test_cases {
            // # Act
            let code = Code::parse(case);

            // # Assert
            assert!(code.is_err(), "Parsed invalid number: {case}",);
        }
    }

    #[test]
    fn code_generates_valid_numbers() {
        for _ in 0..10 {
            // # Act
            let code = Code::generate();

            // # Assert
            assert!(Code::parse(code.0).is_ok(), "Failed for: {}", code.as_u32())
        }
    }
}
