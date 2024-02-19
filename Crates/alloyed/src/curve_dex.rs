// main entry point for curve, address provider.
// 0x0000000022D53366457F9d5E68Ec105046FC4383

// Aggregate getter methods for querying large data sets about a single pool. Designed for off-chain use.
// 0xe64608E223433E8a03a1DaaeFD8Cb638C14B552C
// [{"outputs":[],"inputs":[{"type":"address","name":"_provider"}],"stateMutability":"nonpayable","type":"constructor"},{"name":"get_pool_coins","outputs":[{"type":"address[8]","name":"coins"},{"type":"address[8]","name":"underlying_coins"},{"type":"uint256[8]","name":"decimals"},{"type":"uint256[8]","name":"underlying_decimals"}],"inputs":[{"type":"address","name":"_pool"}],"stateMutability":"view","type":"function","gas":15876},{"name":"get_pool_info","outputs":[{"type":"uint256[8]","name":"balances"},{"type":"uint256[8]","name":"underlying_balances"},{"type":"uint256[8]","name":"decimals"},{"type":"uint256[8]","name":"underlying_decimals"},{"type":"uint256[8]","name":"rates"},{"type":"address","name":"lp_token"},{"type":"tuple","name":"params","components":[{"type":"uint256","name":"A"},{"type":"uint256","name":"future_A"},{"type":"uint256","name":"fee"},{"type":"uint256","name":"admin_fee"},{"type":"uint256","name":"future_fee"},{"type":"uint256","name":"future_admin_fee"},{"type":"address","name":"future_owner"},{"type":"uint256","name":"initial_A"},{"type":"uint256","name":"initial_A_time"},{"type":"uint256","name":"future_A_time"}]}],"inputs":[{"type":"address","name":"_pool"}],"stateMutability":"view","type":"function","gas":35142},{"name":"address_provider","outputs":[{"type":"address","name":""}],"inputs":[],"stateMutability":"view","type":"function","gas":1121}]

