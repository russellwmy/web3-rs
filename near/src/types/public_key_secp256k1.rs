use {
    libsecp256k1 as secp256k1,
    serde::{Deserialize, Serialize},
    std::fmt::{Debug, Display},
};

#[derive(Clone)]
pub struct Secp256k1PublicKey(secp256k1::PublicKey);

impl Serialize for Secp256k1PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let public_key_bytes = self.0.serialize();
        let encoded_public_key = bs58::encode(public_key_bytes).into_string();
        let namespaced_public_key = format!("secp256k1:{}", encoded_public_key);

        serializer.serialize_str(&namespaced_public_key)
    }
}

impl<'de> Deserialize<'de> for Secp256k1PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;
        let public_key_bytes =
            base64::decode(input.split(':').nth(1).unwrap()).expect("base64 decode failed");

        Ok(Secp256k1PublicKey(
            secp256k1::PublicKey::parse_slice(
                &public_key_bytes,
                Some(secp256k1::PublicKeyFormat::Full),
            )
            .unwrap(),
        ))
    }
}

impl From<&str> for Secp256k1PublicKey {
    fn from(s: &str) -> Self {
        let public_key_bytes = bs58::decode(s.split(':').nth(1).unwrap())
            .into_vec()
            .unwrap();
        Secp256k1PublicKey(
            secp256k1::PublicKey::parse_slice(
                &public_key_bytes,
                Some(secp256k1::PublicKeyFormat::Full),
            )
            .unwrap(),
        )
    }
}

impl Display for Secp256k1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = bs58::encode(self.0.serialize()).into_string();
        write!(f, "{}", str)
    }
}

impl Debug for Secp256k1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = bs58::encode(self.0.serialize()).into_string();
        write!(f, "{}", str)
    }
}
