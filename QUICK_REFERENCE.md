# Quick Reference - AVX MCP

Comandos rápidos para uso diário do AVX MCP.

## 🏗️ Build

```bash
# Build debug
cargo build

# Build release (otimizado)
cargo build --release

# Build específico
cargo build -p avx-cli
cargo build -p avx-mcp
```

## 🧪 Test

```bash
# Todos os testes
cargo test

# Com output
cargo test -- --nocapture

# Package específico
cargo test -p avx-mcp
```

## 🔍 Check

```bash
# Verificar compilação
cargo check

# Formatar código
cargo fmt

# Lint
cargo clippy

# Verificar tudo
cargo fmt --check && cargo clippy --all-targets
```

## 📦 Install

```bash
# Instalar globalmente
cargo install --path avx-cli
cargo install --path avx-mcp

# Desinstalar
cargo uninstall avx-cli
cargo uninstall avx-mcp
```

## 🚀 Run

```bash
# Via cargo
cargo run -p avx-cli -- mcp test
cargo run -p avx-cli -- mcp resources
cargo run -p avx-cli -- mcp tools
cargo run -p avx-cli -- mcp serve

# Via binário debug
.\target\debug\avx-cli.exe mcp test

# Via binário release
.\target\release\avx-cli.exe mcp test
```

## 📋 CLI Commands

### MCP Server

```bash
# Testar configuração
avx-cli mcp test

# Listar resources
avx-cli mcp resources

# Listar tools
avx-cli mcp tools

# Iniciar servidor
avx-cli mcp serve
```

### Kubernetes Manifests

```bash
# Gateway
avx-cli k8s --service gateway --replicas 3

# API Core
avx-cli k8s --service api-core --replicas 2

# Events
avx-cli k8s --service events --replicas 2

# Com imagem customizada
avx-cli k8s --service gateway --image ghcr.io/avilaops/gateway:v2.0.0

# Salvar em arquivo
avx-cli k8s --service gateway --output ./deploy/gateway.yaml
```

## 🌍 Environment Variables

```bash
# Windows PowerShell
$env:AVX__STACK = "production"
$env:AVX__LAYER = "core"
$env:AVX__ENV = "prod"
$env:AVX__CLUSTER = "us-east-1"
$env:AVX__MESH = "istio"
$env:RUST_LOG = "avx_mcp=debug"

# Linux/macOS
export AVX__STACK=production
export AVX__LAYER=core
export AVX__ENV=prod
export AVX__CLUSTER=us-east-1
export AVX__MESH=istio
export RUST_LOG=avx_mcp=debug
```

## 📊 Logs

```bash
# Info (padrão)
RUST_LOG=avx_mcp=info avx-cli mcp serve

# Debug
RUST_LOG=avx_mcp=debug avx-cli mcp serve

# Trace (muito verbose)
RUST_LOG=avx_mcp=trace avx-cli mcp serve

# Todos os módulos
RUST_LOG=debug avx-cli mcp serve
```

## 🔧 Development

```bash
# Watch mode (recompila ao salvar)
cargo watch -x "run -p avx-cli -- mcp test"

# Watch + test
cargo watch -x test

# Watch + clippy
cargo watch -x clippy
```

## 📝 Documentation

```bash
# Gerar docs
cargo doc

# Gerar e abrir
cargo doc --open

# Docs sem dependências
cargo doc --no-deps
```

## 🧹 Clean

```bash
# Limpar target/
cargo clean

# Limpar e rebuild
cargo clean && cargo build --release
```

## 📦 Package Info

```bash
# Versão
avx-cli --version

# Help
avx-cli --help
avx-cli mcp --help
avx-cli k8s --help

# Tree de dependências
cargo tree

# Atualizar dependências
cargo update
```

## 🐛 Debug

```bash
# Verbose
cargo build -vv

# Backtrace
RUST_BACKTRACE=1 avx-cli mcp serve

# Full backtrace
RUST_BACKTRACE=full avx-cli mcp serve
```

## 🔐 Claude Desktop

```bash
# Localização do config (Windows)
notepad %APPDATA%\Claude\claude_desktop_config.json

# Testar MCP localmente
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | avx-cli mcp serve
```

## 🎯 Common Workflows

### Deploy Completo

```bash
# 1. Build
cargo build --release

# 2. Gerar manifests
avx-cli k8s --service gateway --output gateway.yaml
avx-cli k8s --service api-core --output api-core.yaml
avx-cli k8s --service events --output events.yaml

# 3. Aplicar
kubectl apply -f gateway.yaml
kubectl apply -f api-core.yaml
kubectl apply -f events.yaml
```

### Dev Cycle

```bash
# 1. Fazer alterações no código
# 2. Testar
cargo test

# 3. Verificar
cargo clippy

# 4. Formatar
cargo fmt

# 5. Build
cargo build

# 6. Run
cargo run -p avx-cli -- mcp test
```

### Release

```bash
# 1. Update version em Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit
git add .
git commit -m "Release v0.2.0"
git tag v0.2.0

# 4. Build release
cargo build --release

# 5. Testar
.\target\release\avx-cli.exe mcp test

# 6. Push
git push origin main
git push origin v0.2.0
```

## 📚 Useful Cargo Commands

```bash
# Informações do package
cargo metadata

# Bench (futuro)
cargo bench

# Audit (segurança)
cargo audit

# Expand macros
cargo expand

# Check sem build
cargo check --all-targets
```

## 🔥 Pro Tips

```bash
# Build rápido (sem otimizações)
cargo build --profile dev

# Build otimizado + pequeno
cargo build --release --profile release-lto

# Parallel tests
cargo test -- --test-threads=4

# Specific test
cargo test test_name

# Show output even for passing tests
cargo test -- --nocapture --test-threads=1
```
