use starknet::core::types::{BlockId, EventFilter};
use starknet::macros::felt;
use starknet::providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};
use url::Url;

use config::NODE;

mod config;

#[tokio::main]
async fn main() {
    let rpc_client = JsonRpcClient::new(HttpTransport::new(
        Url::parse(NODE)
            .unwrap(),
    ));

    let block_number = rpc_client.block_number().await.unwrap();
    println!("{block_number}");

    let event = rpc_client.get_events(EventFilter {
        from_block: Some(BlockId::Number(block_number - 10000)),
        to_block: Some(BlockId::Number(block_number)),
        address: Some(felt!("0x078fcf70e22f475b8ffde567f8118e5d99ded383da150e01e55fa79251c7c808")),
        keys: None,
    }, None, 20).await.unwrap();

    dbg!(event);
}
