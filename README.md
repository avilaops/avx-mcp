# AVX MCP - Model Context Protocol for Avila Experience Fabric

**🇺🇸 English** | [🇧🇷 Português](README.pt-BR.md)

**Complete MCP server in 100% Rust for LLM integration with AVX platform**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/avilaops/avx-mcp)](https://github.com/avilaops/avx-mcp/releases)

## 🎯 What is AVX MCP?

A Model Context Protocol (MCP) server that exposes **Avila Experience Fabric resources and tools** to LLMs (Claude, GPT, etc.), allowing them to:

- 🗄️ **Query AvilaDB**: Read and write to databases
- ⚙️ **Manage AVX Config**: Stack, mesh, and cluster configurations
- ☸️ **Kubernetes Deploy**: Generate and apply manifests
- 📊 **Real-time Telemetry**: CPU, memory, and latency metrics

## 🚀 Quick Start

### Installation

#### Option 1: Via Cargo (From GitHub)
```bash
cargo install --git https://github.com/avilaops/avx-mcp avx-cli
```

#### Option 2: Download Pre-built Binaries
Download the pre-compiled version for your system:

👉 **[Releases](https://github.com/avilaops/avx-mcp/releases/latest)**

- **Windows**: `avx-mcp-windows-x64.zip`
- **Linux**: `avx-mcp-linux-x64.tar.gz`
- **macOS**: `avx-mcp-macos-x64.tar.gz`

Extract and add to system PATH.

#### Option 3: Build from Source
```bash
git clone https://github.com/avilaops/avx-mcp.git
cd avx-mcp
cargo build --release
cargo install --path avx-cli
```

> **Note**: Package not yet available on crates.io. Use the options above.

### Basic Usage

```bash
# 1. Test configuration
avx-cli mcp test

# 2. List resources and tools
avx-cli mcp resources
avx-cli mcp tools

# 3. Start MCP server
avx-cli mcp serve

# 4. Generate K8s manifests
avx-cli k8s --service gateway --replicas 3 --output gateway.yaml
```

## 📦 Project Structure

```
avx-mcp/
├── Cargo.toml              # Root workspace
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── avx-cli/               # Main CLI
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs        # K8s and MCP commands
├── avx-mcp/               # MCP Server
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── main.rs        # Entry point
│       ├── lib.rs
│       ├── server.rs      # JSON-RPC loop
│       ├── protocol/      # MCP types
│       │   ├── mod.rs
│       │   └── types.rs
│       ├── resources/     # AVX resources
│       │   ├── mod.rs
│       │   ├── aviladb.rs
│       │   ├── avx_config.rs
│       │   └── cluster.rs
│       └── tools/         # AVX tools
│           ├── mod.rs
│           ├── aviladb_query.rs
│           ├── deploy.rs
│           └── telemetry.rs
└── avx-config/            # Config library
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## 📚 Available Resources

| URI | Name | Description |
|-----|------|-------------|
| `aviladb://production/users` | Users Database | User database |
| `aviladb://production/events` | Events Database | Events database |
| `avx://config/stack` | AVX Stack Config | Stack configuration |
| `avx://config/mesh` | AVX Mesh Config | Service mesh configuration |
| `avx://cluster/production` | Production Cluster | Production Kubernetes cluster |
| `avx://cluster/staging` | Staging Cluster | Staging Kubernetes cluster |

## 🔧 Available Tools

### `avx_query`
Query AvilaDB with filters and projections.

**Parameters:**
- `database`: Database name (e.g., "production")
- `collection`: Collection name (e.g., "users")
- `query`: JSON filter
- `limit`: Maximum number of results (default: 100)

### `avx_deploy`
Deploy services to AVX clusters.

**Parameters:**
- `service`: Service name ("gateway", "api-core", "events")
- `cluster`: Target cluster
- `namespace`: Kubernetes namespace (default: "avx-core")
- `replicas`: Number of replicas (default: 2)
- `image`: Container image (optional)

### `avx_telemetry`
Get real-time metrics.

**Parameters:**
- `service`: Service name
- `metric`: Metric type ("cpu", "memory", "requests", "latency", "errors")
- `cluster`: Target cluster (default: "production")
- `timeRange`: Time range (e.g., "1h", "24h", "7d")

## 🔌 Claude Desktop Integration

### Option 1: Local (Recommended)
Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "avx": {
      "command": "avx-cli",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "production",
        "AVX__CLUSTER": "us-east-1",
        "AVX__ENV": "prod"
      }
    }
  }
}
```

### Option 2: Via Docker (GHCR.io)
```json
{
  "mcpServers": {
    "avx": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm",
        "-e", "AVX__STACK=production",
        "-e", "AVX__CLUSTER=us-east-1",
        "ghcr.io/avilaops/avx-mcp:latest"
      ]
    }
  }
}
```

Restart Claude Desktop and AVX MCP will be available! 🎉

## 🛠️ Development

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific package
cargo build -p avx-mcp
```

### Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Package-specific tests
cargo test -p avx-config
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check everything
cargo fmt --check && cargo clippy --all-targets
```

## 📖 CLI Commands

### Generate Kubernetes Manifests

```bash
# Gateway
avx-cli k8s --service gateway --namespace avx-core --replicas 3

# API Core with custom image
avx-cli k8s --service api-core --image ghcr.io/avilaops/api:v2.0.0

# Save to file
avx-cli k8s --service events --output ./k8s/events.yaml
```

### Manage MCP Server

```bash
# Start server
avx-cli mcp serve

# List resources
avx-cli mcp resources

# List tools
avx-cli mcp tools

# Test configuration
avx-cli mcp test
```

## ⚙️ Configuration

Use environment variables to configure AVX:

```bash
export AVX__STACK=production
export AVX__LAYER=core
export AVX__ENV=prod
export AVX__CLUSTER=us-east-1
export AVX__MESH=istio
```

## 🎯 Roadmap

- [x] Base MCP protocol (JSON-RPC 2.0)
- [x] Resources: AvilaDB, Config, Clusters
- [x] Tools: Query, Deploy, Telemetry
- [x] CLI with K8s and MCP commands
- [ ] Real AvilaDB integration
- [ ] Webhooks and notifications support
- [ ] Web monitoring dashboard
- [ ] Multi-cluster simultaneous support
- [ ] Caching and performance optimizations

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

1. Fork the project
2. Create a branch: `git checkout -b feature/new-feature`
3. Commit: `git commit -m 'Add: new feature'`
4. Push: `git push origin feature/new-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under MIT OR Apache-2.0. See LICENSE files for details.

## 👥 Authors

- **Nícolas Ávila** - <nicolas@avila.inc>
- **Avila Development Team** - <dev@avila.inc>

## 🔗 Links

- [AVX Documentation](https://arxis.avilaops.com)
- [Model Context Protocol](https://modelcontextprotocol.io)
- [Anthropic MCP](https://github.com/anthropics/mcp)

---

**Made with ❤️ in Rust 🦀**
