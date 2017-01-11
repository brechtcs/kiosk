extern crate hyper;
extern crate nickel;
extern crate shelf;

mod api;
mod kiosk;
mod pamphlet;
mod reject;
mod store;

use std::thread;
use nickel::{Nickel, HttpRouter};
use pamphlet::vhost;

fn main() {
  thread::spawn(|| {
    let mut config = Nickel::new();
    config.get("/", kiosk::get);
    config.get("/_pamphlets", api::get);
    config.delete("/_pamphlets", api::delete);
    config.post("/_pamphlets", api::post);
    config.listen("127.0.0.1:19588").expect("Config service bind failure");
  });

  let mut host = Nickel::new();
  host.utilize(vhost);
  host.keep_alive_timeout(None);
  host.listen("0.0.0.0:80").expect("Host service bind failure");
}

