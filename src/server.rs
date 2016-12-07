use hyper::server as http;
use hyper;
use std::collections::HashMap;
use ::resource::ResourceWrapper;
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

fn process_http_request(method: &hyper::Method, path: &hyper::RequestUri, body: Option<&str>) -> Result<Request, Error> {
  let path_str = if let &hyper::RequestUri::AbsolutePath{path: ref path, ..} = path {
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

// #[derive(Clone)]
pub struct Server {
  resources: HashMap<String, Box<ResourceWrapper>>,
  error_handler: Box<ErrorHandler>,
  value_handler: Box<ValueHandler>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      resources: HashMap::new(),
      error_handler: Box::new(DefaultErrorHandler),
      value_handler: Box::new(DefaultValueHandler),
    }
  }

  pub fn resource<T: ResourceWrapper>(&mut self, name: &str, resource: T) {
    self.resources.insert(name.to_string(), Box::new(resource));
  }

  pub fn error<T: ErrorHandler>(&mut self, handler: T) {
    self.error_handler = Box::new(handler);
  }
}

impl Service for Server {
    type Request = http::Request;
    type Response = http::Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Self::Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
      // let body_string = match String::from_utf8(req.body().collect()) {
      //   Ok(val) => val,
      //   Err(_) => return self.error_handler.handle_http(Error {
      //   msg: "Invalid utf8 in request body.".to_string(),
      //   kind: ::error::ErrorKind::RemoveThis
      // }),
      // };
      let mut uri = if let Some(path) = req.path() {
        path.split('/').skip(1)
      } else {
        return self.error_handler.handle_http(Error {
          msg: "Invalid request path.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        });
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = if let Some(resource_id) = uri.next() {
        if resource_id.is_empty() {
          None
        } else {
          match serde_json::from_str::<Value>(&resource_id) {
            Ok(val) => Some(val),
            Err(_) => return self.error_handler.handle_http(Error {
              msg: "Invalid JSON in id field".to_string(),
              kind: ::error::ErrorKind::RemoveThis
            }),
          }
        }
      } else {
        None
      };

      let body_val = None; // if body_string.is_empty() {
      //   None
      // } else {
      //   Some(match serde_json::from_str::<Value>(&body_string) {
      //     Ok(val) => val,
      //     Err(_) => return self.error_handler.handle_http(Error {
      //   msg: "Invalid JSON in request body".to_string(),
      //   kind: ::error::ErrorKind::RemoveThis
      // }),
      //   })
      // };

      let resource = match self.resources.get(resource_name) {
          Some(resource) => resource,
          None => return self.error_handler.handle_http(Error {
            msg: "No resource with that name known.".to_string(),
            kind: ::error::ErrorKind::RemoveThis
          }),
      };

      let params = ::Params::new(); // TODO ACTUALLY PARSE

      let request_type = match req.method() {
        // TODO should we handle HEAD requests?
        &hyper::Method::Get => {
          if resource_id != None {
            RequestType::Get
          } else {
            RequestType::Find
          }
        }
        &hyper::Method::Post => RequestType::Create,
        &hyper::Method::Put => RequestType::Update,
        &hyper::Method::Patch => RequestType::Patch,
        &hyper::Method::Delete => RequestType::Remove,
        _ => return self.error_handler.handle_http(Error {
          msg: "We don't respond to that HTTP method, sorry.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        }),
      };

      let req = Request{
        request_type: request_type,
        params: params,
        id: resource_id,
        object: body_val,
      };

      if !req.validate() {
        return self.error_handler.handle_http(Error {
          msg: "Invalid request, missing either a body or id or something.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        })
      }

      // TODO actually use value and error handler
      // let ref vh = self.value_handler;
      // let ref eh = self.error_handler;

      // problem is need to move value contained within Server, but passing it to the future
      // means it can't get access
      resource.handle(req).then(move |res| {
        match res {
          Ok(val) => DefaultValueHandler{}.handle_http(val),
          Err(err) => DefaultErrorHandler{}.handle_http(err),
        }
      }).boxed()
    }
}

