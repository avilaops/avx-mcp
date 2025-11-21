# Publicação do AVX MCP

## 📦 Crates.io (Rust)

### Pré-requisitos
```bash
# Login no crates.io
cargo login
```

### Publicar packages

```bash
# 1. Publicar avx-config primeiro (é dependência)
cd avx-config
cargo publish

# 2. Publicar avx-mcp
cd ../avx-mcp
cargo publish

# 3. Publicar avx-cli
cd ../avx-cli
cargo publish
```

### Verificar antes de publicar
```bash
cargo package --list
cargo publish --dry-run
```

---

## 🐙 GitHub

### Criar repositório
```bash
# Inicializar git (se ainda não fez)
git init
git add .
git commit -m "Initial commit: AVX MCP Server and CLI"

# Adicionar remote
git remote add origin https://github.com/nicolasavilaops/avx-mcp.git

# Push
git branch -M main
git push -u origin main
```

### Criar Release
```bash
# Tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

No GitHub:
1. Ir em "Releases"
2. "Create a new release"
3. Escolher tag v0.1.0
4. Adicionar notas do CHANGELOG.md
5. Anexar binários compilados (opcional)

---

## 🐳 Docker Hub

### Criar Dockerfile
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/avx-cli /usr/local/bin/
COPY --from=builder /app/target/release/avx-mcp /usr/local/bin/
CMD ["avx-cli", "mcp", "serve"]
```

### Build e Push
```bash
# Build
docker build -t nicolasavilaops/avx-mcp:0.1.0 .
docker build -t nicolasavilaops/avx-mcp:latest .

# Login
docker login

# Push
docker push nicolasavilaops/avx-mcp:0.1.0
docker push nicolasavilaops/avx-mcp:latest
```

---

## 📦 GitHub Container Registry (GHCR)

```bash
# Login
echo $GITHUB_TOKEN | docker login ghcr.io -u nicolasavilaops --password-stdin

# Build
docker build -t ghcr.io/nicolasavilaops/avx-mcp:0.1.0 .

# Push
docker push ghcr.io/nicolasavilaops/avx-mcp:0.1.0
```

---

## 🌐 NPM (para integração com Claude Desktop)

### Criar package.json wrapper
```json
{
  "name": "@avilaops/avx-mcp",
  "version": "0.1.0",
  "description": "MCP Server for Avila Experience Fabric",
  "bin": {
    "avx-mcp": "./install.js"
  },
  "scripts": {
    "postinstall": "node install.js"
  }
}
```

### Publicar
```bash
npm login
npm publish --access public
```

---

## 🍺 Homebrew (macOS/Linux)

### Criar formula
```ruby
class AvxMcp < Formula
  desc "MCP Server for Avila Experience Fabric"
  homepage "https://github.com/nicolasavilaops/avx-mcp"
  url "https://github.com/nicolasavilaops/avx-mcp/archive/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT OR Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "avx-cli")
    system "cargo", "install", *std_cargo_args(path: "avx-mcp")
  end

  test do
    system "#{bin}/avx-cli", "--version"
  end
end
```

---

## 🪟 Winget (Windows)

### Criar manifest
```yaml
PackageIdentifier: AvilaOps.AvxMcp
PackageVersion: 0.1.0
PackageName: AVX MCP
Publisher: Avila Operations
License: MIT OR Apache-2.0
ShortDescription: MCP Server for Avila Experience Fabric
Installers:
  - Architecture: x64
    InstallerType: exe
    InstallerUrl: https://github.com/nicolasavilaops/avx-mcp/releases/download/v0.1.0/avx-mcp-x64.exe
```

---

## 🐧 Snapcraft (Linux)

### Criar snapcraft.yaml
```yaml
name: avx-mcp
version: '0.1.0'
summary: MCP Server for Avila Experience Fabric
description: |
  Model Context Protocol Server for AVX platform
base: core22
confinement: strict
grade: stable

apps:
  avx-cli:
    command: bin/avx-cli
  avx-mcp:
    command: bin/avx-mcp

parts:
  avx-mcp:
    plugin: rust
    source: .
```

```bash
snapcraft
snapcraft push avx-mcp_0.1.0_amd64.snap
```

---

## 📱 Anthropic MCP Registry (Claude Desktop)

1. Fork https://github.com/anthropics/mcp-registry
2. Adicionar entrada em `registry.json`:

```json
{
  "servers": {
    "avx": {
      "name": "AVX MCP Server",
      "description": "MCP Server for Avila Experience Fabric",
      "author": "Avila Operations",
      "homepage": "https://github.com/nicolasavilaops/avx-mcp",
      "install": {
        "cargo": {
          "package": "avx-cli"
        }
      },
      "config": {
        "command": "avx-cli",
        "args": ["mcp", "serve"]
      }
    }
  }
}
```

3. Abrir Pull Request

---

## ✅ Checklist Pré-Publicação

- [ ] Testar compilação: `cargo build --release`
- [ ] Rodar testes: `cargo test`
- [ ] Verificar clippy: `cargo clippy`
- [ ] Formatar código: `cargo fmt`
- [ ] Atualizar versão em todos os Cargo.toml
- [ ] Atualizar CHANGELOG.md
- [ ] Verificar README.md
- [ ] Adicionar LICENSE files
- [ ] Testar instalação limpa
- [ ] Criar tag git
- [ ] Gerar binários para múltiplas plataformas

---

## 🚀 Distribuição de Binários

### GitHub Actions (CI/CD)

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo build --release
      - uses: actions/upload-artifact@v4
        with:
          name: avx-mcp-${{ matrix.os }}
          path: target/release/avx-cli*
```

---

## 📊 Plataformas Recomendadas

**Essenciais:**
1. ✅ GitHub (código fonte)
2. ✅ Crates.io (Rust)
3. ✅ Docker Hub ou GHCR (containers)

**Opcional mas recomendado:**
4. Anthropic MCP Registry (descoberta)
5. Homebrew (macOS/Linux)
6. Winget (Windows)

**Avançado:**
7. NPM (wrapper)
8. Snapcraft (Linux)
9. AUR (Arch Linux)
10. Flathub (Linux)
