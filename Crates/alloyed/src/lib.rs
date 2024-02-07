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

pub type NodeClient = std::sync::Arc<ethers::providers::Provider<ethers::providers::Http>>;
const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
