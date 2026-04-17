use rmcp::{
    ServerHandler,
    handler::server::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::coindcx::service::CoinDcxService;

/// MCP Server struct
/// Holds the tool router which maps tool names → actual Rust functions
pub struct CoindcxMcpServer {
    tool_router: ToolRouter<CoindcxMcpServer>,
}

/// `#[tool_router]` collects all functions marked with `#[tool]`
/// and builds a router that Claude (via MCP) can call
#[tool_router]
impl CoindcxMcpServer {
    /// Initialize server with generated tool router
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// This function is exposed as an MCP tool
    /// Claude can call this during a conversation
    ///
    /// Example prompt:
    /// "What's my CoinDCX futures wallet balance?"
    #[tool(description = "Get futures wallet balance of the CoinDCX account")]
    pub async fn get_futures_wallet_balance(&self) -> String {
        // Call service layer → fetch real-time data from CoinDCX API
        let wallets = CoinDcxService::get_futures_wallets().await;

        // Convert response into JSON string (MCP expects serializable output)
        serde_json::to_string(&wallets).unwrap_or_else(|err| {
            // Log error if serialization fails
            dbg!("Serialize error: {:?}", err);

            // Return empty array fallback (never break MCP flow)
            "[]".to_string()
        })
    }
}

/// `#[tool_handler]` connects your MCP server to the protocol
/// This is what Claude actually talks to
#[tool_handler]
impl ServerHandler for CoindcxMcpServer {
    /// Provides metadata about the server to Claude
    /// Here we explicitly enable "tools" capability
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools() // tells Claude this server exposes tools
                .build(),
            ..Default::default()
        }
    }
}
