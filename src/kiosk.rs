use nickel::{Request, Response, MiddlewareResult};
use reject::not_allowed;

pub fn get<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  if !req.origin.remote_addr.ip().is_loopback() {
    return not_allowed(res);
  }

  res.send(include_str!("screens/kiosk.html"))
}
