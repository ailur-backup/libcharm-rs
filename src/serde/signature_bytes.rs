use ed25519_dalek::ed25519::SignatureBytes;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(bytes: &SignatureBytes, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(bytes)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SignatureBytes, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
    if bytes.len() != 64 {
        return Err(serde::de::Error::custom("SignatureBytes must be 64 bytes"));
    }
    let mut signature_bytes = [0u8; 64];
    signature_bytes.copy_from_slice(&bytes);
    Ok(signature_bytes)
}
