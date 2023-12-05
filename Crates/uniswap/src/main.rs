use ethers::{
    abi::{encode, Abi, ParamType, Token, TupleParam, Uint},
    contract::Contract,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest, H160, U256},
    utils::{format_units, Units},
};
use eyre::Result;
use serde_json;
use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::Write,
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
const SEP0LIA: &str = "https://eth-sepolia.g.alchemy.com/v2/X6jHEi-MPp4uvNPTgwUDEMIDpVAkPASZ";
const SEPOLIA_TEST_CONTRACT_ADDRESS: &str = "0xaFC3d130544eEab772b910799bb0c2427bD5871F";
const GOERLI: &str = "https://eth-goerli.g.alchemy.com/v2/kNAKUC0Dijrm1XLnPo9yu08WU_lGvZ2w";
const GOERLI_TEST_CONTRACT_ADDRESS: &str = "0x99ed4855c2e5609ab2034922987d0366357665eb";
const GOERLI_WETH: &str = "0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6";
abigen!(TEST_CONTRACT, "./abi/targetswap.json");
const FORK_TEST_CONTRACT_ADDRESS: &str = "0xa68E430060f74F9821D2dC9A9E2CE3aF7d842EBe";
const MAINNET_FORK: &str = "http://127.0.0.1:8545";
const ALICE_PK: &str = "0xac0974bec39a17e36ba4a6b4d238ff94";
const ALICE: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

type NodeClient = Arc<Provider<Ws>>;

