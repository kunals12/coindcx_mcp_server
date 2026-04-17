use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

use crate::{config::Config, reqwest::REQWEST_CLIENT};

type HmacSha256 = Hmac<Sha256>;

pub struct CoinDcxClient {
    api_key: String,
    secret_key: String,
    base_url: String,
    http: reqwest::Client,
}

impl CoinDcxClient {
    pub fn new() -> Self {
        Self {
            api_key: Config::init().coindcx_api_key,
            secret_key: Config::init().coindcx_secret_key,
            base_url: "https://api.coindcx.com".to_string(),
            http: REQWEST_CLIENT.clone(),
        }
    }

    pub fn timestamp() -> i64 {
        Utc::now().timestamp_millis()
    }

    fn sign(&self, payload: &str) -> Option<String> {
        let mut mac = match HmacSha256::new_from_slice(self.secret_key.as_bytes()) {
            Ok(m) => m,
            Err(err) => {
                dbg!("Invalid secret key: {:?}", err);
                return None;
            }
        };

        mac.update(payload.as_bytes());
        let result = mac.finalize().into_bytes();

        Some(hex::encode(result))
    }

    pub async fn post_signed<T: Serialize>(&self, endpoint: &str, body: &T) -> Option<String> {
        let payload = match serde_json::to_string(body) {
            Ok(p) => p,
            Err(err) => {
                dbg!("Serialize error: {:?}", err);
                return None;
            }
        };

        let signature = self.sign(&payload)?;

        let url = format!("{}{}", self.base_url, endpoint);

        let res = match self
            .http
            .post(url)
            .header("X-AUTH-APIKEY", &self.api_key)
            .header("X-AUTH-SIGNATURE", signature)
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await
        {
            Ok(r) => r,
            Err(err) => {
                dbg!("Request error: {:?}", err);
                return None;
            }
        };

        let status = res.status();

        let body = match res.text().await {
            Ok(b) => b,
            Err(err) => {
                dbg!("Read body error: {:?}", err);
                return None;
            }
        };

        if !status.is_success() {
            dbg!("HTTP error: {}, body: {}", status, body);
            return None;
        }

        Some(body)
    }

    pub async fn get_signed<T: Serialize>(&self, endpoint: &str, query: &T) -> Option<String> {
        let payload = match serde_json::to_string(query) {
            Ok(p) => p,
            Err(err) => {
                dbg!("Serialize error: {:?}", err);
                return None;
            }
        };

        let signature = self.sign(&payload)?;

        let url = format!("{}{}", self.base_url, endpoint);

        let res = match self
            .http
            .get(url)
            .header("X-AUTH-APIKEY", &self.api_key)
            .header("X-AUTH-SIGNATURE", signature)
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await
        {
            Ok(r) => r,
            Err(err) => {
                dbg!("GET request error: {:?}", err);
                return None;
            }
        };

        let status = res.status();

        let body = match res.text().await {
            Ok(b) => b,
            Err(err) => {
                dbg!("Read body error: {:?}", err);
                return None;
            }
        };

        if !status.is_success() {
            dbg!("HTTP error: {}, body: {}", status, body);
            return None;
        }

        Some(body)
    }
}
