use std::io;
use hyper::Client;
use hyper::header::Host;
use nickel::{Request, Response, MiddlewareResult, Halt};
use reject::{bad_request, not_found, server_error};
use store::Store;

pub fn vhost<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  let uri = req.origin.uri.to_string();

  match req.origin.headers.get::<Host>() {
    None => bad_request(res),
    Some(host) => {
      let hostname = host.hostname.to_string();
      let method = req.origin.method.to_string().to_uppercase();

      println!("{} {}{}", method, hostname, uri);
      try_host(hostname, uri, res)
    }
  }
}

fn try_host<'app>(hostname: String, uri: String, res: Response<'app>) -> MiddlewareResult<'app> {
  let desk = Store::new("desk");

  match desk.shelf.get(&hostname) {
    Some(target) => try_serve(target, uri, res),
    None => not_found(res)
  }
}

fn try_serve<'app>(address: String, uri: String, mut res: Response<'app>) -> MiddlewareResult<'app> {
  let http = Client::new();
  let url = address + &uri;

  match http.get(&url).send() {
    Err(why) => server_error(res, &why.to_string(), "try_proxy request"),
    Ok(mut remote) => {
      *res.status_mut() = remote.status.clone();
      *res.headers_mut() = remote.headers.clone();

      let mut stream = try!(res.start());
      match io::copy(&mut remote, &mut stream) {
        Err(why) => stream.bail(format!("Stream error: {}", why)),
        Ok(_) => Ok(Halt(stream))
      }
    }
  }
}
