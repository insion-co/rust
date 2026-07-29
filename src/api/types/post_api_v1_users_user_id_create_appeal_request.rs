pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostApiV1UsersUserIdCreateAppealRequest {
    /// The appeal message.
    #[serde(default)]
    pub text: String,
}

impl PostApiV1UsersUserIdCreateAppealRequest {
    pub fn builder() -> PostApiV1UsersUserIdCreateAppealRequestBuilder {
        <PostApiV1UsersUserIdCreateAppealRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostApiV1UsersUserIdCreateAppealRequestBuilder {
    text: Option<String>,
}

impl PostApiV1UsersUserIdCreateAppealRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostApiV1UsersUserIdCreateAppealRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PostApiV1UsersUserIdCreateAppealRequestBuilder::text)
    pub fn build(self) -> Result<PostApiV1UsersUserIdCreateAppealRequest, BuildError> {
        Ok(PostApiV1UsersUserIdCreateAppealRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
