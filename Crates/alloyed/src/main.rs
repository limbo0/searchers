use alloyed::{
    curve_dex::{self, IndexedPools},
    etherscan::{create_contract_instance_for_any_address, get_abi_from_etherscan},
    {fetch_price_on_all_dex, NodeClient},
};
use ethers::{
    abi::Abi,
    middleware::providers::Provider,
    providers::Middleware,
    types::{H160, H256, U256},
    utils::hex,
};
use eyre::Result;
use std::{
    env,
    fs::File,
    sync::Arc,
    time::{self, Duration, Instant},
};
use tokio::time::sleep;

const INFURA_MAINNET: &str = "https://mainnet.infura.io/v3/af270f1023f34ef88fdcf6b85286734c";
const CURVE_FI_DUSD: &str = "0x8038C01A0390a8c547446a0b2c18fc9aEFEcc10c";

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<()> {
    // rpc client, needed to interact with ethereum blockchain.
    let provider = Provider::try_from(INFURA_MAINNET).unwrap();
    let client = Arc::new(provider.clone());
    // let (tx, mut rx) = tokio::sync::mpsc::channel(512);

    fetch_price_on_all_dex(
        "0x6b175474e89094c44da98b954eedeac495271d0f"
            .parse::<H160>()
            .unwrap(),
        client.clone(),
    )
    .await;

    Ok(())
}
