# Backtalk

A web framework for APIs in Rust.

- [ ] rename `Resource` to something better (not `Service`, with Tokio that's confusing)
- [ ] add request and response structs
- [x] add proper `find`, `get`, `create`, `update`, `patch`, `remove` (`setup`?) with `Vec<Self::Object>` results
- [ ] add hooks/filters
- [ ] Add `DatabaseResource` that just accepts a Diesel object/db connection and automatically becomes a full resource.
- [ ] websocket serving with `ws-rs`
- [ ] add messagepack serialization
- [ ] write JS library (backtalk.js) for frontend to talk to Backtalk
- [ ] upgrade to Hyper proper once it integrates with Tokio
- [ ] Remove all unwraps

## Objects

- `Resource` is an object that receives requests. Usually corresponds to a particular type of object. Allows adding hooks and methods and error handlers.
- `Request` is a request for data, either over HTTP or Websockets
- `Response` is a response object representing JSON/BSON data that will be returned to the client
- `Hook` is a function that accepts a Request and returns a HookResult.
- `HookResult` is either an error or a modified request.
