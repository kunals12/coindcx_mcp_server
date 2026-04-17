use crate::mcp::coindcx_server::CoindcxMcpServer;
use rmcp::ServiceExt;

mod coindcx;
mod config;
mod mcp;
mod reqwest;

#[tokio::main]
async fn main() {
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    CoindcxMcpServer::new()
        .serve(transport)
        .await
        .expect("Failed to start MCP server")
        .waiting()
        .await
        .expect("MCP server exited with error");
}
