# coindcx-mcp-server

A minimal [MCP (Model Context Protocol)](https://modelcontextprotocol.io) server written in Rust that exposes your CoinDCX futures wallet balance as a tool to Claude Desktop.

---

## What is an MCP Server?

MCP (Model Context Protocol) is an open standard that lets AI models like Claude interact with external tools and data sources. Instead of Claude only knowing what's in its training data, an MCP server lets you expose real-time data and actions — like fetching your live account balance — directly into a Claude conversation.

Think of it as a plugin system for Claude: you write a server that defines "tools", and Claude can call those tools on your behalf during a conversation.

---

## How It Works

```
Claude Desktop  ──►  MCP Protocol (stdin/stdout)  ──►  coindcx-mcp-server  ──►  CoinDCX API
                ◄──                                ◄──                      ◄──
```

1. Claude Desktop spawns your binary as a subprocess
2. Claude and the server communicate over `stdin`/`stdout` using the MCP protocol
3. When you ask Claude about your balance, it calls the `get_futures_wallet_balance` tool
4. The server makes an authenticated request to the CoinDCX API and returns the result
5. Claude reads the response and presents it to you in the conversation

---

## What This Project Includes

| Tool | Description |
|------|-------------|
| `get_futures_wallet_balance` | Fetches your CoinDCX futures wallet balances in real time |

That's it — intentionally simple. This is a working foundation you can extend with more tools (place orders, get positions, etc.).

---

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A [CoinDCX](https://coindcx.com) account with API access enabled
- [Claude Desktop](https://claude.ai/download)

---

## Clone and Run

### 1. Clone the repository

```bash
git clone https://github.com/your-username/coindcx-mcp-server.git
cd coindcx-mcp-server
```

### 2. Create your `.env` file

```bash
cp .env.example .env
```

Edit `.env` and fill in your CoinDCX API credentials:

```env
COINDCX_API_KEY=your_api_key_here
COINDCX_API_SECRET=your_api_secret_here
```

You can generate API keys from your [CoinDCX API settings](https://coindcx.com/settings/api).

### 3. Build the release binary

```bash
cargo build --release
```

The binary will be at `target/release/coindcx-mcp-server`.

### 4. Test it locally (optional)

```bash
cargo run --release
```

The server will start and wait for MCP messages on stdin. You won't see any output — that's expected. Kill it with `Ctrl+C`.

---

## Claude Desktop Setup

### Locate your config file

| OS | Path |
|----|------|
| macOS | `~/Library/Application Support/Claude/claude_desktop_config.json` |
| Windows | `%APPDATA%\Claude\claude_desktop_config.json` |

### Option A — Recommended: use `cwd` to load `.env` automatically

This keeps your API keys out of the config file. The server reads them from `.env` at startup.

```json
{
  "mcpServers": {
    "coindcx": {
      "command": "/absolute/path/to/coindcx-mcp-server/target/release/coindcx-mcp-server",
      "cwd": "/absolute/path/to/coindcx-mcp-server"
    }
  }
}
```

Replace `/absolute/path/to/coindcx-mcp-server` with the actual path on your machine. For example on macOS:

```json
{
  "mcpServers": {
    "coindcx": {
      "command": "/Users/yourname/projects/coindcx-mcp-server/target/release/coindcx-mcp-server",
      "cwd": "/Users/yourname/projects/coindcx-mcp-server"
    }
  }
}
```

### Option B — Use `cargo run` (no separate build step)

Convenient during development. Cargo will recompile if needed and the `.env` is picked up automatically.

```json
{
  "mcpServers": {
    "coindcx": {
      "command": "/Users/yourname/.cargo/bin/cargo",
      "args": [
        "run",
        "--release",
        "--manifest-path",
        "/Users/yourname/projects/coindcx-mcp-server/Cargo.toml"
      ]
    }
  }
}
```

> **Note:** If `cargo` isn't found, use its full path. Run `which cargo` in your terminal to find it.

### Restart Claude Desktop

After editing the config, fully quit and reopen Claude Desktop. You should see the CoinDCX tools appear in the tools panel (the hammer icon in the chat input).

---

## Usage

Once set up, just ask Claude naturally:

> *"What's my CoinDCX futures wallet balance?"*
> *"Show me my futures account balance on CoinDCX."*

Claude will call the `get_futures_wallet_balance` tool and display your live balance in the conversation.

---

## Security Notes

- Your `.env` file contains sensitive credentials — never commit it to version control
- `.env` is listed in `.gitignore` by default in this project
- The MCP config file is local to your machine and is not transmitted to Anthropic's servers
- API keys are never visible to the Claude model — only the tool's response (your balance) is

---

## Project Structure

```
coindcx-mcp-server/
├── src/
│   ├── main.rs                  # Entry point, starts MCP transport
│   ├── config.rs                # Loads env vars
│   ├── reqwest.rs               # HTTP client setup
│   ├── mcp/
│   │   └── coindcx_server.rs   # MCP tool definitions
│   ├── coindcx/
│       └── service.rs           # CoinDCX API client
├── .env.example                 # Template for credentials
├── Cargo.toml
└── README.md
```

---

## License

MIT
