use {
    super::{Ed25519PublicKey, Secp256k1PublicKey},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PublicKey::Ed25519(public_key) => public_key.serialize(serializer),
            PublicKey::Secp256k1(public_key) => public_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;
        match input.split(':').nth(0).unwrap() {
            "ed25519" => Ok(PublicKey::Ed25519(Ed25519PublicKey::from(input.as_str()))),
            "secp256k1" => Ok(PublicKey::Secp256k1(Secp256k1PublicKey::from(
                input.as_str(),
            ))),
            _ => Err(serde::de::Error::custom("invalid public key")),
        }
    }
}

impl From<&str> for PublicKey {
    fn from(s: &str) -> Self {
        if s.starts_with("ed25519:") {
            PublicKey::Ed25519(Ed25519PublicKey::from(s))
        } else if s.starts_with("secp256k1:") {
            PublicKey::Secp256k1(Secp256k1PublicKey::from(s))
        } else {
            panic!("Invalid public key: {}", s);
        }
    }
}
