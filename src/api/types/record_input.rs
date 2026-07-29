pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordInput {
    /// Your unique identifier for the record.
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    /// URL for the original content.
    #[serde(rename = "clientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_url: Option<String>,
    /// Name or title of the record.
    #[serde(default)]
    pub name: String,
    /// Type of record, such as post, comment, or message.
    #[serde(default)]
    pub entity: String,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInput>,
}

impl RecordInput {
    pub fn builder() -> RecordInputBuilder {
        <RecordInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordInputBuilder {
    client_id: Option<String>,
    client_url: Option<String>,
    name: Option<String>,
    entity: Option<String>,
    content: Option<Content>,
    metadata: Option<Metadata>,
    user: Option<UserInput>,
}

impl RecordInputBuilder {
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

    pub fn content(mut self, value: Content) -> Self {
        self.content = Some(value);
        self
    }

    pub fn metadata(mut self, value: Metadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn user(mut self, value: UserInput) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](RecordInputBuilder::client_id)
    /// - [`name`](RecordInputBuilder::name)
    /// - [`entity`](RecordInputBuilder::entity)
    /// - [`content`](RecordInputBuilder::content)
    pub fn build(self) -> Result<RecordInput, BuildError> {
        Ok(RecordInput {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_url: self.client_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            metadata: self.metadata,
            user: self.user,
        })
    }
}
