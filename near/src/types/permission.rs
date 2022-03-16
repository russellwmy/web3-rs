use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallPermission {
    allowance: Option<u128>,
    receiver_id: String,
    method_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Permission {
    FunctionCall(FunctionCallPermission),
    FullAccess,
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Permission::FunctionCall(function_call_permission) => {
                function_call_permission.serialize(serializer)
            }
            Permission::FullAccess => "full_access".serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let function_call_permission = FunctionCallPermission::deserialize(deserializer);
        match function_call_permission {
            Ok(function_call_permission) => Ok(Permission::FunctionCall(function_call_permission)),
            Err(_) => Ok(Permission::FullAccess),
        }
    }
}
