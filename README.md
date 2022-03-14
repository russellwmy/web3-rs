# web3-rs (WIP)
Rust implementation of web3 library for Solana

# Basic Usage
```rust
extern crate web3_rs as web3;

use web3::{
    core::{Provider, RcpClient},
    solana,
};

let provider = Provider::new_http_provider("https://api.devnet.solana.com".into());
let client = RcpClient::new(provider);
let request = solana::rpc::GetLargestAccountsRequest::new().into();

match client.send(request).await {
    Ok(response) => {
        let result = solana::rpc::GetLargestAccountsResponse::from(response);
        log::info!("{:?}", result);
    }
    Err(error) => {
        log::warn!("{:?}", error);
    }
};
```
