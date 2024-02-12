// main entry point for curve, address provider.
// 0x0000000022D53366457F9d5E68Ec105046FC4383

// The main registry contract. Used to locate pools and query information about them.
// 0x90E00ACe148ca3b23Ac1bC8C240C2a7Dd9c2d7f5
// [{"name":"PoolAdded","inputs":[{"name":"pool","type":"address","indexed":true},{"name":"rate_method_id","type":"bytes","indexed":false}],"anonymous":false,"type":"event"},{"name":"PoolRemoved","inputs":[{"name":"pool","type":"address","indexed":true}],"anonymous":false,"type":"event"},{"stateMutability":"nonpayable","type":"constructor","inputs":[{"name":"_address_provider","type":"address"},{"name":"_gauge_controller","type":"address"}],"outputs":[]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"i","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"get_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1521},{"stateMutability":"view","type":"function","name":"get_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12102},{"stateMutability":"view","type":"function","name":"get_underlying_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12194},{"stateMutability":"view","type":"function","name":"get_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7874},{"stateMutability":"view","type":"function","name":"get_underlying_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7966},{"stateMutability":"view","type":"function","name":"get_rates","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36992},{"stateMutability":"view","type":"function","name":"get_gauges","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[10]"},{"name":"","type":"int128[10]"}],"gas":20157},{"stateMutability":"view","type":"function","name":"get_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":16583},{"stateMutability":"view","type":"function","name":"get_underlying_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":162842},{"stateMutability":"view","type":"function","name":"get_virtual_price_from_lp_token","inputs":[{"name":"_token","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1927},{"stateMutability":"view","type":"function","name":"get_A","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1045},{"stateMutability":"view","type":"function","name":"get_parameters","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"A","type":"uint256"},{"name":"future_A","type":"uint256"},{"name":"fee","type":"uint256"},{"name":"admin_fee","type":"uint256"},{"name":"future_fee","type":"uint256"},{"name":"future_admin_fee","type":"uint256"},{"name":"future_owner","type":"address"},{"name":"initial_A","type":"uint256"},{"name":"initial_A_time","type":"uint256"},{"name":"future_A_time","type":"uint256"}],"gas":6305},{"stateMutability":"view","type":"function","name":"get_fees","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1450},{"stateMutability":"view","type":"function","name":"get_admin_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36454},{"stateMutability":"view","type":"function","name":"get_coin_indices","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"int128"},{"name":"","type":"int128"},{"name":"","type":"bool"}],"gas":27131},{"stateMutability":"view","type":"function","name":"estimate_gas_used","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":32004},{"stateMutability":"view","type":"function","name":"is_meta","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":1900},{"stateMutability":"view","type":"function","name":"get_pool_name","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"string"}],"gas":8323},{"stateMutability":"view","type":"function","name":"get_coin_swap_count","inputs":[{"name":"_coin","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1951},{"stateMutability":"view","type":"function","name":"get_coin_swap_complement","inputs":[{"name":"_coin","type":"address"},{"name":"_index","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2090},{"stateMutability":"view","type":"function","name":"get_pool_asset_type","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":2011},{"stateMutability":"nonpayable","type":"function","name":"add_pool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_underlying_decimals","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":61485845},{"stateMutability":"nonpayable","type":"function","name":"add_pool_without_underlying","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_use_rates","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":31306062},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"},{"name":"_base_pool","type":"address"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"remove_pool","inputs":[{"name":"_pool","type":"address"}],"outputs":[],"gas":779731418758},{"stateMutability":"nonpayable","type":"function","name":"set_pool_gas_estimates","inputs":[{"name":"_addr","type":"address[5]"},{"name":"_amount","type":"uint256[2][5]"}],"outputs":[],"gas":390460},{"stateMutability":"nonpayable","type":"function","name":"set_coin_gas_estimates","inputs":[{"name":"_addr","type":"address[10]"},{"name":"_amount","type":"uint256[10]"}],"outputs":[],"gas":392047},{"stateMutability":"nonpayable","type":"function","name":"set_gas_estimate_contract","inputs":[{"name":"_pool","type":"address"},{"name":"_estimator","type":"address"}],"outputs":[],"gas":72629},{"stateMutability":"nonpayable","type":"function","name":"set_liquidity_gauges","inputs":[{"name":"_pool","type":"address"},{"name":"_liquidity_gauges","type":"address[10]"}],"outputs":[],"gas":400675},{"stateMutability":"nonpayable","type":"function","name":"set_pool_asset_type","inputs":[{"name":"_pool","type":"address"},{"name":"_asset_type","type":"uint256"}],"outputs":[],"gas":72667},{"stateMutability":"nonpayable","type":"function","name":"batch_set_pool_asset_type","inputs":[{"name":"_pools","type":"address[32]"},{"name":"_asset_types","type":"uint256[32]"}],"outputs":[],"gas":1173447},{"stateMutability":"view","type":"function","name":"address_provider","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2048},{"stateMutability":"view","type":"function","name":"gauge_controller","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2078},{"stateMutability":"view","type":"function","name":"pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2217},{"stateMutability":"view","type":"function","name":"pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2138},{"stateMutability":"view","type":"function","name":"coin_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2168},{"stateMutability":"view","type":"function","name":"get_coin","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2307},{"stateMutability":"view","type":"function","name":"get_pool_from_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2443},{"stateMutability":"view","type":"function","name":"get_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2473},{"stateMutability":"view","type":"function","name":"last_updated","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2288}]

