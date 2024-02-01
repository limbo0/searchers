use tokio::*;

use alloyed::{
    create_contract_instance_for_any_address,
    curve_dex::{self, Curve},
    get_abi_from_etherscan,
};
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::Provider,
    types::{Address, U256},
};
use eyre::Result;
use std::{env, sync::Arc};

const INFURA_MAINNET: &str = "https://mainnet.infura.io/v3/af270f1023f34ef88fdcf6b85286734c";
const CURVE_FI_DUSD: &str = "0x8038C01A0390a8c547446a0b2c18fc9aEFEcc10c";

#[tokio::main]
async fn main() -> Result<()> {
    // rpc client, needed to interact with ethereum blockchain.
    let client = Arc::new(Provider::try_from(INFURA_MAINNET).unwrap());
    let cont = create_contract_instance_for_any_address(
        "0x8038C01A0390a8c547446a0b2c18fc9aEFEcc10c",
        client.clone(),
    )
    .await?;

    let v_price: U256 = cont.method("get_virtual_price", ())?.call().await?;
    println!("{:?}", v_price);

    let pool_addresses = curve_dex::Curve::get_pool_address(client.clone()).await?;

    let pools_tokens =
        curve_dex::Curve::get_underlying_coins_in_pool_with_decimal(pool_addresses, client.clone())
            .await?;

    // println!("{:#?}", pools_tokens);

    // println!("{:?}", pools_tokens.get(&CURVE_FI_DUSD.parse::<Address>()?));

    // let dusd = pools_tokens.get(&CURVE_FI_DUSD.parse::<Address>()?)?;
    // println!("{:?}", dusd.iter().next());

    Ok(())
}
