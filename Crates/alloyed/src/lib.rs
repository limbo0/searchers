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

use ethers::types::{H160, U256};
pub type NodeClient = std::sync::Arc<ethers::providers::Provider<ethers::providers::Http>>;
const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

pub async fn fetch_price_on_all_dex(token_in: H160, client: NodeClient) {
    // check on curve_dex
    let all_curve_pools = curve_dex::get_all_pools(client.clone()).await.unwrap();

    let curve_pools_with_tokens = curve_dex::get_tokens_of_pool(all_curve_pools, client.clone())
        .await
        .unwrap();

    // first check: if the pool contains the token_in token.
    for (pool, tokens_list) in curve_pools_with_tokens.iter() {
        // check if token is in the list of tokens in every pool's token list.
        for tokens in tokens_list {
            // this block only executes if the input token is in the pool's token list.
            if *tokens == token_in {
                println!("found: {:?}\nin_pool:{:?}", tokens, pool);

                // get the pool's list of tokens, if token is present in the pool's list.
                let pool_inc_token = curve_pools_with_tokens.get(pool).unwrap();
                let mut index_token_in: usize = 0usize;

                // figuring out index of the input token in the pool.
                for (index, token) in pool_inc_token.iter().enumerate() {
                    println!("{:?} {:?}", index, token);
                    if *token == token_in {
                        // we now know the index of the input token.
                        index_token_in = index;
                    } else {
                        continue;
                    }
                }

                // calls the function with every token as output, except the input token.
                // while executing the swap, the parameters are indexs of the token in the pool.
                for (index, _) in pool_inc_token.iter().enumerate() {
                    if index == index_token_in {
                        continue;
                    } else {
                        let amount_out = curve_dex::expected_output(
                            *pool,
                            index_token_in.try_into().unwrap(),
                            index.try_into().unwrap(),
                            U256::from(1000000000000000000u128),
                            client.clone(),
                        )
                        .await
                        .unwrap();
                        println!("{:?} {:?}", index, amount_out);
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
