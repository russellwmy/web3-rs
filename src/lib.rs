#![no_std]

#[cfg(feature = "near")]
pub use near;
#[cfg(feature = "solana")]
pub use solana;

pub use web3_core as core;
