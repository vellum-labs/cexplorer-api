use cexplorer_api_rs::{
    init_api, get_epoch_list, get_epoch_detail_param, get_epoch_detail_stats,
};

#[tokio::main]
async fn main() {
    // Initialize API
    match init_api("mainnet-stage", "your-api-key-here") {
        Ok(_) => println!("✓ API initialized"),
        Err(e) => {
            eprintln!("✗ Initialization error: {}", e);
            return;
        }
    }

    // Test 1: Get epoch list
    println!("\n--- Test get_epoch_list ---");
    match get_epoch_list().await {
        Ok(response) => {
            println!("✓ Epoch list retrieved");
            println!("  Count: {:?}", response.data.count);
            if !response.data.data.is_empty() {
                let first_epoch = &response.data.data[0];
                println!("  First epoch: #{:?}", first_epoch.no);
                println!("  Start time: {:?}", first_epoch.start_time);
                println!("  Block count: {:?}", first_epoch.blk_count);
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Test 2: Get epoch parameters
    println!("\n--- Test get_epoch_detail_param ---");
    let epoch_no = 500;
    match get_epoch_detail_param(epoch_no).await {
        Ok(response) => {
            println!("✓ Epoch {:?} parameters retrieved", epoch_no);
            println!("  Protocol major: {:?}", response.data.protocol_major);
            println!("  Protocol minor: {:?}", response.data.protocol_minor);
            println!("  Max block size: {:?}", response.data.max_block_size);
            println!("  Min fee a: {:?}", response.data.min_fee_a);
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Test 3: Get epoch stats
    println!("\n--- Test get_epoch_detail_stats ---");
    match get_epoch_detail_stats(epoch_no).await {
        Ok(response) => {
            println!("✓ Epoch {:?} stats retrieved", epoch_no);
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }
}