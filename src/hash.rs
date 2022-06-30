use digest::generic_array::{typenum, GenericArray};
use digest::{Digest, FixedOutput, HashMarker, OutputSizeUser, Reset, Update};
use md4::Md4;

#[derive(Clone, Debug, Default)]
pub struct Ed2kHasher {
    use_legacy_hashing: bool,
    md4_hasher: Md4,
    current_chunk_len: usize,
    num_full_chunks: usize,
    chunk_hasher: Md4,
}

impl Ed2kHasher {
    pub const CHUNK_SIZE: usize = 9_728_000;

    pub fn new() -> Self {
        Self::with_legacy_hashing(false)
    }

    pub fn with_legacy_hashing(use_legacy_hashing: bool) -> Self {
        Self {
            use_legacy_hashing,
            md4_hasher: Default::default(),
            current_chunk_len: 0,
            num_full_chunks: 0,
            chunk_hasher: Default::default(),
        }
    }

    fn finalize_chunk(&mut self) {
        self.num_full_chunks += 1;
        Digest::update(
            &mut self.chunk_hasher,
            self.md4_hasher.finalize_reset().as_slice(),
        );
        self.current_chunk_len = 0;
    }

    fn chunk_remaining(&self) -> usize {
        Self::CHUNK_SIZE - self.current_chunk_len
    }
}

impl HashMarker for Ed2kHasher {}

impl Update for Ed2kHasher {
    fn update(&mut self, mut input: &[u8]) {
        while input.len() > 0 {
            let read_len = std::cmp::min(self.chunk_remaining(), input.len());

            if self.chunk_remaining() == 0 {
                self.finalize_chunk();
            }

            let input_slice = &input[..read_len];
            Digest::update(&mut self.md4_hasher, input_slice);
            self.current_chunk_len += read_len;

            // Legacy hashing means we always need to have a null chunk at the
            // end if the input is a multiple of CHUNK_SIZE
            if self.chunk_remaining() == 0 && self.use_legacy_hashing {
                self.finalize_chunk();
            }

            input = &input[read_len..];
        }
    }
}

impl Reset for Ed2kHasher {
    fn reset(&mut self) {
        *self = Self {
            use_legacy_hashing: self.use_legacy_hashing,
            ..Default::default()
        }
    }
}

impl OutputSizeUser for Ed2kHasher {
    type OutputSize = typenum::U16;
}

impl FixedOutput for Ed2kHasher {
    fn finalize_into(mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.num_full_chunks == 0 {
            FixedOutput::finalize_into(self.md4_hasher, out);
        } else {
            self.finalize_chunk();
            FixedOutput::finalize_into(self.chunk_hasher, out);
        }
        self.num_full_chunks = 0;
        self.current_chunk_len = 0;
    }
}