// Aggregate getter methods for querying large data sets about a single pool. Designed for off-chain use.
// 0xe64608E223433E8a03a1DaaeFD8Cb638C14B552C
// [{"outputs":[],"inputs":[{"type":"address","name":"_provider"}],"stateMutability":"nonpayable","type":"constructor"},{"name":"get_pool_coins","outputs":[{"type":"address[8]","name":"coins"},{"type":"address[8]","name":"underlying_coins"},{"type":"uint256[8]","name":"decimals"},{"type":"uint256[8]","name":"underlying_decimals"}],"inputs":[{"type":"address","name":"_pool"}],"stateMutability":"view","type":"function","gas":15876},{"name":"get_pool_info","outputs":[{"type":"uint256[8]","name":"balances"},{"type":"uint256[8]","name":"underlying_balances"},{"type":"uint256[8]","name":"decimals"},{"type":"uint256[8]","name":"underlying_decimals"},{"type":"uint256[8]","name":"rates"},{"type":"address","name":"lp_token"},{"type":"tuple","name":"params","components":[{"type":"uint256","name":"A"},{"type":"uint256","name":"future_A"},{"type":"uint256","name":"fee"},{"type":"uint256","name":"admin_fee"},{"type":"uint256","name":"future_fee"},{"type":"uint256","name":"future_admin_fee"},{"type":"address","name":"future_owner"},{"type":"uint256","name":"initial_A"},{"type":"uint256","name":"initial_A_time"},{"type":"uint256","name":"future_A_time"}]}],"inputs":[{"type":"address","name":"_pool"}],"stateMutability":"view","type":"function","gas":35142},{"name":"address_provider","outputs":[{"type":"address","name":""}],"inputs":[],"stateMutability":"view","type":"function","gas":1121}]

