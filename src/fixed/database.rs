use std::collections::HashMap;

pub struct Database<K, V> {
    pub data: HashMap<K, V>,
}

impl<K, V> Database<K, V>
where
    K: serde::de::DeserializeOwned
        + serde::Serialize
        + std::cmp::Eq
        + std::cmp::Ord
        + std::fmt::Debug
        + std::hash::Hash,
    V: serde::de::DeserializeOwned + serde::Serialize,
{
    /// Loads from yaml
    /// # Errors
    /// Errors when yaml couldnt be parsed
    pub fn parse_yaml(yaml_str: &str) -> serde_yaml::Result<Self> {
        Ok(Self {
            data: serde_yaml::from_str(yaml_str)?,
        })
    }

    pub(crate) fn p(yaml_str: &str) -> Self {
        Self::parse_yaml(yaml_str).expect("failed to parse statics")
    }

    #[must_use]
    pub fn get(&self, key: &K) -> &V {
        if let Some(value) = self.data.get(key) {
            value
        } else {
            panic!("statics has to contain every key. It is missing {:?}", key)
        }
    }
}
