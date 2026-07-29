pub use crate::prelude::*;

/// Query parameters for listRecords
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRecordsQueryRequest {
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Return items after this Insion ID. Cannot be used with ending_before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Return items before this Insion ID. Cannot be used with starting_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Filter by your record identifier.
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Filter by Insion user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Filter by record entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// Filter by moderation status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetApiV1RecordsRequestStatus>,
}

impl ListRecordsQueryRequest {
    pub fn builder() -> ListRecordsQueryRequestBuilder {
        <ListRecordsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecordsQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    client_id: Option<String>,
    user: Option<String>,
    entity: Option<String>,
    status: Option<GetApiV1RecordsRequestStatus>,
}

impl ListRecordsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn entity(mut self, value: impl Into<String>) -> Self {
        self.entity = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetApiV1RecordsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecordsQueryRequest`].
    pub fn build(self) -> Result<ListRecordsQueryRequest, BuildError> {
        Ok(ListRecordsQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            client_id: self.client_id,
            user: self.user,
            entity: self.entity,
            status: self.status,
        })
    }
}