// metapool factory
// 0xB9fC157394Af804a3578134A6585C0dc9cc990d4
// [{"name":"BasePoolAdded","inputs":[{"name":"base_pool","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"PlainPoolDeployed","inputs":[{"name":"coins","type":"address[4]","indexed":false},{"name":"A","type":"uint256","indexed":false},{"name":"fee","type":"uint256","indexed":false},{"name":"deployer","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"MetaPoolDeployed","inputs":[{"name":"coin","type":"address","indexed":false},{"name":"base_pool","type":"address","indexed":false},{"name":"A","type":"uint256","indexed":false},{"name":"fee","type":"uint256","indexed":false},{"name":"deployer","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"LiquidityGaugeDeployed","inputs":[{"name":"pool","type":"address","indexed":false},{"name":"gauge","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"stateMutability":"nonpayable","type":"constructor","inputs":[{"name":"_fee_receiver","type":"address"}],"outputs":[]},{"stateMutability":"view","type":"function","name":"metapool_implementations","inputs":[{"name":"_base_pool","type":"address"}],"outputs":[{"name":"","type":"address[10]"}],"gas":21716},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"i","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"get_base_pool","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2663},{"stateMutability":"view","type":"function","name":"get_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":2699},{"stateMutability":"view","type":"function","name":"get_meta_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"},{"name":"","type":"uint256"}],"gas":5201},{"stateMutability":"view","type":"function","name":"get_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[4]"}],"gas":9164},{"stateMutability":"view","type":"function","name":"get_underlying_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":21345},{"stateMutability":"view","type":"function","name":"get_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":20185},{"stateMutability":"view","type":"function","name":"get_underlying_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":19730},{"stateMutability":"view","type":"function","name":"get_metapool_rates","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":5281},{"stateMutability":"view","type":"function","name":"get_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":20435},{"stateMutability":"view","type":"function","name":"get_underlying_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":39733},{"stateMutability":"view","type":"function","name":"get_A","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":3135},{"stateMutability":"view","type":"function","name":"get_fees","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"},{"name":"","type":"uint256"}],"gas":5821},{"stateMutability":"view","type":"function","name":"get_admin_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":13535},{"stateMutability":"view","type":"function","name":"get_coin_indices","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"int128"},{"name":"","type":"int128"},{"name":"","type":"bool"}],"gas":33407},{"stateMutability":"view","type":"function","name":"get_gauge","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":3089},{"stateMutability":"view","type":"function","name":"get_implementation_address","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":3119},{"stateMutability":"view","type":"function","name":"is_meta","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":3152},{"stateMutability":"view","type":"function","name":"get_pool_asset_type","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":5450},{"stateMutability":"view","type":"function","name":"get_fee_receiver","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":5480},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_asset_type","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_asset_type","type":"uint256"},{"name":"_implementation_idx","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_metapool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coin","type":"address"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_metapool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coin","type":"address"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_implementation_idx","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_gauge","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":93079},{"stateMutability":"nonpayable","type":"function","name":"add_base_pool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_fee_receiver","type":"address"},{"name":"_asset_type","type":"uint256"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":1206132},{"stateMutability":"nonpayable","type":"function","name":"set_metapool_implementations","inputs":[{"name":"_base_pool","type":"address"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":382072},{"stateMutability":"nonpayable","type":"function","name":"set_plain_implementations","inputs":[{"name":"_n_coins","type":"uint256"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":379687},{"stateMutability":"nonpayable","type":"function","name":"set_gauge_implementation","inputs":[{"name":"_gauge_implementation","type":"address"}],"outputs":[],"gas":38355},{"stateMutability":"nonpayable","type":"function","name":"batch_set_pool_asset_type","inputs":[{"name":"_pools","type":"address[32]"},{"name":"_asset_types","type":"uint256[32]"}],"outputs":[],"gas":1139545},{"stateMutability":"nonpayable","type":"function","name":"commit_transfer_ownership","inputs":[{"name":"_addr","type":"address"}],"outputs":[],"gas":38415},{"stateMutability":"nonpayable","type":"function","name":"accept_transfer_ownership","inputs":[],"outputs":[],"gas":58366},{"stateMutability":"nonpayable","type":"function","name":"set_manager","inputs":[{"name":"_manager","type":"address"}],"outputs":[],"gas":40996},{"stateMutability":"nonpayable","type":"function","name":"set_fee_receiver","inputs":[{"name":"_base_pool","type":"address"},{"name":"_fee_receiver","type":"address"}],"outputs":[],"gas":38770},{"stateMutability":"nonpayable","type":"function","name":"convert_metapool_fees","inputs":[],"outputs":[{"name":"","type":"bool"}],"gas":12880},{"stateMutability":"nonpayable","type":"function","name":"add_existing_metapools","inputs":[{"name":"_pools","type":"address[10]"}],"outputs":[{"name":"","type":"bool"}],"gas":8610792},{"stateMutability":"view","type":"function","name":"admin","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3438},{"stateMutability":"view","type":"function","name":"future_admin","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3468},{"stateMutability":"view","type":"function","name":"manager","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3498},{"stateMutability":"view","type":"function","name":"pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3573},{"stateMutability":"view","type":"function","name":"pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":3558},{"stateMutability":"view","type":"function","name":"base_pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3633},{"stateMutability":"view","type":"function","name":"base_pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":3618},{"stateMutability":"view","type":"function","name":"base_pool_assets","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":3863},{"stateMutability":"view","type":"function","name":"plain_implementations","inputs":[{"name":"arg0","type":"uint256"},{"name":"arg1","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3838},{"stateMutability":"view","type":"function","name":"gauge_implementation","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3708}]

