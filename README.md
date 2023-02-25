# serde_filter

Generic filtering abstractions over json objects (`serde_json::Value`).

## Usage

### AdHoc Filters, and the `filter` Function

_AdHoc_ filters are filters that are defined inline instead of built with a custom configuration. Perhaps the most powerful AdHoc filter, is the `Match` filter. The `Match` filter is a type of filter that wraps a `T` and returns a vector of the underlying type. Simply, a `Match` filter serves to only return values if they match a certain key and can serialize into `T`.
Assume we've just queried some endpoint at some server that returns a large, nested JSON object. Within every nested object, there is a field "activeRegionNum" that contains an unsigned integer that we want to collect. Let's see how we can use the `filter` function with the `Match` filter to create an AdHoc filter.

```Rust
use serde_json::json;
use serde_filter::{prelude::*, filters::*};

fn main() where {
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
```
