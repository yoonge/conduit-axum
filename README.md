# ⌨️ Conduit Axum

![version](https://img.shields.io/badge/version-0.1.0-green) [![license](https://img.shields.io/badge/license-MIT-blue)](./LICENSE) ![axum](https://img.shields.io/badge/axum-0.7.5-a21caf.svg) ![jsonwebtoken](https://img.shields.io/badge/jsonwebtoken-9.3.0-d63aff.svg) ![postgresql](https://img.shields.io/badge/postgresql-16.2-336792.svg) ![rust](https://img.shields.io/badge/rust-1.77.1-black.svg) ![sqlx](https://img.shields.io/badge/sqlx-0.7.4-orange.svg) ![cargo](https://img.shields.io/badge/cargo-1.77.1-black.svg)


## 💡 Introduction

Realworld: "The mother of all demo apps" — Exemplary back-end Medium.com clone (called [Conduit](https://github.com/yoonge/conduit-axum)) in Rust, built with Axum + jsonwebtoken + PostgreSQL + SQLx, it provides RESTful APIs for [Conduit React](https://github.com/yoonge/conduit-react).


## 🔰 Getting Started

```sh
$ git clone https://github.com/yoonge/conduit-axum.git

$ cd conduit-axum

# Change `.env` file with your own username, password, database_name etc.
$ cp .env.example .env

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
