use cexplorer_api_rs::{init_api, get_pools_list, get_pool_detail};

#[tokio::main]
async fn main() {
    init_api("mainnet-stage", "your-api-key-here");

    println!("--- Test get_pools_list ---");
    match get_pools_list(
        Some(10),
        Some(20),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    {
        Ok(response) => {
            println!("Success! Count: {}", response.data.count);
            println!("First pool: {:?}", response.data.data.first());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n--- Test get_pool_detail ---");
    match get_pool_detail(
        Some("pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"),
        None,
    )
    .await
    {
        Ok(response) => {
            println!("Success! Pool: {}", response.data.pool_id);
            println!("Delegators: {:?}", response.data.delegators);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
