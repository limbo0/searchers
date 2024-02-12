// pub mod aura;

// pub mod lido;
//
// pub mod rocketPool;
//
// pub mod stakeDao;
//
// pub mod convexFinance;
//
// pub mod yearnFinance;
//
pub mod curve_dex;

pub mod etherscan;

// pub mod balancerV2;
//
// pub mod beefy;
//
// pub mod stafi;
//
// pub mod anky;
//
// pub mod originEther;
//
// pub mod fraxEther;
//
// pub mod uniswap;

use ethers::{
    types::{H160, U256},
    utils::hex,
};
use std::collections::HashMap;
pub type NodeClient = std::sync::Arc<ethers::providers::Provider<ethers::providers::Http>>;
const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/// finds the index of the input token and fetches output amount,
/// in the context of every other token in the pool.
pub async fn fetch_price_on_all_dex(token_in: H160, client: NodeClient) {
    // check on curve_dex
    let all_curve_pools = curve_dex::get_all_pools(client.clone()).await.unwrap();

    let curve_pools_with_tokens = curve_dex::get_tokens_of_pool(all_curve_pools, client.clone())
        .await
        .unwrap();

    for (pool, tokens_list) in curve_pools_with_tokens.iter() {
        // first check: if the pool contains the input token.
        // check if token is in the list of tokens in every pool's token list.

        for tokens in tokens_list {
            // this block only executes if the input token is in the pool's token list.
            if *tokens == token_in {
                println!("found: {:?}\nin_pool:{:?}", tokens, pool);

                let contract = etherscan::create_contract_instance_for_any_address(
                    hex::encode_prefixed(pool.as_bytes()).to_string(),
                    "curve",
                    client.clone(),
                )
                .await
                .unwrap();

                // will manually update the indexes of the tokens in the pools.
                // keys are tokens, values are its indexes on the pool.
                let mut tokens_index: HashMap<H160, i128> =
                    HashMap::with_capacity(curve_pools_with_tokens.get(pool).unwrap().len());

                // figuring out the index of all tokens in the pool.
                for index in 0..curve_pools_with_tokens.get(pool).unwrap().len() {
                    // let index: i128 = index.try_into().unwrap();
                    println!("current index: {:?}", index);

                    // error handling since some of the params takes i128, while other take U256.
                    let token: H160 = contract
                        .method::<i128, H160>("coins", index.try_into().unwrap())
                        .unwrap_or_else(|_| {
                            contract
                                .method::<U256, H160>("coins", index.try_into().unwrap())
                                .unwrap()
                        })
                        .call()
                        .await
                        .unwrap();

                    println!("inserting\ntoken: {:?} index:{:?}\n", token, index);
                    tokens_index.insert(token, index.try_into().unwrap());
                }

                // initializing with 0
                // since we already have checked that the input token exists on this pool.
                // we can guarentee that the input's token index will be updated.
                let mut index_token_in = 0i128;

                // finding the index of the input token.
                for (token, index) in tokens_index.iter() {
                    if *token == token_in {
                        index_token_in = *index;
                        println!("index of token input: {:?} of pool: {:?}\n", index, pool);
                        break;
                    } else {
                        continue;
                    }
                }

                // calls the function with every token as output, except the input token.
                // while executing the swap, the parameters are indexs of the token in the pool.

                for (token, index) in tokens_index.iter() {
                    println!("Token: {:?} at index: {:?}", token, index);
                    if *index == index_token_in {
                        continue;
                    } else {
                        let amount_out = curve_dex::expected_output(
                            *pool,
                            index_token_in,
                            *index,
                            U256::from(1000000000000000000u128),
                            client.clone(),
                        )
                        .await
                        .unwrap();
                        println!("Swap_index: {:?} output_amount: {:?}\n", index, amount_out);
                    }
                }
                break;
            } else {
                continue;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Arc;

    // #[tokio::test]
    // async fn fetch() {
    //     let provider = ethers::middleware::providers::test_provider::MAINNET.provider();
    //     let client = Arc::new(provider);
    //     fetch_price_on_all_dex(0, 1, U256::from(1000000000000000000u128), client.clone()).await;
    // }
}
