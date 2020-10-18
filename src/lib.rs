use digest::generic_array::{typenum, GenericArray};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

mod hash;

pub use digest::Digest;
pub use hash::Ed2kHasher;

const BUFFER_SIZE: usize = 8192;

#[derive(Debug, Eq, PartialEq)]
pub struct Ed2k {
    filename: OsString,
    size: u64,
    hash: GenericArray<u8, typenum::U16>,
}

pub struct Ed2kLegacy;

fn hash_from_path<P: AsRef<Path>>(path: P, use_legacy_hashing: bool) -> IoResult<Ed2k> {
    let path = path.as_ref();
    let mut hasher = Ed2kHasher::with_legacy_hashing(use_legacy_hashing);
    let mut file = File::open(path)?;

    let mut buf = [0u8; BUFFER_SIZE];
    loop {
        let buf_len = file.read(&mut buf)?;
        hasher.update(&buf[..buf_len]);
        if buf_len == 0 {
            break;
        }
    }

    Ok(Ed2k {
        // Unwrap is safe since we have opened the file
        filename: path.file_name().unwrap().to_os_string(),
        size: path.metadata()?.len(),
        hash: hasher.finalize(),
    })
}

impl Ed2kLegacy {
    pub fn from_path<P: AsRef<Path>>(path: P) -> IoResult<Ed2k> {
        hash_from_path(path, true)
    }
}

impl Ed2k {
    pub fn from_path<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        hash_from_path(path, false)
    }

    pub fn filename(&self) -> &OsStr {
        self.filename.as_os_str()
    }

    pub fn filesize(&self) -> u64 {
        self.size
    }

    pub fn hash(&self) -> &[u8] {
        self.hash.as_slice()
    }
}

impl fmt::Display for Ed2k {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let escaped_filename: String = self
            .filename
            .as_os_str()
            .to_string_lossy()
            .chars()
            .map(|c| {
                let escape = !c.is_ascii() || c == '|';
                let mut buf = [0u8; 6];
                let s = c.encode_utf8(&mut buf);

                if escape {
                    s.bytes().map(|byte| format!("%{:02x}", byte)).collect()
                } else {
                    s.to_string()
                }
            })
            .collect();
        write!(f, "ed2k://|file|{}|{}|", escaped_filename, self.size)?;
        for byte in self.hash.as_slice() {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, "|/")
    }
}