#[allow(unused_variables, dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Ws>::connect(RPC_URL_MAINNET).await?;
    // let provider = Provider::try_from(MAINNET_FORK)?;
    let client = Arc::new(provider);

    let sepolia_chain_id: u64 = 11155111;
    let mainnet_chain_id: u64 = 1;
    println!("connection established sucessfully.");

    let wallet: LocalWallet = env::var("ALICE_PK")
        .unwrap()
        .parse::<LocalWallet>()?
        .with_chain_id(mainnet_chain_id);

    // define addresses involved.
    let weth = WETH.parse::<Address>()?;
    let dai = DAI.parse::<Address>()?;
    let usdc = USDC.parse::<Address>()?;
    let alice = ALICE.parse::<Address>()?;

    let test_contract_abi: Abi = serde_json::from_str(
        r#"[{"inputs":[{"internalType":"address","name":"_balancerVault","type":"address"}],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[{"internalType":"uint256","name":"amount0In","type":"uint256"},{"internalType":"uint256","name":"amount1out","type":"uint256"}],"name":"BOTH_IN_AND_OUT_CANNT_BE_ZERO","type":"error"},{"inputs":[],"name":"ZERO_AMOUNT_INPUT","type":"error"},{"inputs":[],"name":"ZERO_MINIMUN_OUTPUT","type":"error"},{"inputs":[],"name":"ZERO_POOL_ADDRESS","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"int256","name":"amount0Delta","type":"int256"},{"indexed":true,"internalType":"int256","name":"amount1Delta","type":"int256"}],"name":"SwapCallback","type":"event"},{"stateMutability":"payable","type":"fallback"},{"inputs":[{"components":[{"internalType":"address","name":"pool","type":"address"},{"internalType":"address","name":"recipient","type":"address"},{"internalType":"uint160","name":"sqrtPriceLimitX96","type":"uint160"},{"internalType":"uint256","name":"amount0In","type":"uint256"},{"internalType":"uint256","name":"amount0Out","type":"uint256"},{"internalType":"uint256","name":"amount1In","type":"uint256"},{"internalType":"uint256","name":"amount1Out","type":"uint256"},{"internalType":"bool","name":"zeroFurOne","type":"bool"},{"internalType":"bytes","name":"data","type":"bytes"}],"internalType":"struct TargetSwap.SwapParams","name":"sp","type":"tuple"}],"name":"oneForZero","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"contract IERC20[]","name":"tokens","type":"address[]"},{"internalType":"uint256[]","name":"amounts","type":"uint256[]"},{"internalType":"uint256[]","name":"feeAmounts","type":"uint256[]"},{"internalType":"bytes","name":"userData","type":"bytes"}],"name":"receiveFlashLoan","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"token","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"bytes","name":"userData","type":"bytes"}],"name":"requestFlashLoan","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"amount0Delta","type":"int256"},{"internalType":"int256","name":"amount1Delta","type":"int256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"uniswapV3SwapCallback","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"pool","type":"address"},{"internalType":"address","name":"recipient","type":"address"},{"internalType":"uint160","name":"sqrtPriceLimitX96","type":"uint160"},{"internalType":"uint256","name":"amount0In","type":"uint256"},{"internalType":"uint256","name":"amount0Out","type":"uint256"},{"internalType":"uint256","name":"amount1In","type":"uint256"},{"internalType":"uint256","name":"amount1Out","type":"uint256"},{"internalType":"bool","name":"zeroFurOne","type":"bool"},{"internalType":"bytes","name":"data","type":"bytes"}],"internalType":"struct TargetSwap.SwapParams","name":"sp","type":"tuple"}],"name":"v2Swap","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"pool","type":"address"},{"internalType":"address","name":"recipient","type":"address"},{"internalType":"uint160","name":"sqrtPriceLimitX96","type":"uint160"},{"internalType":"uint256","name":"amount0In","type":"uint256"},{"internalType":"uint256","name":"amount0Out","type":"uint256"},{"internalType":"uint256","name":"amount1In","type":"uint256"},{"internalType":"uint256","name":"amount1Out","type":"uint256"},{"internalType":"bool","name":"zeroFurOne","type":"bool"},{"internalType":"bytes","name":"data","type":"bytes"}],"internalType":"struct TargetSwap.SwapParams","name":"sp","type":"tuple"}],"name":"zeroForOne","outputs":[],"stateMutability":"nonpayable","type":"function"},{"stateMutability":"payable","type":"receive"}]"#,
    )?;

    let sepolia_test_contract = Contract::new(
        SEPOLIA_TEST_CONTRACT_ADDRESS.parse::<Address>()?,
        test_contract_abi,
        client.clone(),
    );

    buy_zero_sell_zero(
        weth,
        dai,
        500,
        &client,
        wallet.clone(),
        alice,
        sepolia_test_contract.clone(),
    )
    .await?;

    buy_one_sell_one(
        weth,
        dai,
        500,
        &client,
        wallet,
        alice,
        sepolia_test_contract,
    );
    // .await?;

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

    let pair_v3: Address = factory_contract_v3
        .get_pool(address_0, address_1, pool_fee)
        .call()
        .await?;

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
    let starting_buy_units = U256::from(100);
    let slippage_percent: f64 = 0.5;
    let mut counter: u32 = 0;
    let iter_limit: u32 = 999;

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
        // First swap in this pool, buying cheaper token underlying in the zero'th index of the pool.
        let cheap_pool_zeroth: H160 = hr.l_zero_th_pair.unwrap();

        // buy this token.(token_out)
        let token_out: H160 = hr.l_zero_th_token.unwrap();

        // (token_in)send this token to the pool in exchange of token_out.
        let token_in: H160 = hr.l_one_th_token.unwrap();

        // swap price of a single unit, in terms of the another token involved in the swap pool.
        let single_unit_price: U256 = hr.l_zero_th_price_raw.unwrap();

        // amount_in in fixed point value.
        // this is flash loaned.
        let amount_in: U256 = single_unit_price * starting_buy_units;

        // the decimal percentage is multiplied by power_of_oneth_index
        // which makes it possible to store f64 as U256.
        let slippage = U256::from(
            ((slippage_percent / 100f64) * 10f64.powi(hr.l_zero_th_dec.unwrap().into())) as u128,
        );

        // returns the value in fixed point value of the token.
        // if the buy_units is in FPV then divide by the power or decimal value.
        let slippage = slippage * starting_buy_units;

        // minimun token out units
        let amount_out_min: U256 = (starting_buy_units * power_of_zeroth_index) - slippage;

        // buy amount in fixed point value.
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
        let cheap_pool_oneth: H160 = hr.l_one_th_pair.unwrap();

        // buy this token(token_out)
        let token_out_1: H160 = hr.l_one_th_token.unwrap();

        // (token_in) send this token to the pool in exchange of token_out.
        let token_in_1: H160 = hr.l_zero_th_token.unwrap();

        // swap price in terms of the another token involved in the swap pool.
        let single_unit_price_1: U256 = hr.l_one_th_price_raw.unwrap();

        // checks the balance of token, and sends all for the next swap.
        let buy_units_back_run: U256 = ERC20::new(token_in_1, client.clone())
            .balance_of(wallet.address())
            .call()
            .await?;

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

                println!("current iteration zero_f_one: {}", counter);
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

                // the decimal percentage is multiplied by power_of_zeroth_index
                // which makes it possible to store as U256.
                let slippage_1 = U256::from(
                    ((slippage_percent / 100f64) * 10f64.powi(hr.l_one_th_dec.unwrap().into()))
                        as u128,
                );

                // here we divide by power_of_oneth_index to get the actual value back.
                // since amount_in is already in fixed point value.
                // if amount_in wasn't in FPV then no divide needed.
                let slippage_1 = (slippage_1 * expected_output_1) / power_of_oneth_index;

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
                    if counter.gt(&iter_limit) {
                        break 'outer;
                    }
                    println!("non profitable");
                    continue 'outer;
                } else {
                    println!("Building transaction post slippage check.\n");
                    let mut f = File::options().append(true).open("./logs.txt")?;
                    writeln!(&mut f, "after slippage profit:{}", {
                        amount_out_min_1 - amount_in
                    })?;
                }

                // conditional execution ahead checking dex.
                match hr.l_zero_th_dex == Some("uni_v2") {
                    true => {
                        println!("execution start from uni_v2, buying token underlying on zero'th index.");
                        let flash_loan_selector_str: &str = "0x5107d61e";
                        let flash_loan_selector = function_selector(flash_loan_selector_str);

                        let encoded_v2_swap = Bytes::new();

                        // flash loan encoding, with extra bytes data for swap exe.
                        let encoded_loan_with_bytes_to_swap = sepolia_test_contract
                            .encode_with_selector(
                                flash_loan_selector,
                                (token_in, amount_in, encoded_v2_swap),
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
                        // smart contract executions for swap.
                        let flash_loan_selector_str: &str = "0x5107d61e";
                        let flash_loan_selector = function_selector(flash_loan_selector_str);

                        // token swap encoding.
                        let one_for_zero_selector_str: &str = "0xa2eb605f";
                        let zero_for_one_selector_str: &str = "0x211760ee";

                        let one_for_zero_selector = function_selector(one_for_zero_selector_str);
                        let zero_for_one_selector = function_selector(zero_for_one_selector_str);

                        // one'th indexed token in zero'th indexed token out.
                        let encoded_one_for_zero_swap = sepolia_test_contract
                            .encode_with_selector(
                                one_for_zero_selector,
                                (
                                    cheap_pool_zeroth,
                                    sender,
                                    sqrt_x96,
                                    // amount_in for exact input.
                                    amount_in,
                                    // amount_out for exact output.
                                    U256::from(0),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // zero'th indexed token in one'th indexed token out.
                        let encoded_zero_for_one_swap = sepolia_test_contract
                            .encode_with_selector(
                                zero_for_one_selector,
                                (
                                    cheap_pool_oneth,
                                    sender,
                                    sqrt_x96,
                                    // amount_in for exact input.
                                    amount_in_1,
                                    // amount out for exact output.
                                    U256::from(0),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // token for testing on sepolia.
                        // let _test_token: Address =
                        //     "0x391e06B49B5483877DB943c0041C4aE6097Cd1B3".parse::<Address>()?;

                        // let execution_bytes_data = encode(&[
                        //     Token::Bytes(encoded_one_for_zero_swap.to_vec()),
                        //     Token::Bytes(encoded_zero_for_one_swap.to_vec()),
                        // ]);

                        let execution_bytes_data = encode(&[
                            Token::Bytes(encoded_one_for_zero_swap),
                            Token::Bytes(encoded_zero_for_one_swap),
                        ]);

                        println!("{:?}", execution_bytes_data);

                        // flash loan encoding, with extra bytes data for swap exe.
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
    wallet: LocalWallet,
    sender: Address,
    sepolia_test_contract: Contract<T>,
) -> Result<()> {
    let starting_buy_units: U256 = 10.into();
    let slippage_percent: f64 = 0.5;
    let mut counter = 0;

    let iter_limit: u32 = 999;
    // pair addresses.
    let pair_address = get_pair_address(token0, token1, fee, client).await?;

    'outer: loop {
        counter += 1;
        // fetch for current price and compare.
        let uni_v2_price = get_price_v2(pair_address.v2_pair, client).await?;
        let uni_v3_price = get_price_v3(pair_address.v3_pair, client).await?;
        let hr = return_low_high(uni_v2_price, uni_v3_price);

        println!(
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

        println!(
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
        // variable to buy token underlying on 1th index of the pool.
        // FIRST SWAP STARTS HERE.

        let power_of_zeroth_index = U256::from(10).pow(hr.l_zero_th_dec.unwrap().into());
        let power_of_oneth_index = U256::from(10).pow(hr.l_one_th_dec.unwrap().into());

        // First swap in this pool, buying cheaper token in the one'th index of the pool.
        // returns the pool where the token in one'th index is cheaper.
        let cheap_pool_oneth: H160 = hr.l_one_th_pair.unwrap();

        // buy this token, underlying in the one'th index.
        let token_out: H160 = hr.l_one_th_token.unwrap();

        // sell this token, underlying in the zero'th index.
        let token_in: H160 = hr.l_zero_th_token.unwrap();

        // swap price returns in terms of the other token involved in the swap pool.
        let single_unit_price: U256 = hr.l_one_th_price_raw.unwrap();

        // amount_in in fixed point value, adjusted to token's decimal we are selling.
        // only multiply by buying units, since price already is returned decimalized.
        let amount_in: U256 = single_unit_price * starting_buy_units;

        // the decimal percentage is multiplied by power_of_oneth_index
        // which makes it possible to store f64 as U256.
        let slippage = U256::from(
            ((slippage_percent / 100f64) * 10f64.powi(hr.l_one_th_dec.unwrap().into())) as u128,
        );

        // returns the value in fixed point value of the token.
        // if the buy_units is in FPV then divide by the power or decimal value.
        let slippage = slippage * starting_buy_units;

        // minimun token out units in fixed point value.
        let amount_out_min: U256 = (starting_buy_units * power_of_oneth_index) - slippage;

        // buy amount in fixed point value, adjusted to token's decimal we are buying.
        let buy_amount: U256 = starting_buy_units * power_of_oneth_index;

        let sqrt_x96 = hr.sqrt_x96_v3.unwrap();
        println!("{sqrt_x96}");

        println!(
            "Front run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
            cheap_pool_oneth, token_out, token_in, single_unit_price
        );
        println!(
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

        // sell this token.
        let token_in_1: H160 = hr.l_one_th_token.unwrap();

        // swap price in terms of the another token involved in the swap pool.
        let single_unit_price_1: U256 = hr.l_zero_th_price_raw.unwrap();

        // checks the balance of token, and sends all for the next swap.
        let buy_units_back_run = ERC20::new(token_in_1, client.clone())
            .balance_of(wallet.address())
            .call()
            .await?;

        // amount_in in fixed point value.
        // for mainnet check account balance and use as input.
        let amount_in_1: U256 = amount_out_min; // output amount of previous swap is the current input.

        // minimun token out units.
        // amount_out_min is already fixed point, multiply with power_of_zeroth_index for decimal handle
        // calculates the possible output dividing the Unit price by quantity.
        let expected_output_1 = (amount_out_min * power_of_zeroth_index) / single_unit_price_1;

        // initial check.
        match expected_output_1 > amount_in {
            false => {
                println!(
                    "non profitable:\nexpected_output: {} amount_spent: {} current_loss: {:?}",
                    expected_output_1,
                    amount_in,
                    (amount_in - expected_output_1)
                );
                println!("current iteration oneForZero: {counter}\n");
                if counter.gt(&iter_limit) {
                    break 'outer;
                }
                // break 'outer;
                continue 'outer;
            }

            true => {
                // log for testing purpose.
                let mut f = File::options().append(true).open("./logs.txt")?;
                writeln!(
                    &mut f,
                    "intial check profit B1S1:{}",
                    (expected_output_1 - amount_in)
                )?;

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
                let buy_amount_1: U256 = expected_output_1;

                println!(
                    "Back run tx: pool: {:#?} token_out: {:#?} token_in: {:#?} price: {:#?}",
                    cheap_pool_zeroth, token_out_1, token_in_1, single_unit_price_1
                );
                println!(
                    "buy_amount: {:#?} amount_in: {:#?} amount_out_min: {:#?}",
                    buy_amount_1, amount_in_1, amount_out_min_1
                );

                // second check after slippage.
                // second swap output has to be greater than the input of the first swap.
                if amount_out_min_1 < amount_in {
                    println!("non profitable after slippage");
                    println!("current iteration oneForZero after slippage: {counter}\n");
                    if counter.gt(&iter_limit) {
                        break 'outer;
                    }
                    continue 'outer;
                } else {
                    println!("Building transaction post slippage check B1S1.\n");
                    let mut f = File::options().append(true).open("./logs.txt")?;
                    writeln!(&mut f, "after slippage profit:{}", {
                        amount_out_min_1 - amount_in
                    })?;
                }

                // conditional execution ahead.
                match hr.l_zero_th_dex == Some("uni_v2") {
                    true => {
                        println!("execution start from uni_v2, buying token underlying on zero'th index.");
                        let flash_loan_selector_str: &str = "0x5107d61e";
                        let flash_loan_selector = function_selector(flash_loan_selector_str);

                        let encoded_v2_swap = Bytes::new();

                        // flash loan encoding, with extra bytes data for swap exe.
                        let encoded_loan_with_bytes_to_swap = sepolia_test_contract
                            .encode_with_selector(
                                flash_loan_selector,
                                (token_in, amount_in, encoded_v2_swap),
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

                        // let tx = signing_client
                        //     .get_transaction(receipt.transaction_hash)
                        //     .await?;
                        // println!("Sent tx: {}\n", serde_json::to_string(&tx)?);
                        println!("Tx receipt: {}", serde_json::to_string(&receipt)?);
                    }
                    false => {
                        println!("execution start from uni_v3, buying token underlying on zero'th index.");
                        // smart contract executions for swap.
                        let flash_loan_selector_str: &str = "0x5107d61e";
                        let flash_loan_selector = function_selector(flash_loan_selector_str);

                        // token swap encoding.
                        let one_for_zero_selector_str: &str = "0xa2eb605f";
                        let zero_for_one_selector_str: &str = "0x211760ee";

                        let one_for_zero_selector = function_selector(one_for_zero_selector_str);
                        let zero_for_one_selector = function_selector(zero_for_one_selector_str);

                        // one'th indexed token in zero'th indexed token out.
                        let encoded_one_for_zero_swap = sepolia_test_contract
                            .encode_with_selector(
                                one_for_zero_selector,
                                (
                                    cheap_pool_zeroth,
                                    sender,
                                    sqrt_x96,
                                    // amount_in for exact input.
                                    amount_in_1,
                                    // amount_out for exact output.
                                    U256::from(0),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // zero'th indexed token in one'th indexed token out.
                        let encoded_zero_for_one_swap = sepolia_test_contract
                            .encode_with_selector(
                                zero_for_one_selector,
                                (
                                    cheap_pool_oneth,
                                    sender,
                                    sqrt_x96,
                                    // amount_in for exact input.
                                    amount_in,
                                    // amount out for exact output.
                                    U256::from(0),
                                ),
                            )
                            .unwrap()
                            .to_vec();

                        // token for testing on sepolia.
                        // let _test_token: Address =
                        //     "0x391e06B49B5483877DB943c0041C4aE6097Cd1B3".parse::<Address>()?;

                        let execution_bytes_data = encode(&[
                            Token::Bytes(encoded_zero_for_one_swap),
                            Token::Bytes(encoded_one_for_zero_swap),
                        ]);

                        println!("{:?}", execution_bytes_data);

                        // flash loan encoding, with extra bytes data for swap exe.
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

fn function_selector(selector_str: &str) -> [u8; 4] {
    let four_bytes_selector_hex = u32::from_str_radix(&selector_str[2..], 16).unwrap();

    let flash_loan_selector: [u8; 4] = [
        (four_bytes_selector_hex >> 24) as u8,
        ((four_bytes_selector_hex >> 16) & 0xFF) as u8,
        ((four_bytes_selector_hex >> 8) & 0xFF) as u8,
        (four_bytes_selector_hex & 0xFF) as u8,
    ];
    flash_loan_selector
}
