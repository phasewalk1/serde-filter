#![feature(associated_type_defaults)]

//! ```
//! use serde_filter::prelude::*;
//! use serde_filter::filters::*;
//! use serde_json::json;
//!
//! fn main() {
//!     let json = serde_json::json!({
//!         "Object" : {
//!                "explanation": "test",
//!                "activeRegionNum": 9876897,
//!          },
//!          "2022": {
//!              "Object" : {
//!                  "explanation": "test",
//!                  "activeRegionNum": 1380402,
//!              }
//!          }
//!      });
//!      let nums: Vec<u64> = filter::<Match<u64>>(json, &Match::new("activeRegionNum")).unwrap();
//!      assert_eq!(nums, vec![9876897, 1380402]);
//! }
//! ```

/// Common traits
pub mod filter;
/// The Ignore filter
pub mod ignore;
/// The Match filter
pub mod matches;

/// Re-export filters
pub mod filters {
    pub use super::filter;
    pub use super::filter::Filter;
    pub use super::ignore::Ignore;
    pub use super::matches::{Match, Matchable};
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

#[cfg(test)]
mod filter_test {
    use super::filters::*;

    #[test]
    fn test_match() {
        let json = serde_json::json!({
            "explanation": "test",
            "date": "2020-01-01",
            "title": "test",
            "url": "test",
            "media_type": "test",
            "hdurl": "test",
            "service_version": "test",
            "code": 200,
            "msg": "test"
        });
        let values = filter::<Match<String>>(json, &Match::new("explanation")).unwrap();
        assert_eq!(values, vec!["test".to_string()]);

        let json = serde_json::json!({
            "Object" : {
                "explanation": "test",
                "activeRegionNum": 1,
            },
            "2022": {
                "Object" : {
                    "explanation": "test",
                    "activeRegionNum": 2,
                }
            }
        });
        let values = filter::<Match<u64>>(json, &Match::new("activeRegionNum"));
        if let Ok(nums) = values {
            assert_eq!(nums, vec![1, 2]);
        } else {
            panic!("Failed to match");
        }
    }
}
