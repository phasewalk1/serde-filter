extern crate serde_filter;
extern crate serde_json;
use serde_filter::{filters::*, prelude::*};
use serde_json::json;

fn main() {
    let json = json!({
        "Object": {
            "explanation": "test explanation",
            "activeRegionNum": 23
        },
        "2022-01-11": {
            "Object2": {
                "explanation": "none",
                "activeRegionNum": 98
            }
        }
    });
    let nums = filter::<Match<u64>>(json, &Match::new("activeRegionNum")).unwrap();
    assert_eq!(vec![23 as u64, 98 as u64], nums);
}
