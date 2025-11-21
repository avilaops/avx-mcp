# Exemplos de Uso do AVX MCP

## 1. Query AvilaDB

### Buscar usuários ativos

```bash
# Via tool call
{
  "tool": "avx_query",
  "arguments": {
    "database": "production",
    "collection": "users",
    "query": "{\"status\": \"active\"}",
    "limit": 50
  }
}
```

### Buscar eventos recentes

```bash
{
  "tool": "avx_query",
  "arguments": {
    "database": "production",
    "collection": "events",
    "query": "{\"timestamp\": {\"$gte\": \"2025-11-20T00:00:00Z\"}}",
    "limit": 100
  }
}
```

## 2. Deploy de Serviços

### Deploy Gateway para Produção

```bash
# Via CLI
avx-cli k8s --service gateway \
  --namespace avx-core \
  --replicas 5 \
  --output ./k8s/gateway.yaml

kubectl apply -f ./k8s/gateway.yaml
```

```bash
# Via tool call
{
  "tool": "avx_deploy",
  "arguments": {
    "service": "gateway",
    "cluster": "production",
    "namespace": "avx-core",
    "replicas": 5
  }
}
```

### Deploy API Core para Staging

```bash
{
  "tool": "avx_deploy",
  "arguments": {
    "service": "api-core",
    "cluster": "staging",
    "namespace": "avx-staging",
    "replicas": 2,
    "image": "ghcr.io/avilaops/api-core:v2.1.0"
  }
}
```

## 3. Telemetria

### Monitorar CPU do Gateway

```bash
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "cpu",
    "cluster": "production",
    "timeRange": "1h"
  }
}
```

### Monitorar Latência da API

```bash
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "api-core",
    "metric": "latency",
    "cluster": "production",
    "timeRange": "24h"
  }
}
```

### Dashboard Completo

```bash
# CPU
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "cpu",
    "timeRange": "1h"
  }
}

# Memory
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "memory",
    "timeRange": "1h"
  }
}

# Requests
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "requests",
    "timeRange": "1h"
  }
}

# Errors
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "errors",
    "timeRange": "1h"
  }
}
```

## 4. Leitura de Resources

### Ler configuração do stack

```bash
# Resource URI
avx://config/stack
```

### Ler dados de usuários

```bash
# Resource URI
aviladb://production/users
```

### Informações do cluster

```bash
# Resource URI
avx://cluster/production
```

## 5. Workflow Completo: Deploy de Nova Feature

### Passo 1: Gerar manifests

```bash
# Gateway
avx-cli k8s --service gateway \
  --image ghcr.io/avilaops/gateway:v2.0.0 \
  --replicas 3 \
  --output ./deploy/gateway.yaml

# API Core
avx-cli k8s --service api-core \
  --image ghcr.io/avilaops/api-core:v2.0.0 \
  --replicas 3 \
  --output ./deploy/api-core.yaml
```

### Passo 2: Deploy para staging

```bash
kubectl apply -f ./deploy/gateway.yaml --namespace avx-staging
kubectl apply -f ./deploy/api-core.yaml --namespace avx-staging
```

### Passo 3: Verificar métricas

```bash
# Via MCP tool
{
  "tool": "avx_telemetry",
  "arguments": {
    "service": "gateway",
    "metric": "errors",
    "cluster": "staging",
    "timeRange": "30m"
  }
}
```

### Passo 4: Deploy para produção

```bash
kubectl apply -f ./deploy/gateway.yaml --namespace avx-core
kubectl apply -f ./deploy/api-core.yaml --namespace avx-core
```

## 6. Uso com Claude

### Perguntar ao Claude

```
"Liste todos os usuários ativos no AvilaDB de produção"
```

```
"Faça o deploy do gateway para o cluster de staging com 2 réplicas"
```

```
"Mostre as métricas de CPU dos últimos 24 horas do api-core"
```

```
"Qual é a configuração atual do service mesh?"
```

## 7. Scripts de Automação

### PowerShell: Deploy Completo

```powershell
# deploy-all.ps1
$services = @("gateway", "api-core", "events")

foreach ($service in $services) {
    Write-Host "🚀 Deploying $service..."

    avx-cli k8s --service $service `
        --namespace avx-core `
        --replicas 3 `
        --output "./deploy/$service.yaml"

    kubectl apply -f "./deploy/$service.yaml"

    Write-Host "✅ $service deployed"
}
```

### Bash: Monitoring Loop

```bash
#!/bin/bash
# monitor.sh

while true; do
    echo "📊 Checking metrics..."

    avx-cli mcp test

    sleep 60
done
```

## 8. Integração CI/CD

### GitHub Actions

```yaml
name: Deploy AVX
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install AVX CLI
        run: cargo install --path avx-cli

      - name: Generate manifests
        run: |
          avx-cli k8s --service gateway --output gateway.yaml
          avx-cli k8s --service api-core --output api-core.yaml

      - name: Deploy to Kubernetes
        run: |
          kubectl apply -f gateway.yaml
          kubectl apply -f api-core.yaml
```

## 9. Desenvolvimento Local

### Iniciar servidor MCP local

```bash
# Terminal 1: Servidor
avx-cli mcp serve

# Terminal 2: Testar
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | avx-cli mcp serve
```

### Variáveis de ambiente de dev

```bash
export AVX__STACK=development
export AVX__LAYER=core
export AVX__ENV=dev
export AVX__CLUSTER=local
export AVX__MESH=none
export RUST_LOG=avx_mcp=debug
```
