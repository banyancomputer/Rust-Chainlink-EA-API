// new addr 0xeb3d5882faC966079dcdB909dE9769160a0a00Ac
#[macro_use]
extern crate rocket;
extern crate rust_chainlink_ea_api;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use eyre::Result;
use rocket::serde::{json, Deserialize, Serialize};
use rust_chainlink_ea_api::validate;
//use validate::get_deal_info;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Middleware, Provider},
    types::{Address, H256, U256},
};
//use banyan_shared::types::*;
use std::fs;

use rust_chainlink_ea_api::types;
//use banyan_shared::{types::*, proofs};
use dotenv::dotenv;
use std::str::FromStr;

pub async fn deploy_helper() -> Result<(), anyhow::Error> {
    println!("running deploy helper");
    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/1a39a4b49b9f4b8ba1338cd2064fe8fe")
            .expect("could not instantiate HTTP Provider");
    let address = "0xeb3d5882faC966079dcdB909dE9769160a0a00Ac".parse::<Address>()?; // old addr
    let abi: Abi = serde_json::from_str(
        fs::read_to_string("contract_abi.json")
            .expect("can't read file")
            .as_str(),
    )?;
    let contract = Contract::new(address, abi, provider);
    let deal_id = 55378008;
    let deal_start_block = 2;
    let deal_length_in_blocks = 3;
    let proof_frequency_in_blocks = 4;
    let price = 5;
    let collateral = 6;
    let erc20_token_denomination = "0xf679d8d8a90f66b4d8d9bf4f2697d53279f42bea"; // addr
    let ipfs_file_cid = "Qmd63gzHfXCsJepsdTLd4cqigFa7SuCAeH6smsVoHovdbE";
    let file_size = 941366;
    let blake3_checksum = "c1ae1d61257675c1e1740c2061dabfed"; // ed7575eb27aea8aa4eca88b7d69bd64f";
    let value = json::json!({"offerId": deal_id, 
                                    "deal_start_block": deal_start_block, 
                                    "deal_length_in_blocks": deal_length_in_blocks, 
                                    "proof_frequency_in_blocks": proof_frequency_in_blocks, 
                                    "price": price, 
                                    "collateral": collateral, 
                                    "erc20_token_denomination": erc20_token_denomination, 
                                    "ipfs_file_cid": ipfs_file_cid.as_bytes(), 
                                    "file_size": file_size, 
                                    "blake3_checksum": &blake3_checksum.as_bytes()});
    let deal: types::OnChainDealInfo =  json::from_value(value)?; 
    println!("here2");
    let call = contract.method::<_, H256>("createOffer", deal)?;
    println!("here3");
    let pending_tx = call.send().await?;
    println!("here4");
    let receipt = pending_tx.confirmations(6).await?;
    println!("{:?}", receipt);
    Ok(())
}

pub async fn get_block(offer_id: u64, window_num: u64) -> Result<u64, anyhow::Error> {
    //let api_token = std::env::var("API_KEY").expect("API_KEY must be set.");
    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/1a39a4b49b9f4b8ba1338cd2064fe8fe")
            .expect("could not instantiate HTTP Provider");
    let address = "0xeb3d5882faC966079dcdB909dE9769160a0a00Ac".parse::<Address>()?;
    let abi: Abi = serde_json::from_str(
        fs::read_to_string("contract_abi.json")
            .expect("can't read file")
            .as_str(),
    )?;
    println!("offer_id: {}", offer_id);
    println!("window_num: {}", window_num);

    let contract = Contract::new(address, abi, provider);

    let block: u64 = contract
        .method::<_, U256>("getProofBlock", (offer_id, window_num))?
        .call()
        .await?
        .as_u64();

    return Ok(block);
}

// proof helper 
pub async fn proof_helper() -> Result<(), anyhow::Error> {
    println!("running proof helper");
    let provider =
        Provider::<Http>::try_from("https://goerli.infura.io/v3/1a39a4b49b9f4b8ba1338cd2064fe8fe")
            .expect("could not instantiate HTTP Provider");
    let address = "0xeb3d5882faC966079dcdB909dE9769160a0a00Ac".parse::<Address>()?; // old addr
    let abi: Abi = serde_json::from_str(
        fs::read_to_string("contract_abi.json")
            .expect("can't read file")
            .as_str(),
    )?;
    let contract = Contract::new(address, abi, provider);
    let deal_id :u64= 0;
    let window_num :u64 = 0;

    // read local bao_slice_bad.txt 
    //let mut file = std::fs::read("hardhat_test/bao_slice_bad.txt")?;
    // make file a random vector of bytes
    
    let file = vec![0; 1000];
    let call = contract.method::<_, H256>("save_proof", (file, deal_id, window_num))?;
    let pending_tx = call.send().await?;
    let receipt = pending_tx.confirmations(6).await?;
    println!("{:?}", receipt);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //let block = get_block(0, 0).await?;
    //println!("no chance {:}", block);
    //let _dh = deploy_helper().await?;
    let _ph = proof_helper().await?;
    Ok(())
}
