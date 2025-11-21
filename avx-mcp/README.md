# AVX MCP Server

**Model Context Protocol Server para Avila Experience Fabric**

O AVX MCP Server expõe recursos e ferramentas da plataforma AVX para LLMs (Claude, GPT, etc.), permitindo que agentes de IA interajam com:

- 🗄️ **AvilaDB**: Query e manipulação de dados
- ⚙️ **AVX Config**: Configurações de stack, mesh e clusters
- ☸️ **Kubernetes**: Gerenciamento de deployments e serviços
- 📊 **Telemetria**: Métricas e monitoring em tempo real

## 🚀 Quick Start

### Instalação

```bash
# Clone o repositório
git clone https://github.com/avilaops/avx-mcp.git
cd avx-mcp

# Build do projeto
cargo build --release

# Instalar binários
cargo install --path avx-cli
cargo install --path avx-mcp
```

### Uso Básico

```bash
# Iniciar o servidor MCP
avx-cli mcp serve

# Listar resources disponíveis
avx-cli mcp resources

# Listar tools disponíveis
avx-cli mcp tools

# Testar configuração
avx-cli mcp test
```

### Gerar manifests Kubernetes

```bash
# Gerar manifest para o gateway
avx-cli k8s --service gateway --namespace avx-core --replicas 3

# Salvar em arquivo
avx-cli k8s --service api-core --output ./manifests/api-core.yaml
```

## 📚 Resources Disponíveis

### AvilaDB Resources
- `aviladb://production/users` - Banco de dados de usuários
- `aviladb://production/events` - Banco de dados de eventos

### AVX Config Resources
- `avx://config/stack` - Configuração do stack
- `avx://config/mesh` - Configuração do service mesh

### Cluster Resources
- `avx://cluster/production` - Cluster de produção
- `avx://cluster/staging` - Cluster de staging

## 🔧 Tools Disponíveis

### `avx_query`
Query AvilaDB com filtros e projeções.

```json
{
  "database": "production",
  "collection": "users",
  "query": "{\"status\": \"active\"}",
  "limit": 100
}
```

### `avx_deploy`
Deploy de serviços para clusters AVX.

```json
{
  "service": "gateway",
  "cluster": "production",
  "namespace": "avx-core",
  "replicas": 3
}
```

### `avx_telemetry`
Obter métricas em tempo real.

```json
{
  "service": "api-core",
  "metric": "cpu",
  "cluster": "production",
  "timeRange": "1h"
}
```

## 🔌 Integração com Claude Desktop

Adicione ao seu `claude_desktop_config.json`:

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

## 📖 Arquitetura

```
avx-mcp/
├── avx-cli/          # CLI principal
│   └── src/
│       └── main.rs   # Comandos K8s e MCP
├── avx-mcp/          # Servidor MCP
│   └── src/
│       ├── protocol/ # Tipos e JSON-RPC
│       ├── resources/# Resources expostos
│       ├── tools/    # Tools disponíveis
│       └── server.rs # Loop principal
└── avx-config/       # Biblioteca de config
    └── src/
        └── lib.rs    # AvxConfig
```

## 🛠️ Desenvolvimento

### Build

```bash
cargo build
```

### Testes

```bash
cargo test
```

### Formato e Lint

```bash
cargo fmt
cargo clippy
```

## 📝 Licença

MIT OR Apache-2.0

## 👥 Autores

- Nícolas Ávila <nicolas@avila.inc>
- Avila Development Team <dev@avila.inc>
