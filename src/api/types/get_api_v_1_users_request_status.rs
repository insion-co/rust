pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetApiV1UsersRequestStatus {
    Compliant,
    Suspended,
    Banned,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetApiV1UsersRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Compliant => serializer.serialize_str("Compliant"),
            Self::Suspended => serializer.serialize_str("Suspended"),
            Self::Banned => serializer.serialize_str("Banned"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetApiV1UsersRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Compliant" => Ok(Self::Compliant),
            "Suspended" => Ok(Self::Suspended),
            "Banned" => Ok(Self::Banned),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetApiV1UsersRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compliant => write!(f, "Compliant"),
            Self::Suspended => write!(f, "Suspended"),
            Self::Banned => write!(f, "Banned"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
