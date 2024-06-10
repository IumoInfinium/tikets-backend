# Tikets

## What is tikets?

A fast, simple, multi-threaded and type safe asynchronous REST API in rust managing the tickets or todos.

## Why it is fast?

Tikets is fast and simple due to axum, a lightweight and speedy rust based web application framework.

Tikets is made using

1. Tokio
2. Axum
3. Serde
4. thiserror
5. uuid

## Internals

The exposed API's use JSON based serialization and deserialization, on top of the base low-level private internal API's.

Tikets, can be created, updated, deleted, viewed and can be run as a backend service for communicating the frontend based tiket lister.