// metapool factory
// 0xB9fC157394Af804a3578134A6585C0dc9cc990d4
// [{"name":"BasePoolAdded","inputs":[{"name":"base_pool","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"PlainPoolDeployed","inputs":[{"name":"coins","type":"address[4]","indexed":false},{"name":"A","type":"uint256","indexed":false},{"name":"fee","type":"uint256","indexed":false},{"name":"deployer","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"MetaPoolDeployed","inputs":[{"name":"coin","type":"address","indexed":false},{"name":"base_pool","type":"address","indexed":false},{"name":"A","type":"uint256","indexed":false},{"name":"fee","type":"uint256","indexed":false},{"name":"deployer","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"name":"LiquidityGaugeDeployed","inputs":[{"name":"pool","type":"address","indexed":false},{"name":"gauge","type":"address","indexed":false}],"anonymous":false,"type":"event"},{"stateMutability":"nonpayable","type":"constructor","inputs":[{"name":"_fee_receiver","type":"address"}],"outputs":[]},{"stateMutability":"view","type":"function","name":"metapool_implementations","inputs":[{"name":"_base_pool","type":"address"}],"outputs":[{"name":"","type":"address[10]"}],"gas":21716},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"i","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"get_base_pool","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2663},{"stateMutability":"view","type":"function","name":"get_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":2699},{"stateMutability":"view","type":"function","name":"get_meta_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"},{"name":"","type":"uint256"}],"gas":5201},{"stateMutability":"view","type":"function","name":"get_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[4]"}],"gas":9164},{"stateMutability":"view","type":"function","name":"get_underlying_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":21345},{"stateMutability":"view","type":"function","name":"get_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":20185},{"stateMutability":"view","type":"function","name":"get_underlying_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":19730},{"stateMutability":"view","type":"function","name":"get_metapool_rates","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":5281},{"stateMutability":"view","type":"function","name":"get_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":20435},{"stateMutability":"view","type":"function","name":"get_underlying_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":39733},{"stateMutability":"view","type":"function","name":"get_A","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":3135},{"stateMutability":"view","type":"function","name":"get_fees","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"},{"name":"","type":"uint256"}],"gas":5821},{"stateMutability":"view","type":"function","name":"get_admin_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[4]"}],"gas":13535},{"stateMutability":"view","type":"function","name":"get_coin_indices","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"int128"},{"name":"","type":"int128"},{"name":"","type":"bool"}],"gas":33407},{"stateMutability":"view","type":"function","name":"get_gauge","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":3089},{"stateMutability":"view","type":"function","name":"get_implementation_address","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":3119},{"stateMutability":"view","type":"function","name":"is_meta","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":3152},{"stateMutability":"view","type":"function","name":"get_pool_asset_type","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":5450},{"stateMutability":"view","type":"function","name":"get_fee_receiver","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":5480},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_asset_type","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_plain_pool","inputs":[{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coins","type":"address[4]"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_asset_type","type":"uint256"},{"name":"_implementation_idx","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_metapool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coin","type":"address"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_metapool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_coin","type":"address"},{"name":"_A","type":"uint256"},{"name":"_fee","type":"uint256"},{"name":"_implementation_idx","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"nonpayable","type":"function","name":"deploy_gauge","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":93079},{"stateMutability":"nonpayable","type":"function","name":"add_base_pool","inputs":[{"name":"_base_pool","type":"address"},{"name":"_fee_receiver","type":"address"},{"name":"_asset_type","type":"uint256"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":1206132},{"stateMutability":"nonpayable","type":"function","name":"set_metapool_implementations","inputs":[{"name":"_base_pool","type":"address"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":382072},{"stateMutability":"nonpayable","type":"function","name":"set_plain_implementations","inputs":[{"name":"_n_coins","type":"uint256"},{"name":"_implementations","type":"address[10]"}],"outputs":[],"gas":379687},{"stateMutability":"nonpayable","type":"function","name":"set_gauge_implementation","inputs":[{"name":"_gauge_implementation","type":"address"}],"outputs":[],"gas":38355},{"stateMutability":"nonpayable","type":"function","name":"batch_set_pool_asset_type","inputs":[{"name":"_pools","type":"address[32]"},{"name":"_asset_types","type":"uint256[32]"}],"outputs":[],"gas":1139545},{"stateMutability":"nonpayable","type":"function","name":"commit_transfer_ownership","inputs":[{"name":"_addr","type":"address"}],"outputs":[],"gas":38415},{"stateMutability":"nonpayable","type":"function","name":"accept_transfer_ownership","inputs":[],"outputs":[],"gas":58366},{"stateMutability":"nonpayable","type":"function","name":"set_manager","inputs":[{"name":"_manager","type":"address"}],"outputs":[],"gas":40996},{"stateMutability":"nonpayable","type":"function","name":"set_fee_receiver","inputs":[{"name":"_base_pool","type":"address"},{"name":"_fee_receiver","type":"address"}],"outputs":[],"gas":38770},{"stateMutability":"nonpayable","type":"function","name":"convert_metapool_fees","inputs":[],"outputs":[{"name":"","type":"bool"}],"gas":12880},{"stateMutability":"nonpayable","type":"function","name":"add_existing_metapools","inputs":[{"name":"_pools","type":"address[10]"}],"outputs":[{"name":"","type":"bool"}],"gas":8610792},{"stateMutability":"view","type":"function","name":"admin","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3438},{"stateMutability":"view","type":"function","name":"future_admin","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3468},{"stateMutability":"view","type":"function","name":"manager","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3498},{"stateMutability":"view","type":"function","name":"pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3573},{"stateMutability":"view","type":"function","name":"pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":3558},{"stateMutability":"view","type":"function","name":"base_pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3633},{"stateMutability":"view","type":"function","name":"base_pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":3618},{"stateMutability":"view","type":"function","name":"base_pool_assets","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":3863},{"stateMutability":"view","type":"function","name":"plain_implementations","inputs":[{"name":"arg0","type":"uint256"},{"name":"arg1","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":3838},{"stateMutability":"view","type":"function","name":"gauge_implementation","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":3708}]

use crate::{
    etherscan::{self, create_contract_instance_for_any_address, get_abi_from_etherscan},
    NodeClient,
};

