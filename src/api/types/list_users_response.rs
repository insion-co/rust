pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListUsersResponse {
    #[serde(default)]
    pub data: Vec<User>,
    #[serde(default)]
    pub has_more: bool,
}

impl ListUsersResponse {
    pub fn builder() -> ListUsersResponseBuilder {
        <ListUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersResponseBuilder {
    data: Option<Vec<User>>,
    has_more: Option<bool>,
}

impl ListUsersResponseBuilder {
    pub fn data(mut self, value: Vec<User>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUsersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListUsersResponseBuilder::data)
    /// - [`has_more`](ListUsersResponseBuilder::has_more)
    pub fn build(self) -> Result<ListUsersResponse, BuildError> {
        Ok(ListUsersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
