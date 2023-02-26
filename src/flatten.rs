pub use self::Flattener as Flatten;
use serde_json::{Map, Value};

pub struct Flattener {
    delimiter: char,
}

impl Default for Flattener {
    fn default() -> Self {
        Self { delimiter: '.' }
    }
}

impl Flattener {
    pub fn new(delimiter: char) -> Self {
        Self { delimiter }
    }
    fn flatten_map(&self, map: &Map<String, Value>, prefix: &str, result: &mut Map<String, Value>) {
        for (k, v) in map.iter() {
            let new_pref = if prefix.is_empty() {
                k.clone()
            } else {
                format!("{}{}{}", prefix, self.delimiter, k)
            };
            match v {
                serde_json::Value::Object(inner) => {
                    self.flatten_map(inner, &new_pref, result);
                }
                serde_json::Value::Array(arr) => {
                    self.flatten_array(arr, &new_pref, result);
                }
                _ => {
                    result.insert(new_pref, v.clone());
                }
            }
        }
    }
    fn flatten_array(
        &self,
        array: &[serde_json::Value],
        prefix: &str,
        result: &mut Map<String, Value>,
    ) {
        for (index, value) in array.iter().enumerate() {
            let new_prefix = format!("{}{}{}", prefix, self.delimiter, index);
            match value {
                serde_json::Value::Object(inner_map) => {
                    self.flatten_map(inner_map, &new_prefix, result);
                }
                serde_json::Value::Array(inner_array) => {
                    self.flatten_array(inner_array, &new_prefix, result);
                }
                _ => {
                    result.insert(new_prefix, value.clone());
                }
            }
        }
    }
}

impl crate::filter::Filter for Flattener {
    type Output = Value;

    fn filter(&self, json: Value) -> Result<Self::Output, anyhow::Error> {
        let mut result = Map::new();
        match json {
            Value::Object(map) => {
                self.flatten_map(&map, "", &mut result);
            }
            Value::Array(arr) => {
                self.flatten_array(&arr, "", &mut result);
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Flattener can only be used on objects or arrays"
                ))
            }
        }
        return Ok(Value::Object(result));
    }
}

#[cfg(test)]
mod flatten_test {
    use super::*;
    use crate::prelude::{filter, Filter};

    #[test]
    fn test_flatten() {
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
}
