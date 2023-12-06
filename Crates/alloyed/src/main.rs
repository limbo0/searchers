use alloy_primitives::{address, Address, FixedBytes, I256, U160, U256};

use alloy_sol_types::{sol, SolCall, SolStruct, SolValue};

// const MAINNET_CHAIN_ID: u64 = 1;
//
// const POOL: &str = "0x60594a405d53811d3bc4766596efd80fd545a270";
sol! {
    struct SwapParams{
        address pool;
        address recipient;
        uint160 sqrtPriceLimitX96;
        uint256 amount0In;
        uint256 amount0Out;
        uint256 amount1In;
        uint256 amount1Out;
        bool zeroFurOne;
        bytes data;
    }

    function zeroForOne(SwapParams memory sp);

}

const POOL_ADDRESS: Address = address!("60594a405d53811d3bc4766596efd80fd545a270");
const ALICE: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

fn main() {
    // let mut buf = [0; 42];
    // let checksummed: &str = POOL_ADDRESS.to_checksum_raw(&mut buf, None);

    let sp = SwapParams {
        pool: POOL_ADDRESS,
        recipient: ALICE,
        sqrtPriceLimitX96: "1677300184619083287613059028".parse::<U256>().unwrap(),
        amount0In: "44819026804412600".parse::<U256>().unwrap(),
        amount0Out: "0".parse::<U256>().unwrap(),
        amount1Out: "99500000000000000000".parse::<U256>().unwrap(),
        amount1In: "0".parse::<U256>().unwrap(),
        zeroFurOne: true,
        data: [0; 32].into(),
    };
    //
    let _ = sp.abi_encode();
    // `SolCall`
    let my_function_call = zeroForOneCall { sp };
    let check_encode = my_function_call.abi_encode();
    println!("{check_encode:?}");
}
