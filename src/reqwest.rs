use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

pub static REQWEST_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10)) // Node-like
        .pool_idle_timeout(Duration::from_secs(60))
        .tcp_keepalive(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});
