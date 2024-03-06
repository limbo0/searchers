use alloyed::{
    curve_dex::{
        self, get_all_pools, get_tokens_of_pool, get_underlying_tokens_of_pool,
        pools_contains_itoken,
    },
    max_output_pool, NodeClient,
};
use ethers::{
    abi::token,
    middleware::providers::Provider,
    types::{H160, H256, U256},
};
use eyre::Result;
use std::{collections::HashMap, fmt::format, sync::Arc};

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

    let input_token = USDC.parse::<H160>().unwrap();
    let one_dai: U256 = U256::from(1000000000000000000u128);
    let one_usdc: U256 = U256::from(1000000u32);

    let all_curve_pools = get_all_pools(client.clone()).await.unwrap();
    let curve_pools_coins = get_tokens_of_pool(&all_curve_pools, client.clone())
        .await
        .unwrap();

    // let curve_pools_underlying_coins =
    //     get_underlying_tokens_of_pool(&all_curve_pools, client.clone())
    //         .await
    //         .unwrap();

    // assuming if the input token exist in pool, a swap is possible.
    let pools_containing_it =
        pools_contains_itoken(input_token, client.clone(), &curve_pools_coins)
            .await
            .unwrap();

    let max_output = max_output_pool(pools_containing_it, input_token, one_usdc, client.clone())
        .await
        .unwrap();

    println!("{:#?}", max_output);

    for x in max_output.iter() {
        println!("{:#?}", x);
        // let max_out = max_output_pool(
        //     pools_contains_itoken(*x.0, client.clone(), &curve_pools_coins)
        //         .await
        //         .unwrap(),
        //     *x.0,
        //     x.1.output_amount,
        //     client.clone(),
        // )
        // .await
        // .unwrap();
        //
        // println!("{:#?}", max_out);
    }

    Ok(())
}
