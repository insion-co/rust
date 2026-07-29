pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct User {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default)]
    pub protected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "actionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_status: Option<UserActionStatus>,
    #[serde(rename = "actionStatusCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_status_created_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "appealUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_url: Option<String>,
}

impl User {
    pub fn builder() -> UserBuilder {
        <UserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBuilder {
    id: Option<String>,
    client_id: Option<String>,
    client_url: Option<String>,
    email: Option<String>,
    name: Option<String>,
    username: Option<String>,
    protected: Option<bool>,
    metadata: Option<Metadata>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    action_status: Option<UserActionStatus>,
    action_status_created_at: Option<DateTime<FixedOffset>>,
    appeal_url: Option<String>,
}

impl UserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_url(mut self, value: impl Into<String>) -> Self {
        self.client_url = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn protected(mut self, value: bool) -> Self {
        self.protected = Some(value);
        self
    }

    pub fn metadata(mut self, value: Metadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn action_status(mut self, value: UserActionStatus) -> Self {
        self.action_status = Some(value);
        self
    }

    pub fn action_status_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.action_status_created_at = Some(value);
        self
    }

    pub fn appeal_url(mut self, value: impl Into<String>) -> Self {
        self.appeal_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`User`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserBuilder::id)
    /// - [`client_id`](UserBuilder::client_id)
    /// - [`protected`](UserBuilder::protected)
    /// - [`created_at`](UserBuilder::created_at)
    /// - [`updated_at`](UserBuilder::updated_at)
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_url: self.client_url,
            email: self.email,
            name: self.name,
            username: self.username,
            protected: self
                .protected
                .ok_or_else(|| BuildError::missing_field("protected"))?,
            metadata: self.metadata,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            action_status: self.action_status,
            action_status_created_at: self.action_status_created_at,
            appeal_url: self.appeal_url,
        })
    }
}
