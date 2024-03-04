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

use crate::curve_dex::expected_output;
use curve_dex::SwapMetadata;

pub type NodeClient = std::sync::Arc<ethers::providers::Provider<ethers::providers::Http>>;

#[derive(Debug)]
pub struct PoolPrice {
    pool: H160,
    output_amount: U256,
}

pub async fn max_output_pool(
    pools_containing_it: HashMap<H160, SwapMetadata>,
    input_token: H160,
    amount_in: U256,
    client: NodeClient,
) -> Result<HashMap<H160, PoolPrice>> {
    let mut max_output_buffer: HashMap<H160, PoolPrice> = HashMap::new();

    for (pool, smdata) in pools_containing_it {
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
                match max_pool_data.1.is_zero() {
                    true => {
                        println!("zero value returned for token output: {:?}\nmanually calculating on the pool: {:?}", token, pool);
                        let ex_out = expected_output(
                            pool,
                            smdata.token_in_index,
                            *index,
                            amount_in,
                            client.clone(),
                        )
                        .await
                        .unwrap();
                        println!("return price from pool: {:?}\n", ex_out);
                        continue;

                        //TODO: Seperating a function to update the return data.
                        // Comparing the expected outputs, returns the highest output only.
                    }
                    false => {
                        println!("initial check happening.");
                        // println!("{:?} {:#?}", *token, max_pool_data);

                        // if the output token is in the buffer
                        // check if the pool to swap for it matchs the current returned pool
                        // if false, compare output amount's and update if current is greater.
                        if max_output_buffer.contains_key(token)
                            && (max_output_buffer.get_key_value(token).unwrap().1.pool
                                != max_pool_data.0)
                        {
                            // update if current value is greater than the prev.
                            println!("token already exist but swap pool is different!\ncomparing the output amount's!");
                            if max_pool_data.1
                                > (max_output_buffer
                                    .get_key_value(token)
                                    .unwrap()
                                    .1
                                    .output_amount)
                            {
                                println!("Updating price and swap pool.\n");
                                if let Some(pp) = max_output_buffer.get_mut(token) {
                                    pp.pool = max_pool_data.0;
                                    pp.output_amount = max_pool_data.1;
                                }
                            } else {
                                println!(
                                    "didnt need to update.\nprev_price: {:?} prev_pool: {:?} curr_price: {:?} curr_pool: {:?}",
                                    max_output_buffer
                                        .get(token)
                                        .unwrap()
                                        .output_amount,
                                    max_output_buffer
                                        .get(token)
                                        .unwrap()
                                        .pool,
                                    max_pool_data.1,
                                    max_pool_data.0,
                                )
                            }
                        } else {
                            println!("initial insert happening!\n");
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
            } else {
                continue;
            }
        }
    }
    Ok(max_output_buffer)
}
