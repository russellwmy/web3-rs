#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

pub mod rpc;
pub mod types;

pub use web3_core as core;
