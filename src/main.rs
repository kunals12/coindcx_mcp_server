use crate::coindcx::service::CoinDcxService;

mod coindcx;
mod config;
mod reqwest;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let balance = CoinDcxService::get_futures_wallets().await;
    dbg!(&balance);
}
