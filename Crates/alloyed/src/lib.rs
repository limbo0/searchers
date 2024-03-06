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

pub mod helpers;
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

use ethers::types::{H160, H256, U256};

use eyre::Result;

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use curve_dex::{expected_output, SwapMetadata};

pub type NodeClient = std::sync::Arc<ethers::providers::Provider<ethers::providers::Http>>;

#[derive(Debug)]
pub struct PoolPrice {
    pub pool: H160,
    pub output_amount: U256,
}

pub async fn max_output_pool(
    pools_containing_it: HashMap<H160, SwapMetadata>,
    input_token: H160,
    amount_in: U256,
    client: NodeClient,
) -> Result<HashMap<H160, PoolPrice>> {
    let mut max_output_buffer: HashMap<H160, PoolPrice> = HashMap::new();

    for (pool, smdata) in pools_containing_it {
        println!("currently looking at pool: {:?}", pool);
        for (token, index) in smdata.tokens_and_indexes.iter() {
            if *index != smdata.token_in_index {
                // get the max data.
                let max_pool_data = curve_dex::pool_which_returns_most_output(
                    input_token,
                    *token,
                    amount_in,
                    client.clone(),
                )
                .await
                .unwrap();

                // initial check
                // manual price calculation if zero value was returned above.
                match max_pool_data.1.is_zero() {
                    true => {
                        println!(
                            "Swap address: {:?} found for token: {:?}, calculating price manually",
                            max_pool_data.0, token
                        );
                        let ex_out = expected_output(
                            pool,
                            smdata.token_in_index,
                            *index,
                            amount_in,
                            client.clone(),
                        )
                        .await
                        .unwrap();
                        println!("Output_amount: {:?} from pool: {:?}\n", ex_out, pool);

                        match max_output_buffer.contains_key(token) {
                            true => {
                                if max_output_buffer
                                    .get_key_value(token)
                                    .unwrap()
                                    .1
                                    .output_amount
                                    < ex_out
                                {
                                    println!("Updating price and swap pool with new values.\n");
                                    if let Some(pp) = max_output_buffer.get_mut(token) {
                                        pp.pool = pool;
                                        pp.output_amount = ex_out;
                                    }
                                }
                            }

                            false => {
                                println!("Initial insert after manual output_amount check.");
                                max_output_buffer.insert(
                                    *token,
                                    PoolPrice {
                                        pool,
                                        output_amount: ex_out,
                                    },
                                );
                            }
                        };
                    }
                    false => {
                        println!("initial check happening.");

                        // if the output token is in the buffer
                        // compare the output amount and update if necessary.
                        match max_output_buffer.contains_key(token) {
                            true => {
                                if max_output_buffer
                                    .get_key_value(token)
                                    .unwrap()
                                    .1
                                    .output_amount
                                    < max_pool_data.1
                                {
                                    dbg!("Updating price and swap pool with new values.\n");
                                    if let Some(pp) = max_output_buffer.get_mut(token) {
                                        pp.pool = max_pool_data.0;
                                        pp.output_amount = max_pool_data.1;
                                    }
                                } else {
                                    println!("No update needed.");
                                }
                            }
                            false => {
                                println!("Initial insert.");
                                max_output_buffer.insert(
                                    *token,
                                    PoolPrice {
                                        pool: max_pool_data.0,
                                        output_amount: max_pool_data.1,
                                    },
                                );
                            }
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }
    Ok(max_output_buffer)
}