use ethers::{
    abi::Abi,
    contract::Contract,
    types::{H160, U256},
    utils::hex,
};
use eyre::Result;
use serde_json;
use std::collections::{hash_map::HashMap, VecDeque};

const CURVE_REGISTERY: &str = "0x90E00ACe148ca3b23Ac1bC8C240C2a7Dd9c2d7f5";
const CURVE_EXCHANG: &str = "0xD1602F68CC7C4c7B59D686243EA35a9C73B0c6a2";
const MAINNET_FORK: &str = "http://127.0.0.1:8545";
const INFURA_MAINNET: &str = "https://mainnet.infura.io/v3/af270f1023f34ef88fdcf6b85286734c";

/// Plain pool: a pool where two or more stablecoins are paired against one another.
/// Meta pool: a pool where a stablecoin is paired against the LP token from another pool.
// pub struct Curve {}

/// The main registry contract. Used to locate pools and query information about them.
pub async fn get_curve_registery_abi() -> Result<Abi> {
    Ok(serde_json::from_str(
        r#"[{"name":"PoolAdded","inputs":[{"name":"pool","type":"address","indexed":true},{"name":"rate_method_id","type":"bytes","indexed":false}],"anonymous":false,"type":"event"},{"name":"PoolRemoved","inputs":[{"name":"pool","type":"address","indexed":true}],"anonymous":false,"type":"event"},{"stateMutability":"nonpayable","type":"constructor","inputs":[{"name":"_address_provider","type":"address"},{"name":"_gauge_controller","type":"address"}],"outputs":[]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"find_pool_for_coins","inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"i","type":"uint256"}],"outputs":[{"name":"","type":"address"}]},{"stateMutability":"view","type":"function","name":"get_n_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1521},{"stateMutability":"view","type":"function","name":"get_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12102},{"stateMutability":"view","type":"function","name":"get_underlying_coins","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[8]"}],"gas":12194},{"stateMutability":"view","type":"function","name":"get_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7874},{"stateMutability":"view","type":"function","name":"get_underlying_decimals","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":7966},{"stateMutability":"view","type":"function","name":"get_rates","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36992},{"stateMutability":"view","type":"function","name":"get_gauges","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"address[10]"},{"name":"","type":"int128[10]"}],"gas":20157},{"stateMutability":"view","type":"function","name":"get_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":16583},{"stateMutability":"view","type":"function","name":"get_underlying_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":162842},{"stateMutability":"view","type":"function","name":"get_virtual_price_from_lp_token","inputs":[{"name":"_token","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1927},{"stateMutability":"view","type":"function","name":"get_A","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1045},{"stateMutability":"view","type":"function","name":"get_parameters","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"A","type":"uint256"},{"name":"future_A","type":"uint256"},{"name":"fee","type":"uint256"},{"name":"admin_fee","type":"uint256"},{"name":"future_fee","type":"uint256"},{"name":"future_admin_fee","type":"uint256"},{"name":"future_owner","type":"address"},{"name":"initial_A","type":"uint256"},{"name":"initial_A_time","type":"uint256"},{"name":"future_A_time","type":"uint256"}],"gas":6305},{"stateMutability":"view","type":"function","name":"get_fees","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[2]"}],"gas":1450},{"stateMutability":"view","type":"function","name":"get_admin_balances","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256[8]"}],"gas":36454},{"stateMutability":"view","type":"function","name":"get_coin_indices","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"int128"},{"name":"","type":"int128"},{"name":"","type":"bool"}],"gas":27131},{"stateMutability":"view","type":"function","name":"estimate_gas_used","inputs":[{"name":"_pool","type":"address"},{"name":"_from","type":"address"},{"name":"_to","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":32004},{"stateMutability":"view","type":"function","name":"is_meta","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"bool"}],"gas":1900},{"stateMutability":"view","type":"function","name":"get_pool_name","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"string"}],"gas":8323},{"stateMutability":"view","type":"function","name":"get_coin_swap_count","inputs":[{"name":"_coin","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":1951},{"stateMutability":"view","type":"function","name":"get_coin_swap_complement","inputs":[{"name":"_coin","type":"address"},{"name":"_index","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2090},{"stateMutability":"view","type":"function","name":"get_pool_asset_type","inputs":[{"name":"_pool","type":"address"}],"outputs":[{"name":"","type":"uint256"}],"gas":2011},{"stateMutability":"nonpayable","type":"function","name":"add_pool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_underlying_decimals","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":61485845},{"stateMutability":"nonpayable","type":"function","name":"add_pool_without_underlying","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_rate_info","type":"bytes32"},{"name":"_decimals","type":"uint256"},{"name":"_use_rates","type":"uint256"},{"name":"_has_initial_A","type":"bool"},{"name":"_is_v1","type":"bool"},{"name":"_name","type":"string"}],"outputs":[],"gas":31306062},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"add_metapool","inputs":[{"name":"_pool","type":"address"},{"name":"_n_coins","type":"uint256"},{"name":"_lp_token","type":"address"},{"name":"_decimals","type":"uint256"},{"name":"_name","type":"string"},{"name":"_base_pool","type":"address"}],"outputs":[]},{"stateMutability":"nonpayable","type":"function","name":"remove_pool","inputs":[{"name":"_pool","type":"address"}],"outputs":[],"gas":779731418758},{"stateMutability":"nonpayable","type":"function","name":"set_pool_gas_estimates","inputs":[{"name":"_addr","type":"address[5]"},{"name":"_amount","type":"uint256[2][5]"}],"outputs":[],"gas":390460},{"stateMutability":"nonpayable","type":"function","name":"set_coin_gas_estimates","inputs":[{"name":"_addr","type":"address[10]"},{"name":"_amount","type":"uint256[10]"}],"outputs":[],"gas":392047},{"stateMutability":"nonpayable","type":"function","name":"set_gas_estimate_contract","inputs":[{"name":"_pool","type":"address"},{"name":"_estimator","type":"address"}],"outputs":[],"gas":72629},{"stateMutability":"nonpayable","type":"function","name":"set_liquidity_gauges","inputs":[{"name":"_pool","type":"address"},{"name":"_liquidity_gauges","type":"address[10]"}],"outputs":[],"gas":400675},{"stateMutability":"nonpayable","type":"function","name":"set_pool_asset_type","inputs":[{"name":"_pool","type":"address"},{"name":"_asset_type","type":"uint256"}],"outputs":[],"gas":72667},{"stateMutability":"nonpayable","type":"function","name":"batch_set_pool_asset_type","inputs":[{"name":"_pools","type":"address[32]"},{"name":"_asset_types","type":"uint256[32]"}],"outputs":[],"gas":1173447},{"stateMutability":"view","type":"function","name":"address_provider","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2048},{"stateMutability":"view","type":"function","name":"gauge_controller","inputs":[],"outputs":[{"name":"","type":"address"}],"gas":2078},{"stateMutability":"view","type":"function","name":"pool_list","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2217},{"stateMutability":"view","type":"function","name":"pool_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2138},{"stateMutability":"view","type":"function","name":"coin_count","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2168},{"stateMutability":"view","type":"function","name":"get_coin","inputs":[{"name":"arg0","type":"uint256"}],"outputs":[{"name":"","type":"address"}],"gas":2307},{"stateMutability":"view","type":"function","name":"get_pool_from_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2443},{"stateMutability":"view","type":"function","name":"get_lp_token","inputs":[{"name":"arg0","type":"address"}],"outputs":[{"name":"","type":"address"}],"gas":2473},{"stateMutability":"view","type":"function","name":"last_updated","inputs":[],"outputs":[{"name":"","type":"uint256"}],"gas":2288}]"#,
    )?)
}

