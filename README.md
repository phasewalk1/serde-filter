# Overview

`serde_filter` is a library crate that provides filtering abstractions for JSON objects and arrays
using `serde` as a backend. It allows you to easily filter and transform complex JSON structures by providing a set of configurable filters.

The crate provides a number of out-of-the-box filters for common use cases, such as filtering by key or value, flattening nested objects and arrays, and more. You can also implement your own filters by implementing the `Filter` trait.

## Using Pre-Built Filters

```Rust
use serde_json::json;
use serde_filter::prelude::*;

fn main() where {
    // ******* Match Filter ********
    let json = json!({
        "Object" : {
            "explanation": "test",
            "activeRegionNum": 9876897,
        },
        "2022": {
            "Object" : {
                "explanation": "test",
                "activeRegionNum": 1380402,
            }
        }
    });
    let nums = filter::<Match<u64>>(json, &Match::new("activeRegionNum"));
    if let Ok(n) = nums {
        assert_eq!(nums, vec![9876897 as u64, 1380402 as u64]);
    }
    // ******** Flatten Filter *********
    let json = serde_json::json!({
        "a": {
            "b": {
                "c": {
                    "d": "value"
                }
            }
        },
        "e": "value"
    });
    
    let expected = serde_json::json!({
        "a.b.c.d": "value",
        "e": "value"
    });

    let result = filter::<Flatten>(json, &Flatten::default()).unwrap();
    println!("{:?}", result);
    assert_eq!(result, expected);
}
```
