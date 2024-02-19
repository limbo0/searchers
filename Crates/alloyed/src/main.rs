use alloyed::{
    curve_dex::{
        self, best_pool_to_swap_in_curve, get_all_pools, index_tokens_in_pools, tokens_and_decimals,
    },
    etherscan::{create_contract_instance_for_any_address, get_abi_from_etherscan},
    NodeClient,
};
use ethers::{
    abi::token,
    middleware::providers::Provider,
    types::{H160, H256, U256},
};
use eyre::Result;
use std::sync::Arc;

const INFURA_MAINNET: &str = "https://mainnet.infura.io/v3/af270f1023f34ef88fdcf6b85286734c";
const CURVE_FI_DUSD: &str = "0x8038C01A0390a8c547446a0b2c18fc9aEFEcc10c";
const WETH: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
const DAI: &str = "0x6b175474e89094c44da98b954eedeac495271d0f";
const USDC: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<()> {
    // rpc client, needed to interact with ethereum blockchain.
    let provider = Provider::try_from(INFURA_MAINNET).unwrap();
    let client = Arc::new(provider.clone());
    // let (tx, mut rx) = tokio::sync::mpsc::channel(512);

    // fetch_price_on_all_dex(USDC.parse::<H160>().unwrap(), client.clone()).await;

    let returnsdata = index_tokens_in_pools(USDC.parse::<H160>().unwrap(), client.clone())
        .await
        .unwrap();

    println!("{:#?}", returnsdata);

    Ok(())
}