/// find pools, query exchange rates and perform swaps.
pub async fn get_curve_exchange_contract_abi() -> Result<Abi> {
    Ok(serde_json::from_str(
        r#"[{"name":"TokenExchange","inputs":[{"type":"address","name":"buyer","indexed":true},{"type":"address","name":"receiver","indexed":true},{"type":"address","name":"pool","indexed":true},{"type":"address","name":"token_sold","indexed":false},{"type":"address","name":"token_bought","indexed":false},{"type":"uint256","name":"amount_sold","indexed":false},{"type":"uint256","name":"amount_bought","indexed":false}],"anonymous":false,"type":"event"},{"outputs":[],"inputs":[{"type":"address","name":"_address_provider"},{"type":"address","name":"_calculator"}],"stateMutability":"nonpayable","type":"constructor"},{"stateMutability":"payable","type":"fallback"},{"name":"exchange_with_best_rate","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"},{"type":"uint256","name":"_expected"}],"stateMutability":"payable","type":"function"},{"name":"exchange_with_best_rate","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"},{"type":"uint256","name":"_expected"},{"type":"address","name":"_receiver"}],"stateMutability":"payable","type":"function"},{"name":"exchange","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"},{"type":"uint256","name":"_expected"}],"stateMutability":"payable","type":"function"},{"name":"exchange","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"},{"type":"uint256","name":"_expected"},{"type":"address","name":"_receiver"}],"stateMutability":"payable","type":"function"},{"name":"get_best_rate","outputs":[{"type":"address","name":""},{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"}],"stateMutability":"view","type":"function","gas":298910689},{"name":"get_exchange_amount","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"}],"stateMutability":"view","type":"function","gas":3911},{"name":"get_input_amount","outputs":[{"type":"uint256","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256","name":"_amount"}],"stateMutability":"view","type":"function","gas":15391},{"name":"get_exchange_amounts","outputs":[{"type":"uint256[100]","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_from"},{"type":"address","name":"_to"},{"type":"uint256[100]","name":"_amounts"}],"stateMutability":"view","type":"function","gas":17958},{"name":"get_calculator","outputs":[{"type":"address","name":""}],"inputs":[{"type":"address","name":"_pool"}],"stateMutability":"view","type":"function","gas":2462},{"name":"update_registry_address","outputs":[{"type":"bool","name":""}],"inputs":[],"stateMutability":"nonpayable","type":"function","gas":36751},{"name":"set_calculator","outputs":[{"type":"bool","name":""}],"inputs":[{"type":"address","name":"_pool"},{"type":"address","name":"_calculator"}],"stateMutability":"nonpayable","type":"function","gas":37192},{"name":"set_default_calculator","outputs":[{"type":"bool","name":""}],"inputs":[{"type":"address","name":"_calculator"}],"stateMutability":"nonpayable","type":"function","gas":37007},{"name":"claim_balance","outputs":[{"type":"bool","name":""}],"inputs":[{"type":"address","name":"_token"}],"stateMutability":"nonpayable","type":"function","gas":37831},{"name":"set_killed","outputs":[{"type":"bool","name":""}],"inputs":[{"type":"bool","name":"_is_killed"}],"stateMutability":"nonpayable","type":"function","gas":37067},{"name":"registry","outputs":[{"type":"address","name":""}],"inputs":[],"stateMutability":"view","type":"function","gas":1571},{"name":"default_calculator","outputs":[{"type":"address","name":""}],"inputs":[],"stateMutability":"view","type":"function","gas":1601},{"name":"is_killed","outputs":[{"type":"bool","name":""}],"inputs":[],"stateMutability":"view","type":"function","gas":1631}]"#,
    )?)
}
/// creates a contract instance for the pool, and calls the get_dy view function on pools contract.
pub async fn expected_output(
    pool: H160,
    token_in_index: i128,
    token_out_index: i128,
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
        .method("get_dy", (token_in_index, token_out_index, amount_in))
        .unwrap_or_else(|_| {
            contract
                .method::<(U256, U256, U256), U256>(
                    "get_dy",
                    (token_in_index.into(), token_out_index.into(), amount_in),
                )
                .unwrap()
        })
        .call()
        .await
        .expect("failed to calculate expected return!"))
}

