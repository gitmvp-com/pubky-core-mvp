//! Basic usage example for Pubky MVP
//!
//! This example demonstrates:
//! 1. Creating a keypair
//! 2. Storing data via HTTP
//! 3. Retrieving data
//! 4. Listing keys
//! 5. Deleting data

use pubky_common::Keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new keypair
    let keypair = Keypair::random();
    let public_key = keypair.public_key();

    println!("=== Pubky MVP Example ===");
    println!("\nGenerated keypair:");
    println!("Public Key: {}", public_key);
    println!("Public Key (z32): {}", public_key.to_z32());

    let base_url = "http://127.0.0.1:3000";
    let client = reqwest::Client::new();

    println!("\n=== Testing Storage Operations ===");

    // 1. PUT - Store some data
    let path = "my-app/hello.txt";
    let data = "Hello, Pubky MVP!";
    let put_url = format!("{}/{}/{}", base_url, public_key.to_z32(), path);

    println!("\n1. PUT {}", put_url);
    let response = client.put(&put_url).body(data).send().await?;

    if response.status().is_success() {
        println!("   ✓ Data stored successfully");
    } else {
        println!("   ✗ Failed to store data: {}", response.status());
        return Ok(());
    }

    // 2. GET - Retrieve the data
    let get_url = format!("{}/{}/{}", base_url, public_key.to_z32(), path);
    println!("\n2. GET {}", get_url);
    let response = client.get(&get_url).send().await?;

    if response.status().is_success() {
        let retrieved = response.text().await?;
        println!("   ✓ Retrieved: {}", retrieved);
        assert_eq!(retrieved, data);
    } else {
        println!("   ✗ Failed to retrieve data: {}", response.status());
    }

    // 3. PUT - Store more data
    println!("\n3. Storing additional files...");
    let files = vec![
        ("my-app/data1.txt", "Content 1"),
        ("my-app/data2.txt", "Content 2"),
        ("other/data3.txt", "Content 3"),
    ];

    for (file_path, content) in &files {
        let url = format!("{}/{}/{}", base_url, public_key.to_z32(), file_path);
        client.put(&url).body(*content).send().await?;
        println!("   ✓ Stored: {}", file_path);
    }

    // 4. LIST - List all files under my-app/
    let list_url = format!("{}/{}/my-app/", base_url, public_key.to_z32());
    println!("\n4. LIST {}", list_url);
    let response = client.get(&list_url).send().await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("   ✓ Found {} files:", json["count"]);
        if let Some(keys) = json["keys"].as_array() {
            for key in keys {
                println!("     - {}", key.as_str().unwrap());
            }
        }
    }

    // 5. DELETE - Delete a file
    let delete_url = format!("{}/{}/{}", base_url, public_key.to_z32(), path);
    println!("\n5. DELETE {}", delete_url);
    let response = client.delete(&delete_url).send().await?;

    if response.status().is_success() {
        println!("   ✓ File deleted successfully");
    } else {
        println!("   ✗ Failed to delete: {}", response.status());
    }

    // 6. Verify deletion
    println!("\n6. Verifying deletion...");
    let response = client.get(&get_url).send().await?;
    if response.status() == 404 {
        println!("   ✓ File no longer exists");
    } else {
        println!("   ✗ File still exists!");
    }

    println!("\n=== Example Complete ===");
    println!("\nNote: Make sure the server is running with: cargo run --bin server");

    Ok(())
}
