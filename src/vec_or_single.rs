use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VecOrSingle<T> {
    Single(T),
    Vec(Vec<T>),
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match VecOrSingle::deserialize(deserializer) {
        Ok(value) => match value {
            VecOrSingle::Single(value) => Ok(vec![value]),
            VecOrSingle::Vec(value) => Ok(value),
        },
        Err(err) => Err(err),
    }
}

pub fn serialize<T, S>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value.len() {
        1 => value.first().serialize(serializer),
        _ => value.serialize(serializer),
    }
}
