# Arquitetura do AVX MCP

## Visão Geral

O AVX MCP é composto por três componentes principais:

```
┌─────────────────────────────────────────────────────────┐
│                      Claude Desktop                      │
│                    (ou outro LLM)                        │
└────────────────────┬────────────────────────────────────┘
                     │ JSON-RPC 2.0 via stdio
                     ▼
┌─────────────────────────────────────────────────────────┐
│                    AVX MCP Server                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │              Protocol Layer                       │  │
│  │  • JSON-RPC 2.0 Handler                          │  │
│  │  • Initialize/Initialized                        │  │
│  │  • Request/Response                              │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │              Resources Layer                      │  │
│  │  • aviladb://production/users                    │  │
│  │  • avx://config/stack                            │  │
│  │  • avx://cluster/production                      │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │              Tools Layer                          │  │
│  │  • avx_query (AvilaDB queries)                   │  │
│  │  • avx_deploy (K8s deployments)                  │  │
│  │  • avx_telemetry (Metrics)                       │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Avila Experience Fabric                     │
│  • AvilaDB (Database)                                   │
│  • Kubernetes Clusters                                   │
│  • Service Mesh (Istio)                                 │
│  • Telemetry System                                     │
└─────────────────────────────────────────────────────────┘
```

## Componentes

### 1. Protocol Layer (`avx-mcp/src/protocol/`)

Implementa o Model Context Protocol (MCP) usando JSON-RPC 2.0:

- **types.rs**: Tipos MCP (Request, Response, Error)
- Lifecycle: `initialize` → `initialized`
- Métodos:
  - `resources/list`: Lista resources disponíveis
  - `resources/read`: Lê conteúdo de um resource
  - `tools/list`: Lista tools disponíveis
  - `tools/call`: Executa uma tool

### 2. Resources Layer (`avx-mcp/src/resources/`)

Expõe dados e configurações como resources:

#### AvilaDB Resources (`aviladb.rs`)
```rust
aviladb://production/users
aviladb://production/events
```

#### AVX Config Resources (`avx_config.rs`)
```rust
avx://config/stack
avx://config/mesh
```

#### Cluster Resources (`cluster.rs`)
```rust
avx://cluster/production
avx://cluster/staging
```

### 3. Tools Layer (`avx-mcp/src/tools/`)

Implementa ações que LLMs podem executar:

#### avx_query (`aviladb_query.rs`)
Query no AvilaDB com filtros JSON.

**Input Schema:**
```json
{
  "database": "string",
  "collection": "string",
  "query": "string (JSON)",
  "limit": "number"
}
```

#### avx_deploy (`deploy.rs`)
Deploy de serviços no Kubernetes.

**Input Schema:**
```json
{
  "service": "gateway|api-core|events",
  "cluster": "string",
  "namespace": "string",
  "replicas": "number",
  "image": "string (optional)"
}
```

#### avx_telemetry (`telemetry.rs`)
Coleta métricas em tempo real.

**Input Schema:**
```json
{
  "service": "string",
  "metric": "cpu|memory|requests|latency|errors",
  "cluster": "string",
  "timeRange": "string"
}
```

### 4. Server (`avx-mcp/src/server.rs`)

Loop principal que:
1. Lê requests do stdin (JSON-RPC)
2. Roteia para o handler apropriado
3. Executa a ação (resource read ou tool call)
4. Retorna response no stdout

```rust
loop {
    let request = read_stdin();
    let response = match request.method {
        "initialize" => handle_initialize(),
        "resources/list" => handle_resources_list(),
        "tools/call" => handle_tools_call(),
        _ => error("Method not found")
    };
    write_stdout(response);
}
```

### 5. CLI (`avx-cli/src/main.rs`)

Interface de linha de comando com dois modos:

#### Modo K8s
Gera manifests Kubernetes:
```bash
avx-cli k8s --service gateway --replicas 3
```

