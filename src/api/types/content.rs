pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Content {
    String(String),

    ContentExternalUrls(ContentExternalUrls),
}

impl Content {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_content_external_urls(&self) -> bool {
        matches!(self, Self::ContentExternalUrls(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_content_external_urls(&self) -> Option<&ContentExternalUrls> {
        match self {
            Self::ContentExternalUrls(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_content_external_urls(self) -> Option<ContentExternalUrls> {
        match self {
            Self::ContentExternalUrls(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::ContentExternalUrls(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
