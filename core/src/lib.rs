#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

mod providers;
mod rpc_client;
mod rpc_error;
mod rpc_request;
mod rpc_response;

pub use {
    providers::{HttpProvider, Provider},
    rpc_client::RcpClient,
    rpc_error::RpcError,
    rpc_request::RpcRequest,
    rpc_response::RpcResponse,
};

pub type RpcResult<T> = std::result::Result<T, RpcError>;
