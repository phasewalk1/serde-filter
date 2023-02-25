/// A Filter transforms a JSON value into a Vec of a specific type
pub trait Filter {
    type Output;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error>;
}
