pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngestRecordResponse {
    #[serde(flatten)]
    pub success_response_fields: SuccessResponse,
    /// Insion record ID.
    #[serde(default)]
    pub id: String,
    /// Insion moderation ID when moderation was queued; otherwise null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<String>,
    /// Insion user ID when a user was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl IngestRecordResponse {
    pub fn builder() -> IngestRecordResponseBuilder {
        <IngestRecordResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestRecordResponseBuilder {
    success_response_fields: Option<SuccessResponse>,
    id: Option<String>,
    moderation: Option<String>,
    user: Option<String>,
}

impl IngestRecordResponseBuilder {
    pub fn success_response_fields(mut self, value: SuccessResponse) -> Self {
        self.success_response_fields = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn moderation(mut self, value: impl Into<String>) -> Self {
        self.moderation = Some(value.into());
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IngestRecordResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success_response_fields`](IngestRecordResponseBuilder::success_response_fields)
    /// - [`id`](IngestRecordResponseBuilder::id)
    pub fn build(self) -> Result<IngestRecordResponse, BuildError> {
        Ok(IngestRecordResponse {
            success_response_fields: self
                .success_response_fields
                .ok_or_else(|| BuildError::missing_field("success_response_fields"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            moderation: self.moderation,
            user: self.user,
        })
    }
}
