use rmcp::{
    ServerHandler,
    handler::server::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::coindcx::service::CoinDcxService;

pub struct CoindcxMcpServer {
    tool_router: ToolRouter<CoindcxMcpServer>,
}

#[tool_router]
impl CoindcxMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get futures wallet balance of the CoinDCX account")]
    pub async fn get_futures_wallet_balance(&self) -> String {
        let wallets = CoinDcxService::get_futures_wallets().await;

        serde_json::to_string(&wallets).unwrap_or_else(|err| {
            dbg!("Serialize error: {:?}", err);
            "[]".to_string()
        })
    }
}

#[tool_handler]
impl ServerHandler for CoindcxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
