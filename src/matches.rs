/// The Match filter
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Match<M> {
    key: String,
    values: Option<Vec<M>>,
}

impl<M> Match<M>
where
    M: crate::prelude::Matchable,
{
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            values: None,
        }
    }
}

impl<M> crate::prelude::Filter for Match<M>
where
    M: crate::prelude::Matchable,
{
    type Output = Vec<M>;
    fn filter(&self, json: serde_json::Value) -> Result<Self::Output, anyhow::Error> {
        let key = self.key.clone();
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(json);
        while let Some(value) = stack.pop() {
            match value {
                serde_json::Value::Object(map) => {
                    for (k, v) in map {
                        if k == key {
                            if !v.is_null() {
                                result.push(M::from_json(v).unwrap());
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

        Ok(result)
    }
}

impl crate::prelude::Matchable for String {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_str().map(|s| s.to_owned())
    }
}

impl crate::prelude::Matchable for i64 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_i64()
    }
}

impl crate::prelude::Matchable for i32 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_i64().map(|i| i as i32)
    }
}

impl crate::prelude::Matchable for i16 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_i64().map(|i| i as i16)
    }
}

impl crate::prelude::Matchable for u64 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_u64()
    }
}

impl crate::prelude::Matchable for u32 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_u64().map(|u| u as u32)
    }
}

impl crate::prelude::Matchable for u16 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_u64().map(|u| u as u16)
    }
}

impl crate::prelude::Matchable for f64 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_f64()
    }
}

impl crate::prelude::Matchable for f32 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_f64().map(|f| f as f32)
    }
}

impl crate::prelude::Matchable for bool {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_bool()
    }
}

impl crate::prelude::Matchable for (String, String) {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        if let serde_json::Value::Object(map) = json {
            if let (Some(k), Some(v)) = (map.get("key"), map.get("value")) {
                if let (Some(k), Some(v)) = (k.as_str(), v.as_str()) {
                    return Some((k.to_owned(), v.to_owned()));
                }
            }
        }
        None
    }
}
