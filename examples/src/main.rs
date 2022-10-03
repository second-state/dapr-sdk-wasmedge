use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;
    println!("1500 ms have elapsed");

    let client = dapr::Dapr::new(3503);

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
    /*
    let kvs = json!([
        {
          "key": "weapon",
          "value": "DeathStar"
        }
    ]);
    */
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
        ],
        "metadata": {
          "partitionKey": "planet"
        }
    });
    client.transact_state("starwars", ops).await?;
    println!("Transacted!");

    Ok(())
}
