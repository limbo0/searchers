// Import from our own library/Crate.
use log::info;
use uniswap::{
    custom_standard_encode, function_selector, stream_new_blocks, tokio_thread, Event, SwapParam,
};

use ethers::{
    abi::{encode, token, Abi, AbiEncode, Function, ParamType, Token, Tokenize},
    contract::{encode_function_data, Contract},
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest, H160, U256},
    utils::{format_units, hex},
};
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinSet;

use eyre::Result;
use serde_json;
use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::Write,
    ops::Add,
    sync::Arc,
};

// Mainnet and forked mainnet helpers.
abigen!(UNI_V2_FACTORY, "./uniswapABI/UniswapV2FactoryABI.json");
abigen!(UNI_V3_FACTORY, "./uniswapABI/UniswapV3Factory.json");
abigen!(UNI_V2_PAIR, "./uniswapABI/V2PairABI.json");
abigen!(UNI_V3_PAIR, "./uniswapABI/V3PairABI.json");
abigen!(ERC20, "./abi/erc20.json");
const RPC_URL_MAINNET: &str = "wss://eth-mainnet.g.alchemy.com/v2/AM3AXcpsHM2QfhCpSVDadRAwEe70bHEy";
const WETH: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
const DAI: &str = "0x6b175474e89094c44da98b954eedeac495271d0f";
const USDC: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const UNI_V2_FACTORY_ADDRESS: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
const UNI_V3_FACTORY_ADDRESS: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";

// local testing helpers
const SEP0LIA_RPC: &str = "https://eth-sepolia.g.alchemy.com/v2/X6jHEi-MPp4uvNPTgwUDEMIDpVAkPASZ";
const SEPOLIA_TEST_CONTRACT_ADDRESS: &str = "0x956719531Fd93726546eFf525983e6799F03e17B";
const GOERLI_RPC: &str = "https://eth-goerli.g.alchemy.com/v2/kNAKUC0Dijrm1XLnPo9yu08WU_lGvZ2w";
const GOERLI_TEST_CONTRACT_ADDRESS: &str = "0x6452C95c2aC5a26640ACf91ce8029D0bF825B32B";
const GOERLI_WETH: &str = "0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6";
const SOME_GOERLI_TOKEN: &str = "0xCc7bb2D219A0FC08033E130629C2B854b7bA9195";
const FORK_TEST_CONTRACT_ADDRESS: &str = "0xA3e3C0145B945C9B72Ed11C4d0f6fc08C5C3cdAE";
const MAINNET_FORK: &str = "http://127.0.0.1:8545";
const ALICE: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const DEV_WALLET: &str = "0x0039E6b1430f74844fBc0052c2288Dd5bDd19918";

// function selectors
// 1fz0x22ef6460, zf10x7e84fede, v20x813a6090, fl0x5107d61e, wet0xa0ef91df
// 0x22ef6460, 0x7e84fede, 0x813a6090, 0x5107d61e, 0xa0ef91df

const FLASH_LOAN_SELECTOR_STR: &str = "0x5107d61e";
const ONE_FOR_ZERO_SELECTOR_STR: &str = "0x22ef6460";
const ZERO_FOR_ONE_SELECTOR_STR: &str = "0x7e84fede";
const V2_SWAP_SELECTOR_STR: &str = "0x813a6090";
const WITHDRAW_ETH: &str = "0xa0ef91df";

type NodeClient = Arc<Provider<Http>>;

#[allow(unused_variables, dead_code)]
#[tokio::main]

async fn main() -> Result<()> {
    // tokio_thread::salt();
    // let provider = Provider::<Ws>::connect(RPC_URL_MAINNET).await?;
    let provider =
        Provider::try_from("wss://eth-mainnet.g.alchemy.com/v2/AM3AXcpsHM2QfhCpSVDadRAwEe70bHEy")?;
    let client = Arc::new(provider);
    let mainnet_chain_id: u64 = 1;
    eprintln!("connection established sucessfully -> {:?}\n", client);

    let wallet: LocalWallet = env::var("PK")
        .unwrap()
        .parse::<LocalWallet>()?
        .with_chain_id(mainnet_chain_id);

    // address helpers for mainnet.
    let weth = WETH.parse::<Address>()?;
    let dai = DAI.parse::<Address>()?;
    let usdc = USDC.parse::<Address>()?;
    let dev_wallet = DEV_WALLET.parse::<Address>()?;

    // address helpers for testnet.
    let alice = ALICE.parse::<Address>()?;
    let goe_weth = GOERLI_WETH.parse::<Address>().unwrap();
    let some_goe_token = SOME_GOERLI_TOKEN.parse::<Address>().unwrap();

    let test_contract_abi: Abi = serde_json::from_str(
        r#"[{"inputs":[{"internalType":"address","name":"_balancerVault","type":"address"}],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[{"internalType":"uint256","name":"amount0In","type":"uint256"},{"internalType":"uint256","name":"amount1out","type":"uint256"}],"name":"BOTH_IN_AND_OUT_CANNT_BE_ZERO","type":"error"},{"inputs":[{"internalType":"int256","name":"delta","type":"int256"},{"internalType":"int256","name":"minOut","type":"int256"}],"name":"DELTA_LESS_THAN_MIN_OUTPUT","type":"error"},{"inputs":[],"name":"ZERO_AMOUNT_INPUT","type":"error"},{"inputs":[],"name":"ZERO_MINIMUN_OUTPUT","type":"error"},{"inputs":[],"name":"ZERO_POOL_ADDRESS","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"int256","name":"amount0Delta","type":"int256"},{"indexed":true,"internalType":"int256","name":"amount1Delta","type":"int256"}],"name":"SwapCallback","type":"event"},{"stateMutability":"payable","type":"fallback"},{"inputs":[{"internalType":"bytes","name":"sp","type":"bytes"}],"name":"oneForZero","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"contract IERC20[]","name":"tokens","type":"address[]"},{"internalType":"uint256[]","name":"amounts","type":"uint256[]"},{"internalType":"uint256[]","name":"feeAmounts","type":"uint256[]"},{"internalType":"bytes","name":"userData","type":"bytes"}],"name":"receiveFlashLoan","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"token","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"bytes","name":"userData","type":"bytes"}],"name":"requestFlashLoan","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"amount0Delta","type":"int256"},{"internalType":"int256","name":"amount1Delta","type":"int256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"uniswapV3SwapCallback","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"sp","type":"bytes"}],"name":"v2Swap","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"withdrawEth","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"token","type":"address"}],"name":"withdrawToken","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"sp","type":"bytes"}],"name":"zeroForOne","outputs":[],"stateMutability":"nonpayable","type":"function"},{"stateMutability":"payable","type":"receive"}]"#,
    )?;

    let sepolia_test_contract = Contract::new(
        SEPOLIA_TEST_CONTRACT_ADDRESS.parse::<Address>()?,
        test_contract_abi.clone(),
        client.clone(),
    );

    let goerli_test_contract = Contract::new(
        GOERLI_TEST_CONTRACT_ADDRESS.parse::<Address>()?,
        test_contract_abi.clone(),
        client.clone(),
    );

    let fork_test_contract = Contract::new(
        FORK_TEST_CONTRACT_ADDRESS.parse::<Address>()?,
        test_contract_abi,
        client.clone(),
    );

    // buy_zero_sell_zero(
    //     weth,
    //     dai,
    //     500,
    //     &client,
    //     wallet.clone(),
    //     alice,
    //     sepolia_test_contract.clone(),
    // )
    // .await?;

    buy_one_sell_one(
        goe_weth,
        some_goe_token,
        500,
        &client,
        &wallet,
        goerli_test_contract.address(),
        goerli_test_contract,
    )
    .await?;

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct HighLowOutput<'a> {
    h_zero_th_dex: Option<&'a str>,
    h_zero_th_pair: Option<Address>,
    h_zero_th_token: Option<Address>,
    h_zero_th_price_raw: Option<U256>,
    h_zero_th_price: Option<f64>,
    h_zero_th_dec: Option<u8>,

    l_zero_th_dex: Option<&'a str>,
    l_zero_th_pair: Option<Address>,
    l_zero_th_token: Option<Address>,
    l_zero_th_price_raw: Option<U256>,
    l_zero_th_price: Option<f64>,
    l_zero_th_dec: Option<u8>,

    h_one_th_dex: Option<&'a str>,
    h_one_th_pair: Option<Address>,
    h_one_th_token: Option<Address>,
    h_one_th_price_raw: Option<U256>,
    h_one_th_price: Option<f64>,
    h_one_th_dec: Option<u8>,

    l_one_th_dex: Option<&'a str>,
    l_one_th_pair: Option<Address>,
    l_one_th_token: Option<Address>,
    l_one_th_price_raw: Option<U256>,
    l_one_th_price: Option<f64>,
    l_one_th_dec: Option<u8>,

    sqrt_x96_v3: Option<U256>,
}