/// checks how many pools exist, and calls the method with number of pools.
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

/// Use it as a helper, to get the decimals of the tokens.
pub async fn tokens_and_decimals(
    all_curve_pools: VecDeque<H160>,
    client: NodeClient,
) -> Result<HashMap<H160, U256>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );

    let mut token_and_decimal: HashMap<H160, U256> = HashMap::new();

    for pool in all_curve_pools {
        // returns every tokens in th current iterating pool.
        let mut coins: Vec<H160> = curve_contract.method("get_coins", pool)?.call().await?;
        let mut underlying_coins: Vec<H160> = curve_contract
            .method("get_underlying_coins", pool)?
            .call()
            .await?;

        coins.append(&mut underlying_coins);

        // returns decimals of every tokens in current iterating pool.
        let mut decimals: Vec<U256> = curve_contract.method("get_decimals", pool)?.call().await?;
        let mut underlying_decimals: Vec<U256> = curve_contract
            .method("get_underlying_decimals", pool)?
            .call()
            .await?;

        decimals.append(&mut underlying_decimals);

        for (token, decimal) in coins.into_iter().zip(decimals.into_iter()) {
            // the above method call often retuns an zero address.
            if token.is_zero() {
                continue;
            // don't wanna add a duplicate in the hash_map.
            } else {
                match token_and_decimal.contains_key(&token) {
                    true => continue,
                    false => {
                        token_and_decimal.insert(token, decimal);
                    }
                }
            }
        }
    }
    Ok(token_and_decimal)
}

