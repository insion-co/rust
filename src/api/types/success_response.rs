pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SuccessResponse {
    #[serde(default)]
    pub message: String,
}

impl SuccessResponse {
    pub fn builder() -> SuccessResponseBuilder {
        <SuccessResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SuccessResponseBuilder {
    message: Option<String>,
}

impl SuccessResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SuccessResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SuccessResponseBuilder::message)
    pub fn build(self) -> Result<SuccessResponse, BuildError> {
        Ok(SuccessResponse {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
