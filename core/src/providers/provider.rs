use crate::HttpProvider;

#[derive(Clone)]
pub enum Provider {
    Http(HttpProvider),
}

impl Provider {
    pub fn new_http_provider(url: String) -> Self {
        Self::Http(HttpProvider::new(url))
    }
}
