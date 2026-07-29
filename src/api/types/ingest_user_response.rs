pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngestUserResponse {
    #[serde(flatten)]
    pub success_response_fields: SuccessResponse,
    /// Insion user ID.
    #[serde(default)]
    pub id: String,
}

impl IngestUserResponse {
    pub fn builder() -> IngestUserResponseBuilder {
        <IngestUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestUserResponseBuilder {
    success_response_fields: Option<SuccessResponse>,
    id: Option<String>,
}

impl IngestUserResponseBuilder {
    pub fn success_response_fields(mut self, value: SuccessResponse) -> Self {
        self.success_response_fields = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IngestUserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success_response_fields`](IngestUserResponseBuilder::success_response_fields)
    /// - [`id`](IngestUserResponseBuilder::id)
    pub fn build(self) -> Result<IngestUserResponse, BuildError> {
        Ok(IngestUserResponse {
            success_response_fields: self
                .success_response_fields
                .ok_or_else(|| BuildError::missing_field("success_response_fields"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