// impl HighLowOutput {
//     pub fn get_high_zeroth_dex(&self) -> str {
//         &self.h_zero_th_dex
//     }
//
//     pub fn get_high_oneth_dex(&self) -> str {
//         &self.h_one_th_dex
//     }
// }

// we return none while there are no amount disrepency, meaning no arb oppertunity.
fn return_low_high<'a>(output_v2: TokenAndPrice, output_v3: TokenAndPrice) -> HighLowOutput<'a> {
    let v2_pair_address = output_v2.pair_address;
    let v2_t_0 = output_v2.token_0;
    let v2_t_1 = output_v2.token_1;
    let v2_p_0_raw = output_v2.price_0_raw;
    let v2_p_0 = output_v2.price_0;
    let v2_p_1_raw = output_v2.price_1_raw;
    let v2_p_1 = output_v2.price_1;
    let v2_d_0 = output_v2.decimal_0;
    let v2_d_1 = output_v2.decimal_1;

    let v3_pair_address = output_v3.pair_address;
    let v3_t_0 = output_v3.token_0;
    let v3_t_1 = output_v3.token_1;
    let v3_p_0_raw = output_v3.price_0_raw;
    let v3_p_0 = output_v3.price_0;
    let v3_p_1_raw = output_v3.price_1_raw;
    let v3_p_1 = output_v3.price_1;
    let v3_d_0 = output_v3.decimal_0;
    let v3_d_1 = output_v3.decimal_1;

    let pair_token_price: HighLowOutput = if v2_p_0 > v3_p_0 && v2_p_1 < v3_p_1 {
        HighLowOutput {
            h_zero_th_dex: Some("uni_v2"),
            h_zero_th_pair: Some(v2_pair_address),
            h_zero_th_token: Some(v2_t_0),
            h_zero_th_price_raw: Some(v2_p_0_raw),
            h_zero_th_price: Some(v2_p_0),
            h_zero_th_dec: Some(v2_d_0),

            h_one_th_dex: Some("uni_v3"),
            h_one_th_pair: Some(v3_pair_address),
            h_one_th_token: Some(v3_t_1),
            h_one_th_price_raw: Some(v3_p_1_raw),
            h_one_th_price: Some(v3_p_1),
            h_one_th_dec: Some(v3_d_1),
            sqrt_x96_v3: Some(output_v3.sqrt_x96.unwrap()),

            l_zero_th_dex: Some("uni_v3"),
            l_zero_th_pair: Some(v3_pair_address),
            l_zero_th_token: Some(v3_t_0),
            l_zero_th_price_raw: Some(v3_p_0_raw),
            l_zero_th_price: Some(v3_p_0),
            l_zero_th_dec: Some(v3_d_0),

            l_one_th_dex: Some("uni_v2"),
            l_one_th_pair: Some(v2_pair_address),
            l_one_th_token: Some(v2_t_1),
            l_one_th_price_raw: Some(v2_p_1_raw),
            l_one_th_price: Some(v2_p_1),
            l_one_th_dec: Some(v2_d_1),
        }
    } else if v3_p_0 > v2_p_0 && v3_p_1 < v2_p_1 {
        HighLowOutput {
            h_zero_th_dex: Some("uni_v3"),
            h_zero_th_pair: Some(v3_pair_address),
            h_zero_th_token: Some(v3_t_0),
            h_zero_th_price_raw: Some(v3_p_0_raw),
            h_zero_th_price: Some(v3_p_0),
            h_zero_th_dec: Some(v3_d_0),
            sqrt_x96_v3: Some(output_v3.sqrt_x96.unwrap()),

            h_one_th_dex: Some("uni_v2"),
            h_one_th_pair: Some(v2_pair_address),
            h_one_th_token: Some(v2_t_1),
            h_one_th_price_raw: Some(v2_p_1_raw),
            h_one_th_price: Some(v2_p_1),
            h_one_th_dec: Some(v2_d_1),

            l_zero_th_dex: Some("uni_v2"),
            l_zero_th_pair: Some(v2_pair_address),
            l_zero_th_token: Some(v2_t_0),
            l_zero_th_price_raw: Some(v2_p_0_raw),
            l_zero_th_price: Some(v2_p_0),
            l_zero_th_dec: Some(v2_d_0),

            l_one_th_dex: Some("uni_v3"),
            l_one_th_pair: Some(v3_pair_address),
            l_one_th_token: Some(v3_t_1),
            l_one_th_price_raw: Some(v3_p_1_raw),
            l_one_th_price: Some(v3_p_1),
            l_one_th_dec: Some(v3_d_1),
        }
    } else {
        HighLowOutput {
            h_zero_th_dex: None,
            h_zero_th_pair: None,
            h_zero_th_token: None,
            h_zero_th_price_raw: None,
            h_zero_th_price: None,
            h_zero_th_dec: None,
            l_zero_th_dex: None,
            l_zero_th_pair: None,
            l_zero_th_token: None,
            l_zero_th_price_raw: None,
            l_zero_th_price: None,
            l_zero_th_dec: None,
            h_one_th_dex: None,
            h_one_th_pair: None,
            h_one_th_token: None,
            h_one_th_price_raw: None,
            h_one_th_price: None,
            h_one_th_dec: None,
            l_one_th_dex: None,
            l_one_th_pair: None,
            l_one_th_token: None,
            l_one_th_price_raw: None,
            l_one_th_price: None,
            l_one_th_dec: None,
            sqrt_x96_v3: None,
        }
    };

    pair_token_price
}

