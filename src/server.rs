use hyper::server as http;
use hyper;
use tokio_service::Service;
use futures::{BoxFuture, Future, finished};
use ::{Error, Value, Request, RequestType};
use serde_json;

pub struct APIServer<T: Service<Request=Request, Response=Value, Error=Error>> {
  service: T,
}
impl <T: Service<Request=Request, Response=Value, Error=Error>> APIServer<T> {
  pub fn new(service: T) -> APIServer<T> {
    APIServer {
      service: service,
    }
  }
}


impl <T: Service<Request=Request, Response=Value, Error=Error, Future=BoxFuture<Value, Error>>> Service for APIServer<T> {
  type Request = http::Request;
  type Response = http::Response;
  type Error = hyper::Error;
  type Future = BoxFuture<Self::Response, hyper::Error>;

  fn call(&self, http_request: Self::Request) -> Self::Future {
    // TODO actually use value and error handler
    // let ref vh = self.value_handler;
    // let ref eh = DefaultErrorHandler{};

    // problem is need to move value contained within Server, but passing it to the future
    // means it can't get access
    let uri = http_request.uri();
    let method = http_request.method();
    let request = match process_http_request(method, uri, None) { // TODO ACTUALLY GET BODY
      Ok(req) => req,
      Err(err) => return DefaultErrorHandler{}.handle_http(err),
    };
    self.service.call(request).then(move |res| {
      match res {
        Ok(val) => DefaultValueHandler{}.handle_http(val),
        Err(err) => DefaultErrorHandler{}.handle_http(err),
      }
    }).boxed()
  }
}

fn process_http_request(method: &hyper::Method, path: &hyper::RequestUri, body: Option<&str>) -> Result<Request, Error> {
  let path_str = if let &hyper::RequestUri::AbsolutePath{ref path, ..} = path {
    path
  } else {
    return Err(Error {
      msg: "Invalid path, sorry".to_string(),
      kind: ::error::ErrorKind::RemoveThis
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
      kind: ::error::ErrorKind::RemoveThis
    }),
  };
  let req = Request {
    resource: resource_name.to_string(),
    request_type: req_type,
    params: ::Params::new(), // TODO PARSE PARAMS
    object: body_val,
    id: id_val,
  };
  if req.validate() {
    Ok(req)
  } else {
    Err(Error {
      msg: "Invalid request, missing either a body or id or something.".to_string(),
      kind: ::error::ErrorKind::RemoveThis
    })
  }
}

fn parse_json(json_str: &str) -> Result<Value, Error> {
  serde_json::from_str::<Value>(&json_str).map_err(|err| {
    Error {
      msg: err.to_string(),
      kind: ::error::ErrorKind::RemoveThis
    }
  })
}

fn parse_url(path: &str) -> Result<(&str, Option<&str>), Error> {
  let mut uri = path.split('/').skip(1);

  let resource_name = match uri.next() {
    None => return Err(Error {
      msg: "Invalid TODO somehow you got here".to_string(),
      kind: ::error::ErrorKind::RemoveThis
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
