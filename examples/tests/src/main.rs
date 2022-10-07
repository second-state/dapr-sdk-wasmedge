use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;
    println!("1500 ms have elapsed");

    // This is the echo sidecar
    let client = dapr::Dapr::new(3502);
    client.is_healthy().await?;
    println!("Dapr echo is healthy!");

    let kvs = json!({ "message": "WasmEdge" });
    let val = client.invoke_service("echo-service", "echo", kvs).await?;
    println!("Echo: {}", val);

    // let client = dapr::Dapr::new(3503);

    let kvs = json!([
        {
          "key": "weapon",
          "value": "DeathStar"
        },
        {
          "key": "planet",
          "value": {
            "name": "Tatooine"
          }
        }
    ]);
    client.save_state("starwars", kvs).await?;
    println!("Saved!");

    let val = client.get_state("starwars", "weapon").await?;
    println!("State for weapon: {}", val);

    let keys = vec!["weapon", "planet"]
        .into_iter().map(|s| s.to_owned()).collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon and planet: {}", val);

    client.delete_state("starwars", "weapon").await?;
    println!("Deleted!");
    
    let keys = vec!["weapon", "planet"]
        .into_iter().map(|s| s.to_owned()).collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon and planet: {}", val);

    let ops = json!({
        "operations": [
          {
            "operation": "upsert",
            "request": {
              "key": "key1",
              "value": "myData"
            }
          },
          {
            "operation": "upsert",
            "request": {
              "key": "key2",
              "value": "yourData"
            }
          },
          {
            "operation": "delete",
            "request": {
              "key": "key2"
            }
          }
        ]
    });
    client.transact_state("starwars", ops).await?;
    println!("Transacted!");

    let keys = vec!["weapon", "planet", "key1", "key2"]
        .into_iter().map(|s| s.to_owned()).collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon, planet, key1 and key2: {}", val);

    let val = client.get_secret("local-store", "DB_URL:MYSQL").await?;
    println!("Secret for DB_URL:MYSQL {}", val);

    Ok(())
}
