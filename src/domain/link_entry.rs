use crate::domain::{Code, Href, Slug};

#[derive(Debug)]
pub struct LinkEntry {
    pub href: Href,
    pub slug: Slug,
    pub code: Code,
}
