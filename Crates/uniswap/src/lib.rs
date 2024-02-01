use ethers::{
    abi::{encode, short_signature, ParamType, Token},
    providers::{Http, Middleware, Provider, Ws},
    types::{Address, Bytes, Filter, Log, Transaction, U256, U64},
    utils::hex,
};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use eyre;
use std::sync::Arc;

pub mod tokio_thread;
pub fn function_selector(selector_str: &str) -> [u8; 4] {
    let four_bytes_selector_hex = u32::from_str_radix(&selector_str[2..], 16).unwrap();

    let flash_loan_selector: [u8; 4] = [
        (four_bytes_selector_hex >> 24) as u8,
        ((four_bytes_selector_hex >> 16) & 0xFF) as u8,
        ((four_bytes_selector_hex >> 8) & 0xFF) as u8,
        (four_bytes_selector_hex & 0xFF) as u8,
    ];
    flash_loan_selector
}

#[derive(Debug)]
pub struct SwapParam {
    pool_address: Address,
    recipient: Address,
    sqrt_x96: U256,
    amount0_in: U256,
    amount0_out: U256,
    amount1_in: U256,
    amount1_out: U256,
    amount_out_min: U256,
    zero_fur_one: bool,
    data: Bytes,
}

#[allow(clippy::too_many_arguments)]
impl SwapParam {
    /// Creates a new [`SwapParam`].
    pub fn new(
        pool_address: Address,
        recipient: Address,
        sqrt_x96: U256,
        amount0_in: U256,
        amount0_out: U256,
        amount1_in: U256,
        amount1_out: U256,
        amount_out_min: U256,
        zero_fur_one: bool,
        data: Bytes,
    ) -> Self {
        Self {
            pool_address,
            recipient,
            sqrt_x96,
            amount0_in,
            amount0_out,
            amount1_in,
            amount1_out,
            amount_out_min,
            zero_fur_one,
            data,
        }
    }
}

pub fn custom_standard_encode(selector: [u8; 4], sp: SwapParam) -> Vec<u8> {
    encode(&[
        Token::FixedBytes(selector.to_vec()),
        Token::Tuple(vec![
            // pool address
            Token::Address(sp.pool_address),
            // receiver
            Token::Address(sp.recipient),
            // sqrt_x96
            Token::Uint(sp.sqrt_x96),
            // amount_0_in
            Token::Uint(sp.amount0_in),
            // amount_0_out
            Token::Uint(sp.amount0_out),
            // amount_1_in
            Token::Uint(sp.amount1_in),
            // amount_1_out
            Token::Uint(sp.amount1_out),
            // amount out min
            Token::Uint(sp.amount_out_min),
            // zeroIn oneOut ?
            Token::Bool(sp.zero_fur_one),
            // bytes call data
            Token::Bytes(sp.data.to_vec()),
        ]),
    ])
}

pub fn swap_param_struct_selector() -> [u8; 4] {
    short_signature(
        "v2Swap",
        &[ParamType::Tuple(vec![
            ParamType::Address,
            ParamType::Address,
            ParamType::Uint(160),
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Uint(256),
            ParamType::Bool,
            ParamType::Bytes,
        ])],
    )
}

pub fn encod3() -> Vec<u8> {
    encode(&[
        Token::FixedBytes(short_signature("withdrawToken", &[ParamType::Address]).to_vec()),
        Token::Address(
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                .parse::<Address>()
                .unwrap(),
        ),
    ])
}

#[derive(Default, Debug, Clone)]
pub struct NewBlock {
    pub block_number: U64,
    pub base_fee: U256,
    pub next_base_fee: U256,
}

#[derive(Debug, Clone)]
pub enum Event {
    Block(NewBlock),
    PendingTx(Transaction),
    Log(Log),
}

pub async fn stream_new_blocks(provider: Arc<Provider<Ws>>, event_sender: Sender<Event>) {
    let stream = provider.subscribe_blocks().await.unwrap();
    let mut stream = stream.filter_map(|block| match block.number {
        Some(number) => Some(NewBlock {
            block_number: number,
            base_fee: block.base_fee_per_gas.unwrap_or_default(),
            next_base_fee: U256::from(0),
        }),
        None => None,
    });
    while let Some(block) = stream.next().await {
        match event_sender.send(Event::Block(block)) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
