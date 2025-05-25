# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição
Este projeto é uma implementação de um sistema de busca otimizado utilizando Rust para a loja fictícia MegaStore, focando em desempenho, escalabilidade e uso de tabelas hash.

## Tecnologias
- Rust
- HashMap (estrutura de dados para indexação)
- Serde/Serde JSON (para persistência futura)

## Como executar

```bash
cargo build
cargo run
```

## Como testar

```bash
cargo test
```

## Exemplos de uso
- Buscar por termos simples como “Dell” ou “Samsung”

## Arquitetura
- `lib.rs`: lógica de indexação e busca
- `main.rs`: entrada principal
- `search_tests.rs/`: testes de unidade

## Estruturas de dados
- `HashMap<String, Vec<u32>>` para indexação por palavras-chave

## Escalabilidade
- Pode ser adaptado para indexação por múltiplos critérios e carregamento por arquivo

Desenvolvido por Yuri de Oliveira Melo
