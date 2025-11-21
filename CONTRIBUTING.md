# Contributing to AVX MCP

Obrigado por considerar contribuir para o AVX MCP! 🎉

## 🚀 Como Contribuir

### Reportar Bugs

Abra uma issue com:
- Descrição clara do problema
- Steps para reproduzir
- Comportamento esperado vs atual
- Versão do Rust e sistema operacional

### Sugerir Features

Abra uma issue com:
- Descrição da feature
- Use case e motivação
- Exemplos de uso

### Pull Requests

1. Fork o repositório
2. Crie uma branch: `git checkout -b feature/nova-feature`
3. Faça suas mudanças
4. Teste: `cargo test`
5. Format: `cargo fmt`
6. Lint: `cargo clippy`
7. Commit: `git commit -m 'Add: nova feature'`
8. Push: `git push origin feature/nova-feature`
9. Abra um Pull Request

## 📝 Padrões de Código

- Use `cargo fmt` antes de commitar
- Siga as convenções Rust (rustfmt)
- Adicione testes para novas features
- Documente funções públicas
- Use tipos seguros (evite `unwrap()` em produção)

## 🧪 Testes

```bash
# Rodar todos os testes
cargo test

# Rodar testes específicos
cargo test --package avx-mcp

# Com output
cargo test -- --nocapture
```

## 📖 Documentação

```bash
# Gerar docs
cargo doc --open

# Verificar docs
cargo doc --no-deps
```

## 🎯 Prioridades

1. Estabilidade do protocolo MCP
2. Expansão de tools e resources
3. Integração com mais serviços AVX
4. Performance e otimizações
5. Documentação e exemplos

## 💬 Comunicação

- Issues: Para bugs e features
- Discussions: Para perguntas e ideias
- Email: dev@avila.inc

## 📄 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob MIT OR Apache-2.0.