#### Modo MCP
Gerencia o servidor MCP:
```bash
avx-cli mcp serve      # Inicia servidor
avx-cli mcp resources  # Lista resources
avx-cli mcp tools      # Lista tools
avx-cli mcp test       # Testa configuração
```

### 6. Config Library (`avx-config/src/lib.rs`)

Biblioteca compartilhada para configuração AVX:

```rust
pub struct AvxConfig {
    pub stack: String,    // AVX__STACK
    pub layer: String,    // AVX__LAYER
    pub env: String,      // AVX__ENV
    pub cluster: String,  // AVX__CLUSTER
    pub mesh: String,     // AVX__MESH
}
```

## Fluxo de Dados

### 1. Inicialização

```
Claude Desktop
    │
    │ {"method": "initialize", ...}
    ▼
MCP Server
    │
    │ Registra resources e tools
    │
    │ {"result": {"capabilities": {...}}}
    ▼
Claude Desktop
    │
    │ {"method": "initialized"}
    ▼
MCP Server (pronto!)
```

### 2. Listar Resources

```
Claude: "Liste os resources disponíveis"
    │
    │ {"method": "resources/list"}
    ▼
MCP Server
    │
    │ get_all_resources()
    │
    │ {"result": {"resources": [...]}}
    ▼
Claude: Mostra lista para o usuário
```

### 3. Executar Tool

```
Claude: "Faça query no AvilaDB"
    │
    │ {"method": "tools/call", "params": {
    │   "name": "avx_query",
    │   "arguments": {"database": "prod", ...}
    │ }}
    ▼
MCP Server
    │
    │ execute_query(args)
    │ ↓
    │ [Conecta no AvilaDB]
    │ ↓
    │ Retorna resultados
    │
    │ {"result": {"content": [{"type": "text", ...}]}}
    ▼
Claude: Processa e mostra resultados
```

## Async Runtime

Todo o servidor usa **Tokio** como async runtime:

```rust
#[tokio::main]
async fn main() {
    let mut server = McpServer::new();
    server.run().await
}
```

Benefícios:
- Operações I/O não bloqueantes
- Suporte a múltiplas conexões
- Performance otimizada

## Logging e Tracing

Sistema de logging com **tracing**:

```rust
tracing::info!("🚀 Server starting");
tracing::debug!("Received: {}", request);
tracing::error!("Failed: {}", error);
```

Níveis:
- `RUST_LOG=avx_mcp=info` (padrão)
- `RUST_LOG=avx_mcp=debug` (verbose)
- `RUST_LOG=avx_mcp=trace` (muito verbose)

## Segurança

- **Validação de Input**: Todos os inputs são validados
- **Type Safety**: Rust garante segurança de tipos
- **Error Handling**: Errors são tratados e logados
- **No Panics**: Evita panics em produção

## Performance

- **Zero-copy quando possível**: Usa referências
- **Async I/O**: Não bloqueia threads
- **Serialização eficiente**: serde_json
- **Minimal allocations**: Reusa buffers

## Extensibilidade

### Adicionar novo Resource

1. Criar arquivo em `resources/`
2. Implementar função que retorna `Vec<Resource>`
3. Adicionar em `resources/mod.rs::get_all_resources()`

### Adicionar nova Tool

1. Criar arquivo em `tools/`
2. Implementar função que retorna `Tool` com schema
3. Adicionar handler em `server.rs::handle_tools_call()`
4. Adicionar em `tools/mod.rs::get_all_tools()`

## Deployment

### Desenvolvimento
```bash
cargo run -p avx-cli -- mcp serve
```

### Produção
```bash
cargo build --release
./target/release/avx-cli mcp serve
```

### Docker (futuro)
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/avx-cli /usr/local/bin/
CMD ["avx-cli", "mcp", "serve"]
```

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Manual Testing
```bash
# Terminal 1
avx-cli mcp serve

# Terminal 2
echo '{"jsonrpc":"2.0","id":1,"method":"initialize",...}' | nc localhost 8080
```