/// Curve's contract finds pool, where we can execute swap based on out input and output tokens.
pub async fn find_pools_to_swap(
    token_in: H160,
    token_out: H160,
    client: NodeClient,
) -> Result<H160> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );
    // crate::helpers::function_selector("0xa064072b")
    let pool_address: H160 = curve_contract
        .method("find_pool_for_coins", (token_in, token_out))?
        .call()
        .await?;

    Ok(pool_address)
}

///Returns the address of the pool offering the best rate, and the expected amount received in the swap.
pub async fn find_best_pool_to_swap(
    token_in: H160,
    token_out: H160,
    amount_in: U256,
    client: NodeClient,
) -> Result<(H160, U256)> {
    let exchange_contract = Contract::new(
        CURVE_EXCHANG.parse::<H160>()?,
        get_curve_exchange_contract_abi().await?,
        client.clone(),
    );

    let pool_and_price: (H160, U256) = exchange_contract
        .method("get_best_rate", (token_in, token_out, amount_in))?
        .call()
        .await?;

    Ok(pool_and_price)
}

/// Returns all the tokens in the pools.
/// key: pool_address, value: vector of tokens.
pub async fn get_coins_of_pool(
    curve_pools: &VecDeque<H160>,
    client: NodeClient,
) -> Result<HashMap<H160, VecDeque<H160>>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );

    let mut tokens_in_pool: HashMap<H160, VecDeque<H160>> =
        HashMap::with_capacity(curve_pools.len());

    for pool in curve_pools {
        let tokens_addresses: Vec<H160> = curve_contract.method("get_coins", *pool)?.call().await?;

        // every length of vec depends on the number of tokens in the pool.
        let mut vec_md: VecDeque<H160> = VecDeque::with_capacity(tokens_addresses.len());

        for token in tokens_addresses {
            if token.is_zero() {
                continue;
            } else {
                vec_md.push_back(token);
            }
        }
        tokens_in_pool.insert(*pool, vec_md);
    }
    Ok(tokens_in_pool)
}

/// Returns all the underlying tokens in the pools.
/// key: pool_address, value: vector of tokens.
pub async fn get_underlying_coins_of_pool(
    all_curve_pools: &VecDeque<H160>,
    client: NodeClient,
) -> Result<HashMap<H160, VecDeque<H160>>> {
    let curve_contract = Contract::new(
        CURVE_REGISTERY.parse::<H160>()?,
        get_curve_registery_abi().await?,
        client.clone(),
    );

    let mut underlying_tokens_in_pool: HashMap<H160, VecDeque<H160>> =
        HashMap::with_capacity(all_curve_pools.len());

    // Get a list of the swappable coins within a pool.
    for pool in all_curve_pools {
        let tokens_addresses: Vec<H160> = curve_contract
            .method("get_underlying_coins", *pool)?
            .call()
            .await?;

        // every length of vec depends on the number of tokens in the pool.
        let mut vec_md: VecDeque<H160> = VecDeque::with_capacity(tokens_addresses.len());

        for token in tokens_addresses {
            if token.is_zero() {
                continue;
            } else {
                vec_md.push_back(token);
            }
        }
        underlying_tokens_in_pool.insert(*pool, vec_md);
    }
    Ok(underlying_tokens_in_pool)
}

