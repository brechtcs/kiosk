use nickel::{Response, MiddlewareResult};

pub fn bad_request<'app>(res: Response<'app>) -> MiddlewareResult<'app> {
  res.send("Four'O'Zero")
}

pub fn not_found<'app>(res: Response<'app>) -> MiddlewareResult<'app> {
  res.send("Four'O'Four")
}

pub fn server_error<'app>(res: Response<'app>, why: &str, info: &str) -> MiddlewareResult<'app> {
  println!("500: {} ({})", why, info);
  res.send("Five'O'Zero")
}
