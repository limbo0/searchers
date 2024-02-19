use ethers::{abi::Abi, contract::Contract, types::H160};

use crate::NodeClient;

use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::Path,
    sync::Arc,
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    message: String,
    result: String,
}

/// checks if the given address has a json file locally, if not call etherscan api to fetch abi.
/// returns a contract instance, then calling methods is possible.
pub async fn create_contract_instance_for_any_address(
    any_address: String,
    dex_name: &str,
    client: NodeClient,
) -> Result<Contract<NodeClient>> {
    let mut str_buffer = String::new();

    let mut abi_file = get_abi_from_etherscan(&any_address, dex_name).await?;

    abi_file.read_to_string(&mut str_buffer).unwrap();

    Ok(Contract::new(
        any_address.parse::<H160>()?,
        Abi::from(serde_json::from_str(str_buffer.as_str()).unwrap()),
        Arc::new(client.clone()),
    ))
}

// TODO: fetching abi for proxy contracts.
/// call etherscan api if json abi isn't available locally, writes a json abi file on local specified path.
pub async fn get_abi_from_etherscan(contract_address: &str, dex_name: &str) -> Result<File> {
    let file_parent = format!("{}{}{}", "./Crates/alloyed/abi/", dex_name, "/");
    let file_stem_ext = format!("{}{}", contract_address, ".json");
    let file_parent = file_parent.as_str();
    let file_stem_ext = file_stem_ext.as_str();

    // first checks if the directory exists, if not creates one.
    fs::read_dir(Path::new(file_parent)).unwrap_or_else(|_| {
        fs::create_dir_all(Path::new(file_parent)).unwrap();
        fs::read_dir(Path::new(Path::new(file_parent))).unwrap()
    });

    let file_path = format!("{}{}", file_parent, file_stem_ext);
    let file_path = Path::new(&file_path);

    let file_path: &Path = Path::new(&file_path);

    let if_file_exists = fs::metadata(file_path).is_ok();

    if !if_file_exists {
        eprint!(
            "{}'s abi dosen't exist, fetching from etherscan!\n",
            contract_address
        );
        let etherscan_token = env::var("ETHERSCAN_TOKEN").unwrap();
        let target = "https://api.etherscan.io/api?module=contract&action=getabi&address=";
        let api = "&apikey=";
        let appi = etherscan_token.as_str();

        // endpoint to fetch abi from.
        let endpoint = format!("{}{}{}{}", target, contract_address, api, appi);
        let endpoint = endpoint.as_str();

        let result = handle_etherscan_api_calls(endpoint, file_path).await?;

        //TODO: Error handling while more than five calls are made under 1 second.
        if result == "exceed limit" {
            tokio::time::sleep(Duration::from_secs(10)).await;

            fs::write(
                file_path,
                handle_etherscan_api_calls(endpoint, file_path).await?,
            )
            .unwrap();

            Ok(fs::OpenOptions::new().read(true).open(file_path).unwrap())
        } else {
            // over write on the same file path, with only the abi of the contract.
            fs::write(file_path, result).unwrap();

            Ok(fs::OpenOptions::new().read(true).open(file_path).unwrap())
        }
    } else {
        // eprint!(
        //     "{}'s abi exist, fetching from local file!\n",
        //     contract_address
        // );
        Ok(fs::OpenOptions::new().read(true).open(file_path).unwrap())
    }
}

pub async fn handle_etherscan_api_calls(endpoint: &str, file_path: &Path) -> Result<String> {
    let response = reqwest::get(endpoint).await?.text().await?;

    // writes the entire response to this file path.
    fs::write(file_path, response).unwrap();
    // open the previously written file.
    let mut written_file = File::open(file_path).unwrap();

    // create a temp buffer, and read the conternts on the buffer.
    let mut read_buffer = String::new();
    written_file.read_to_string(&mut read_buffer).unwrap();

    // parse the file into our custon response struct.
    let parsed_response: ApiResponse = serde_json::from_str(&read_buffer).unwrap();

    // we only need the result, which has the abi for the contract.
    // it may also contain error response while exceeding the api limit.
    let result = parsed_response.result;

    Ok(result)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn check_file_path() {}
}
