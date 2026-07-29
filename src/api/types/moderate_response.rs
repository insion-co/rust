pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModerateResponse {
    /// Insion record ID.
    #[serde(default)]
    pub id: String,
    /// Moderation status.
    pub status: ModerateResponseStatus,
    /// Insion moderation ID.
    #[serde(default)]
    pub moderation: String,
    /// Insion user ID when a user was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default)]
    pub message: String,
    /// Deprecated. True when status is Flagged.
    #[serde(default)]
    pub flagged: bool,
    /// IDs of rules that matched the record.
    #[serde(rename = "categoryIds")]
    #[serde(default)]
    pub category_ids: Vec<String>,
}

impl ModerateResponse {
    pub fn builder() -> ModerateResponseBuilder {
        <ModerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerateResponseBuilder {
    id: Option<String>,
    status: Option<ModerateResponseStatus>,
    moderation: Option<String>,
    user: Option<String>,
    message: Option<String>,
    flagged: Option<bool>,
    category_ids: Option<Vec<String>>,
}

impl ModerateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ModerateResponseStatus) -> Self {
        self.status = Some(value);
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

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn flagged(mut self, value: bool) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn category_ids(mut self, value: Vec<String>) -> Self {
        self.category_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ModerateResponseBuilder::id)
    /// - [`status`](ModerateResponseBuilder::status)
    /// - [`moderation`](ModerateResponseBuilder::moderation)
    /// - [`message`](ModerateResponseBuilder::message)
    /// - [`flagged`](ModerateResponseBuilder::flagged)
    /// - [`category_ids`](ModerateResponseBuilder::category_ids)
    pub fn build(self) -> Result<ModerateResponse, BuildError> {
        Ok(ModerateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            moderation: self
                .moderation
                .ok_or_else(|| BuildError::missing_field("moderation"))?,
            user: self.user,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            flagged: self
                .flagged
                .ok_or_else(|| BuildError::missing_field("flagged"))?,
            category_ids: self
                .category_ids
                .ok_or_else(|| BuildError::missing_field("category_ids"))?,
        })
    }
}