// not using struct-tuple coz the variable names will come handy.
#[derive(Debug)]
struct TokenAndPrice {
    pair_address: Address,
    token_0: Address,
    // Price in bignumber.
    price_0_raw: U256,
    // Price in decimal.
    price_0: f64,
    decimal_0: u8,
    token_1: Address,
    // Price in bignumber.
    price_1_raw: U256,
    // Price in decimal.
    price_1: f64,
    decimal_1: u8,
    // need this for uni v3 swap exe.
    sqrt_x96: Option<U256>,
}

async fn get_price_v3(v3_pair_address: Address, client: &NodeClient) -> Result<TokenAndPrice> {
    let pair_instance = UNI_V3_PAIR::new(v3_pair_address, client.clone());
    let token_0 = pair_instance.token_0().call().await?;
    let token_1 = pair_instance.token_1().call().await?;

    let erc20_abi: Abi = serde_json::from_str(
        r#"[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"guy","type":"address"},{"name":"wad","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"src","type":"address"},{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"wad","type":"uint256"}],"name":"withdraw","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"deposit","outputs":[],"payable":true,"stateMutability":"payable","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"},{"name":"","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"payable":true,"stateMutability":"payable","type":"fallback"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":true,"name":"guy","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":true,"name":"dst","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"dst","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Deposit","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Withdrawal","type":"event"}]"#,
    )?;

    let token_0_dec: u8 = Contract::new(token_0, erc20_abi.clone(), client.clone())
        .method("decimals", ())?
        .call()
        .await?;

    let token_1_dec: u8 = Contract::new(token_1, erc20_abi, client.clone())
        .method("decimals", ())?
        .call()
        .await?;

    //Tuple([Uint(1735334720669675743951363989854996), Int(199897), Uint(494), Uint(722), Uint(722), Uint(0), Bool(true)])
    let slot_0 = pair_instance.slot_0().call().await?;
    let sqrt_x96: U256 = slot_0.0;
    let base: U256 = U256::from(2).pow(96.into()); // (79228162514264337593543950336)
    let power_of_eighteen: U256 = U256::from(10).pow(18.into()); // (1000000000000000000)

    // some maths to return the values as a fixed point number matching its corresponding decimal
    let check: U256 = ((sqrt_x96 * power_of_eighteen) / base).pow(2.into()) / power_of_eighteen;
    let after_check = (((power_of_eighteen * power_of_eighteen) / check) * (power_of_eighteen))
        / power_of_eighteen;

    // convert U256 to primitive type u128.
    let sqrt_x96_con: u128 = sqrt_x96.as_u128();
    let base_math: f64 = (sqrt_x96_con as f64 / 2f64.powi(96)).powi(2);

    // no extra math is needed if both tokens decimals are same.
    if token_0_dec == token_1_dec {
        // convert primitive u128 to f64 and raise to the power (powi)
        let tp = TokenAndPrice {
            pair_address: v3_pair_address,
            token_0,
            decimal_0: token_0_dec,
            price_0_raw: check,
            price_0: base_math,
            token_1,
            price_1_raw: after_check,
            price_1: 1f64 / base_math,
            decimal_1: token_1_dec,
            sqrt_x96: Some(sqrt_x96),
        };
        Ok(tp)
    } else {
        let bigger_dec = max(token_0_dec, token_1_dec);
        let smaller_dec = min(token_0_dec, token_1_dec);
        let dec_diff = bigger_dec - smaller_dec;

        // one more step if both the decimals of the tokens involved aren't same.
        // value / 10 ** (decimal_of_token_0 - decimal_of_token_1)
        let base_math_inner = base_math / 10f64.powi(dec_diff as i32);
        let power_of_dec_diff = U256::from(10).pow(dec_diff.into());
        let power_of_token_0_dec: U256 = U256::from(10).pow(token_0_dec.into());

        // let price_first = (sqrt_x96 as f64 / 2f64.powi(96)).powi(2);
        let tp = TokenAndPrice {
            pair_address: v3_pair_address,
            token_0,
            decimal_0: token_0_dec,
            price_0_raw: check / power_of_dec_diff,
            // price_0_raw: U256::from(
            //     ((base_math * 10f64.powi(token_0_dec as i32)) / 10f64.powi(dec_diff as i32))
            //         as u128,
            // ),
            price_0: base_math_inner,
            token_1,
            price_1_raw: (((power_of_eighteen * power_of_eighteen) / (check / power_of_dec_diff))
                * (power_of_token_0_dec))
                / power_of_eighteen,
            price_1: 1f64 / base_math_inner,
            decimal_1: token_1_dec,
            sqrt_x96: Some(sqrt_x96),
        };
        Ok(tp)
    }
}

