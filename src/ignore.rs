use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    pub fn sift(json: &serde_json::Value, ignore_keys: &HashSet<String>) -> serde_json::Value {
        match json {
            serde_json::Value::Null => json.clone(),
            serde_json::Value::Bool(_) => json.clone(),
            serde_json::Value::Number(_) => json.clone(),
            serde_json::Value::String(_) => json.clone(),
            serde_json::Value::Array(arr) => {
                let filtered_arr: Vec<serde_json::Value> = arr
                    .iter()
                    .map(|v| Self::sift(v, ignore_keys))
                    .filter(|v| !v.is_null())
                    .collect();
                serde_json::Value::Array(filtered_arr)
            }
            serde_json::Value::Object(map) => {
                let filtered_map: serde_json::Map<String, serde_json::Value> = map
                    .iter()
                    .filter(|(k, _)| !&ignore_keys.contains(k.as_str()))
                    .map(|(k, v)| (k.clone(), Self::sift(v, ignore_keys)))
                    .filter(|(_, v)| !v.is_null())
                    .collect();
                serde_json::Value::Object(filtered_map)
            }
        }
    }
}

impl super::filter::Filter for Ignore {
    type Output = serde_json::Value;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error> {
        let ignore_k = self.keys.clone();
        let filtered = Ignore::sift(&json, &ignore_k.into_iter().collect());
        Ok(filtered)
    }
}

#[cfg(test)]
mod ignore_test {
    use super::Ignore;
    use crate::prelude::filter;
    use serde_json::json;

    #[test]
    fn test_ignore() {
        let json = json!(
            {
                "explanation": "test",
                "media_type": "test",
                "hdurl": "test",
                "service_version": "test",
                "code": 200,
                "msg": "test"
            }
        );
        let values = filter::<Ignore>(json, &Ignore::new(vec!["explanation", "media_type"]));
        if let Ok(trimmed) = values {
            println!("TRIMMED: {:#?}", trimmed);
            assert!(trimmed.get("explanation").is_none());
            assert!(trimmed.get("media_type").is_none());
            assert!(trimmed.get("hdurl").is_some());
        } else {
            panic!();
        }

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
        println!("TRIMMED: {:#?}", trimmed);
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
    }
}
