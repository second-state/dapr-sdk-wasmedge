use std::collections::HashMap;
use serde_json::Value;
use reqwest::Error;

pub struct Dapr {
    url_base: &'static str,
}

impl Dapr {
    pub fn new() -> Dapr {
        Dapr {
            url_base: "http://localhost:3503/v1.0/",
        }
    }
}

impl Dapr {
    pub async fn save_state (&self, store_name:&str, kvs:Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name +"?metadata.contentType=application/json";

        // let mut data = HashMap::new();
        // data.insert("key", key);
        // data.insert("value", value);

        let client = reqwest::Client::new();
        client.post(&url)
            .json(&kvs)
            .send()
            .await?;
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
        client.delete(&url)
            .send()
            .await?;
        Ok(())
    }

    pub async fn transact_state (&self, store_name:&str, ops:Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/transaction?metadata.contentType=application/json";

        let client = reqwest::Client::new();
        client.post(&url)
            .json(&ops)
            .send()
            .await?;
        Ok(())
    }
}