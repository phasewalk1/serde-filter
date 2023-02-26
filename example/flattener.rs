extern crate serde_filter;
extern crate serde_json;
use serde_filter::prelude::*;
use serde_json::json;

fn main() {
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

    let flattener = Flatten::default(); // default is '.'
    let result = filter::<Flatten>(json, &flattener).unwrap();
    println!("{:?}", result);
    assert_eq!(result, expected);
}
