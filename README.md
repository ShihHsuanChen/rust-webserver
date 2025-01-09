# rust-webserver

This is a simple multithreaded web server written in Rust language for practice.
Please don't use this for any production.

## Requirements

- `Rust==1.83.0`
- `Cargo==1.83.0`

## Dependencies

## Modules
### `http`

This module defines some HTTP constants

#### METHOD

HTTP Methods including *GET*, *POST*, *PUT*, *PATCH*, *DELETE*, *HEAD* and *OPTION*

#### PROTOCOL

Including *http/1.0*, *http/1.1*, *http/2.0*

### `request`

Parse HTTP requests.

### `response`

Construct responses

#### `template`

To make responses from templates.

### `thread_pool`

This module implements worker and thread pool to make the web server multithreaded.

## Run

```
$ Cargo run
```

## Reference
[Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
