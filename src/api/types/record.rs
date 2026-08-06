pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Record {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_url: Option<String>,
    /// Name or title of the record. Null when submitted using passthrough moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub entity: String,
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
    #[serde(rename = "moderationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_status: Option<RecordModerationStatus>,
    #[serde(rename = "moderationStatusCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_status_created_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "moderationPending")]
    #[serde(default)]
    pub moderation_pending: bool,
    #[serde(rename = "moderationPendingCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_pending_created_at: Option<DateTime<FixedOffset>>,
    /// Associated Insion user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Record {
    pub fn builder() -> RecordBuilder {
        <RecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordBuilder {
    id: Option<String>,
    client_id: Option<String>,
    client_url: Option<String>,
    name: Option<String>,
    entity: Option<String>,
    protected: Option<bool>,
    metadata: Option<Metadata>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    moderation_status: Option<RecordModerationStatus>,
    moderation_status_created_at: Option<DateTime<FixedOffset>>,
    moderation_pending: Option<bool>,
    moderation_pending_created_at: Option<DateTime<FixedOffset>>,
    user: Option<String>,
}

impl RecordBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn entity(mut self, value: impl Into<String>) -> Self {
        self.entity = Some(value.into());
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

    pub fn moderation_status(mut self, value: RecordModerationStatus) -> Self {
        self.moderation_status = Some(value);
        self
    }

    pub fn moderation_status_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.moderation_status_created_at = Some(value);
        self
    }

    pub fn moderation_pending(mut self, value: bool) -> Self {
        self.moderation_pending = Some(value);
        self
    }

    pub fn moderation_pending_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.moderation_pending_created_at = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Record`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecordBuilder::id)
    /// - [`client_id`](RecordBuilder::client_id)
    /// - [`entity`](RecordBuilder::entity)
    /// - [`protected`](RecordBuilder::protected)
    /// - [`created_at`](RecordBuilder::created_at)
    /// - [`updated_at`](RecordBuilder::updated_at)
    /// - [`moderation_pending`](RecordBuilder::moderation_pending)
    pub fn build(self) -> Result<Record, BuildError> {
        Ok(Record {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_url: self.client_url,
            name: self.name,
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
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
            moderation_status: self.moderation_status,
            moderation_status_created_at: self.moderation_status_created_at,
            moderation_pending: self
                .moderation_pending
                .ok_or_else(|| BuildError::missing_field("moderation_pending"))?,
            moderation_pending_created_at: self.moderation_pending_created_at,
            user: self.user,
        })
    }
}