async fn get_price_v2(v2_pair_address: Address, client: &NodeClient) -> Result<TokenAndPrice> {
    let pair_instance = UNI_V2_PAIR::new(v2_pair_address, client.clone());
    let token_0 = pair_instance.token_0().call().await?;
    let token_1 = pair_instance.token_1().call().await?;
    let reserve_data = pair_instance.get_reserves().call().await?;
    println!("{token_0:?} {token_1:?} {reserve_data:?}");

    let token_0_dec: u8 = ERC20::new(token_0, client.clone())
        .decimals()
        .call()
        .await?;

    let token_1_dec: u8 = ERC20::new(token_1, client.clone())
        .decimals()
        .call()
        .await?;

    let reserve_0: u128 = reserve_data.0;
    let reserve_1: u128 = reserve_data.1;
    // let time_stamp: u32 = reserve_data.2;

    let price_reserve_token_0 = format_units(reserve_0, token_0_dec as u32)
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let price_reserve_token_1 = format_units(reserve_1, token_1_dec as u32)
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let tp = TokenAndPrice {
        pair_address: v2_pair_address,
        token_0,
        price_0_raw: U256::from(
            ((price_reserve_token_1 / price_reserve_token_0) * 10f64.powi(token_1_dec as i32))
                as u128,
        ),
        price_0: price_reserve_token_1 / price_reserve_token_0,
        decimal_0: token_0_dec,
        token_1,
        price_1_raw: U256::from(
            ((price_reserve_token_0 / price_reserve_token_1) * 10f64.powi(token_0_dec as i32))
                as u128,
        ),
        price_1: price_reserve_token_0 / price_reserve_token_1,
        decimal_1: token_1_dec,
        sqrt_x96: None,
    };

    Ok(tp)
}

#[derive(Debug)]
struct PairAddress {
    v2_pair: Address,
    v3_pair: Address,
}

async fn get_pair_address(
    address_0: Address,
    address_1: Address,
    pool_fee: u32,
    client: &NodeClient,
) -> Result<PairAddress> {
    // initiate a v2_factory instance.
    let uni_v2_factory_address = UNI_V2_FACTORY_ADDRESS.parse::<Address>()?;

    let factory_contract_v2 = UNI_V2_FACTORY::new(uni_v2_factory_address, client.clone());

    // initiate v3 factory instance.
    let uni_v3_factory_address = UNI_V3_FACTORY_ADDRESS.parse::<Address>()?;
    let factory_contract_v3 = UNI_V3_FACTORY::new(uni_v3_factory_address, client.clone());

    let pair_v2: Address = factory_contract_v2
        .get_pair(address_0, address_1)
        .call()
        .await?;

    if pair_v2.is_zero() {
        panic!("Pair address non existent in uniV2.");
    }

    let pair_v3: Address = factory_contract_v3
        .get_pool(address_0, address_1, pool_fee)
        .call()
        .await?;

    if pair_v3.is_zero() {
        panic!("Pair address non existent in uni_v3.");
    }

    Ok(PairAddress {
        v2_pair: pair_v2,
        v3_pair: pair_v3,
    })
}

// SWAP EXECUTION STEPS
// BUY THE TOKEN UNDERLYING IN THE ZERO'TH INDEX.
// SELL THE PREVIOUS BOUGHT TOKENS
// ONLY EXECUTE IF THE OUTPUT ON SELL IS > SPENT ON BUY.

