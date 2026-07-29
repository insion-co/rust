pub use crate::prelude::*;

/// Query parameters for listUsers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUsersQueryRequest {
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Return items after this Insion ID. Cannot be used with ending_before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Return items before this Insion ID. Cannot be used with starting_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Filter by your user identifier.
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Filter by user email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Filter by user action status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetApiV1UsersRequestStatus>,
    /// Filter by Insion user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ListUsersQueryRequest {
    pub fn builder() -> ListUsersQueryRequestBuilder {
        <ListUsersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    client_id: Option<String>,
    email: Option<String>,
    status: Option<GetApiV1UsersRequestStatus>,
    user: Option<String>,
}

impl ListUsersQueryRequestBuilder {
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

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetApiV1UsersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListUsersQueryRequest`].
    pub fn build(self) -> Result<ListUsersQueryRequest, BuildError> {
        Ok(ListUsersQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            client_id: self.client_id,
            email: self.email,
            status: self.status,
            user: self.user,
        })
    }
}
