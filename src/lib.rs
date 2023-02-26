#![feature(associated_type_defaults)]

//! # Overview
//! `serde_filter` is a library crate that provides filtering abstractions for JSON objects and arrays
//! using `serde` as a backend. It allows you to easily filter and transform complex JSON structures by
//! providing a set of configurable filters.
//!
//! The crate provides a number of out-of-the-box filters for common use cases, such as filtering by key
//! or value, flattening nested objects and arrays, and more. You can also implement your own filters
//! by implementing the `Filter` trait.
//!
//! ## Using Pre-Built Filters
//! ```
//! use serde_filter::prelude::*;
//! use serde_json::json;
//!
//! fn main() where {
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
//!      assert_eq!(nums, vec![9876897u64, 1380402u64]);
//! }
//! ```

/// The Filter trait and Adhoc filter function
pub mod filter;
/// Flattens a JSON object into a single level
/// ### Example
/// ```no_run
/// let json = serde_json::json!({
///       "a": {
///       	  "b": {
///			      "c": {
///                   "d": "value"
///                }
///            }
///        },
///        "e": "value"
///        });
///        
///	let expected = serde_json::json!({
/// 	"a.b.c.d": "value",
///     "e": "value"
/// });
///
/// let flattener = Flattener::default(); // default delimiter is '.'
/// let result = filter::<Flatten>(json, &flattener).unwrap();
/// println!("{:?}", result);
/// assert_eq!(result, expected);
pub mod flatten;
/// The Ignore filter
pub mod ignore;
/// The Match filter
pub mod matches;

/// Re-export filters
pub mod prelude {
    pub use super::filter::filter;
    pub use super::filter::Filter;
    pub use super::flatten::Flatten;
    pub use super::ignore::Ignore;
    pub use super::matches::{Match, Matchable};
}

#[cfg(test)]
mod filter_test {
    use super::prelude::{filter, Match};

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
        let values = filter::<Match<String>>(json.clone(), &Match::new("explanation")).unwrap();
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
