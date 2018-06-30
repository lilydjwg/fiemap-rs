extern crate fiemap;

use std::io::Error;
use std::env::args;

fn main() -> Result<(), Error> {
  for f in args().skip(1) {
    println!("{}:", f);
    for fe in fiemap::fiemap(f)? {
      println!("  {:?}", fe?);
    }
  }

  Ok(())
}
