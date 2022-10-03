use std::collections::HashMap;
use serde_json::Value;
use reqwest::Error;

pub struct Dapr {
    pub url_base: String,
}

impl Dapr {
    pub fn new(port: u32) -> Dapr {
        Dapr {
            url_base: "http://localhost:".to_string() + &(port.to_string()) + "/v1.0/",
        }
    }
}

impl Dapr {
    pub async fn save_state (&self, store_name:&str, kvs:Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name +"?metadata.contentType=application/json";
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.post(&url)
            .json(&kvs)
            .send()
            .await?;

        println!("Status code is {}", res.status().as_str());
        Ok(())
    }

    pub async fn get_state (&self, store_name:&str, key:&str) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/" + key + "?metadata.contentType=application/json";
        let json = reqwest::get(&url)
            .await?
            .json()
            .await?;
        Ok(json)
    }

    pub async fn get_bulk_state (&self, store_name:&str, keys:Vec<String>) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "state/" + store_name;

        let mut data = HashMap::new();
        data.insert("keys", keys);

        let client = reqwest::Client::new();
        let json = client.post(&url)
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(json)
    }

    pub async fn delete_state (&self, store_name:&str, key:&str) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/" + key + "?metadata.contentType=application/json";
        let client = reqwest::Client::new();
        let res = client.delete(&url)
            .send()
            .await?;
        println!("Status code is {}", res.status().as_str());
        Ok(())
    }

    pub async fn transact_state (&self, store_name:&str, ops:Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/transaction?metadata.contentType=application/json";

        let client = reqwest::Client::new();
        let res = client.post(&url)
            .json(&ops)
            .send()
            .await?;
        println!("Status code is {}", res.status().as_str());
        Ok(())
    }
}
