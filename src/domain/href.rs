use url::Url;

pub struct Href(String);

impl Href {
    pub fn parse(input: String) -> Result<Self, String> {
        let url = Url::parse(&input).map_err(|_| format!("{} is not a valid URL.", input))?;

        if url.scheme() == "javascript" {
            return Err(format!("{} is not an allowed href.", input));
        }

        Ok(Self(input))
    }
}

impl AsRef<str> for Href {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
