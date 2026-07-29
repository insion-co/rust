pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserInput {
    /// Your unique identifier for the user.
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    /// URL for the user's profile.
    #[serde(rename = "clientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_url: Option<String>,
    /// The user's Stripe account ID.
    #[serde(rename = "stripeAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account_id: Option<String>,
    /// The user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether the user is protected from automated moderation actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UserInput {
    pub fn builder() -> UserInputBuilder {
        <UserInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserInputBuilder {
    client_id: Option<String>,
    client_url: Option<String>,
    stripe_account_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    username: Option<String>,
    protected: Option<bool>,
    metadata: Option<Metadata>,
}

impl UserInputBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_url(mut self, value: impl Into<String>) -> Self {
        self.client_url = Some(value.into());
        self
    }

    pub fn stripe_account_id(mut self, value: impl Into<String>) -> Self {
        self.stripe_account_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`UserInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](UserInputBuilder::client_id)
    pub fn build(self) -> Result<UserInput, BuildError> {
        Ok(UserInput {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_url: self.client_url,
            stripe_account_id: self.stripe_account_id,
            email: self.email,
            name: self.name,
            username: self.username,
            protected: self.protected,
            metadata: self.metadata,
        })
    }
}
