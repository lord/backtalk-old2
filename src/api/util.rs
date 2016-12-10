use hyper;
use futures::future::{BoxFuture, finished, failed, Future};
use futures::stream::Stream;
use serde_json;
use serde::{Serialize, Deserialize};
use ::api::{Value, Error, ErrorKind, Request, RequestData, Params};

trait ErrorHandler: 'static + Sync + Send {
  fn handle_http(&self, Error) -> BoxFuture<hyper::server::Response, hyper::Error>;
}

trait ValueHandler: 'static + Sync + Send {
  fn handle_http(&self, Value) -> BoxFuture<hyper::server::Response, hyper::Error>;
}

struct DefaultErrorHandler;
impl ErrorHandler for DefaultErrorHandler {
  fn handle_http(&self, err: Error) -> BoxFuture<hyper::server::Response, hyper::Error> {
    let resp = hyper::server::Response::new().with_body(err.msg.into_bytes());
    finished(resp).boxed()
  }
}

struct DefaultValueHandler;
impl ValueHandler for DefaultValueHandler {
  fn handle_http(&self, val: Value) -> BoxFuture<hyper::server::Response, hyper::Error> {
    let resp_body = serde_json::to_vec(&val).unwrap();
    let http_resp = hyper::server::Response::new().with_body(resp_body);
    finished(http_resp).boxed()
  }
}

pub fn serialize<T, U, F>(val: Value, func: F) -> BoxFuture<Value, Error>
  where F: FnOnce(T) -> BoxFuture<U, Error>,
  T: Deserialize,
  U: Serialize + 'static
{
  match serde_json::from_value(val) {
    Ok(input) => {
      func(input).map(|res| {
        serde_json::to_value(res)
      }).boxed()
    },
    Err(_) => failed(Error{
      kind: ErrorKind::InvalidRequest,
      msg: "Failed to serialize TODO.".to_string()
    }).boxed(),
  }
}

pub fn wrap_api<T>(http_request: hyper::server::Request, api_func: &T) -> BoxFuture<hyper::server::Response, hyper::Error>
  where T: Fn(Request) -> BoxFuture<Value, Error> + 'static {
  let uri = http_request.uri();
  let method = http_request.method();
  // http_request.body().collect().map(|bod| {println!("{:?}", bod);});
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
  fn make_err(msg: &str) -> Result<Request, Error> {
    Err(Error{msg: msg.to_string(), kind: ErrorKind::InvalidRequest})
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

  let path_str = if let &hyper::RequestUri::AbsolutePath{ref path, ..} = path {
    path
  } else {
    return make_err("Invalid path, sorry");
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
  let data = match (method, id_val, body_val) {
    // TODO should we handle HEAD requests?
    (&hyper::Method::Get, Some(v), None) => RequestData::Get(v),
    (&hyper::Method::Get, None, None) => RequestData::Find,
    (&hyper::Method::Post, None, Some(obj)) => RequestData::Create(obj),
    (&hyper::Method::Put, Some(id), Some(obj)) => RequestData::Update(id, obj),
    (&hyper::Method::Patch, Some(id), Some(obj)) => RequestData::Patch(id, obj),
    (&hyper::Method::Delete, Some(id), None) => RequestData::Remove(id),
    (&hyper::Method::Get, _, Some(_)) =>
      return make_err("GET methods don't allow request bodies, but one was provided."),
    (&hyper::Method::Post, Some(_), _) =>
      return make_err("POST methods don't accept a resource id, but one was provided."),
    (&hyper::Method::Post, _, None) =>
      return make_err("POST methods require a request body."),
    (&hyper::Method::Put, _, _) =>
      return make_err("PUT methods require a request body and a resource id."),
    (&hyper::Method::Delete, _, Some(_)) =>
      return make_err("Delete methods don't allow request bodies, but one was provided."),
    (&hyper::Method::Delete, _, _) =>
      return make_err("Delete methods require a resource id."),
    (&hyper::Method::Patch, _, _) =>
      return make_err("PATCH methods require a request body and a resource id."),
    _ => return make_err("We don't respond to that HTTP method, sorry."),
  };
  Ok(Request {
    resource: resource_name.to_string(),
    data: data,
    params: Params::new(), // TODO PARSE PARAMS
  })
}

