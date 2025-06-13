use crate::domain::{Href, Slug};

#[derive(Debug)]
pub struct LinkEntry {
    pub href: Href,
    pub slug: Slug,
}
