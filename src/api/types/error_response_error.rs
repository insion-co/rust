pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorResponseError {
    #[serde(default)]
    pub message: String,
}

impl ErrorResponseError {
    pub fn builder() -> ErrorResponseErrorBuilder {
        <ErrorResponseErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseErrorBuilder {
    message: Option<String>,
}

impl ErrorResponseErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponseError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ErrorResponseErrorBuilder::message)
    pub fn build(self) -> Result<ErrorResponseError, BuildError> {
        Ok(ErrorResponseError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
