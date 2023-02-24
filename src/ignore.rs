use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ignore {
    keys: Vec<String>,
    values: Option<Vec<serde_json::Value>>,
}

impl Ignore {
    pub fn new(keys: Vec<&str>) -> Self {
        let keys = keys.into_iter().map(|s| s.to_string()).collect();
        Self { keys, values: None }
    }
}

impl super::prelude::Filter for Ignore {
    type Output = serde_json::Value;
    fn filter(&self, json: serde_json::Value) -> Result<Vec<Self::Output>, anyhow::Error> {
        let ignore_keys = self.keys.clone();
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(json);
        while let Some(value) = stack.pop() {
            match value {
                serde_json::Value::Object(map) => {
                    for (k, v) in map {
                        if !ignore_keys.contains(&k) {
                            if !v.is_null() {
                                let mut map = serde_json::Map::new();
                                map.insert(k, v);
                                result.push(serde_json::Value::Object(map));
                            }
                        } else {
                            stack.push(v);
                        }
                    }
                }
                serde_json::Value::Array(array) => {
                    for v in array {
                        stack.push(v);
                    }
                }
                _ => {}
            }
        }

        let mut my_map = serde_json::Map::new();
        let res_c = result.clone();
        res_c.into_iter().for_each(|v| {
            if let serde_json::Value::Object(map) = v {
                for (k, v) in map {
                    my_map.insert(k, v);
                }
            }
        });

        let result = vec![serde_json::Value::Object(my_map)];
        Ok(result)
    }
}

#[cfg(test)]
mod ignore_test {
    use super::*;
    use crate::filters::filter;
    use crate::prelude::Filter;
    use nerva::clients::apod::*;
    use nerva::prelude::*;

    #[test]
    fn test_ignore() {
        let json = serde_json::json!(
            {
                "explanation": "test",
                "media_type": "test",
                "hdurl": "test",
                "service_version": "test",
                "code": 200,
                "msg": "test"
            }
        );
        let values =
            filter::<super::Ignore>(json, &super::Ignore::new(vec!["explanation", "media_type"]));
        if let Ok(trimmed) = values {
            assert!(trimmed[0].get("explanation").is_none());
            assert!(trimmed[0].get("media_type").is_none());
        } else {
            panic!();
        }
    }
}
