pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserResponse {
    #[serde(default)]
    pub data: User,
}

impl UserResponse {
    pub fn builder() -> UserResponseBuilder {
        <UserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserResponseBuilder {
    data: Option<User>,
}

impl UserResponseBuilder {
    pub fn data(mut self, value: User) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](UserResponseBuilder::data)
    pub fn build(self) -> Result<UserResponse, BuildError> {
        Ok(UserResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
