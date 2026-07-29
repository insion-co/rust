pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetApiV1RecordsRequestStatus {
    Compliant,
    Flagged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetApiV1RecordsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Compliant => serializer.serialize_str("Compliant"),
            Self::Flagged => serializer.serialize_str("Flagged"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetApiV1RecordsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Compliant" => Ok(Self::Compliant),
            "Flagged" => Ok(Self::Flagged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetApiV1RecordsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compliant => write!(f, "Compliant"),
            Self::Flagged => write!(f, "Flagged"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
