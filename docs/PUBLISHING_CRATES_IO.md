# Como Publicar no Crates.io

## Passo 1: Login no Crates.io

1. Crie conta em https://crates.io
2. Vá em Account Settings → API Tokens
3. Gere novo token
4. Execute:

```bash
cargo login
# Cole o token quando solicitado
```

## Passo 2: Verificar Pacotes

```bash
# Verificar se está tudo ok
cargo package --list -p avx-config
cargo package --list -p avx-mcp
cargo package --list -p avx-cli

# Dry run (simula publicação)
cargo publish --dry-run -p avx-config
cargo publish --dry-run -p avx-mcp
cargo publish --dry-run -p avx-cli
```

## Passo 3: Publicar na Ordem Correta

**IMPORTANTE**: Publicar na ordem de dependências!

```bash
# 1. Publicar avx-config primeiro (não tem dependências)
cd avx-config
cargo publish
# Aguardar 1-2 minutos para indexar

# 2. Publicar avx-mcp (depende de avx-config)
cd ../avx-mcp
cargo publish
# Aguardar 1-2 minutos

# 3. Publicar avx-cli (depende de avx-config e avx-mcp)
cd ../avx-cli
cargo publish
```

## Passo 4: Verificar

Após alguns minutos, verificar em:
- https://crates.io/crates/avx-config
- https://crates.io/crates/avx-mcp
- https://crates.io/crates/avx-cli

## Passo 5: Testar Instalação

```bash
# Agora funcionará!
cargo install avx-cli
```

## Troubleshooting

### Erro: "crate name already taken"
```bash
# Escolher outro nome
# Editar Cargo.toml:
name = "avx-cli-avilaops"  # ou outro nome único
```

### Erro: "dependency not found"
```bash
# Aguardar indexação (1-2 minutos)
# Ou publicar dependências primeiro
```

### Erro: "missing description"
```bash
# Adicionar em Cargo.toml:
description = "Sua descrição aqui"
license = "MIT OR Apache-2.0"
repository = "https://github.com/avilaops/avx-mcp"
```

## Atualizar Versão Futura

```bash
# 1. Editar Cargo.toml (aumentar version)
version = "0.1.1"

# 2. Commit
git add .
git commit -m "chore: bump version to 0.1.1"
git tag v0.1.1
git push --tags

# 3. Publicar
cargo publish -p avx-config
cargo publish -p avx-mcp
cargo publish -p avx-cli
```

## Depois de Publicar

Atualizar README.md com:

```markdown
## 📦 Instalação

### Via Cargo (Recomendado)
\`\`\`bash
cargo install avx-cli
\`\`\`

### Via GitHub
\`\`\`bash
cargo install --git https://github.com/avilaops/avx-mcp avx-cli
\`\`\`
```
