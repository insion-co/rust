pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentExternalUrls {
    /// Text content to moderate.
    #[serde(default)]
    pub text: String,
    /// Image URLs to moderate.
    #[serde(rename = "imageUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_urls: Option<Vec<String>>,
    /// External page URLs to moderate.
    #[serde(rename = "externalUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Vec<String>>,
}

impl ContentExternalUrls {
    pub fn builder() -> ContentExternalUrlsBuilder {
        <ContentExternalUrlsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentExternalUrlsBuilder {
    text: Option<String>,
    image_urls: Option<Vec<String>>,
    external_urls: Option<Vec<String>>,
}

impl ContentExternalUrlsBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn image_urls(mut self, value: Vec<String>) -> Self {
        self.image_urls = Some(value);
        self
    }

    pub fn external_urls(mut self, value: Vec<String>) -> Self {
        self.external_urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentExternalUrls`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ContentExternalUrlsBuilder::text)
    pub fn build(self) -> Result<ContentExternalUrls, BuildError> {
        Ok(ContentExternalUrls {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            image_urls: self.image_urls,
            external_urls: self.external_urls,
        })
    }
}
