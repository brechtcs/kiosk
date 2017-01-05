extern crate hyper;
extern crate nickel;
extern crate shelf;

mod api;
mod pamphlet;
mod reject;
mod store;

use nickel::{Nickel, HttpRouter};
use pamphlet::vhost;

fn main() {
  let mut routes = Nickel::router();
  routes.get("/_pamphlets", api::get);
  routes.post("/_pamphlets", api::post);

  let mut app = Nickel::new();
  app.utilize(routes);
  app.utilize(vhost);
  app.listen("0.0.0.0:80").expect("Server bind failure");
}

