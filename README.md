# Actix User Auth API

A user authorization API where access to endpoints is restricted using JWTs.


## Tech Stack

**Server:** Rust, Actix-Web, SurrealDB


## Installation

This project uses SurrealDB to store users. Check out the [installation instructions](https://surrealdb.com/docs/surrealdb/installation) for details on how to install.

## Run Locally

Clone the project

```bash
  git clone https://github.com/rosswilson2306/actix_auth_api.git
  cd actix_auth_api
```

Start surrealDB on default port 8000

```bash
    surreal start file:actix_users.db --user root --password root
```

Install dependencies and start the server on port 8080

```bash
    cargo run
```

---
TODO

## API Reference

#### Get all items

```http
  GET /auth/login
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `api_key` | `string` | **Required**. Your API key |

#### Get item

```http
  GET /auth/verify/${token}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `token`   | `string` | **Required**. JWT to verify |
