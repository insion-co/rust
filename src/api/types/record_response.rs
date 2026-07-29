pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordResponse {
    #[serde(default)]
    pub data: Record,
}

impl RecordResponse {
    pub fn builder() -> RecordResponseBuilder {
        <RecordResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordResponseBuilder {
    data: Option<Record>,
}

impl RecordResponseBuilder {
    pub fn data(mut self, value: Record) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RecordResponseBuilder::data)
    pub fn build(self) -> Result<RecordResponse, BuildError> {
        Ok(RecordResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
