# ⌨️ Conduit Axum

![version](https://img.shields.io/badge/version-0.1.0-green) [![license](https://img.shields.io/badge/license-MIT-blue)](./LICENSE) ![axum](https://img.shields.io/badge/axum-0.7.5-a21caf.svg) ![jwt](https://img.shields.io/badge/jwt-9.3.0-d63aff.svg) ![postgresql](https://img.shields.io/badge/postgresql-16.2-336792.svg) ![sqlx](https://img.shields.io/badge/sqlx-0.7.4-orange.svg) ![cargo](https://img.shields.io/badge/cargo-1.77.1-black.svg)


## 💡 Introduction

Realworld: "The mother of all demo apps" — Exemplary SSR fullstack Medium.com clone (called [Conduit](https://github.com/yoonge/conduit-axum)), built with Axum + JWT + PostgreSQL + SQLx.

Besides，this repository also provides RESTful APIs for [Conduit React](https://github.com/yoonge/conduit-react).


## 🔰 Getting Started

```sh
$ git clone https://github.com/yoonge/conduit-axum.git

$ cd conduit-axum

# Change the URL with your own username, password, and database_name
$ echo "DATABASE_URL=postgresql://username:password@localhost:5432/database_name" > .env

$ echo "RUST_LOG=debug" >> .env

$ cargo install sqlx-cli

$ sqlx migrate run

$ cargo run
```


<!-- ## 📁 Index -->


<!-- ## ⚡ Features -->


<!-- ## 📌 TODO -->


## 📄 License

Conduit Axum is [MIT-licensed](./LICENSE).


<!-- ## 🔗 Links -->


----


## 🏗️ Scaffold

```sh
$ cargo init conduit-axum && cd conduit-axum
```