/// curve's method call which return the best possible output.
pub async fn best_pool_to_swap_in_curve(token_in: H160, client: NodeClient) {
    let all_curve_pools = get_all_pools(client.clone()).await.unwrap();

    let curve_pools_with_tokens = get_coins_of_pool(&all_curve_pools, client.clone())
        .await
        .unwrap();

    // iterates all pools and returns references to the tokens in each pool.
    for (pool, tokens_list) in curve_pools_with_tokens.iter() {
        for tokens in tokens_list {
            if *tokens != token_in {
                let (pool, price) =
                    find_best_pool_to_swap(token_in, *tokens, U256::from(1000000), client.clone())
                        .await
                        .unwrap();
                println!(
                    "best_pool: {:?} for_token: {:?}\nbest_price: {:?}",
                    pool, tokens, price
                );
            } else {
                continue;
            }
        }
    }
}

/// contains tokens and its indexes.
#[derive(Debug)]
pub struct SwapMetadata {
    token_in_index: i128,
    tokens_and_indexes: HashMap<H160, i128>,
}

/// finds the index of the input token and fetches output amount,
/// in the context of every other token in the pool.
pub async fn index_tokens_in_pools(
    token_in: H160,
    client: NodeClient,
) -> Result<HashMap<H160, SwapMetadata>> {
    let all_curve_pools = get_all_pools(client.clone()).await.unwrap();

    // returns all tokens in each pool.
    let curve_pools_with_tokens = get_coins_of_pool(&all_curve_pools, client.clone())
        .await
        .unwrap();

    let mut swap_info: HashMap<H160, SwapMetadata> = HashMap::new();

    for (pool, tokens_list) in curve_pools_with_tokens.iter() {
        // first check: if the pool contains the input token.
        for tokens in tokens_list {
            // this block only executes if the input token is in the pool's token list.
            if tokens_list.contains(&token_in) {
                // println!("found: {:?} in_pool:{:?}\n", tokens, pool);

                // will create a contract instance for the pool, where the token was found.
                let contract = etherscan::create_contract_instance_for_any_address(
                    hex::encode_prefixed(pool.as_bytes()).to_string(),
                    "curve",
                    client.clone(),
                )
                .await
                .unwrap();

                // keys are tokens, values are its indexes on the pool.
                // this will hold the token and it's correct indexes of any pools.
                let mut tokens_and_indexes: HashMap<H160, i128> =
                    HashMap::with_capacity(curve_pools_with_tokens.get(pool).unwrap().len());

                // This part is rearranging the correct token's index of every pool.
                // every iteration's length depends on the number of tokens in the pool.
                for index in 0..curve_pools_with_tokens.get(pool).unwrap().len() {
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

                    // println!("inserting\ntoken: {:?} index:{:?}\n", token, index);
                    tokens_and_indexes.insert(token, index.try_into().unwrap());
                }

                // initializing with 0
                // since we already have checked that the input token exists on this pool.
                // we can guarentee that the input's token index will be updated.
                let mut token_in_index = 0i128;

                // finding the index of the input token, in the current pool.
                for (token, index) in tokens_and_indexes.iter() {
                    if *token == token_in {
                        token_in_index = *index;
                        // println!("index of token input: {:?} of pool: {:?}\n", index, pool);
                        break;
                    } else {
                        continue;
                    }
                }

                swap_info.insert(
                    *pool,
                    SwapMetadata {
                        token_in_index,
                        tokens_and_indexes,
                    },
                );
            } else {
                continue;
            }
        }
    }
    Ok(swap_info)
}

#[cfg(test)]
pub mod tests {
    // use super::*;
}
