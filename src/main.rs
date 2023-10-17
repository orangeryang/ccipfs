use std::str::FromStr;
use std::vec;

use starknet::core::types::{BlockId, BlockTag, EventFilter, FieldElement, FunctionCall};
use starknet::macros::{felt, selector};
use starknet::providers::{Provider, SequencerGatewayProvider};

// mod config;

#[tokio::main]
async fn main() {
    //
    // let rpc_client = JsonRpcClient::new(HttpTransport::new(
    //     Url::parse(NODE)
    //         .unwrap(),
    // ));
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    
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
    
    let events = provider.get_events(EventFilter {
        from_block: Some(BlockId::Tag(BlockTag::Latest)),
        to_block: Some(BlockId::Tag(BlockTag::Latest)),
        address: Some(contract_address),
        keys: Some(vec![vec![FieldElement::from_str("0x010bf1371d2e8135b470f00faa9f2c8b17d141530da60511c52125e2228908e5").unwrap()]]),
    }, None, 20).await.unwrap();
    dbg!(&events);
    
    let token_id: u16 = FieldElement::try_into(events.events[0].data[0]).expect("overflow");
    println!("token_id: {}", token_id);
    
    provider.call(FunctionCall {
        contract_address,
        entry_point_selector: selector!("token_uri"),
        calldata: vec![],
    }, BlockId::Tag(BlockTag::Latest)).await.expect("failed to call contract");
    
    
    //
}
