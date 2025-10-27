use cexplorer_api_rs::{
    init_api, inspect_address, get_address_detail,
    get_address_utxo, get_address_list, AddressListParams,
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

    // Test address (Cardano mainnet)
    let test_address = "addr1q8elqhkuvtyelgcedpup58r893awhg3l87a4rz5d5acatuj9y84nruafrmta2rewd5l46g8zxy4l49ly8kye79ddr3ksqal35g";

    // Test 1: Inspect address
    println!("\n--- Test inspect_address ---");
    match inspect_address(test_address).await {
        Ok(response) => {
            println!("✓ Address inspected successfully");
            println!("  Magic: {:?}", response.data.magic);
            println!("  Header: {:?}", response.data.header);
            println!("  Payment: {:?}", response.data.payment);
            println!("  Stake: {:?}", response.data.stake);
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Test 2: Get address detail
    println!("\n--- Test get_address_detail ---");
    match get_address_detail(test_address).await {
        Ok(response) => {
            println!("✓ Address details retrieved");
            println!("  Count: {}", response.data.count);
            if !response.data.data.is_empty() {
                let addr = &response.data.data[0];
                println!("  Balance: {} lovelace", addr.balance);
                println!("  Activity count: {}", addr.activity.count);
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Test 3: Get address UTXOs
    println!("\n--- Test get_address_utxo ---");
    match get_address_utxo(test_address).await {
        Ok(response) => {
            println!("✓ UTXOs retrieved");
            println!("  Count: {}", response.data.count);
            if !response.data.data.is_empty() {
                println!("  Total sum: {} lovelace", response.data.data[0].sum);
                println!("  UTXO sets: {}", response.data.data[0].utxo_set.len());
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Test 4: Get address list
    println!("\n--- Test get_address_list ---");
    let params = AddressListParams {
        payment_cred: None,
        view: Some(test_address.to_string()),
        order: Some("balance".to_string()),
        watchlist_only: None,
    };

    match get_address_list(params).await {
        Ok(response) => {
            println!("✓ Address list retrieved");
            println!("  Count: {}", response.data.count);
            if !response.data.data.is_empty() {
                let addr = &response.data.data[0];
                println!("  Address: {}", addr.address);
                if let Some(balance) = addr.balance {
                    println!("  Balance: {} lovelace", balance);
                }
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }
}
