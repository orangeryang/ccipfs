use std::collections::HashMap;

use starknet::macros::felt;
use starknet::providers::{Provider, SequencerGatewayProvider};

mod config;

#[tokio::main]
async fn main() {
    //
    // let provider = JsonRpcClient::new(HttpTransport::new(
    //     url::Url::parse(config::NODE)
    //         .unwrap(),
    // ));
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let url = "https://alpha4.starknet.io/feeder_gateway/call_contract?blockNumber=pending";
    
    let contract_address = felt!("0x078fcf70e22f475b8ffde567f8118e5d99ded383da150e01e55fa79251c7c808");
    
    // let block_number =
    //     match rpc_client.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await.unwrap() {
    //         MaybePendingBlockWithTxHashes::Block(value) => value.block_number,
    //         MaybePendingBlockWithTxHashes::PendingBlock(_v) => 0
    //     };
    // if block_number == 0 {
    //     println!("Current block number is pending");
    // } else {
    //     println!("Current block number: {block_number}");
    // }
    
    // let events = provider.get_events(EventFilter {
    //     from_block: Some(BlockId::Tag(BlockTag::Latest)),
    //     to_block: Some(BlockId::Tag(BlockTag::Latest)),
    //     address: Some(contract_address),
    //     keys: Some(vec![vec![FieldElement::from_str("0x010bf1371d2e8135b470f00faa9f2c8b17d141530da60511c52125e2228908e5").unwrap()]]),
    // }, None, 20).await.unwrap();
    // dbg!(&events);
    //
    // let token_id: u16 = FieldElement::try_into(events.events[0].data[0]).expect("overflow");
    // println!("token_id: {}", token_id);
    //
    
    // let result = provider.call(FunctionCall {
    //     contract_address,
    //     entry_point_selector: selector!("token_uri"),
    //     calldata: vec![felt!("1")],
    // }, BlockId::Tag(BlockTag::Latest)).await.expect("failed to call contract");
    //
    // dbg!(result);
    
    let mut body = HashMap::new();
    body.insert("contract_address", "0x078fcf70e22f475b8ffde567f8118e5d99ded383da150e01e55fa79251c7c808");
    body.insert("entry_point_selector", "0x226ad7e84c1fe08eb4c525ed93cccadf9517670341304571e66f7c4f95cbe54");
    // body.insert("calldata", "[\"1\",\"0\"]");
    dbg!(&body);
    
    // let mut head = HeaderMap::new();
    // head.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    // head.insert(AUTHORIZATION, "alpha4.starknet.io".parse().unwrap());
    // head.insert(ORIGIN, "https://testnet.starkscan.co".parse().unwrap());
    // head.insert(REFERER, "https://testnet.starkscan.co/".parse().unwrap());
    // head.insert(DNT, "1".parse().unwrap());
    // head.insert(ACCEPT, "/".parse().unwrap());
    // head.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36".parse().unwrap());
    // head.insert(ACCEPT_ENCODING,"gzip, deflate, br".parse().unwrap());
    // head.insert(ACCEPT_LANGUAGE,"en,zh-CN;q=0.9,zh;q=0.8".parse().unwrap());
    // dbg!(&head);
    
    // let ccc = reqwest::Client::new();
    // let params = [("blockNumber", "pending")];
    // dbg!(ccc.post(url).form(&params).json(&body));
    
    let response = reqwest::Client::new().post(url).json(&body).send().await.unwrap();
    dbg!(response);
    
    // response.json().await.unwrap();
    
    //
}
