use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CoinDcxTickerResponse {
    pub market: String,

    #[serde(default, deserialize_with = "de_string_from_any")]
    pub last_price: Option<String>,
}

pub fn de_string_from_any<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(Some(s)),
        Value::Number(n) => Ok(Some(n.to_string())),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("invalid type")),
    }
}

/*
* "market": "REQBTC",
    "change_24_hour": "-1.621",
    "high": "0.00002799",
    "low": "0.00002626",
    "volume": "14.10",
    "last_price": "0.00002663",
    "bid": "0.00002663",
    "ask": "0.00002669",
    "timestamp": 1524211224
*/
#[derive(Debug, Deserialize)]
pub struct CoinDCXTicker {
    pub change_24_hour: f64, // Changed from String to f64
    pub high: f64,           // Changed from String to f64
    pub low: f64,            // Changed from String to f64
    pub volume: f64,         // Changed from String to f64
    pub market: String,      // Remains String
    pub last_price: f64,     // Changed from String to f64
    pub bid: f64,            // Changed from String to f64
    pub ask: f64,            // Changed from String to f64
    pub timestamp: i64,      // Remains i64
}

#[derive(Debug, Deserialize)]
pub struct CoinDcxBalance {
    pub currency: String,
    pub balance: Decimal,
    pub locked_balance: Decimal,
}

#[derive(Serialize)]
pub struct CoinDcxTimestamp {
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct CoinDcxOrder {
    pub market: String,
    pub total_quantity: Decimal,
    pub price_per_unit: Option<Decimal>,
    pub side: String,       // "buy" or "sell"
    pub order_type: String, // "market_order" or "limit_order"
    pub ecode: String,      // "I" for INR markets
    pub client_order_id: Option<String>,
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct CoinDcxOrderResponse {
    pub orders: Vec<OrderDetails>,
}

#[derive(Deserialize)]
pub struct OrderDetails {
    pub id: String,
    pub client_order_id: Option<String>,
    pub market: String,
    pub order_type: String,
    pub side: String,
    pub status: String,
    pub fee_amount: Decimal,
    pub fee: Decimal,
    pub total_quantity: Decimal,
    pub remaining_quantity: Decimal,
    pub avg_price: Decimal,
    pub price_per_unit: Decimal,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDcxFuturesWallet {
    pub id: String,
    pub currency_short_name: String,

    // Ignore this field per docs
    pub balance: Option<Decimal>,

    pub locked_balance: Decimal,
    pub cross_order_margin: Decimal,
    pub cross_user_margin: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDcxPosition {
    pub id: String,
    pub pair: String,

    pub active_pos: Decimal,

    pub inactive_pos_buy: Decimal,
    pub inactive_pos_sell: Decimal,

    pub avg_price: Decimal,
    pub liquidation_price: Option<Decimal>,

    pub locked_margin: Decimal,
    pub locked_user_margin: Decimal,
    pub locked_order_margin: Decimal,

    pub take_profit_trigger: Option<Decimal>,
    pub stop_loss_trigger: Option<Decimal>,

    pub leverage: Decimal,
    pub maintenance_margin: Decimal,

    pub mark_price: Decimal,

    pub margin_type: Option<String>, // "crossed" | "isolated" | null
    pub margin_currency_short_name: String,

    pub updated_at: i64,
}

#[derive(Serialize)]
pub struct CoinDcxPositionsRequest {
    pub timestamp: i64,
    pub page: String,
    pub size: String,

    pub pairs: Option<String>,
    pub position_ids: Option<String>,

    pub margin_currency_short_name: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinDcxCancelResponse {
    pub message: String,
    pub status: u16,
    pub code: u16,
}

#[derive(Serialize)]
pub struct CancelAllOrdersRequest {
    pub timestamp: i64,
    pub margin_currency_short_name: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct CancelOrdersByPositionRequest {
    pub timestamp: i64,
    pub id: String,
}
