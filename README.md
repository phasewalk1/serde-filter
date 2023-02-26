# Overview

`serde_filter` is a library crate that provides filtering abstractions for JSON objects and arrays
using `serde` as a backend. It allows you to easily filter and transform complex JSON structures by providing a set of configurable filters.

The crate provides a number of out-of-the-box filters for common use cases, such as filtering by key or value, flattening nested objects and arrays, and more. You can also implement your own filters by implementing the `Filter` trait.

## Using Pre-Built Filters

```Rust
let condensed = filter::<Flatten>(json.clone(), &Flatten::new('.'))?;
let trimmed = filter::<Ignore>(json.clone(), &Ignore::new(vec!["explanation", "media_type"]))?;
let nums = filter::<Match<u64>>(json.clone(), &Match::new("activeRegionNum"))?;
```

### Matching on a Key

The Match filter allows you to filter a JSON object by matching on a specific key. You can use it to retrieve all values in a nested object that have a certain key, or to retrieve a specific value associated with a given key.

```Rust
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
```

### Flattening Nested Structures

`example/flatten.rs` showcases how to use the Flatten filter to flatten a nested JSON object:

```Rust
let json = json!({
    "name": "John Smith",
    "age": 30,
    "address": {
        "street": "123 Main St",
        "city": "Anytown",
        "state": "CA"
    }
});

let flattened = filter::<Flatten>(json, &Flattener::default())?;

assert_eq!(flattened, json!({
    "name": "John Smith",
    "age": 30,
    "address.street": "123 Main St",
    "address.city": "Anytown",
    "address.state": "CA"
}));
```

### Ignoring Keys

The Ignore filter allows you to remove specific keys from a JSON object. This can be useful when you want to exclude certain fields from a response or when you want to anonymize sensitive data.

```Rust
let json = json!({
    "2020-01-01": {
        "explanation": "test_1",
        "media_type": "test",
        "hdurl": "test",
        "service_version": "test",
        "code": 200,
        "msg": "test"
    },
    "Object": {
        "2023-01-11": {
            "Object": {
                "explanation": "test_3",
                "media_type": "test",
                "hdurl": "test",
                "service_version": "test",
                "code": 200,
                "msg": "test"
            },
            "explanation": "test_2",
        }
    },
    "explanation": "test_0"
});

let ignore = Ignore::new(vec!["explanation"]);
let trimmed = filter::<Ignore>(json, &ignore).unwrap();

assert!(trimmed["2020-01-01"].get("explanation").is_none(), "test_1");
assert!(
    trimmed["Object"]["2023-01-11"]["Object"]
        .get("explanation")
        .is_none(),
    "test_3"
);
assert!(
    trimmed["Object"]["2023-01-11"].get("explanation").is_none(),
    "test_2"
);
assert!(trimmed.get("explanation").is_none(), "test_0");
```

### More Examples

The examples/ directory of this repository contains additional examples showcasing how to use various filters provided by serde_filter.

# Experimental and Unstable

Please note that serde_filter is currently an experimental library and is still in early development. While we have made every effort to ensure that the library is functional and correct, it is not yet stable and may be subject to breaking changes in future releases.

We welcome feedback, bug reports, and contributions from the community to help improve the library and make it more useful for everyone. However, please be aware that any code you write using this library may need to be updated as the library evolves.

We appreciate your interest in serde_filter and look forward to hearing your thoughts and feedback as we continue to develop and improve the library.
