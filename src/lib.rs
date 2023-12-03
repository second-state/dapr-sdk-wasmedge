use anyhow::{anyhow, Error};
use serde_json::Value;
use std::collections::HashMap;

pub struct Dapr {
    pub url_base: String,
    pub api_token: String,
}

impl Dapr {
    pub fn new_with_url(url_base_: String, api_token_: String) -> Dapr {
        Dapr {
            url_base: url_base_ + "/v1.0/",
            api_token: api_token_,
        }
    }
}

impl Dapr {
    pub fn new(port: u32) -> Dapr {
        Dapr::new_with_url("http://localhost:".to_string() + &port.to_string(), "".to_string())
    }
}

impl Dapr {
    pub async fn invoke_service(
        &self,
        app_id: &str,
        method_name: &str,
        kvs: Value,
    ) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "invoke/" + app_id + "/method/" + method_name;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let json = client.post(&url).header("dapr-api-token", self.api_token.as_str()).json(&kvs).send().await?.json().await?;
        Ok(json)
    }

    pub async fn save_state(&self, store_name: &str, kvs: Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.post(&url).header("dapr-api-token", self.api_token.as_str()).json(&kvs).send().await?;

        if res.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(anyhow!(
                "Dapr failed to save the state! Status code: {}",
                res.status().as_str()
            ))
        }
    }

    pub async fn get_state(&self, store_name: &str, key: &str) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/" + key;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let json = client.get(&url).header("dapr-api-token", self.api_token.as_str()).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_bulk_state(
        &self,
        store_name: &str,
        keys: Vec<String>,
    ) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/bulk";
        println!("URL is {}", url);

        let mut data = HashMap::new();
        data.insert("keys", keys);

        let client = reqwest::Client::new();
        let json = client.post(&url).header("dapr-api-token", self.api_token.as_str()).json(&data).send().await?.json().await?;
        Ok(json)
    }

    pub async fn delete_state(&self, store_name: &str, key: &str) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/" + key;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.delete(&url).header("dapr-api-token", self.api_token.as_str()).send().await?;

        if res.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(anyhow!(
                "Dapr failed to delete the state! Status code: {}",
                res.status().as_str()
            ))
        }
    }

    pub async fn transact_state(&self, store_name: &str, ops: Value) -> Result<(), Error> {
        let url = self.url_base.to_string() + "state/" + store_name + "/transaction";
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.post(&url).header("dapr-api-token", self.api_token.as_str()).json(&ops).send().await?;

        if res.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(anyhow!(
                "Dapr failed to complete the state tx! Status code: {}",
                res.status().as_str()
            ))
        }
    }

    pub async fn get_secret(&self, store_name: &str, key: &str) -> Result<Value, Error> {
        let url = self.url_base.to_string() + "secrets/" + store_name + "/" + key;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let json = client.get(&url).header("dapr-api-token", self.api_token.as_str()).send().await?.json().await?;
        Ok(json)
    }

    pub async fn is_healthy(&self) -> Result<(), Error> {
        let url = self.url_base.to_string() + "healthz/";
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.get(&url).header("dapr-api-token", self.api_token.as_str()).send().await?;
        println!("Status code is {}", res.status().as_str());

        if res.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(anyhow!(
                "Dapr is not healthy! Status code: {}",
                res.status().as_str()
            ))
        }
    }

    pub async fn publish(
        &self,
        pubsubname: &str,
        topic_name: &str,
        metadata: Value,
    ) -> Result<(), Error> {
        let url = self.url_base.to_string() + "publish/" + pubsubname + "/" + topic_name;
        println!("URL is {}", url);

        let client = reqwest::Client::new();
        let res = client.post(&url).header("dapr-api-token", self.api_token.as_str()).json(&metadata).send().await?;
        println!("Status code is {}", res.status().as_str());

        if res.status().as_u16() == 204 {
            Ok(())
        } else {
            Err(anyhow!(
                "Dapr failed to complete the pub request! Status code: {}",
                res.status().as_str()
            ))
        }
    }
}
