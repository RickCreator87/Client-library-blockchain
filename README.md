# -nextgen-cli
   nextgen-cli  Universal CLI for managing Gitdigital projects.

# Nextgen CLI

The central command-line interface for the **Gitdigital Products** ecosystem.  
Control, monitor, and interact with all services from one terminal tool.

## 🚀 Features
- `ping` → check CLI is alive
- `status` → show system status
- `api <endpoint>` → call API endpoints

## 🛠️ Setup
```bash
# Build
cargo build

# Run CLI
cargo run -- ping
cargo run -- status
cargo run -- api health
