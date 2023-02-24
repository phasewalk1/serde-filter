# serde_filter

Generic filtering abstractions over json objects (`serde_json::Value`).

## Usage

Below are some example usa cases of `serde_filter`. We use the `nerva` crate to query the NASA Open API servers which yield responses in JSON.

```Rust
extern crate nerva;
extern crate serde_filter;
use nerva::clients::donki::flr::*;
use nerva::prelude::*;
use serde_filter::prelude::*;
use serde_filter::filters::*
```

### AdHoc Filters, and the `filter` Function

_AdHoc_ filters are filters that are defined inline instead of built with a custom configuration. Perhaps the most powerful AdHoc filter, is the `Match` filter. The `Match` filter is a type of filter that wraps a `T` and returns a vector of the underlying type. Simply, a `Match` filter serves to only return values if they match a certain key and can serialize into `T`.
Assume we've just queried some endpoint at some server that returns a large, nested JSON object. Within every nested object, there is a field "activeRegionNum" that contains an unsigned integer that we want to collect. Let's see how we can use the `filter` function with the `Match` filter to create an AdHoc filter.

```Rust
fn main()
{
    let flr = FLR::default();
    let query = FLRParams::default();
    let res: serde_json::Value = flr.get(query).unwrap();
    // Construct an AdHoc filter
    let values = filter::<Match<u64>>(res, &Match::new("activeRegionNum"));
    if let Ok(region_nums) = values {
        println!("{:#?}", region_nums)
    }
}
```
