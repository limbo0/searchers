use alloyed::{
    curve_dex::{self, IndexedPools},
    etherscan::{create_contract_instance_for_any_address, get_abi_from_etherscan},
    NodeClient,
};
use ethers::{
    contract::Contract,
    providers::Provider,
    types::{H160, U256},
    utils::hex,
};
use eyre::Result;
use std::{
    env,
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
    let client = Arc::new(Provider::try_from(INFURA_MAINNET).unwrap());

    let pool_addresses = curve_dex::get_pool_address(client.clone()).await?;

    // let pools_tokens =
    //     curve_dex::get_underlying_coins_in_pool_with_decimal(&pool_addresses, client.clone())
    //         .await?;
    //
    // println!("{:#?}", pools_tokens);

    let (tx, mut rx) = tokio::sync::mpsc::channel(512);

    // let first = pool_addresses.first().unwrap().address;
    // let cont = create_contract_instance_for_any_address(
    //     hex::encode_prefixed(first.as_bytes()).to_string(),
    //     client.clone(),
    // )
    // .await?;
    //
    // let balances: U256 = cont
    //     .method("balances", U256::from(0))
    //     .unwrap()
    //     .call()
    //     .await?;
    // println!("{:?}", balances);

    // let client = Arc::new(Provider::try_from(INFURA_MAINNET).unwrap());

    let tx1 = tx.clone();

    tokio::spawn(async move {
        for i in &pool_addresses[..] {
            let cont = create_contract_instance_for_any_address(
                hex::encode_prefixed(i.address.as_bytes()).to_string(),
                "curve",
                client.clone(),
            )
            .await
            .unwrap();

            let balances: U256 = cont
                .method("get_virtual_price", ())
                .unwrap()
                .call()
                .await
                .unwrap();

            tx1.send(balances).await.unwrap();
        }
    });

    drop(tx);

    while let Some(data) = rx.recv().await {
        println!("{:?}", data);
    }

    Ok(())
}
