ed2k-rs
=======
An [ed2k](https://en.wikipedia.org/wiki/Ed2k_URI_scheme) hash implementation in
Rust. It supports both current and legacy variants of calculating the ed2k hash.
It does not support AICH hashes.

This crate implements the [digest](https://github.com/RustCrypto/traits/tree/master/digest)
crate's traits for digest algorithms. This means that any changes to this trait
will affect the API of this crate.


Disclaimer
----------
* This is alpha software and the API may change at any time
* Please don't use this for illegal file sharing


Example
-------
How to calculate the hash of a file:

```rust
use std::io::Read;
use std::fs::File;
const BUFFER_SIZE: usize = 4096;

// Use ed2k::Ed2kHasher::with_legacy_hashing(true) for legacy variant
let mut hasher = ed2k::Ed2kHasher::new();
let mut file = File::open("/path/to/file")?;

let mut buf = [0u8; BUFFER_SIZE];
loop {
    let buf_len = file.read(&mut buf)?;
    hasher.update(&buf[..buf_len]);

    if buf_len == 0 {
        break
    }
}

println!("Hash is: {:?}", hasher.finalize());
```

Since file hashing is so common there is a convenience wrapper for this:

```rust
// Use Ed2kLegacy::from_path(...) for legacy variant
let ed2k = Ed2k::from_path("/path/to/file")?;

// The Display trait provides an ed2k link in the format
// ed2k://|file|<filename>|<filesize>|<hash>|/
println!("ed2k URL is {}", ed2k);
```
