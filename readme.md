# Backtalk

A web framework for APIs in Rust.

- TODO Tokio

## Objects

- `Service` is an object that receives requests. Usually corresponds to a particular type of object. Allows adding hooks and methods and error handlers.
- `Request` is a request for data, either over HTTP or Websockets
- `Response` is a response object representing JSON/BSON data that will be returned to the client
- `Hook` is a function that accepts a Request and returns a HookResult.
- `HookResult` is either an error or a modified request.
