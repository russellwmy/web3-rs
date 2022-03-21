#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

mod constants;
pub mod rpc;

pub use {solana_sdk as sdk, web3_core as core};
