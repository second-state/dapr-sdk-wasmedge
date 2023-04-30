use serde_json::json;
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = match env::var_os("EXAMPLE_DAPR_URL") {
        Some(v) => v.into_string().unwrap(),
        None => "http://localhost:3502".to_string(),
    };

    println!("App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;
    println!("1500 ms have elapsed");

    // This is the echo sidecar
    let client = dapr::Dapr::new_with_url(url);
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
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon and planet: {}", val);

    client.delete_state("starwars", "weapon").await?;
    println!("Deleted!");

    let keys = vec!["weapon", "planet"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
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
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let val = client.get_bulk_state("starwars", keys).await?;
    println!("State for weapon, planet, key1 and key2: {}", val);

    let val = client.get_secret("local-store", "DB_URL:MYSQL").await?;
    println!("Secret for DB_URL:MYSQL {}", val);

    let kvs = json!([
        {
          "key": "value"
        },
    ]);

    client.publish("pubsub", "A", kvs).await?;
    println!("Published to pubsubname: pubsub topic: A");

    let kvs = json!([
        {
          "key2": "val2"
        },
    ]);

    client.publish("pubsub", "B", kvs).await?;
    println!("Published to pubsubname: pubsub topic: B");

    let val = client
        .actor_invoke_method("stormtrooper", "50", "performAction")
        .await?;
    println!("Actor invoke method response: {}", val);

    let val = client
        .actor_state(
            "stormtrooper",
            "50",
            json!([
             {
               "operation": "upsert",
               "request": {
                 "key": "key1",
                 "value": "myData"
               }
             },
             {
               "operation": "delete",
               "request": {
                 "key": "key2"
               }
             }
            ]),
        )
        .await?;
    println!("Actor state response: {}", val);

    let val = client
        .actor_state_key("stormtrooper", "50", "location")
        .await?;
    println!("Actor state key response: {}", val);

    let val = client
        .actor_create_reminder(
            "stormtrooper",
            "50",
            "checkRebels",
            json!({
              "data": "someData",
              "dueTime": "1m",
              "period": "20s"
            }),
        )
        .await?;
    println!("Actor create reminder response: {}", val);

    let val = client
        .actor_get_reminder("stormtrooper", "50", "checkRebels")
        .await?;
    println!("Actor get reminder response: {}", val);

    client
        .actor_delete_reminder("stormtrooper", "50", "checkRebels")
        .await?;

    let val = client
        .actor_create_timer(
            "stormtrooper",
            "50",
            "checkRebels",
            json!({
              "data": "someData",
              "dueTime": "1m",
              "period": "20s",
              "callback": "myEventHandler"
            }),
        )
        .await?;
    println!("Actor create reminder response: {}", val);

    client
        .actor_delete_timer("stormtrooper", "50", "checkRebels")
        .await?;

    Ok(())
}
