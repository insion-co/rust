pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteApiV1IngestRequest {
    /// Your unique identifier for the record.
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
}

impl DeleteApiV1IngestRequest {
    pub fn builder() -> DeleteApiV1IngestRequestBuilder {
        <DeleteApiV1IngestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteApiV1IngestRequestBuilder {
    client_id: Option<String>,
}

impl DeleteApiV1IngestRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteApiV1IngestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](DeleteApiV1IngestRequestBuilder::client_id)
    pub fn build(self) -> Result<DeleteApiV1IngestRequest, BuildError> {
        Ok(DeleteApiV1IngestRequest {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
        })
    }
}
