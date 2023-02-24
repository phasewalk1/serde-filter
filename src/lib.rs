#![feature(associated_type_defaults)]

//! ```no_run
//! use nerva::clients::donki::flr::*;
//! use nerva::prelude::*;
//! use serde_filter::prelude::*;
//! use serde_filter::filters::*;
//!
//! fn main() {
//!     let flr = FLR::default();
//!     let params = FLRParams::default();
//!     let response = flr.get(params).unwrap();
//!     let values = filter::<Match<u64>>(response, &Match::new("activeRegionNum")).unwrap();
//!     println!("{:?}", values)
//! }
//! ```

pub mod ignore;
/// The Match filter
pub mod matches;
/// Common traits
pub mod prelude;

pub use prelude::{Filter, Matchable};
/// Common filters
pub mod filters {
    pub use super::filter;
    pub use super::matches::*;
    pub use super::Filter;
}

/// AdHoc filter function
pub fn filter<F>(json: serde_json::Value, filter: &F) -> Result<Vec<F::Output>, anyhow::Error>
where
    F: Filter,
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
