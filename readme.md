<p align="center">
  <img src="https://raw.githubusercontent.com/lord/img/master/logo-backtalk.png" alt="Backtalk: API Web Server" width="226">
  <br>
  <a href="https://travis-ci.org/lord/slate"><img src="https://travis-ci.org/lord/backtalk.svg?branch=master" alt="Build Status"></a>
  <a href="https://crates.io/crates/backtalk"><img src="https://img.shields.io/crates/v/backtalk.svg" alt="Crate Info"></a>
  <a href="https://docs.rs/backtalk"><img src="https://img.shields.io/badge/docs.rs-visit-green.svg" alt="Documentation"></a>
</p>

Backtalk is an experimental asynchronous web framework for Rust. We try to provide simple tools that are easily composed and extended.

- [ ] Create HTTP Request and Response structs.
- [ ] more convenience functions/stuff for non-API routes.
- [ ] simplify resource.rs stuff, maybe it can just return a value directly? and users can wrap with a serialization function if they want, but by default, it's values.
- [ ] simplify api::Request, maybe switch back to enum of operations?
- [ ] have a websockets/realtime notifications plan
- [ ] Add `DatabaseResource` that just accepts a Diesel object/db connection and automatically becomes a full resource.
- [ ] parse query string properly
- [ ] `handle` should return a string and accept a serializer
- [ ] websocket serving with `ws-rs`
- [ ] write JS library (backtalk.js) for frontend to talk to Backtalk
- [x] upgrade to Hyper proper once it integrates with Tokio
- [ ] Remove all unwraps

## Objects

- `Resource` is an object that receives requests. Usually corresponds to a particular type of object. Allows adding hooks and methods and error handlers.
- `Request` is a request for data, either over HTTP or Websockets
- `Reply` is a response object representing JSON/BSON data that will be returned to the client, and a HTTP status (from a subset of subset of the messages)
- `Guard` is a function that accepts a Request and returns a Future<Request, Reply>.
- `Filter` is a function that accepts a Reply and returns a Future<Reply, Reply>.
