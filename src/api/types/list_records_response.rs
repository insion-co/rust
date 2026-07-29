pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRecordsResponse {
    #[serde(default)]
    pub data: Vec<Record>,
    #[serde(default)]
    pub has_more: bool,
}

impl ListRecordsResponse {
    pub fn builder() -> ListRecordsResponseBuilder {
        <ListRecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecordsResponseBuilder {
    data: Option<Vec<Record>>,
    has_more: Option<bool>,
}

impl ListRecordsResponseBuilder {
    pub fn data(mut self, value: Vec<Record>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecordsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRecordsResponseBuilder::data)
    /// - [`has_more`](ListRecordsResponseBuilder::has_more)
    pub fn build(self) -> Result<ListRecordsResponse, BuildError> {
        Ok(ListRecordsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
