extern crate fiemap;
extern crate walkdir;
extern crate histo;

use std::env::args;
use std::path::Path;
use std::fmt::Display;
use std::io::Error;

use walkdir::WalkDir;
use histo::Histogram;

fn process_entry(histogram: &mut Histogram, entry: &walkdir::DirEntry) -> Result<(), Error> {
  if entry.file_type().is_file() {
    let mut count = 0;
    for fe in fiemap::fiemap(entry.path())? {
      fe?;
      count += 1;
    }
    histogram.add(count as u64);
  }
  Ok(())
}

fn process<P: AsRef<Path> + Display>(dir: P) {
  let mut histogram = Histogram::with_buckets(10);
  let mut errored = false;

  for entry in WalkDir::new(dir.as_ref()) {
    match entry {
      Ok(entry) => {
        if let Err(e) = process_entry(&mut histogram, &entry) {
          eprintln!("{}: Error {:?}", entry.path().display(), e);
          errored = true;
        }
      },
      Err(e) =>  eprintln!("{}: Error {:?}", dir, e),
    }
  }
  if !errored {
    println!("{}:\n{}\n", dir, histogram);
  }
}

fn main() {
  for f in args().skip(1) {
    process(f);
  }
}
