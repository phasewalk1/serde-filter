/// A Filter transforms a JSON value into a Vec of a specific type
pub trait Filter {
    type Output;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error>;
}

/// A Matchable type is a type that can be matched against a JSON value
pub trait Matchable
where
    Self: std::fmt::Debug
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Clone
        + std::fmt::Debug
        + PartialEq,
{
    fn from_json(json: serde_json::Value) -> Option<Self>;
}
