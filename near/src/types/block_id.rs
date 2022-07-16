use serde::Serialize;

#[derive(Debug, Clone, Deserialize)]
pub enum BlockId {
    Height(u64),
    Hash(String),
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BlockId::Height(height) => height.serialize(serializer),
            BlockId::Hash(hash) => hash.serialize(serializer),
        }
    }
}
