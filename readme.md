# Backtalk

A web framework for APIs in Rust.

- [x] add proper `find`, `get`, `create`, `update`, `patch`, `remove` (`setup`?) with `Vec<Self::Object>` results
- [x] add request and response structs
- [x] multiple resources per resource server
- [x] custom error handlers for server
- [x] call resource methods properly based on what was called
- [ ] add hooks/filters
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
- `Response` is a response object representing JSON/BSON data that will be returned to the client
- `Hook` is a function that accepts a Request and returns a HookResult.
- `HookResult` is either an error or a modified request.
