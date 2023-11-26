# rust
# REST API com Rust

Este é um projeto de exemplo que demonstra como criar uma API REST usando Rust.

## Tecnologias utilizadas

- Linguagem de programação: Rust
- Framework web: Axum
- Serialização: Serde
- UUID: uuid
- Documentação da API: utoipa e utoipa-swagger-ui

## Funcionalidades

A API suporta as seguintes operações CRUD em um recurso de produto:

- GET /products: Lista todos os produtos
- POST /products: Adiciona um novo produto
- DELETE /products/{id}: Deleta um produto pelo ID

## Como executar

1. Certifique-se de ter o Rust e o Cargo instalados em seu sistema.
2. Clone este repositório.
3. Navegue até o diretório do projeto.
4. Execute `cargo run` para iniciar o servidor.
5. Acesse `http://localhost:8080/swagger-ui` para visualizar a documentação da API.

## Nota

Este projeto é apenas para fins de demonstração e não deve ser usado em produção.