extern crate hyper;
extern crate nickel;
extern crate shelf;

mod api;
mod pamphlet;
mod reject;
mod store;

use nickel::{Nickel, HttpRouter};

fn main() {
  let mut app = Nickel::new();
  app.post("/_pamphlets", api::post);
  app.get("/_pamphlets", api::get);
  app.get("**", pamphlet::serve);
  app.listen("0.0.0.0:80").expect("Server bind failure");
}

