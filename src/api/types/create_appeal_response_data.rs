pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppealResponseData {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "actionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_status: Option<CreateAppealResponseDataActionStatus>,
    #[serde(rename = "actionStatusCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub action_status_created_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "appealUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_url: Option<String>,
}

impl CreateAppealResponseData {
    pub fn builder() -> CreateAppealResponseDataBuilder {
        <CreateAppealResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppealResponseDataBuilder {
    id: Option<String>,
    action_status: Option<CreateAppealResponseDataActionStatus>,
    action_status_created_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    appeal_url: Option<String>,
}

impl CreateAppealResponseDataBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn action_status(mut self, value: CreateAppealResponseDataActionStatus) -> Self {
        self.action_status = Some(value);
        self
    }

    pub fn action_status_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.action_status_created_at = Some(value);
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

    pub fn appeal_url(mut self, value: impl Into<String>) -> Self {
        self.appeal_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAppealResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateAppealResponseDataBuilder::id)
    /// - [`created_at`](CreateAppealResponseDataBuilder::created_at)
    /// - [`updated_at`](CreateAppealResponseDataBuilder::updated_at)
    pub fn build(self) -> Result<CreateAppealResponseData, BuildError> {
        Ok(CreateAppealResponseData {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            action_status: self.action_status,
            action_status_created_at: self.action_status_created_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            appeal_url: self.appeal_url,
        })
    }
}