// dai/usdc/usdt pool address
// 0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7

use crate::{
    etherscan::{create_contract_instance_for_any_address, get_abi_from_etherscan},
    NodeClient,
};

use ethers::{
    abi::Abi,
    contract::Contract,
    middleware::providers::Provider,
    types::{H160, U256},
};
use eyre::Result;
use serde_json;
use std::{
    collections::{hash_map::HashMap, VecDeque},
    sync::Arc,
};

const CURVE_REGISTERY: &str = "0x90E00ACe148ca3b23Ac1bC8C240C2a7Dd9c2d7f5";
const MAINNET_FORK: &str = "http://127.0.0.1:8545";
const INFURA_MAINNET: &str = "https://mainnet.infura.io/v3/af270f1023f34ef88fdcf6b85286734c";

#[derive(Debug, PartialEq)]
pub struct TokensMetadata<'a> {
    name: Option<&'a str>,
    symblol: Option<&'a str>,
    pub address: H160,
    decimals: U256,
}

impl<'a> TokensMetadata<'a> {
    pub fn new() -> Self {
        TokensMetadata {
            name: None,
            symblol: None,
            address: H160::zero(),
            decimals: U256::zero(),
        }
    }

    pub fn set_address(&mut self, new_address: H160) {
        self.address = new_address;
    }

    // returns decimals of the token.
    pub fn get_decimals(&self) -> U256 {
        self.decimals
    }
}

/// Plain pool: a pool where two or more stablecoins are paired against one another.
/// Meta pool: a pool where a stablecoin is paired against the LP token from another pool.
// pub struct Curve {}

/// pool address and its index on the curve registery contract
/// the ordering of this list is not fixed. Index values of addresses may change as pools are added or removed.
#[derive(Debug)]
pub struct IndexedPools {
    pub index: U256,
    pub address: H160,
}
impl IndexedPools {
    pub async fn get_underlying_coins_in_pool(&self, client: NodeClient) -> Result<VecDeque<H160>> {
        let curve_contract = Contract::new(
            CURVE_REGISTERY.parse::<H160>()?,
            get_curve_registery_abi().await?,
            client.clone(),
        );

        let tokens_addresses: Vec<H160> = curve_contract
            .method("get_underlying_coins", self.address)?
            .call()
            .await?;

        let mut vec_meta = VecDeque::with_capacity(tokens_addresses.len());
        let mut token_counter = 0usize;

        for tokens in tokens_addresses {
            if tokens == crate::ZERO_ADDRESS.parse::<H160>()? {
                continue;
            } else {
                vec_meta.push_back(tokens);
                token_counter += 1usize;
            }
        }

        vec_meta.resize(token_counter, H160::zero());
        Ok(vec_meta)
    }

    fn get_address(&self) -> H160 {
        self.address
    }
}

/// creates a contract instance for the pool, and calls the get_dy view function.
pub async fn expected_output(
    pool: H160,
    token_in: i128,
    token_out: i128,
    amount_in: U256,
    client: NodeClient,
) -> Result<U256> {
    let contract = create_contract_instance_for_any_address(
        ethers::utils::hex::encode_prefixed(pool.as_bytes()).to_string(),
        "curve",
        client,
    )
    .await
    .expect("failed to create contract!");

    Ok(contract
        .method("get_dy", (token_in, token_out, amount_in))
        .unwrap_or_else(|_| {
            contract
                .method::<(U256, U256, U256), U256>(
                    "get_dy",
                    (token_in.into(), token_out.into(), amount_in),
                )
                .unwrap()
        })
        .call()
        .await
        .expect("failed to calculate expected return!"))
}

