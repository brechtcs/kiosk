use std::io::Read;

use hyper::Client;
use hyper::header::Host;
use nickel::{Request, Response, MiddlewareResult};
use reject::{bad_request, not_found, server_error};
use store::Store;

pub fn serve<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  let uri = req.origin.uri.to_string();

  match req.origin.headers.get::<Host>() {
    None => bad_request(res),
    Some(host) => try_host(host.hostname.to_string(), uri, res)
  }
}

fn try_host<'app>(hostname: String, uri: String, res: Response<'app>) -> MiddlewareResult<'app> {
  let desk = Store::new("desk");

  match desk.shelf.get(&hostname) {
    Some(target) => try_proxy(target, uri, res),
    None => not_found(res)
  }
}

fn try_proxy<'app>(address: String, uri: String, mut res: Response<'app>) -> MiddlewareResult<'app> {
  let http = Client::new();
  let url = address + &uri;

  match http.get(&url).send() {
    Err(why) => server_error(res, &why.to_string(), "try_proxy request"),
    Ok(mut remote) => {
      let mut body = String::new();

      *res.status_mut() = remote.status.clone();
      *res.headers_mut() = remote.headers.clone();

      match remote.read_to_string(&mut body) {
        Err(why) => server_error(res, &why.to_string(), "try_proxy reader"),
        Ok(_) => res.send(body)
      }
    }
  }
}
