use rust_decimal::Decimal;

use crate::coindcx::{
    client::CoinDcxClient,
    dto::{
        CancelAllOrdersRequest, CancelOrdersByPositionRequest, CoinDcxBalance,
        CoinDcxCancelResponse, CoinDcxFuturesWallet, CoinDcxOrder, CoinDcxOrderResponse,
        CoinDcxPosition, CoinDcxPositionsRequest, CoinDcxTimestamp,
    },
};

pub struct CoinDcxService;

impl CoinDcxService {
    pub async fn get_balance() -> Vec<CoinDcxBalance> {
        let client = CoinDcxClient::new();

        let body = CoinDcxTimestamp {
            timestamp: CoinDcxClient::timestamp(),
        };

        let response = match client
            .post_signed("/exchange/v1/users/balances", &body)
            .await
        {
            Some(r) => r,
            None => return vec![],
        };

        let mut balances: Vec<CoinDcxBalance> = match serde_json::from_str(&response) {
            Ok(b) => b,
            Err(err) => {
                dbg!("Parse balance error: {:?}", err);
                return vec![];
            }
        };

        balances.retain(|b| b.balance > Decimal::ZERO);
        balances.sort_by(|a, b| b.balance.cmp(&a.balance));

        balances
    }

    pub async fn place_multiple_orders(orders: Vec<CoinDcxOrder>) -> Option<CoinDcxOrderResponse> {
        let client = CoinDcxClient::new();

        let body = serde_json::json!({ "orders": orders });

        let response = client
            .post_signed("/exchange/v1/orders/create_multiple", &body)
            .await?;

        match serde_json::from_str(&response) {
            Ok(res) => Some(res),
            Err(err) => {
                dbg!("Parse order response error: {:?}", err);
                None
            }
        }
    }

    pub async fn get_futures_wallets() -> Vec<CoinDcxFuturesWallet> {
        let client = CoinDcxClient::new();

        let body = CoinDcxTimestamp {
            timestamp: CoinDcxClient::timestamp(),
        };

        let response = match client
            .get_signed("/exchange/v1/derivatives/futures/wallets", &body)
            .await
        {
            Some(r) => r,
            None => return vec![],
        };

        match serde_json::from_str(&response) {
            Ok(wallets) => wallets,
            Err(err) => {
                dbg!("Parse wallets error: {:?}", err);
                vec![]
            }
        }
    }

    pub async fn get_futures_positions(page: u32, size: u32) -> Vec<CoinDcxPosition> {
        let client = CoinDcxClient::new();

        let body = CoinDcxPositionsRequest {
            timestamp: CoinDcxClient::timestamp(),
            page: page.to_string(),
            size: size.to_string(),
            margin_currency_short_name: Some(vec!["USDT".to_string()]),
            pairs: None,
            position_ids: None,
        };

        let response = match client
            .post_signed("/exchange/v1/derivatives/futures/positions", &body)
            .await
        {
            Some(r) => r,
            None => return vec![],
        };

        let mut positions: Vec<CoinDcxPosition> = match serde_json::from_str(&response) {
            Ok(p) => p,
            Err(err) => {
                dbg!("Parse positions error: {:?}", err);
                return vec![];
            }
        };

        positions.retain(|p| p.active_pos != Decimal::ZERO);

        positions
    }

    pub async fn get_position_by_id(position_ids: Vec<String>) -> Vec<CoinDcxPosition> {
        let client = CoinDcxClient::new();

        let body = CoinDcxPositionsRequest {
            timestamp: CoinDcxClient::timestamp(),
            page: "1".to_string(),
            size: position_ids.len().to_string(),
            pairs: None,
            position_ids: Some(position_ids.join(",")),
            margin_currency_short_name: Some(vec!["USDT".to_string()]),
        };

        let response = match client
            .post_signed("/exchange/v1/derivatives/futures/positions", &body)
            .await
        {
            Some(r) => r,
            None => return vec![],
        };

        match serde_json::from_str(&response) {
            Ok(p) => p,
            Err(err) => {
                dbg!("Parse position by id error: {:?}", err);
                vec![]
            }
        }
    }

    pub async fn cancel_all_orders() -> Option<CoinDcxCancelResponse> {
        let client = CoinDcxClient::new();

        let body = CancelAllOrdersRequest {
            timestamp: CoinDcxClient::timestamp(),
            margin_currency_short_name: Some(vec!["USDT".to_string()]),
        };

        let response = client
            .post_signed(
                "/exchange/v1/derivatives/futures/positions/cancel_all_open_orders",
                &body,
            )
            .await?;

        match serde_json::from_str(&response) {
            Ok(res) => Some(res),
            Err(err) => {
                dbg!("Parse cancel all error: {:?}", err);
                None
            }
        }
    }

    pub async fn cancel_orders_by_position(position_id: String) -> Option<CoinDcxCancelResponse> {
        let client = CoinDcxClient::new();

        let body = CancelOrdersByPositionRequest {
            timestamp: CoinDcxClient::timestamp(),
            id: position_id,
        };

        let response = client
            .post_signed(
                "/exchange/v1/derivatives/futures/positions/cancel_all_open_orders_for_position",
                &body,
            )
            .await?;

        match serde_json::from_str(&response) {
            Ok(res) => Some(res),
            Err(err) => {
                dbg!("Parse cancel by position error: {:?}", err);
                None
            }
        }
    }
}
