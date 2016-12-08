use hyper::server as http;
use hyper;
use tokio_service::Service;
use futures::{BoxFuture, Future, finished};
use ::api::{Error, Value, Request, RequestType, ErrorKind, Params};
use api;
use serde_json;

pub struct Server<T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static> {
  func: T,
}
impl <T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static> Server<T> {
  pub fn run<U>(bind: &str, func: U)
    where U: Fn() -> T + 'static + Send {

    let server = http::Server::http(&bind.parse().unwrap()).unwrap();
    let (listening, server) = server.standalone(move || {
      let srv = Server{func: func()};
      Ok(srv)
    }).unwrap();

    println!("Listening on http://{}", listening);
    server.run();
  }
}

impl <T> Service for Server<T>
  where T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static {
  type Request = http::Request;
  type Response = http::Response;
  type Error = hyper::Error;
  type Future = BoxFuture<Self::Response, hyper::Error>;

  fn call(&self, req: Self::Request) -> Self::Future {
    (self.func)(req)
  }
}

pub fn wrap_api<T>(http_request: http::Request, api_func: &T) -> BoxFuture<http::Response, hyper::Error>
  where T: Fn(api::Request) -> BoxFuture<api::Value, api::Error> + 'static {
  let uri = http_request.uri();
  let method = http_request.method();
  let request = match process_http_request(method, uri, None) { // TODO ACTUALLY GET BODY
    Ok(req) => req,
    Err(err) => return DefaultErrorHandler{}.handle_http(err),
  };
  api_func(request).then(move |res| {
    match res {
      Ok(val) => DefaultValueHandler{}.handle_http(val),
      Err(err) => DefaultErrorHandler{}.handle_http(err),
    }
  }).boxed()
}

fn process_http_request(method: &hyper::Method, path: &hyper::RequestUri, body: Option<&str>) -> Result<Request, Error> {
  let path_str = if let &hyper::RequestUri::AbsolutePath{ref path, ..} = path {
    path
  } else {
    return Err(Error {
      msg: "Invalid path, sorry".to_string(),
      kind: ErrorKind::RemoveThis
    })
  };

  let (resource_name, resource_id) = try!(parse_url(&path_str));
  let body_val = match body {
    Some(s) => Some(try!(parse_json(s))),
    None => None,
  };
  let id_val = match resource_id {
    Some(s) => Some(try!(parse_json(s))),
    None => None,
  };
  let req_type = match method {
    // TODO should we handle HEAD requests?
    &hyper::Method::Get => {
      if let Some(_) = resource_id {
        RequestType::Get
      } else {
        RequestType::Find
      }
    }
    &hyper::Method::Post => RequestType::Create,
    &hyper::Method::Put => RequestType::Update,
    &hyper::Method::Patch => RequestType::Patch,
    &hyper::Method::Delete => RequestType::Remove,
    _ => return Err(Error {
      msg: "We don't respond to that HTTP method, sorry.".to_string(),
      kind: ErrorKind::RemoveThis
    }),
  };
  let req = Request {
    resource: resource_name.to_string(),
    request_type: req_type,
    params: Params::new(), // TODO PARSE PARAMS
    object: body_val,
    id: id_val,
  };
  if req.validate() {
    Ok(req)
  } else {
    Err(Error {
      msg: "Invalid request, missing either a body or id or something.".to_string(),
      kind: ErrorKind::RemoveThis
    })
  }
}

fn parse_json(json_str: &str) -> Result<Value, Error> {
  serde_json::from_str::<Value>(&json_str).map_err(|err| {
    Error {
      msg: err.to_string(),
      kind: ErrorKind::RemoveThis
    }
  })
}

fn parse_url(path: &str) -> Result<(&str, Option<&str>), Error> {
  let mut uri = path.split('/').skip(1);

  let resource_name = match uri.next() {
    None | Some("") => return Err(Error {
      msg: "enter a resource name!".to_string(),
      kind: ErrorKind::RemoveThis
    }),
    Some(v) => v,
  };

  let resource_id = uri.next().and_then(|id| {
    if id.is_empty() {
      None
    } else {
      Some(id)
    }
  });

  Ok((resource_name, resource_id))
}

pub trait ErrorHandler: 'static + Sync + Send {
  fn handle_http(&self, Error) -> BoxFuture<http::Response, hyper::Error>;
}

pub trait ValueHandler: 'static + Sync + Send {
  fn handle_http(&self, Value) -> BoxFuture<http::Response, hyper::Error>;
}

struct DefaultErrorHandler;
impl ErrorHandler for DefaultErrorHandler {
  fn handle_http(&self, err: Error) -> BoxFuture<http::Response, hyper::Error> {
    let resp = http::Response::new().body(err.msg.into_bytes());
    finished(resp).boxed()
  }
}

struct DefaultValueHandler;
impl ValueHandler for DefaultValueHandler {
  fn handle_http(&self, val: Value) -> BoxFuture<http::Response, hyper::Error> {
    let resp_body = serde_json::to_vec(&val).unwrap();
    let http_resp = http::Response::new().body(resp_body);
    finished(http_resp).boxed()
  }
}
