use digest::generic_array::{ArrayLength, GenericArray};
use digest::Digest;

fn array_to_string<N: ArrayLength<u8>>(array: GenericArray<u8, N>) -> String {
    let mut out = String::new();
    for byte in array.as_slice() {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

fn hash(input: &[u8]) -> String {
    let mut hasher = ed2k::Ed2kHasher::new();
    hasher.update(input);
    array_to_string(hasher.finalize())
}

#[test]
fn test_single_chunk() {
    // We expect this to work like MD4
    assert_eq!(&hash(b""), "31d6cfe0d16ae931b73c59d7e0c089c0");
    assert_eq!(&hash(b"foobar"), "547aefd231dcbaac398625718336f143");
    assert_eq!(
        &hash(b"The quick brown fox jumps over the lazy dog"),
        "1bee69a46ba811185c194762abaeae90"
    );
}

#[test]
fn test_multi_chunk() {
    let mut zero_chunk: Vec<u8> = Vec::with_capacity(ed2k::Ed2kHasher::CHUNK_SIZE);
    zero_chunk.extend(std::iter::repeat(0).take(zero_chunk.capacity()));

    // Single chunk
    {
        let mut hasher = ed2k::Ed2kHasher::new();
        hasher.update(&zero_chunk[..]);
        let hash = hasher.finalize();
        assert_eq!(array_to_string(hash), "d7def262a127cd79096a108e7a9fc138");
    }

    // Double chunk
    {
        let mut hasher = ed2k::Ed2kHasher::new();
        hasher.update(&zero_chunk[..]);
        hasher.update(&zero_chunk[..]);
        let hash = hasher.finalize();
        assert_eq!(array_to_string(hash), "194ee9e4fa79b2ee9f8829284c466051");
    }
}

#[test]
fn test_multi_chunk_legacy() {
    let mut zero_chunk: Vec<u8> = Vec::with_capacity(ed2k::Ed2kHasher::CHUNK_SIZE);
    zero_chunk.extend(std::iter::repeat(0).take(zero_chunk.capacity()));

    // Single chunk
    {
        let mut hasher = ed2k::Ed2kHasher::with_legacy_hashing(true);
        hasher.update(&zero_chunk[..]);
        let hash = hasher.finalize();
        assert_eq!(array_to_string(hash), "fc21d9af828f92a8df64beac3357425d");
    }

    // Double chunk
    {
        let mut hasher = ed2k::Ed2kHasher::with_legacy_hashing(true);
        hasher.update(&zero_chunk[..]);
        hasher.update(&zero_chunk[..]);
        let hash = hasher.finalize();
        assert_eq!(array_to_string(hash), "114b21c63a74b6ca922291a11177dd5c");
    }
}