pub async fn get_curve_registery_abi() -> Result<Abi> {
    Ok(serde_json::from_str(
        r#"[{"name":"PoolAdded","inputs":[{"name":"pool","type":"address","indexed":true},{"name":"rate_method_id","type":"bytes","indexed":false}],"anonymous":false,"type":"event"},{"name":"PoolRemoved","inputs":[{"name":"pool","type":"address","indexed":true}],"anonymous":false,"type":"event"},{"stateMutability":"nonpayable","type":"constructor","inputs":[{"name":"_address_provider","type":"address"},{"name":"_gauge_controller","type":"address"}],"outputs":[]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"i","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"get_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1521},{"stateMutability":"view","type":"function","name":"get_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12102},{"stateMutability":"view","type":"function","name":"get_underlying_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12194},{"stateMutability":"view","type":"function","name":"get_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7874},{"stateMutability":"view","type":"function","name":"get_underlying_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7966},{"stateMutability":"view","type":"function","name":"get_rates","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36992},{"stateMutability":"view","type":"function","name":"get_gauges","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[10]"},{"name":"","type":"int128[10]"}],"gas":20157},{"stateMutability":"view","type":"function","name":"get_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":16583},{"stateMutability":"view","type":"function","name":"get_underlying_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":162842},{"stateMutability":"view","type":"function","name":"get_virtual_price_from_lp_token","inputs":[{"name":"_token","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1927},{"stateMutability":"view","type":"function","name":"get_A","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1045},{"stateMutability":"view","type":"function","name":"get_parameters","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"A","type":"uint256"},{"name":"future_A","type":"uint256"},{"name":"fee","type":"uint256"},{"name":"admin_fee","type":"uint256"},{"name":"future_fee","type":"uint256"},{"name":"future_admin_fee","type":"uint256"},{"name":"future_owner","type":"address"},{"name":"initial_A","type":"uint256"},{"name":"initial_A_time","type":"uint256"},{"name":"future_A_time","type":"uint256"}],"gas":6305},{"stateMutability":"view","type":"function","name":"get_fees","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1450},{"stateMutability":"view","type":"function","name":"get_admin_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36454},{"stateMutability":"view","type":"function","name":"get_coin_indices","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"int128"},{"name":"","type":"int128"},{"name":"","type":"bool"}],"gas":27131},{"stateMutability":"view","type":"function","name":"estimate_gas_used","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":32004},{"stateMutability":"view","type":"function","name":"is_meta","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":1900},{"stateMutability":"view","type":"function","name":"get_pool_name","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"string"}],"gas":8323},{"stateMutability":"view","type":"function","name":"get_coin_swap_count","inputs":[{"name":"_coin","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1951},{"stateMutability":"view","type":"function","name":"get_coin_swap_complement","inputs":[{"name":"_coin","type":"address"},{"name":"_index","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2090},{"stateMutability":"view","type":"function","name":"get_pool_asset_type","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":2011},{"stateMutability":"nonpayable","type":"function","name":"add_pool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_underlying_decimals","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":61485845},{"stateMutability":"nonpayable","type":"function","name":"add_pool_without_underlying","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_use_rates","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":31306062},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"},{"name":"_base_pool","type":"address"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"remove_pool","inputs":[{"name":"_pool","type":"address"}],"outputs":[],"gas":779731418758},{"stateMutability":"nonpayable","type":"function","name":"set_pool_gas_estimates","inputs":[{"name":"_addr","type":"address[5]"},{"name":"_amount","type":"uint256[2][5]"}],"outputs":[],"gas":390460},{"stateMutability":"nonpayable","type":"function","name":"set_coin_gas_estimates","inputs":[{"name":"_addr","type":"address[10]"},{"name":"_amount","type":"uint256[10]"}],"outputs":[],"gas":392047},{"stateMutability":"nonpayable","type":"function","name":"set_gas_estimate_contract","inputs":[{"name":"_pool","type":"address"},{"name":"_estimator","type":"address"}],"outputs":[],"gas":72629},{"stateMutability":"nonpayable","type":"function","name":"set_liquidity_gauges","inputs":[{"name":"_pool","type":"address"},{"name":"_liquidity_gauges","type":"address[10]"}],"outputs":[],"gas":400675},{"stateMutability":"nonpayable","type":"function","name":"set_pool_asset_type","inputs":[{"name":"_pool","type":"address"},{"name":"_asset_type","type":"uint256"}],"outputs":[],"gas":72667},{"stateMutability":"nonpayable","type":"function","name":"batch_set_pool_asset_type","inputs":[{"name":"_pools","type":"address[32]"},{"name":"_asset_types","type":"uint256[32]"}],"outputs":[],"gas":1173447},{"stateMutability":"view","type":"function","name":"address_provider","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2048},{"stateMutability":"view","type":"function","name":"gauge_controller","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2078},{"stateMutability":"view","type":"function","name":"pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2217},{"stateMutability":"view","type":"function","name":"pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2138},{"stateMutability":"view","type":"function","name":"coin_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2168},{"stateMutability":"view","type":"function","name":"get_coin","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2307},{"stateMutability":"view","type":"function","name":"get_pool_from_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2443},{"stateMutability":"view","type":"function","name":"get_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2473},{"stateMutability":"view","type":"function","name":"last_updated","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2288}]"#,
    )?)
}

