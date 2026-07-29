pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppealResponse {
    #[serde(default)]
    pub data: CreateAppealResponseData,
}

impl CreateAppealResponse {
    pub fn builder() -> CreateAppealResponseBuilder {
        <CreateAppealResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppealResponseBuilder {
    data: Option<CreateAppealResponseData>,
}

impl CreateAppealResponseBuilder {
    pub fn data(mut self, value: CreateAppealResponseData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAppealResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CreateAppealResponseBuilder::data)
    pub fn build(self) -> Result<CreateAppealResponse, BuildError> {
        Ok(CreateAppealResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
