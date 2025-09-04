use std::env::args;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    for f in args().skip(1) {
        println!("{}:", f);
        let file = File::open(f)?;
        for fe in fiemap::Fiemap::new(&file) {
            println!("  {:?}", fe?);
        }
    }

    Ok(())
}
