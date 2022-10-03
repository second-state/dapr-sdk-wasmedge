use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = dapr::Dapr::new();

    let kvs = json!([
        {
          "key": "weapon",
          "value": "DeathStar",
          "etag": "1234"
        },
        {
          "key": "planet",
          "value": {
            "name": "Tatooine"
          }
        }
    ]);
    client.save_state("starwars", kvs).await?;

    let val = client.get_state("starwars", "weapon").await?;
    println!("State for weapon: {}", val);

    let keys = vec!["weapon", "planet"]
        .into_iter().map(|s| s.to_owned()).collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon and planet: {}", val);

    client.delete_state("starwars", "weapon").await?;

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

    Ok(())
}
