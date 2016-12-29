use std::env;
use std::path::PathBuf;
use shelf::Shelf;

pub struct Store {
  pub shelf: Shelf
}

impl Store {
  pub fn new(name: &str) -> Store {
    let shelf: Shelf;

    if name == "network" {
      shelf = Shelf::new(&network_dir());
    }
    else {
      shelf = Shelf::new(&desk_dir());
    }

    Store {
      shelf: shelf
    }
  }
}

fn desk_dir() -> String {
  let mut path = match env::home_dir() {
    None => PathBuf::new(),
    Some(dir) => dir
  };
  path.push(".pm");

  path.to_str()
    .expect("env::home_dir produced invalid Unicode")
    .to_string()
}

fn network_dir() -> String {
  let mut path = env::temp_dir();
  path.push("pamphlets");
  path.push("network");

  path.to_str()
    .expect("env::temp_dir produced invalid Unicode")
    .to_string()
}
