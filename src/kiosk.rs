use nickel::{Request, Response, MiddlewareResult};

pub fn get<'app>(_req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  if !req.origin.remote_addr.ip().is_loopback() {
    return not_allowed(res);
  }

  res.send(include_str!("views/kiosk.html"))
}
