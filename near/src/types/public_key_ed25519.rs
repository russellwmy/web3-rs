use {
    ed25519_dalek as ed25519,
    serde::{Deserialize, Serialize},
    std::fmt::{Debug, Display},
};

#[derive(Clone)]
pub struct Ed25519PublicKey(ed25519::PublicKey);

impl Serialize for Ed25519PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let public_key_bytes = self.0.to_bytes();
        let encoded_public_key = bs58::encode(public_key_bytes).into_string();
        let namespaced_public_key = format!("ed25519:{}", encoded_public_key);

        serializer.serialize_str(&namespaced_public_key)
    }
}

impl<'de> Deserialize<'de> for Ed25519PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;
        let public_key_bytes =
            base64::decode(input.split(':').nth(1).unwrap()).expect("base64 decode failed");

        Ok(Ed25519PublicKey(
            ed25519::PublicKey::from_bytes(&public_key_bytes).unwrap(),
        ))
    }
}

impl From<&str> for Ed25519PublicKey {
    fn from(s: &str) -> Self {
        let public_key_bytes = bs58::decode(s.split(':').nth(1).unwrap())
            .into_vec()
            .unwrap();
        Ed25519PublicKey(ed25519::PublicKey::from_bytes(&public_key_bytes).unwrap())
    }
}

impl Debug for Ed25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = bs58::encode(self.0.to_bytes()).into_string();
        write!(f, "{}", str)
    }
}

impl Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = bs58::encode(self.0.to_bytes()).into_string();
        write!(f, "{}", str)
    }
}