pub async fn get_all_pools(client: NodeClient) -> Result<VecDeque<H160>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>().unwrap(),
        get_curve_registery_abi().await?,
        client.clone(),
    );

    // total number of pools in curve registery contract.
    let pool_count: U256 = curve_contract.method("pool_count", ())?.call().await?;
    eprintln!("{:?} pools in curve registery contract.", pool_count);

    let mut pool_index: U256 = U256::from(0);

    // creates a vector of IndexedPools
    // IndexedPools consists of pool address and its index on the registery contract.
    let mut curve_registery_pool_addresses = VecDeque::with_capacity(pool_count.as_usize());

    loop {
        if pool_index == pool_count {
            break;
        } else {
            let pool_address: H160 = curve_contract
                .method("pool_list", pool_index)?
                .call()
                .await?;

            curve_registery_pool_addresses.push_back(pool_address);

            pool_index += U256::from(1);
        }
    }

    Ok(curve_registery_pool_addresses)
}

/// Returns all the pool address in the curve registery contract with it's corresponding index.
pub async fn get_pool_address_with_index(client: NodeClient) -> Result<Vec<IndexedPools>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>().unwrap(),
        get_curve_registery_abi().await?,
        client.clone(),
    );

    // total number of pools in curve registery contract.
    let pool_count: U256 = curve_contract.method("pool_count", ())?.call().await?;
    eprintln!("{:?} pools in curve registery contract.", pool_count);

    let mut pool_index: U256 = U256::from(0);

    // creates a vector of IndexedPools
    // IndexedPools consists of pool address and its index on the registery contract.
    let mut curve_registery_pool_addresses: Vec<IndexedPools> = Vec::new();

    loop {
        if pool_index == pool_count {
            break;
        } else {
            let pool_address: H160 = curve_contract
                .method("pool_list", pool_index)?
                .call()
                .await?;

            curve_registery_pool_addresses.push(IndexedPools {
                index: pool_index,
                address: pool_address,
            });

            pool_index += U256::from(1);
        }
    }

    Ok(curve_registery_pool_addresses)
}

