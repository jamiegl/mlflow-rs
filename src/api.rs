pub mod client;
pub mod error;
pub mod experiment;
pub mod id;
pub mod metric;
pub mod run;
pub mod search;
pub mod model_versions;

pub mod limits {
    pub const BATCH_TOTAL: usize = 1000;
    pub const BATCH_METRICS: usize = 1000;
    pub const BATCH_PARAMS: usize = 100;
    pub const BATCH_TAGS: usize = 100;
}

// serialize i64 as str
mod str_int {
    use std::str::FromStr;

    use serde::de::{self, Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i64::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S>(int: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: String = format!("{}", int);
        s.serialize(serializer)
    }
}
// serialize Option<i64> as Option<str>
mod opt_str_int {
    use std::str::FromStr;

    use serde::de::{self, Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        if let Some(s) = s {
            Ok(Some(i64::from_str(&s).map_err(de::Error::custom)?))
        } else {
            Ok(None)
        }
    }

    pub fn serialize<S>(int: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = int.map(|int| format!("{}", int));
        s.serialize(serializer)
    }
}
