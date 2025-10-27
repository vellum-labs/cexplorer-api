use cexplorer_api_rs::{init_api, get_block_detail, get_block_list, BlockListParams};

#[tokio::main]
async fn main() {
    // Api init
    match init_api("mainnet-stage", "your-api-key-here") {
        Ok(_) => println!("Api initialized"),
        Err(e) => {
            eprintln!("Initialization error: {}", e);
            return;
        }
    }

    println!("\n--- Test get_block_list ---");
    let params = BlockListParams {
        limit: Some(5),
        offset: Some(0),
        pool_id: None,
        epoch_no: None,
        hash: None,
        slot_no: None,
        block_no: None,
    };

    match get_block_list(params).await {
        Ok(response) => {
          println!("✓ Got blocks: {}", response.data.count);
          if !response.data.data.is_empty() {
              println!("  First block: #{}", response.data.data[0].block.block_no);
          }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    println!("\n--- Test get_block_detail ---");
    let test_hash = "a951eee85659818a54a34a66e9e53e6c658a28a5156df1b6702c44128dc1d15b";

    match get_block_detail(test_hash).await {
      Ok(response) => {
          println!("✓ Block: #{}", response.data.block_no);
          println!("  Epoch: {}", response.data.epoch_no);
          println!("  Txs: {}", response.data.tx_count);
          println!("  Hash: {}", response.data.hash);
      }
      Err(e) => eprintln!("✗ Error: {}", e),
    }
}

