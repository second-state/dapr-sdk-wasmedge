use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Please POST to /echo",
        ))),

        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        (&Method::GET, "/dapr/subscribe") => Ok(Response::new(Body::from(json!([
                                                                                        {
                                                                                             "pubsubname":"pubsub",
                                                                                             "topic":"A",
                                                                                             "route":"A",
                                                                                        },
                                                                                        {
                                                                                             "pubsubname":"pubsub",
                                                                                             "topic":"B",
                                                                                             "route":"B",
                                                                                        }

        ]).to_string()))),
        (&Method::POST, "/A") => {
            println!("Received from A {}",String::from_utf8(hyper::body::to_bytes(req.into_body()).await?.to_vec()).unwrap());
            Ok(Response::default())
        },
        (&Method::POST, "/B") =>  {
            println!("Received from B {}",String::from_utf8(hyper::body::to_bytes(req.into_body()).await?.to_vec()).unwrap());
            Ok(Response::default())
        },

        (&Method::GET, "/dapr/config") => Ok(Response::new(Body::from(
            json!({
                "entities":["stormtrooper"],
                "actorIdleTimeout": "1h",
                "actorScanInterval": "30s",
                "drainOngoingCallTimeout": "30s",
                "drainRebalancedActors": true,
                "reentrancy": {
                  "enabled": true,
                  "maxStackDepth": 32
                },
                "entitiesConfig": [
                    {
                        "entities": ["stormtrooper"],
                        "actorIdleTimeout": "10m",
                        "drainOngoingCallTimeout": "10s",
                        "reentrancy": {
                            "enabled": false
                        }
                    }
                ]
              }).to_string()
        ))),

        (&Method::DELETE, "/actors/stormtrooper/50") =>  {
            println!("Delete req for stormtrooper/50");
            Ok(Response::default())
        },

        (&Method::PUT, "/actors/stormtrooper/50/method/performAction") =>  {
            println!("Perform Action method invoked");
            Ok(Response::default())
        },

        (&Method::PUT, "/actors/stormtrooper/50/method/remind/checkRebels") =>  {
            println!("Reminder check rebels invoked");
            Ok(Response::default())
        },
        
        (&Method::PUT, "/actors/stormtrooper/50/method/timer/checkRebels") =>  {
            println!("Timer check rebels invoked");
            Ok(Response::default())
        },

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9004));
    let make_svc = make_service_fn(|_| async move {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req)))
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
