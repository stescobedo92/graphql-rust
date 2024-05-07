# graphql-rust

This is a simple example demonstrating how to use Async GraphQL with Axum to create a GraphQL API server.

## Overview

This example consists of three modules:

- `db`: Contains the database related functionalities.
- `query_engine`: Defines the GraphQL query types and resolver functions.
- `user_service`: Placeholder for user-related functionalities.

## Getting Started

To run this example, follow these steps:

1. Clone this repository.
2. Navigate to the project directory.
3. Build the project using `cargo build`.
4. Run the project with `cargo run`.

## Usage

Once the server is running, you can send GraphQL queries to `http://localhost:3000/gql` using POST requests.

## Dependencies

This project relies on the following dependencies:

- `async-graphql`: For building GraphQL servers asynchronously.
- `async-graphql-axum`: Integration between Async GraphQL and Axum.
- `axum`: A web framework built on top of Tokio and Hyper.

## Example

```graphql
query dump{
   getUsers {
     name
   }
}
```