/// returns tokens and decimals for each token involved in the pool.
pub async fn get_coins_in_pool_with_decimal<'a>(
    all_curve_pools: VecDeque<H160>,
    client: NodeClient,
) -> Result<HashMap<H160, VecDeque<TokensMetadata<'a>>>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );

    // creates a hash_map with only needed capacity.
    // list of token address and its decimals, of its corresponding pool.
    let mut tokens_in_pool: HashMap<H160, VecDeque<TokensMetadata>> =
        HashMap::with_capacity(all_curve_pools.len());

    // Get a list of the swappable coins within a pool.
    for pool in all_curve_pools {
        let tokens_addresses: Vec<H160> = curve_contract.method("get_coins", pool)?.call().await?;

        let tokens_decimals: Vec<U256> =
            curve_contract.method("get_decimals", pool)?.call().await?;

        // convert to VecDeque and sort.
        let mut tokens_addresses = VecDeque::from(tokens_addresses);
        tokens_addresses.make_contiguous().sort_unstable();

        // convert to VecDeque and sort.
        let mut tokens_decimals = VecDeque::from(tokens_decimals);
        tokens_decimals.make_contiguous().sort_unstable();

        // every length of vec depends on the number of tokens in the pool.
        let mut vec_md: VecDeque<TokensMetadata> = VecDeque::with_capacity(tokens_addresses.len());

        for (token, decimal) in tokens_addresses
            .into_iter()
            .zip(tokens_decimals.into_iter())
        {
            if token.is_zero() {
                continue;
            } else {
                vec_md.push_back(TokensMetadata {
                    name: None,
                    symblol: None,
                    address: token,
                    decimals: decimal,
                });
            }
        }
        tokens_in_pool.insert(pool, vec_md);
    }
    Ok(tokens_in_pool)
}

/// Returns all the tokens in the pools.
pub async fn get_tokens_of_pool(
    all_curve_pools: VecDeque<H160>,
    client: NodeClient,
) -> Result<HashMap<H160, VecDeque<H160>>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );

    // creates a hash_map with only needed capacity.
    // list of token address and its decimals, of its corresponding pool.
    let mut tokens_in_pool: HashMap<H160, VecDeque<H160>> =
        HashMap::with_capacity(all_curve_pools.len());

    // Get a list of the swappable coins within a pool.
    for pool in all_curve_pools {
        let tokens_addresses: Vec<H160> = curve_contract.method("get_coins", pool)?.call().await?;

        // convert to VecDeque and sort.
        let mut tokens_addresses = VecDeque::from(tokens_addresses);
        tokens_addresses.make_contiguous().sort_unstable();

        // every length of vec depends on the number of tokens in the pool.
        let mut vec_md: VecDeque<H160> = VecDeque::with_capacity(tokens_addresses.len());

        for token in tokens_addresses {
            if token.is_zero() {
                continue;
            } else {
                vec_md.push_back(token);
            }
        }
        tokens_in_pool.insert(pool, vec_md);
    }
    Ok(tokens_in_pool)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_expected_output() {
    //     let provider = Provider::try_from(INFURA_MAINNET).unwrap();
    //     let client = Arc::new(provider.clone());
    //
    //     let pool_addresses = get_pool_address_with_index(client.clone()).await.unwrap();
    //
    //     let expected = pool_addresses
    //         .first()
    //         .expect("empty vector")
    //         .expected_output(
    //             0.into(),
    //             1.into(),
    //             U256::from(1000000000000000000000u128),
    //             client.clone(),
    //         )
    //         .await
    //         .unwrap();
    //
    //     assert_ne!(expected, U256::from(0));
    // }

    // #[test]
    // fn test_expected_output() {
    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async {
    //             let provider = Provider::try_from(INFURA_MAINNET).unwrap();
    //             let client = Arc::new(provider.clone());
    //
    //             let pool_addresses = get_pool_address(client.clone()).await.unwrap();
    //
    //             let expected = pool_addresses
    //                 .first()
    //                 .expect("empty vector")
    //                 .expected_output(
    //                     0.into(),
    //                     1.into(),
    //                     U256::from(1000000000000000000000u128),
    //                     client.clone(),
    //                 )
    //                 .await;
    //
    //             assert_ne!(expected, U256::from(0));
    //         });
    // }
}
