# rust-webserver

This is a simple multithreaded web server written in Rust language for practice.
Please don't use this for any production.

## Requirements

- `Rust==1.83.0`
- `Cargo==1.83.0`

## Dependencies

## Modules
#### *mod* `webserver::http`

This module defines some HTTP constants

#### *struct* `webserver::http::Protocol` 

A `struct` to define protocol.

#### *mod* `webserver::http::PROTOCOL`

Define constants for each HTTP versions. Including *http/1.0*, *http/1.1*, *http/2.0*

#### *fn* `webserver::http::get_protocol_from_str`

Return a protocol constant by string recorded in request content.

#### *stuct* `webserver::http::Method`

A `struct` to define HTTP Method.

#### *mod* `webserver::http::METHOD`

Define constants for each HTTP methods. Including *GET*, *POST*, *PUT*, *PATCH*, *DELETE*, *HEAD* and *OPTION*

#### *fn* `webserver::http::get_method_from_str`

Return a method constant by string recorded in request content.

#### *struct* `webserver::http::Status`

A `struct` to define response status codes.

#### *mod* `webserver::http::STATUS`

Define constants for each HTTP response status codes. Now only some frequently used status codes are implemented.

#### *fn* `webserver::http::get_status_from_code`

Return a status constant by status code.

#### *mod* `webserver::request`

Module to parse and handle http requests.

#### *struct* `webserver::request::Request`

A `struct` to put the structralized http request content in.

#### *mod* `webserver::response`

Construct responses

TODO

#### *mod* `webserver::response::template`

To make responses from templates.

TODO 

#### *mod* `webserver::thread_pool`

This module implements worker and thread pool to make the web server multithreaded.

TODO 

## Run

```
$ Cargo run
```

## Reference
[Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
