use serde::{Deserialize, Deserializer, Serializer};

use base64::{engine::general_purpose, Engine};

#[allow(dead_code)]
pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    let b64encoded = general_purpose::STANDARD.encode(v);
    s.serialize_str(&b64encoded)
}

pub fn deserialize<'a, D: Deserializer<'a>>(d: D) -> Result<Vec<u8>, D::Error> {
    let b64encoded = String::deserialize(d)?;
    let v = general_purpose::STANDARD
        .decode(b64encoded)
        .map_err(|e| serde::de::Error::custom(e))?;
    Ok(v)
}