#[allow(unreachable_code, dead_code, unused_labels)]
async fn buy_zero_sell_zero<T>(
    token0: Address,
    token1: Address,
    fee: u32,
    client: &NodeClient,
    wallet: LocalWallet,
    sender: Address,
    sepolia_test_contract: Contract<T>,
) -> Result<()> {
    let starting_buy_units = U256::from(10000);
    let slippage_percent: f64 = 0.2;
    let mut counter: u32 = 0;
    let iter_limit: u32 = 499;
    let zero = U256::from(0);

    // pair addresses.
    let pair_address = get_pair_address(token0, token1, fee, client).await?;

    'outer: loop {
        counter += 1;
        // fetch for current price and compare.
        let uni_v2_price = get_price_v2(pair_address.v2_pair, client).await?;
        let uni_v3_price = get_price_v3(pair_address.v3_pair, client).await?;
        let hr = return_low_high(uni_v2_price, uni_v3_price);

        println!(
            "low__zero: {:#?} {:#?} {:#?} {:#?} \nhigh_zero: {:#?} {:#?} {:#?} {:#?}\n",
            hr.l_zero_th_pair.unwrap(),
            hr.l_zero_th_dex.unwrap(),
            hr.l_zero_th_token.unwrap(),
            hr.l_zero_th_price.unwrap(),
            hr.h_zero_th_pair.unwrap(),
            hr.h_zero_th_dex.unwrap(),
            hr.h_zero_th_token.unwrap(),
            hr.h_zero_th_price.unwrap()
        );

        println!(
            "low__one: {:#?} {:#?} {:#?} {:#?} \nhigh_one: {:#?} {:#?} {:#?} {:#?}\n",
            hr.l_one_th_pair.unwrap(),
            hr.l_one_th_dex.unwrap(),
            hr.l_one_th_token.unwrap(),
            hr.l_one_th_price.unwrap(),
            hr.h_one_th_pair.unwrap(),
            hr.h_one_th_dex.unwrap(),
            hr.h_one_th_token.unwrap(),
            hr.h_one_th_price.unwrap()
        );

        let power_of_zeroth_index = U256::from(10).pow(hr.l_zero_th_dec.unwrap().into());
        let power_of_oneth_index = U256::from(10).pow(hr.l_one_th_dec.unwrap().into());

        // variable to buy token underlying on 0th index of the pool.
        // FIRST SWAP STARTS HERE.
        // Returns the pool where the token underlying on zero'th index is cheaper.
        let cheap_pool_zeroth = hr.l_zero_th_pair.unwrap();

        // buy this token, underlying in the zero'th index.
        let token_out = hr.l_zero_th_token.unwrap();

        // sell this token, underlying in the one'th index.
        let token_in = hr.l_one_th_token.unwrap();

        // swap price of a single unit, in terms of the another token involved in the swap pool.
        let single_unit_price: U256 = hr.l_zero_th_price_raw.unwrap();

        // amount_in in fixed point value, adjusted to token's decimal we are selling.
        // only multiply by buying units, since unit_price already is returned decimalized.
        // Amount is flash loaned.
        let amount_in: U256 = single_unit_price * starting_buy_units;

        // the decimal percentage is multiplied by power_of_zeroth_index
        // which makes it possible to store f64 as U256.
        let slippage = U256::from(
            ((slippage_percent / 100f64) * 10f64.powi(hr.l_zero_th_dec.unwrap().into())) as u128,
        );

        // returns the value in fixed point value of the token buying currently.
        // if the buy_units is in FPV then divide by the power or decimal value.
        let slippage = slippage * starting_buy_units;

        // minimun token out units in fized point value.
        let amount_out_min: U256 = (starting_buy_units * power_of_zeroth_index) - slippage;

        // buy amount in fixed point value, adjusted to token's decimal we are buying.
        let buy_amount: U256 = starting_buy_units * power_of_zeroth_index;

        let sqrt_x96 = hr.sqrt_x96_v3.unwrap();
        println!("{sqrt_x96}");

        // this first txn could be thought as exact_output type swap
        // since price is calculated as token_price * token_units_to_buy.
        // so we know how much is needed to buy for output tokens.
        println!(
            "Front run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
            cheap_pool_zeroth, token_out, token_in, single_unit_price
        );
        println!(
            "buy_amount: {:#?} amount_in: {:#?} amount_out_min: {:#?}\n",
            buy_amount, amount_in, amount_out_min
        );

        //----------------------------------------------------------------------------------------------------------------------
        // variable to buy token underlying on 1th index of the pool.
        // SECOND SWAP STARTS HERE.
        // Second swap in this pool,where the token on 1'th index is cheaper.
        let cheap_pool_oneth = hr.l_one_th_pair.unwrap();

        // buy this token
        let token_out_1 = hr.l_one_th_token.unwrap();

        // sell this token to the pool in exchange of token_out.
        let token_in_1 = hr.l_zero_th_token.unwrap();

        // swap price in terms of the another token involved in the swap pool.
        let single_unit_price_1: U256 = hr.l_one_th_price_raw.unwrap();

        // amount_in in fixed point value.
        // on local simulation use output of the prev swap.
        // on mainnet execution check balance and swap using available units.
        let amount_in_1: U256 = amount_out_min;

        // minimun token out units.
        // amount_out_min is already fixed point, multiply with power_of_oneth_index for decimal handle
        // output is calculated by dividing a single unit price of token by quantity.
        let expected_output_1 = (amount_out_min * power_of_oneth_index) / single_unit_price_1;

        // initial check.
        match expected_output_1 > amount_in {
            false => {
                println!(
                    "non profitable before slippage adjustment:\nexpected_output: {expected_output_1} amount_spent: {amount_in} current_loss: {}\n", amount_in - expected_output_1
                );

                println!("current iteration buyingZeroSellZero: {counter}\n");
                if counter.gt(&iter_limit) {
                    break 'outer;
                }
                continue 'outer;
            }

            true => {
                // log for testing purpose.
                let mut f = File::options().append(true).open("./logs.txt")?;
                writeln!(
                    &mut f,
                    "intial check profit:{}",
                    (expected_output_1 - amount_in)
                )?;

                // the decimal percentage is multiplied by power_of_oneth_index
                // which makes it possible to store as U256.
                let slippage_1 = U256::from(
                    ((slippage_percent / 100f64) * 10f64.powi(hr.l_one_th_dec.unwrap().into()))
                        as u128,
                );

                // here we divide by power_of_oneth_index to get the actual value back.
                // since amount_in is already in fixed point value.
                // if amount_in wasn't in FPV then no divide needed.
                let slippage_1 = (slippage_1 * expected_output_1) / power_of_oneth_index;

                // minimun token out units.
                // minimum output for this swap has to be atleat the input amount of previous swap.
                let amount_out_min_1 = expected_output_1 - slippage_1;

                // buy amount in fixed point value.
                // buy amount is the output of prev swap.
                let buy_amount_1: U256 = expected_output_1;

                // this txn can be thought as exact_input in
                // expecting as much possible output as possible.
                println!(
                    "Back run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
                    cheap_pool_oneth, token_out_1, token_in_1, single_unit_price_1
                );
                println!(
                    "buy_amount: {:#?} amount_in: {:#?} amount_out_min: {:#?}\n",
                    buy_amount_1, amount_in_1, amount_out_min_1
                );

                // second check after slippage.
                // second swap output has to be greater than the input of the first swap.
                if amount_out_min_1 < amount_in {
                    println!("non profitable after slippage check!");
                    if counter.gt(&iter_limit) {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                    continue 'outer;
                } else if amount_out_min_1.gt(&amount_in.add(slippage)) {
                    println!("Building transaction post slippage check.\n");
                    let mut f = File::options().append(true).open("./logs.txt")?;
                    writeln!(&mut f, "after slippage profit:{}", {
                        amount_out_min_1 - amount_in
                    })?;
                } else {
                    continue 'outer;
                }

                // conditional execution ahead checking dex.
                match hr.l_zero_th_dex == Some("uni_v2") {
                    true => {
                        println!("execution start from uni_v2, buying token underlying on zero'th index.");

                        // Start with flash loan.
                        // Swap data is passed in as bytes in an ordered fashion, which will be decoded in the contract.
                        let v2_swap_selector = function_selector(V2_SWAP_SELECTOR_STR);

                        // First swap.
                        // one'th indexed token in zero'th indexed token out.
                        let encoded_v2_swap = sepolia_test_contract
                            .encode_with_selector(
                                v2_swap_selector,
                                (
                                    cheap_pool_zeroth,
                                    sender,
                                    sqrt_x96,
                                    zero,           // amount0In
                                    amount_out_min, // amount0Out
                                    amount_in,      // amount1In
                                    zero,           // amount1out
                                    false,          // zeroIn oneOut ?
                                    Bytes::new(),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // Second swap.
                        // zero'th indexed token in one'th indexed token out.
                        let zero_for_one_selector = function_selector(ZERO_FOR_ONE_SELECTOR_STR);
                        let encoded_zero_for_one_swap = if hr.l_one_th_dex == Some("uni_v3") {
                            sepolia_test_contract
                                .encode_with_selector(
                                    zero_for_one_selector,
                                    (
                                        cheap_pool_oneth,
                                        sender,
                                        sqrt_x96,
                                        amount_in_1,      // amount0In
                                        zero,             // amount0Out
                                        zero,             // amount1In
                                        amount_out_min_1, // amount1out
                                        true,             // zeroIn oneOut ? :only care on v2_swap.
                                        Bytes::new(),
                                    ),
                                )
                                .unwrap()
                                .to_vec()
                        } else {
                            println!("Shouldn't happen!");
                            panic!("shouldn't")
                        };

                        let execution_bytes_data = encode(&[
                            Token::Bytes(encoded_v2_swap),
                            Token::Bytes(encoded_zero_for_one_swap),
                        ]);

                        println!("{:?}", execution_bytes_data);

                        // flash loan encoding, with extra bytes data for swap exe.
                        let flash_loan_selector = function_selector(FLASH_LOAN_SELECTOR_STR);
                        let encoded_loan_with_bytes_to_swap = sepolia_test_contract
                            .encode_with_selector(
                                flash_loan_selector,
                                (token_in, amount_in, execution_bytes_data),
                            )
                            .unwrap();

                        // send transaction only after back run txn simulation outputs profitable result.
                        let signing_client = SignerMiddleware::new(client.clone(), wallet.clone());
                        let tx = TransactionRequest::new()
                            .to(SEPOLIA_TEST_CONTRACT_ADDRESS.parse::<Address>()?)
                            .data(encoded_loan_with_bytes_to_swap);
                        let pending_tx = signing_client.send_transaction(tx, None).await?;
                        let receipt = pending_tx
                            .await?
                            .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;

                        let tx = signing_client
                            .get_transaction(receipt.transaction_hash)
                            .await?;
                        println!("Sent tx: {}\n", serde_json::to_string(&tx)?);
                        println!("Tx receipt: {}", serde_json::to_string(&receipt)?);
                    }

                    false => {
                        println!("execution start from uni_v3, buying token underlying on zero'th index.");

                        // Swap data is passed in as bytes in an orderly fashion, which will be decoded in the contract.
                        let one_for_zero_selector = function_selector(ONE_FOR_ZERO_SELECTOR_STR);

                        // First swap.
                        // one'th indexed token in zero'th indexed token out.
                        let encoded_one_for_zero_swap = sepolia_test_contract
                            .encode_with_selector(
                                one_for_zero_selector,
                                (
                                    cheap_pool_zeroth,
                                    sender,
                                    sqrt_x96,
                                    zero,           // amount0In
                                    amount_out_min, // amount0Out
                                    amount_in,      // amount1In
                                    zero,           // amount1out
                                    false,          // zeroIn oneOut ? :only care on v2_swap.
                                    Bytes::new(),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // Second swap.
                        // zero'th indexed token in one'th indexed token out.
                        let v2_swap_selector = function_selector(V2_SWAP_SELECTOR_STR);
                        let encoded_v2_swap = if hr.l_one_th_dex == Some("uni_v3") {
                            // expecting to be false.
                            sepolia_test_contract
                                .encode_with_selector(
                                    v2_swap_selector,
                                    (
                                        cheap_pool_oneth,
                                        sender,
                                        sqrt_x96,
                                        amount_in_1,      // amount0In
                                        zero,             // amount0Out
                                        zero,             // amount1In
                                        amount_out_min_1, // amount1Out
                                        true,             // zeroIn oneOut ?
                                        Bytes::new(),
                                    ),
                                )
                                .unwrap()
                                .to_vec()
                        } else {
                            println!("Shouldn't happen!");
                            panic!("shouldn't")
                        };

                        let execution_bytes_data = encode(&[
                            Token::Bytes(encoded_one_for_zero_swap),
                            Token::Bytes(encoded_v2_swap),
                        ]);

                        println!("{:?}", execution_bytes_data);

                        // flash loan encoding, with extra bytes data for swap exe.
                        let flash_loan_selector = function_selector(FLASH_LOAN_SELECTOR_STR);
                        let encoded_loan_with_bytes_to_swap = sepolia_test_contract
                            .encode_with_selector(
                                flash_loan_selector,
                                (token_in, amount_in, execution_bytes_data),
                            )
                            .unwrap();

                        // send transaction only after back run txn simulation outputs profitable result.
                        let signing_client = SignerMiddleware::new(client.clone(), wallet.clone());
                        let tx = TransactionRequest::new()
                            .to(SEPOLIA_TEST_CONTRACT_ADDRESS.parse::<Address>()?)
                            .data(encoded_loan_with_bytes_to_swap);
                        let pending_tx = signing_client.send_transaction(tx, None).await?;
                        let receipt = pending_tx
                            .await?
                            .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;

                        let tx = signing_client
                            .get_transaction(receipt.transaction_hash)
                            .await?;
                        println!("Sent tx: {}\n", serde_json::to_string(&tx)?);
                        println!("Tx receipt: {}", serde_json::to_string(&receipt)?);
                    }
                }
            }
        }
    }

    Ok(())
}

// SWAP EXECUTION STEPS
// BUY THE TOKEN UNDERLYING IN THE ONE'TH INDEX.
// SELL THE PREVIOUS BOUGHT TOKENS
// ONLY EXECUTE IF THE OUTPUT ON SELL IS > SPENT ON BUY.
#[allow(unreachable_code, dead_code, unused_labels)]
async fn buy_one_sell_one<T>(
    token0: Address,
    token1: Address,
    fee: u32,
    client: &NodeClient,
    wallet: &LocalWallet,
    receiver: Address,
    exe_contract: Contract<T>,
) -> Result<()> {
    let starting_buy_units: U256 = 10.into();
    let slippage_percent: f64 = 0.5;
    let mut counter = 0;
    let iter_limit: u32 = 1;
    let zero = U256::from(0);

    // pair addresses.
    let pair_address = get_pair_address(token0, token1, fee, client).await?;

    'outer: loop {
        counter += 1;
        // fetch for current price and compare.
        let uni_v2_price = get_price_v2(pair_address.v2_pair, client).await?;
        let uni_v3_price = get_price_v3(pair_address.v3_pair, client).await?;
        let hr = return_low_high(uni_v2_price, uni_v3_price);

        eprintln!(
            "low__zero: {} {} {} {} \nhigh_zero: {} {} {} {}\n",
            hr.l_zero_th_pair.unwrap(),
            hr.l_zero_th_dex.unwrap(),
            hr.l_zero_th_token.unwrap(),
            hr.l_zero_th_price.unwrap(),
            hr.h_zero_th_pair.unwrap(),
            hr.h_zero_th_dex.unwrap(),
            hr.h_zero_th_token.unwrap(),
            hr.h_zero_th_price.unwrap()
        );

        eprintln!(
            "low__one: {} {} {} {} \nhigh_one: {} {} {} {}\n",
            hr.l_one_th_pair.unwrap(),
            hr.l_one_th_dex.unwrap(),
            hr.l_one_th_token.unwrap(),
            hr.l_one_th_price.unwrap(),
            hr.h_one_th_pair.unwrap(),
            hr.h_one_th_dex.unwrap(),
            hr.h_one_th_token.unwrap(),
            hr.h_one_th_price.unwrap()
        );

        //----------------------------------------------------------------------------------------------------------------------

        let power_of_zeroth_index = U256::from(10).pow(hr.l_zero_th_dec.unwrap().into());
        let power_of_oneth_index = U256::from(10).pow(hr.l_one_th_dec.unwrap().into());

        // variable to buy token underlying on 1th index of the pool.
        // FIRST SWAP STARTS HERE.
        // Returns the pool where the token underlying on one'th index is cheaper.
        let cheap_pool_oneth: H160 = hr.l_one_th_pair.unwrap();

        // buy this token, underlying in the one'th index.
        let token_out: H160 = hr.l_one_th_token.unwrap();

        // sell this token, underlying in the zero'th index.
        let token_in: H160 = hr.l_zero_th_token.unwrap();

        // swap price returns in terms of the other token involved in the swap pool.
        let single_unit_price: U256 = hr.l_one_th_price_raw.unwrap();

        // amount_in in fixed point value, adjusted to token's decimal we are selling.
        // only multiply by buying units, since unit_price already is returned decimalized.
        // Amount is flash loaned.
        let amount_in: U256 = single_unit_price * starting_buy_units;

        // the decimal percentage is multiplied by power_of_oneth_index
        // which makes it possible to store f64 as U256.
        let slippage = U256::from(
            ((slippage_percent / 100f64) * 10f64.powi(hr.l_one_th_dec.unwrap().into())) as u128,
        );

        // returns the value in fixed point value of the token buying currently.
        // if the buy_units is in FPV then divide by the power or decimal value.
        let slippage = slippage * starting_buy_units;

        // minimun token out units in fixed point value.
        let amount_out_min: U256 = (starting_buy_units * power_of_oneth_index) - slippage;

        // buy amount in fixed point value, adjusted to token's decimal we are buying.
        let buy_amount: U256 = starting_buy_units * power_of_oneth_index;

        let sqrt_x96 = hr.sqrt_x96_v3.unwrap();
        eprintln!("sqrt_x96: {sqrt_x96}\n");

        // this first txn could be thought as exact_output type swap
        // since price is calculated as token_price * token_units_to_buy.
        // so we know how much is needed to buy for output tokens.
        eprintln!(
            "Front run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
            cheap_pool_oneth, token_out, token_in, single_unit_price
        );
        eprintln!(
            "buy_amount: {:#?} amount_in: {:#?} amount_out_min: {:#?}\n",
            buy_amount, amount_in, amount_out_min
        );

        //----------------------------------------------------------------------------------------------------------------------
        // variable to buy token underlying on 0th index of the pool.
        // SECOND SWAP STARTS HERE.
        // Second swap in this pool,where the token on 0'th index is cheaper.
        let cheap_pool_zeroth: H160 = hr.l_zero_th_pair.unwrap();

        // buy this token.
        let token_out_1: H160 = hr.l_zero_th_token.unwrap();

        // sell this token to the pool in exchange of the token out.
        let token_in_1: H160 = hr.l_one_th_token.unwrap();

        // swap price in terms of the another token involved in the swap pool.
        let single_unit_price_1: U256 = hr.l_zero_th_price_raw.unwrap();

        // amount_in in fixed point value.
        // on local simulation use output of the prev swap.
        // on mainnet execution check balance and swap using available units.
        let amount_in_1: U256 = amount_out_min; // output amount of previous swap is the current input.

        // minimun token out units.
        // amount_out_min is already fixed point, multiply with power_of_zeroth_index for decimal handle
        // calculates the possible output dividing the Unit price by quantity.
        let expected_output_1 = (amount_out_min * power_of_zeroth_index) / single_unit_price_1;

        // initial check.|| >
        match expected_output_1 > amount_in {
            false => {
                eprintln!(
                    "non profitable before slippage adjustment:\nexpected_output: {expected_output_1} amount_spent: {amount_in} \n");

                eprintln!("current iteration buyingOneSellOne: {counter}\n");
                if counter >= iter_limit {
                    break 'outer;
                }
                continue 'outer;
            }

            true => {
                // log for testing purpose.
                // let mut f = File::options().append(true).open("./logs.txt")?;
                // writeln!(
                //     &mut f,
                //     "intial check profit B1S1:{}",
                //     (expected_output_1 - amount_in)
                // )?;

                // the decimal percentage is multiplied by power_of_zeroth_index
                // which makes it possible to store as U256.
                let slippage_1 = U256::from(
                    ((slippage_percent / 100f64) * 10f64.powi(hr.l_zero_th_dec.unwrap().into()))
                        as u128,
                );

                // here we divide by power_of_zeroth_index to get the actual value back.
                // since amount_in is already in fixed point value.
                // if amount_in wasn't in FPV then no divide needed.
                let slippage_1 = (slippage_1 * expected_output_1) / power_of_zeroth_index;

                // minimun token out units.
                // minimum output for this swap has to be atleat the input amount of previous swap.
                let amount_out_min_1 = expected_output_1 - slippage_1;

                // buy amount in fixed point value.
                // buy amount is the output of the prev swap.
                let buy_amount_1: U256 = expected_output_1;

                // this txn can be thought as exact_input in
                // expecting as much possible output as possible.
                eprintln!(
                    "Back run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
                    cheap_pool_zeroth, token_out_1, token_in_1, single_unit_price_1
                );
                eprintln!(
                    "expected_output: {:#?} amount_in: {:#?} amount_out_min: {:#?}\n\n",
                    buy_amount_1, amount_in_1, amount_out_min_1
                );

                // second check after slippage.
                // second swap output has to be greater than the input of the first swap.
                if amount_out_min_1 < amount_in {
                    eprintln!("non profitable on back run after slippage calculation");
                    eprintln!("current iteration oneForZero after slippage: {counter}\n");
                    if counter >= iter_limit {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                    continue 'outer;
                } else {
                    eprintln!("Passed:  slippage check.\n");
                    let mut f = File::options().append(true).open("./logs.txt")?;
                    writeln!(&mut f, "after slippage profit:{}", {
                        amount_out_min_1 - amount_in
                    })?;
                }
                // conditional execution ahead.
                match hr.l_one_th_dex == Some("uni_v2") {
                    true => {
                        eprintln!(
                            "execution start from uni_v2, buying token underlying on one'th index."
                        );

                        // transferring the first token to the contract.
                        // while it swaps using flash loan, no need to transfer to the contract.
                        // let erc_20_abi: Abi = serde_json::from_str(
                        //     r#"[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"guy","type":"address"},{"name":"wad","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"src","type":"address"},{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"wad","type":"uint256"}],"name":"withdraw","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"deposit","outputs":[],"payable":true,"stateMutability":"payable","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"},{"name":"","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"payable":true,"stateMutability":"payable","type":"fallback"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":true,"name":"guy","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":true,"name":"dst","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"dst","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Deposit","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"src","type":"address"},{"indexed":false,"name":"wad","type":"uint256"}],"name":"Withdrawal","type":"event"}]"#,
                        // )?;

                        // Second swap bytes data is passed in to the first swap data param.
                        // one'th indexed token in zero'th indexed token out.
                        // Second swap init.
                        let one_for_zero_selector = function_selector(ONE_FOR_ZERO_SELECTOR_STR);
                        let second_swap_param = SwapParam::new(
                            cheap_pool_zeroth,
                            receiver,
                            sqrt_x96,
                            zero,
                            zero,
                            amount_in_1,
                            zero,
                            amount_out_min_1,
                            false,
                            Bytes::new(),
                        );

                        let encoded_second_swap = if hr.l_zero_th_dex == Some("uni_v3") {
                            custom_standard_encode(one_for_zero_selector, second_swap_param)
                        } else {
                            eprintln!("Shouldn't happen!");
                            panic!("shouldn't")
                        };

                        eprintln!(
                            "second_swap: {:?}",
                            hex::encode_prefixed(&encoded_second_swap)
                        );

                        // First swap is sent to the flash loan function
                        // Decodes there and send the bytes calldata to its corresponding.
                        // zero'th indexed token in one'th indexed token out.
                        // First swap init.
                        let v2_swap_selector = function_selector(V2_SWAP_SELECTOR_STR);
                        let first_swap_param = SwapParam::new(
                            cheap_pool_oneth,
                            receiver,
                            sqrt_x96,
                            amount_in,
                            zero,
                            zero,
                            zero,
                            amount_out_min,
                            true,
                            encoded_second_swap.into(),
                        );
                        let encoded_first_swap =
                            custom_standard_encode(v2_swap_selector, first_swap_param);

                        eprintln!(
                            "First swap: {:#?}",
                            hex::encode_prefixed(&encoded_first_swap)
                        );

                        eprintln!("{:#?}", encoded_first_swap);

                        // flash loan encoding, with bytes call data for swap exe.
                        // let flash_loan_selector = function_selector(FLASH_LOAN_SELECTOR_STR);
                        // let encoded_flash_swap = encode(&[
                        //     Token::Bytes(flash_loan_selector.into()),
                        //     Token::Address(token_in),
                        //     Token::Uint(amount_in),
                        //     Token::Bytes(encoded_first_swap.clone()),
                        // ]);

                        // send transaction only after back run txn simulation outputs profitable result.
                        build_send_tx(
                            client,
                            wallet,
                            exe_contract.clone(),
                            encoded_first_swap.into(),
                        )
                        .await?;
                    }
                    false => {
                        eprintln!(
                            "execution start from uni_v3, buying token underlying on one'th index."
                        );
                        let withdraw_eth_selector = function_selector(WITHDRAW_ETH);

                        // Second swap
                        // one'th indexed token in zero'th indexed token out.
                        let v2_swap_selector = function_selector(V2_SWAP_SELECTOR_STR);
                        let second_swap_param = SwapParam::new(
                            cheap_pool_zeroth,
                            receiver,
                            sqrt_x96,
                            zero,
                            amount_out_min_1,
                            amount_in_1,
                            zero,
                            amount_out_min_1,
                            false,
                            Bytes::from_static(b"0x"),
                        );
                        let encoded_second_swap = if hr.l_zero_th_dex == Some("uni_v2") {
                            Bytes(
                                custom_standard_encode(v2_swap_selector, second_swap_param).into(),
                            )
                        } else {
                            eprintln!("Shouldn't happen!");
                            panic!("shouldn't")
                        };
                        eprintln!("Second swap\n {:?}", &encoded_second_swap);

                        // First swap.
                        // zero'th indexed token in one'th indexed token out.
                        let zero_for_one_selector = function_selector(ZERO_FOR_ONE_SELECTOR_STR);
                        let first_swap_param = SwapParam::new(
                            cheap_pool_oneth,
                            receiver,
                            sqrt_x96,
                            amount_in,
                            zero,
                            zero,
                            zero,
                            amount_out_min,
                            true,
                            encoded_second_swap,
                        );

                        let encoded_first_swap =
                            custom_standard_encode(zero_for_one_selector, first_swap_param);

                        // eprintln!("First swap\n{:?}", &encoded_first_swap);

                        // flash loan encoding, with bytes call data for swap exe.
                        let flash_loan_selector: [u8; 4] =
                            function_selector(FLASH_LOAN_SELECTOR_STR);

                        // final data to send to contract for execution.
                        // let encoded_send = exe_contract
                        //     .encode_with_selector(
                        //         flash_loan_selector,
                        //         (
                        //             token_in,
                        //             amount_in,
                        //             Bytes(encoded_first_swap.clone().into()),
                        //         ),
                        //     )
                        //     .unwrap();

                        let encode_send = Bytes(
                            encode(&[
                                Token::FixedBytes(flash_loan_selector.to_vec()),
                                Token::Tuple(vec![
                                    Token::Address(token_in),
                                    Token::Uint(amount_in),
                                    Token::Bytes(encoded_first_swap.clone()),
                                ]),
                            ])
                            .into(),
                        );

                        // eprintln!("Exe swap\n{:?}", &encoded_send);

                        build_send_tx(client, wallet, exe_contract.clone(), encode_send).await?;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn build_send_tx<T>(
    client: &NodeClient,
    wallet: &LocalWallet,
    contract: Contract<T>,
    encoded_data: Bytes,
) -> Result<()> {
    println!("Building transacton.");
    let signing_client = SignerMiddleware::new(client.clone(), wallet.clone());
    let tx = TransactionRequest::new()
        .to(contract.address())
        .data(encoded_data);
    let pending_tx = signing_client.send_transaction(tx, None).await?;
    let receipt = pending_tx
        .await?
        .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;

    let tx = signing_client
        .get_transaction(receipt.transaction_hash)
        .await?;
    println!("Sent tx: {}\n", serde_json::to_string(&tx)?);
    println!("Tx receipt: {}", serde_json::to_string(&receipt)?);

    Ok(())
}
