use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::{self, Deserialize, Deserializer, Serializer};
use uuid::Uuid;

pub fn serialize<S>(id: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&*URL_SAFE_NO_PAD.encode(id.as_bytes()))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let bytes = URL_SAFE_NO_PAD
        .decode(s.as_bytes())
        .map_err(serde::de::Error::custom)?;
    Uuid::from_slice(&bytes).map_err(serde::de::Error::custom)
}
