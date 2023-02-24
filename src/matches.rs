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
    type Output = M;
    fn filter(&self, json: serde_json::Value) -> Result<Vec<Self::Output>, anyhow::Error> {
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

impl crate::prelude::Matchable for f32 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_f64().map(|f| f as f32)
    }
}

impl crate::prelude::Matchable for u64 {
    fn from_json(json: serde_json::Value) -> Option<Self> {
        json.as_u64()
    }
}
