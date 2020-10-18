use ed2k::{Digest, Ed2k, Ed2kLegacy};
use std::env;

fn main() -> Result<(), std::io::Error> {
    for path in env::args().skip(1) {
        let ed2k = Ed2k::from_path(&path)?;
        println!("{}", ed2k);
    }
    Ok(())
}
