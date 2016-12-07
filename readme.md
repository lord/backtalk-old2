<p align="center">
  <img src="https://raw.githubusercontent.com/lord/img/master/logo-backtalk.png" alt="Backtalk: API Web Server" width="226">
  <br>
  <a href="https://travis-ci.org/lord/slate"><img src="https://travis-ci.org/lord/backtalk.svg?branch=master" alt="Build Status"></a>
</p>

A web framework for APIs in Rust.

- [x] add proper `find`, `get`, `create`, `update`, `patch`, `remove` (`setup`?) with `Vec<Self::Object>` results
- [x] add request and response structs
- [x] multiple resources per resource server
- [x] custom error handlers for server
- [x] call resource methods properly based on what was called
- [ ] create `Data` struct that corresponds to a non-error response from server. probably wraps/reexports serde's json Value
- [ ] remove all references to `hyper` from everywhere except `server.rs`.
- [ ] add guards (request->future<request,error> map) and filters (response->future<response,error> map)
- [ ] Add `DatabaseResource` that just accepts a Diesel object/db connection and automatically becomes a full resource.
- [ ] parse query string properly
- [ ] `handle` should return a string and accept a serializer
- [ ] websocket serving with `ws-rs`
- [ ] write JS library (backtalk.js) for frontend to talk to Backtalk
- [ ] upgrade to Hyper proper once it integrates with Tokio
- [ ] Remove all unwraps

## Objects

- `Resource` is an object that receives requests. Usually corresponds to a particular type of object. Allows adding hooks and methods and error handlers.
- `Request` is a request for data, either over HTTP or Websockets
- `Reply` is a response object representing JSON/BSON data that will be returned to the client, and a HTTP status (from a subset of subset of the messages)
- `Guard` is a function that accepts a Request and returns a Future<Request, Reply>.
- `Filter` is a function that accepts a Reply and returns a Future<Reply, Reply>.
