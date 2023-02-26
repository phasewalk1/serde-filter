/// A Filter transforms a JSON value into a Filter::Output
pub trait Filter {
    type Output;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error>;
}

/// Runs a filter function on a JSON value
/// ### Example
/// ```no_run
/// use serde_filter::prelude::*;
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
