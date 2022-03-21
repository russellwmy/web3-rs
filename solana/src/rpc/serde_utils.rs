use {
    serde::{Deserialize, Deserializer, Serializer},
    solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Signature},
    std::str::FromStr,
};

pub fn serialize_public_key<S>(public_key: &Option<Pubkey>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(public_key.unwrap().to_string().as_str())
}

pub fn deserialize_public_key<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Pubkey::from_str(s.as_str()).unwrap())
}

pub fn deserialize_hash<'de, D>(deserializer: D) -> Result<Hash, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Hash::from_str(s.as_str()).unwrap())
}

pub fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Signature::from_str(s.as_str()).unwrap())
}
