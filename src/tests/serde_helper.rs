//backmagic here
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(vec: &Vec<(u16, u8)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let list: Vec<[u16; 2]> = vec.iter().map(|&(a, b)| [a, b as u16]).collect();
    list.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<(u16, u8)>, D::Error>
where
    D: Deserializer<'de>,
{
    let list: Vec<[u16; 2]> = Deserialize::deserialize(deserializer)?;

    Ok(list.into_iter().map(|[a, b]| (a as u16, b as u8)).collect())
}
