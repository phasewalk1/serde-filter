/// A Filter transforms a JSON value into a Vec of a specific type
pub trait Filter {
    type Output;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error>;
}

/// Run a filter function on a JSON value
/// ### Example
/// ```no_run
/// use serde_filter::{filters::*, prelude::*};
///  let json = serde_json::json!({
///      "explanation": "test",
///      "date": "2020-01-01",
///      "title": "test",
///       "url": "test",
///  });
///  let values = filter::<Match<String>>(json, &Match::new("explanation")).unwrap();
///  assert_eq!(values, vec!["test".to_string()]);
/// ```
pub fn filter<F>(json: serde_json::Value, filter: &F) -> Result<F::Output, anyhow::Error>
where
    F: crate::filter::Filter,
{
    filter.filter(json)
}
