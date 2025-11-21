# Configuração de Exemplo para Claude Desktop

Este arquivo mostra como configurar o AVX MCP Server no Claude Desktop.

## Localização do Arquivo

### Windows
```
%APPDATA%\Claude\claude_desktop_config.json
```

### macOS
```
~/Library/Application Support/Claude/claude_desktop_config.json
```

### Linux
```
~/.config/Claude/claude_desktop_config.json
```

## Configuração Completa

```json
{
  "mcpServers": {
    "avx": {
      "command": "avx-cli",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "production",
        "AVX__LAYER": "core",
        "AVX__ENV": "prod",
        "AVX__CLUSTER": "us-east-1",
        "AVX__MESH": "istio",
        "RUST_LOG": "avx_mcp=info"
      }
    }
  }
}
```

## Configuração com Caminho Completo

Se `avx-cli` não estiver no PATH:

```json
{
  "mcpServers": {
    "avx": {
      "command": "C:\\Users\\YourUser\\.cargo\\bin\\avx-cli.exe",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "production",
        "AVX__CLUSTER": "us-east-1"
      }
    }
  }
}
```

## Múltiplos Ambientes

Você pode configurar múltiplos servidores MCP para diferentes ambientes:

```json
{
  "mcpServers": {
    "avx-production": {
      "command": "avx-cli",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "production",
        "AVX__ENV": "prod",
        "AVX__CLUSTER": "us-east-1"
      }
    },
    "avx-staging": {
      "command": "avx-cli",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "staging",
        "AVX__ENV": "staging",
        "AVX__CLUSTER": "us-west-2"
      }
    },
    "avx-dev": {
      "command": "avx-cli",
      "args": ["mcp", "serve"],
      "env": {
        "AVX__STACK": "development",
        "AVX__ENV": "dev",
        "AVX__CLUSTER": "local"
      }
    }
  }
}
```

## Variáveis de Ambiente Disponíveis

| Variável | Descrição | Padrão |
|----------|-----------|--------|
| `AVX__STACK` | Nome do stack | `default` |
| `AVX__LAYER` | Layer da arquitetura | `core` |
| `AVX__ENV` | Ambiente | `dev` |
| `AVX__CLUSTER` | Nome do cluster | `local` |
| `AVX__MESH` | Service mesh | `default` |
| `RUST_LOG` | Nível de log | `avx_mcp=info` |

## Após Configurar

1. Salve o arquivo `claude_desktop_config.json`
2. Reinicie o Claude Desktop completamente
3. Abra uma nova conversa
4. O AVX MCP Server estará disponível! 🎉

Você pode verificar se está funcionando pedindo ao Claude:

```
"Liste os resources disponíveis no AVX MCP"
```

ou

```
"Quais tools estão disponíveis no servidor AVX?"
```

## Troubleshooting

### Servidor não aparece

1. Verifique se `avx-cli` está instalado: `avx-cli --version`
2. Verifique o caminho completo: `where avx-cli` (Windows) ou `which avx-cli` (Unix)
3. Teste manualmente: `avx-cli mcp test`
4. Verifique os logs do Claude Desktop

### Logs

Para ver logs detalhados, adicione:

```json
"env": {
  "RUST_LOG": "avx_mcp=debug,avx_cli=debug"
}
```

### Permissões

No Windows, você pode precisar adicionar `avx-cli` às exceções do Windows Defender ou firewall.
